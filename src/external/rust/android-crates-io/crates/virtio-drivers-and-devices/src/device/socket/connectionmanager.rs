use super::{
    protocol::VsockAddr, vsock::ConnectionInfo, DisconnectReason, SocketError, VirtIOSocket,
    VirtIOSocketDevice, VirtIOSocketManager, VsockEvent, VsockEventType, DEFAULT_RX_BUFFER_SIZE,
};
use crate::{
    transport::{DeviceTransport, InterruptStatus, Transport},
    DeviceHal, Hal, Lock, LockFactory, Result,
};
use alloc::{boxed::Box, sync::Arc, vec::Vec};
use core::cmp::min;
use core::convert::TryInto;
use core::hint::spin_loop;
use log::debug;
use zerocopy::FromZeros;

const DEFAULT_PER_CONNECTION_BUFFER_CAPACITY: u32 = 1024;

/// A higher level interface for VirtIO socket (vsock) drivers.
///
/// This keeps track of multiple vsock connections.
///
/// `RX_BUFFER_SIZE` is the size in bytes of each buffer used in the RX virtqueue. This must be
/// bigger than `size_of::<VirtioVsockHdr>()`.
///
/// # Example
///
/// ```
/// # use virtio_drivers_and_devices::{Error, Hal, LockFactory};
/// # use virtio_drivers_and_devices::transport::Transport;
/// use virtio_drivers_and_devices::device::socket::{VirtIOSocket, VsockAddr, VsockConnectionManager};
///
/// # fn example<HalImpl: Hal, T: Transport, L: LockFactory>(transport: T) -> Result<(), Error> {
/// let mut socket = VsockConnectionManager::new(VirtIOSocket::<HalImpl, _, L>::new(transport)?);
///
/// // Start a thread to call `socket.poll()` and handle events.
///
/// let remote_address = VsockAddr { cid: 2, port: 42 };
/// let local_port = 1234;
/// socket.connect(remote_address, local_port)?;
///
/// // Wait until `socket.poll()` returns an event indicating that the socket is connected.
///
/// socket.send(remote_address, local_port, "Hello world".as_bytes())?;
///
/// socket.shutdown(remote_address, local_port)?;
/// # Ok(())
/// # }
/// ```
pub struct VsockConnectionManager<
    H: Hal,
    T: Transport,
    L: LockFactory,
    const RX_BUFFER_SIZE: usize = DEFAULT_RX_BUFFER_SIZE,
>(VsockConnectionManagerCommon<VirtIOSocket<H, T, L, RX_BUFFER_SIZE>, L>);

/// A high level interface for VirtIO socket (vsock) devices.
pub struct VsockDeviceConnectionManager<H: DeviceHal, T: DeviceTransport, L: LockFactory>(
    VsockConnectionManagerCommon<VirtIOSocketDevice<H, T, L>, L>,
);

/// A trait defining shared behavior for VirtIO socket devices and drivers.
///
/// All methods are implemented for VsockConnectionManager and VsockDeviceConnectionManager though
/// the device side must not call the connect method. These are equivalent to the inherent methods
/// which are kept for backwards compatibility.
pub trait VsockManager: Send + Sync {
    /// Accepts an incoming connection, registering it and acknowledging it to the peer.
    fn accept(&self, c: Connection) -> Result;
    /// Sends a request to connect to the given destination on the driver side.
    ///
    /// This returns as soon as the request is sent; you should wait until `poll` returns a
    /// `VsockEventType::Connected` event indicating that the peer has accepted the connection
    /// before sending data. This panics if called from the device side.
    fn connect(&self, destination: VsockAddr, src_port: u32) -> Result;

    /// Sends the buffer to the destination.
    fn send(&self, dest: VsockAddr, src_port: u32, buffer: &[u8]) -> Result;

    /// Sends a credit update to the given peer.
    fn update_credit(&self, peer: VsockAddr, src_port: u32) -> Result;

    /// Forcibly closes the connection without waiting for the peer.
    fn force_close(&self, dest: VsockAddr, src_port: u32) -> Result;

    /// Allows incoming connections on the given port number.
    fn listen(&self, port: u32);

    /// Polls the vsock device to receive data or other updates.
    fn poll(&self) -> Result<Option<VsockEvent>>;

    /// Polls the vsock device, surfacing events directly to the caller without automatically
    /// accepting or rejecting connections.
    ///
    /// Unlike [`poll`](Self::poll), an incoming `ConnectionRequest` is returned with its
    /// `new_connection` populated so the caller can decide whether to [`accept`](Self::accept) it.
    fn poll_direct(&self) -> Result<Option<VsockEvent>>;

    /// Returns the local CID, i.e. the CID of the guest on the driver side and the CID of the host
    /// on the device side.
    fn local_cid(&self) -> u64;

    /// Requests to shut down the connection cleanly, telling the peer that we won't send or receive
    /// any more data.
    ///
    /// This returns as soon as the request is sent; you should wait until `poll` returns a
    /// `VsockEventType::Disconnected` event if you want to know that the peer has acknowledged the
    /// shutdown.
    fn shutdown(&self, dest: VsockAddr, src_port: u32) -> Result;

    /// Reads data received from the given connection.
    fn recv(&self, peer: VsockAddr, src_port: u32, buffer: &mut [u8]) -> Result<usize>;

    /// Returns the number of bytes in the receive buffer available to be read by `recv`.
    ///
    /// When the available bytes is 0, it indicates that the receive buffer is empty and does not
    /// contain any data.
    fn recv_buffer_available_bytes(&self, peer: VsockAddr, src_port: u32) -> Result<usize>;

    /// Acknowledges an interrupt from the device.
    ///
    /// This is useful when you cannot soundly get a mutable reference to the VsockManager impl, such
    /// as from an interrupt handler. It may only be called on the driver side as it will panic if
    /// called on the device side.
    ///
    /// # Safety
    ///
    /// The implementation may acknowledge the interrupt through a raw pointer derived from `&self`,
    /// mutating the underlying transport. The caller must ensure that no other references to the
    /// driver or its transport are concurrently active for the duration of this call.
    unsafe fn ack_interrupt(&self) -> InterruptStatus;
}

struct VsockConnectionManagerInner<L: LockFactory> {
    per_connection_buffer_capacity: u32,
    connections: Vec<Arc<L::Lock<Connection>>>,
    listening_ports: Vec<u32>,
}

struct VsockConnectionManagerCommon<M: VirtIOSocketManager<L>, L: LockFactory> {
    driver: M,
    inner: L::Lock<VsockConnectionManagerInner<L>>,
}

/// The state of a single vsock connection managed by a [`VsockConnectionManager`].
#[derive(Debug)]
pub struct Connection {
    pub(crate) info: ConnectionInfo,
    buffer: RingBuffer,
    /// The peer sent a SHUTDOWN request, but we haven't yet responded with a RST because there is
    /// still data in the buffer.
    peer_requested_shutdown: bool,
}

impl Connection {
    fn new(peer: VsockAddr, local_port: u32, buffer_capacity: u32) -> Self {
        let mut info = ConnectionInfo::new(peer, local_port);
        info.buf_alloc = buffer_capacity;
        Self {
            info,
            buffer: RingBuffer::new(buffer_capacity.try_into().unwrap()),
            peer_requested_shutdown: false,
        }
    }
}

impl<H: Hal, T: Transport, L: LockFactory, const RX_BUFFER_SIZE: usize>
    VsockConnectionManager<H, T, L, RX_BUFFER_SIZE>
{
    /// Construct a new connection manager wrapping the given low-level VirtIO socket driver.
    pub fn new(driver: VirtIOSocket<H, T, L, RX_BUFFER_SIZE>) -> Self {
        Self::new_with_capacity(driver, DEFAULT_PER_CONNECTION_BUFFER_CAPACITY)
    }

    /// Construct a new connection manager wrapping the given low-level VirtIO socket driver, with
    /// the given per-connection buffer capacity.
    pub fn new_with_capacity(
        driver: VirtIOSocket<H, T, L, RX_BUFFER_SIZE>,
        per_connection_buffer_capacity: u32,
    ) -> Self {
        Self(VsockConnectionManagerCommon {
            driver,
            inner: L::Lock::new(VsockConnectionManagerInner {
                connections: Vec::new(),
                listening_ports: Vec::new(),
                per_connection_buffer_capacity,
            }),
        })
    }

    /// Returns the CID which has been assigned to this guest.
    pub fn guest_cid(&self) -> u64 {
        self.0.local_cid()
    }

    /// Accepts an incoming connection, registering it and acknowledging it to the peer.
    pub fn accept(&self, c: Connection) -> Result {
        let info = c.info.clone();
        let new_connection = Arc::new(L::Lock::new(c));
        // Adding the connection to the list before we call `accept` prevents a
        // potential race where the peer could send follow-up packets before the
        // connection gets added to the list.
        self.0.inner.lock().connections.push(new_connection.clone());
        if let Err(e) = self.0.driver.accept(info) {
            // We unlocked inner so connections could have changed. Remove newly
            // inserted connection via lookup rather than by possibly-stale idx.
            // TODO: Use slotmap or similar to avoid inefficient, linear search.
            self.0
                .inner
                .lock()
                .connections
                .retain(|c| !Arc::ptr_eq(c, &new_connection));
            return Err(e);
        }
        Ok(())
    }
    /// Sends a request to connect to the given destination.
    ///
    /// This returns as soon as the request is sent; you should wait until `poll` returns a
    /// `VsockEventType::Connected` event indicating that the peer has accepted the connection
    /// before sending data.
    pub fn connect(&self, destination: VsockAddr, src_port: u32) -> Result {
        let mut inner = self.0.inner.lock();
        if inner.connections.iter().any(|connection| {
            let connection = connection.lock();
            connection.info.dst == destination && connection.info.src_port == src_port
        }) {
            return Err(SocketError::ConnectionExists.into());
        }

        let new_connection =
            Connection::new(destination, src_port, inner.per_connection_buffer_capacity);
        let new_connection = Arc::new(L::Lock::new(new_connection));
        inner.connections.push(new_connection.clone());
        drop(inner);

        if let Err(e) = self.0.driver.connect(new_connection.clone()) {
            // We unlocked inner so connections could have changed. Remove newly
            // inserted connection via lookup rather than by possibly-stale idx.
            // TODO: Use slotmap or similar to avoid inefficient, linear search.
            self.0
                .inner
                .lock()
                .connections
                .retain(|c| !Arc::ptr_eq(c, &new_connection));
            return Err(e);
        }
        debug!("Connection requested: {:?}", new_connection.lock().info);
        Ok(())
    }
    /// Allows incoming connections on the given port number.
    pub fn listen(&self, port: u32) {
        self.0.listen(port)
    }

    /// Stops allowing incoming connections on the given port number.
    pub fn unlisten(&self, port: u32) {
        self.0.unlisten(port)
    }

    /// Sends the buffer to the destination.
    pub fn send(&self, destination: VsockAddr, src_port: u32, buffer: &[u8]) -> Result {
        self.0.send(destination, src_port, buffer)
    }

    /// Polls the vsock device to receive data or other updates.
    pub fn poll(&self) -> Result<Option<VsockEvent>> {
        self.0.poll()
    }

    /// Polls the vsock device, surfacing events directly to the caller without automatically
    /// accepting or rejecting connections. See [`VsockManager::poll_direct`].
    pub fn poll_direct(&self) -> Result<Option<VsockEvent>> {
        self.0.poll_direct()
    }

    /// Reads data received from the given connection.
    pub fn recv(&self, peer: VsockAddr, src_port: u32, buffer: &mut [u8]) -> Result<usize> {
        self.0.recv(peer, src_port, buffer)
    }

    /// Returns the number of bytes in the receive buffer available to be read by `recv`.
    ///
    /// When the available bytes is 0, it indicates that the receive buffer is empty and does not
    /// contain any data.
    pub fn recv_buffer_available_bytes(&self, peer: VsockAddr, src_port: u32) -> Result<usize> {
        self.0.recv_buffer_available_bytes(peer, src_port)
    }

    /// Sends a credit update to the given peer.
    pub fn update_credit(&self, peer: VsockAddr, src_port: u32) -> Result {
        self.0.update_credit(peer, src_port)
    }

    /// Blocks until we get some event from the vsock device.
    pub fn wait_for_event(&self) -> Result<VsockEvent> {
        self.0.wait_for_event()
    }

    /// Requests to shut down the connection cleanly, telling the peer that we won't send or receive
    /// any more data.
    ///
    /// This returns as soon as the request is sent; you should wait until `poll` returns a
    /// `VsockEventType::Disconnected` event if you want to know that the peer has acknowledged the
    /// shutdown.
    pub fn shutdown(&self, destination: VsockAddr, src_port: u32) -> Result {
        self.0.shutdown(destination, src_port)
    }

    /// Forcibly closes the connection without waiting for the peer.
    pub fn force_close(&self, destination: VsockAddr, src_port: u32) -> Result {
        self.0.force_close(destination, src_port)
    }
}

impl<H: DeviceHal, T: DeviceTransport, L: LockFactory> VsockDeviceConnectionManager<H, T, L> {
    /// Construct a new connection manager wrapping the given low-level VirtIO socket driver.
    pub fn new(driver: VirtIOSocketDevice<H, T, L>) -> Self {
        Self::new_with_capacity(driver, DEFAULT_PER_CONNECTION_BUFFER_CAPACITY)
    }

    /// Construct a new connection manager wrapping the given low-level VirtIO socket driver, with
    /// the given per-connection buffer capacity.
    pub fn new_with_capacity(
        driver: VirtIOSocketDevice<H, T, L>,
        per_connection_buffer_capacity: u32,
    ) -> Self {
        Self(VsockConnectionManagerCommon {
            driver,
            inner: L::Lock::new(VsockConnectionManagerInner {
                connections: Vec::new(),
                listening_ports: Vec::new(),
                per_connection_buffer_capacity,
            }),
        })
    }

    /// Allows incoming connections on the given port number.
    pub fn listen(&self, port: u32) {
        self.0.listen(port)
    }

    /// Stops allowing incoming connections on the given port number.
    pub fn unlisten(&self, port: u32) {
        self.0.unlisten(port)
    }

    /// Accepts an incoming connection, registering it and acknowledging it to the peer.
    pub fn accept(&self, c: Connection) -> Result {
        let info = c.info.clone();
        let new_connection = Arc::new(L::Lock::new(c));
        // Adding the connection to the list before we call `accept` prevents a
        // potential race where the peer could send follow-up packets before the
        // connection gets added to the list.
        self.0.inner.lock().connections.push(new_connection.clone());
        if let Err(e) = <_ as VirtIOSocketManager<L>>::accept(&self.0.driver, info) {
            // We unlocked inner so connections could have changed. Remove newly
            // inserted connection via lookup rather than by possibly-stale idx.
            self.0
                .inner
                .lock()
                .connections
                .retain(|c| !Arc::ptr_eq(c, &new_connection));
            return Err(e);
        }
        Ok(())
    }

    /// Sends the buffer to the destination.
    pub fn send(&self, destination: VsockAddr, src_port: u32, buffer: &[u8]) -> Result {
        self.0.send(destination, src_port, buffer)
    }

    /// Polls the vsock device to receive data or other updates.
    pub fn poll(&self) -> Result<Option<VsockEvent>> {
        self.0.poll()
    }

    /// Polls the vsock device, surfacing events directly to the caller without automatically
    /// accepting or rejecting connections. See [`VsockManager::poll_direct`].
    pub fn poll_direct(&self) -> Result<Option<VsockEvent>> {
        self.0.poll_direct()
    }

    /// Reads data received from the given connection.
    pub fn recv(&self, peer: VsockAddr, src_port: u32, buffer: &mut [u8]) -> Result<usize> {
        self.0.recv(peer, src_port, buffer)
    }

    /// Returns the number of bytes in the receive buffer available to be read by `recv`.
    ///
    /// When the available bytes is 0, it indicates that the receive buffer is empty and does not
    /// contain any data.
    pub fn recv_buffer_available_bytes(&self, peer: VsockAddr, src_port: u32) -> Result<usize> {
        self.0.recv_buffer_available_bytes(peer, src_port)
    }

    /// Sends a credit update to the given peer.
    pub fn update_credit(&self, peer: VsockAddr, src_port: u32) -> Result {
        self.0.update_credit(peer, src_port)
    }

    /// Blocks until we get some event from the vsock device.
    pub fn wait_for_event(&self) -> Result<VsockEvent> {
        self.0.wait_for_event()
    }

    /// Requests to shut down the connection cleanly, telling the peer that we won't send or receive
    /// any more data.
    ///
    /// This returns as soon as the request is sent; you should wait until `poll` returns a
    /// `VsockEventType::Disconnected` event if you want to know that the peer has acknowledged the
    /// shutdown.
    pub fn shutdown(&self, destination: VsockAddr, src_port: u32) -> Result {
        self.0.shutdown(destination, src_port)
    }

    /// Forcibly closes the connection without waiting for the peer.
    pub fn force_close(&self, destination: VsockAddr, src_port: u32) -> Result {
        self.0.force_close(destination, src_port)
    }
}

impl<H: Hal, T: Transport, L: LockFactory, const RX_BUFFER_SIZE: usize> VsockManager
    for VsockConnectionManager<H, T, L, RX_BUFFER_SIZE>
{
    fn accept(&self, c: Connection) -> Result {
        Self::accept(self, c)
    }
    fn connect(&self, destination: VsockAddr, src_port: u32) -> Result {
        Self::connect(self, destination, src_port)
    }
    fn send(&self, dest: VsockAddr, src_port: u32, buffer: &[u8]) -> Result {
        Self::send(self, dest, src_port, buffer)
    }
    fn update_credit(&self, peer: VsockAddr, src_port: u32) -> Result {
        Self::update_credit(self, peer, src_port)
    }
    fn force_close(&self, dest: VsockAddr, src_port: u32) -> Result {
        Self::force_close(self, dest, src_port)
    }
    fn listen(&self, port: u32) {
        Self::listen(self, port)
    }
    fn poll(&self) -> Result<Option<VsockEvent>> {
        Self::poll(self)
    }
    fn poll_direct(&self) -> Result<Option<VsockEvent>> {
        Self::poll_direct(self)
    }
    fn local_cid(&self) -> u64 {
        self.0.local_cid()
    }
    fn shutdown(&self, dest: VsockAddr, src_port: u32) -> Result {
        Self::shutdown(self, dest, src_port)
    }
    fn recv(&self, peer: VsockAddr, src_port: u32, buffer: &mut [u8]) -> Result<usize> {
        Self::recv(self, peer, src_port, buffer)
    }
    fn recv_buffer_available_bytes(&self, peer: VsockAddr, src_port: u32) -> Result<usize> {
        Self::recv_buffer_available_bytes(self, peer, src_port)
    }
    unsafe fn ack_interrupt(&self) -> InterruptStatus {
        let vsock_driver_ptr = (&raw const self.0.driver).cast_mut();
        // SAFETY: This function's safety requirements ensure that `self` points to a valid
        // VsockConnectionManager so this gives a valid pointer to the field.
        unsafe { VirtIOSocket::<H, T, L, RX_BUFFER_SIZE>::ack_interrupt(vsock_driver_ptr) }
    }
}

impl<H: DeviceHal, T: DeviceTransport, L: LockFactory> VsockManager
    for VsockDeviceConnectionManager<H, T, L>
{
    fn accept(&self, c: Connection) -> Result {
        Self::accept(self, c)
    }
    fn connect(&self, _destination: VsockAddr, _src_port: u32) -> Result {
        unreachable!("vsock devices should not make outgoing connections")
    }
    fn send(&self, dest: VsockAddr, src_port: u32, buffer: &[u8]) -> Result {
        Self::send(self, dest, src_port, buffer)
    }
    fn update_credit(&self, peer: VsockAddr, src_port: u32) -> Result {
        Self::update_credit(self, peer, src_port)
    }
    fn force_close(&self, dest: VsockAddr, src_port: u32) -> Result {
        Self::force_close(self, dest, src_port)
    }
    fn listen(&self, port: u32) {
        Self::listen(self, port)
    }
    fn poll(&self) -> Result<Option<VsockEvent>> {
        Self::poll(self)
    }
    fn poll_direct(&self) -> Result<Option<VsockEvent>> {
        Self::poll_direct(self)
    }
    fn local_cid(&self) -> u64 {
        self.0.local_cid()
    }
    fn shutdown(&self, dest: VsockAddr, src_port: u32) -> Result {
        Self::shutdown(self, dest, src_port)
    }
    fn recv(&self, peer: VsockAddr, src_port: u32, buffer: &mut [u8]) -> Result<usize> {
        Self::recv(self, peer, src_port, buffer)
    }
    fn recv_buffer_available_bytes(&self, peer: VsockAddr, src_port: u32) -> Result<usize> {
        Self::recv_buffer_available_bytes(self, peer, src_port)
    }
    unsafe fn ack_interrupt(&self) -> InterruptStatus {
        panic!("vsock devices cannot acknowledge interrupts")
    }
}

impl<M: VirtIOSocketManager<L>, L: LockFactory> VsockConnectionManagerCommon<M, L> {
    /// Allows incoming connections on the given port number.
    pub fn listen(&self, port: u32) {
        let mut inner = self.inner.lock();
        if !inner.listening_ports.contains(&port) {
            inner.listening_ports.push(port);
        }
    }

    /// Stops allowing incoming connections on the given port number.
    pub fn unlisten(&self, port: u32) {
        self.inner.lock().listening_ports.retain(|p| *p != port);
    }

    /// Sends the buffer to the destination.
    pub fn send(&self, destination: VsockAddr, src_port: u32, buffer: &[u8]) -> Result {
        let inner = self.inner.lock();
        let (_, connection) = get_connection::<L>(&inner.connections, destination, src_port)?;
        drop(inner);

        self.driver.send(buffer, connection)
    }

    pub fn poll(&self) -> Result<Option<VsockEvent>> {
        let mut inner = self.inner.lock();
        let local_cid = self.driver.local_cid();
        let per_connection_buffer_capacity = inner.per_connection_buffer_capacity;

        let result = self.driver.poll(|event, body| {
            let connection = get_connection_for_event::<L>(&inner.connections, &event, local_cid);

            // Skip events which don't match any connection we know about, unless they are a
            // connection request.
            let connection = if let Some((_, connection)) = connection {
                connection
            } else if let VsockEventType::ConnectionRequest = event.event_type {
                // If the requested connection already exists or the CID isn't ours, ignore it.
                if connection.is_some() || event.destination.cid != local_cid {
                    return Ok(None);
                }
                // Add the new connection to our list, at least for now. It will be removed again
                // below if we weren't listening on the port.
                let new_connection = Arc::new(L::Lock::new(Connection::new(
                    event.source,
                    event.destination.port,
                    per_connection_buffer_capacity,
                )));
                inner.connections.push(new_connection.clone());
                new_connection
            } else {
                return Ok(None);
            };

            let mut connection = connection.lock();
            // Update stored connection info.
            connection.info.update_for_event(&event);

            if let VsockEventType::Received { length } = event.event_type {
                // Copy to buffer
                if !connection.buffer.add(body) {
                    return Err(SocketError::OutputBufferTooShort(length).into());
                }
            }

            Ok(Some(event))
        })?;

        let Some(event) = result else {
            return Ok(None);
        };
        // The connection must exist because we found it above in the callback.
        let (connection_index, connection) =
            get_connection_for_event::<L>(&inner.connections, &event, local_cid).unwrap();

        match event.event_type {
            VsockEventType::ConnectionRequest => {
                if inner.listening_ports.contains(&event.destination.port) {
                    drop(inner);
                    self.driver.accept(connection.lock().info.clone())?;
                } else {
                    inner.connections.swap_remove(connection_index);
                    drop(inner);

                    // Reject the connection request and remove it from our list.
                    self.driver.force_close(connection)?;

                    // No need to pass the request on to the client, as we've already rejected it.
                    return Ok(None);
                }
            }
            VsockEventType::Connected => {}
            VsockEventType::Disconnected { reason } => {
                // Wait until client reads all data before removing connection.
                let mut connection_guard = connection.lock();
                if connection_guard.buffer.is_empty() {
                    inner.connections.swap_remove(connection_index);
                    drop(connection_guard);
                    drop(inner);

                    if reason == DisconnectReason::Shutdown {
                        self.driver.force_close(connection)?;
                    }
                } else {
                    connection_guard.peer_requested_shutdown = true;
                }
            }
            VsockEventType::Received { .. } => {
                // Already copied the buffer in the callback above.
            }
            VsockEventType::CreditRequest => {
                drop(inner);
                // If the peer requested credit, send an update.
                self.driver.credit_update(connection)?;
                // No need to pass the request on to the client, we've already handled it.
                return Ok(None);
            }
            VsockEventType::CreditUpdate => {}
        }

        Ok(Some(event))
    }

    // Polls the vsock device to receive data or other updates.
    pub fn poll_direct(&self) -> Result<Option<VsockEvent>> {
        let local_cid = self.driver.local_cid();
        self.driver.poll(|mut event, body| {
            if event.destination.cid != local_cid {
                return Ok(None);
            }
            let mut inner_guard = self.inner.lock();
            if event.event_type == VsockEventType::ConnectionRequest {
                if !inner_guard
                    .listening_ports
                    .contains(&event.destination.port)
                {
                    // TODO: Return reject connection action here instead
                    return Ok(None);
                }

                let mut c = Connection::new(
                    event.source,
                    event.destination.port,
                    inner_guard.per_connection_buffer_capacity,
                );
                c.info.update_for_event(&event);
                event.new_connection = Some(c);
                // TODO: We deliberately do not add the connection to `inner.connections` until the
                // caller accepts the connection. This means that if the peer follows up with a
                // reset packet before the connection is accepted, it will be treated as an
                // unknown connection.
                return Ok(Some(event));
            }
            let Some((connection_index, existing_connection)) =
                get_connection_for_event::<L>(&inner_guard.connections, &event, local_cid)
            else {
                return Ok(None);
            };
            if let VsockEventType::Disconnected {
                reason: DisconnectReason::Reset,
            } = event.event_type
            {
                // Remove the connection immediately
                inner_guard.connections.swap_remove(connection_index);
                return Ok(Some(event));
            }
            drop(inner_guard);

            let mut existing_connection = existing_connection.lock();
            existing_connection.info.update_for_event(&event);

            if let VsockEventType::Received { length } = event.event_type {
                // Copy to buffer
                if !existing_connection.buffer.add(body) {
                    return Err(SocketError::OutputBufferTooShort(length).into());
                }
            }

            Ok(Some(event))
        })
    }

    /// Returns the local CID of the vsock device.
    pub fn local_cid(&self) -> u64 {
        self.driver.local_cid()
    }

    /// Reads data received from the given connection.
    pub fn recv(&self, peer: VsockAddr, src_port: u32, buffer: &mut [u8]) -> Result<usize> {
        let mut inner = self.inner.lock();
        let (connection_index, connection) =
            get_connection::<L>(&inner.connections, peer, src_port)?;

        let mut connection_guard = connection.lock();
        // Copy from ring buffer
        let bytes_read = connection_guard.buffer.drain(buffer);

        connection_guard.info.done_forwarding(bytes_read);

        // If buffer is now empty and the peer requested shutdown, finish shutting down the
        // connection.
        if connection_guard.peer_requested_shutdown && connection_guard.buffer.is_empty() {
            inner.connections.swap_remove(connection_index);
            drop(connection_guard);
            drop(inner);

            self.driver.force_close(connection)?;
        }

        Ok(bytes_read)
    }

    /// Returns the number of bytes in the receive buffer available to be read by `recv`.
    ///
    /// When the available bytes is 0, it indicates that the receive buffer is empty and does not
    /// contain any data.
    pub fn recv_buffer_available_bytes(&self, peer: VsockAddr, src_port: u32) -> Result<usize> {
        let inner = self.inner.lock();
        let (_, connection) = get_connection::<L>(&inner.connections, peer, src_port)?;
        let connection = connection.lock();
        Ok(connection.buffer.used())
    }

    /// Sends a credit update to the given peer.
    pub fn update_credit(&self, peer: VsockAddr, src_port: u32) -> Result {
        let inner = self.inner.lock();
        let (_, connection) = get_connection::<L>(&inner.connections, peer, src_port)?;
        drop(inner);

        self.driver.credit_update(connection)
    }

    /// Blocks until we get some event from the vsock device.
    pub fn wait_for_event(&self) -> Result<VsockEvent> {
        loop {
            if let Some(event) = self.poll()? {
                return Ok(event);
            } else {
                spin_loop();
            }
        }
    }

    /// Requests to shut down the connection cleanly, telling the peer that we won't send or receive
    /// any more data.
    ///
    /// This returns as soon as the request is sent; you should wait until `poll` returns a
    /// `VsockEventType::Disconnected` event if you want to know that the peer has acknowledged the
    /// shutdown.
    pub fn shutdown(&self, destination: VsockAddr, src_port: u32) -> Result {
        let inner = self.inner.lock();
        let (_, connection) = get_connection::<L>(&inner.connections, destination, src_port)?;
        drop(inner);

        self.driver.shutdown(connection)
    }

    /// Forcibly closes the connection without waiting for the peer.
    pub fn force_close(&self, destination: VsockAddr, src_port: u32) -> Result {
        let mut inner = self.inner.lock();
        let (index, connection) = get_connection::<L>(&inner.connections, destination, src_port)?;
        inner.connections.swap_remove(index);
        drop(inner);

        self.driver.force_close(connection)?;

        Ok(())
    }
}

/// Returns the connection from the given list matching the given peer address and local port, and
/// its index.
///
/// Returns `Err(SocketError::NotConnected)` if there is no matching connection in the list.
fn get_connection<L: LockFactory>(
    connections: &[Arc<L::Lock<Connection>>],
    peer: VsockAddr,
    local_port: u32,
) -> core::result::Result<(usize, Arc<L::Lock<Connection>>), SocketError> {
    connections
        .iter()
        .enumerate()
        .find(|(_, connection)| {
            let connection = connection.lock();
            connection.info.dst == peer && connection.info.src_port == local_port
        })
        .map(|(idx, connection)| (idx, Arc::clone(connection)))
        .ok_or(SocketError::NotConnected)
}

/// Returns the connection from the given list matching the event, if any, and its index.
fn get_connection_for_event<L: LockFactory>(
    connections: &[Arc<L::Lock<Connection>>],
    event: &VsockEvent,
    local_cid: u64,
) -> Option<(usize, Arc<L::Lock<Connection>>)> {
    connections
        .iter()
        .enumerate()
        .find(|(_, connection)| event.matches_connection(&connection.lock().info, local_cid))
        .map(|(idx, connection)| (idx, Arc::clone(connection)))
}

#[derive(Debug)]
struct RingBuffer {
    buffer: Box<[u8]>,
    /// The number of bytes currently in the buffer.
    used: usize,
    /// The index of the first used byte in the buffer.
    start: usize,
}

impl RingBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: FromZeros::new_box_zeroed_with_elems(capacity).unwrap(),
            used: 0,
            start: 0,
        }
    }

    /// Returns the number of bytes currently used in the buffer.
    pub fn used(&self) -> usize {
        self.used
    }

    /// Returns true iff there are currently no bytes in the buffer.
    pub fn is_empty(&self) -> bool {
        self.used == 0
    }

    /// Returns the number of bytes currently free in the buffer.
    pub fn free(&self) -> usize {
        self.buffer.len() - self.used
    }

    /// Adds the given bytes to the buffer if there is enough capacity for them all.
    ///
    /// Returns true if they were added, or false if they were not.
    pub fn add(&mut self, bytes: &[u8]) -> bool {
        if bytes.len() > self.free() {
            return false;
        }

        // The index of the first available position in the buffer.
        let first_available = (self.start + self.used) % self.buffer.len();
        // The number of bytes to copy from `bytes` to `buffer` between `first_available` and
        // `buffer.len()`.
        let copy_length_before_wraparound = min(bytes.len(), self.buffer.len() - first_available);
        self.buffer[first_available..first_available + copy_length_before_wraparound]
            .copy_from_slice(&bytes[0..copy_length_before_wraparound]);
        if let Some(bytes_after_wraparound) = bytes.get(copy_length_before_wraparound..) {
            self.buffer[0..bytes_after_wraparound.len()].copy_from_slice(bytes_after_wraparound);
        }
        self.used += bytes.len();

        true
    }

    /// Reads and removes as many bytes as possible from the buffer, up to the length of the given
    /// buffer.
    pub fn drain(&mut self, out: &mut [u8]) -> usize {
        let bytes_read = min(self.used, out.len());

        // The number of bytes to copy out between `start` and the end of the buffer.
        let read_before_wraparound = min(bytes_read, self.buffer.len() - self.start);
        // The number of bytes to copy out from the beginning of the buffer after wrapping around.
        let read_after_wraparound = bytes_read
            .checked_sub(read_before_wraparound)
            .unwrap_or_default();

        out[0..read_before_wraparound]
            .copy_from_slice(&self.buffer[self.start..self.start + read_before_wraparound]);
        out[read_before_wraparound..bytes_read]
            .copy_from_slice(&self.buffer[0..read_after_wraparound]);

        self.used -= bytes_read;
        self.start = (self.start + bytes_read) % self.buffer.len();

        bytes_read
    }
}

#[cfg(all(test, feature = "spin"))]
mod tests {
    use super::*;
    use crate::{
        config::ReadOnly,
        device::socket::{
            protocol::{
                SocketType, StreamShutdown, VirtioVsockConfig, VirtioVsockHdr, VirtioVsockOp,
            },
            vsock::{VsockBufferStatus, QUEUE_SIZE, RX_QUEUE_IDX, TX_QUEUE_IDX},
        },
        hal::fake::FakeHal,
        transport::{
            fake::{FakeTransport, QueueStatus, State},
            DeviceType,
        },
        SpinLockFactory,
    };
    use alloc::{sync::Arc, vec};
    use core::mem::size_of;
    use std::{sync::Mutex, thread};
    use zerocopy::{FromBytes, IntoBytes};

    #[test]
    fn send_recv() {
        let host_cid = 2;
        let guest_cid = 66;
        let host_port = 1234;
        let guest_port = 4321;
        let host_address = VsockAddr {
            cid: host_cid,
            port: host_port,
        };
        let hello_from_guest = "Hello from guest";
        let hello_from_host = "Hello from host";

        let config_space = VirtioVsockConfig {
            guest_cid_low: ReadOnly::new(66),
            guest_cid_high: ReadOnly::new(0),
        };
        let state = Arc::new(Mutex::new(State::new(
            vec![
                QueueStatus::default(),
                QueueStatus::default(),
                QueueStatus::default(),
            ],
            config_space,
        )));
        let transport = FakeTransport {
            device_type: DeviceType::Socket,
            max_queue_size: 32,
            device_features: 0,
            state: state.clone(),
        };
        let mut socket = VsockConnectionManager::new(
            VirtIOSocket::<FakeHal, FakeTransport<VirtioVsockConfig>, SpinLockFactory>::new(
                transport,
            )
            .unwrap(),
        );

        // Start a thread to simulate the device.
        let handle = thread::spawn(move || {
            // Wait for connection request.
            State::wait_until_queue_notified(&state, TX_QUEUE_IDX);
            assert_eq!(
                VirtioVsockHdr::read_from_bytes(
                    state
                        .lock()
                        .unwrap()
                        .read_from_queue::<QUEUE_SIZE>(TX_QUEUE_IDX)
                        .as_slice()
                )
                .unwrap(),
                VirtioVsockHdr {
                    op: VirtioVsockOp::Request.into(),
                    src_cid: guest_cid.into(),
                    dst_cid: host_cid.into(),
                    src_port: guest_port.into(),
                    dst_port: host_port.into(),
                    len: 0.into(),
                    socket_type: SocketType::Stream.into(),
                    flags: 0.into(),
                    buf_alloc: 1024.into(),
                    fwd_cnt: 0.into(),
                }
            );

            // Accept connection and give the peer enough credit to send the message.
            state.lock().unwrap().write_to_queue::<QUEUE_SIZE>(
                RX_QUEUE_IDX,
                VirtioVsockHdr {
                    op: VirtioVsockOp::Response.into(),
                    src_cid: host_cid.into(),
                    dst_cid: guest_cid.into(),
                    src_port: host_port.into(),
                    dst_port: guest_port.into(),
                    len: 0.into(),
                    socket_type: SocketType::Stream.into(),
                    flags: 0.into(),
                    buf_alloc: 50.into(),
                    fwd_cnt: 0.into(),
                }
                .as_bytes(),
            );

            // Expect the guest to send some data.
            State::wait_until_queue_notified(&state, TX_QUEUE_IDX);
            let request = state
                .lock()
                .unwrap()
                .read_from_queue::<QUEUE_SIZE>(TX_QUEUE_IDX);
            assert_eq!(
                request.len(),
                size_of::<VirtioVsockHdr>() + hello_from_guest.len()
            );
            assert_eq!(
                VirtioVsockHdr::read_from_prefix(request.as_slice())
                    .unwrap()
                    .0,
                VirtioVsockHdr {
                    op: VirtioVsockOp::Rw.into(),
                    src_cid: guest_cid.into(),
                    dst_cid: host_cid.into(),
                    src_port: guest_port.into(),
                    dst_port: host_port.into(),
                    len: (hello_from_guest.len() as u32).into(),
                    socket_type: SocketType::Stream.into(),
                    flags: 0.into(),
                    buf_alloc: 1024.into(),
                    fwd_cnt: 0.into(),
                }
            );
            assert_eq!(
                &request[size_of::<VirtioVsockHdr>()..],
                hello_from_guest.as_bytes()
            );

            println!("Host sending");

            // Send a response.
            let mut response = vec![0; size_of::<VirtioVsockHdr>() + hello_from_host.len()];
            VirtioVsockHdr {
                op: VirtioVsockOp::Rw.into(),
                src_cid: host_cid.into(),
                dst_cid: guest_cid.into(),
                src_port: host_port.into(),
                dst_port: guest_port.into(),
                len: (hello_from_host.len() as u32).into(),
                socket_type: SocketType::Stream.into(),
                flags: 0.into(),
                buf_alloc: 50.into(),
                fwd_cnt: (hello_from_guest.len() as u32).into(),
            }
            .write_to_prefix(response.as_mut_slice())
            .unwrap();
            response[size_of::<VirtioVsockHdr>()..].copy_from_slice(hello_from_host.as_bytes());
            state
                .lock()
                .unwrap()
                .write_to_queue::<QUEUE_SIZE>(RX_QUEUE_IDX, &response);

            // Expect a shutdown.
            State::wait_until_queue_notified(&state, TX_QUEUE_IDX);
            assert_eq!(
                VirtioVsockHdr::read_from_bytes(
                    state
                        .lock()
                        .unwrap()
                        .read_from_queue::<QUEUE_SIZE>(TX_QUEUE_IDX)
                        .as_slice()
                )
                .unwrap(),
                VirtioVsockHdr {
                    op: VirtioVsockOp::Shutdown.into(),
                    src_cid: guest_cid.into(),
                    dst_cid: host_cid.into(),
                    src_port: guest_port.into(),
                    dst_port: host_port.into(),
                    len: 0.into(),
                    socket_type: SocketType::Stream.into(),
                    flags: (StreamShutdown::SEND | StreamShutdown::RECEIVE).into(),
                    buf_alloc: 1024.into(),
                    fwd_cnt: (hello_from_host.len() as u32).into(),
                }
            );
        });

        socket.connect(host_address, guest_port).unwrap();
        let event = socket.wait_for_event().unwrap();
        assert_eq!(event.source, host_address);
        assert_eq!(
            event.destination,
            VsockAddr {
                cid: guest_cid,
                port: guest_port,
            }
        );
        assert_eq!(event.event_type, VsockEventType::Connected);
        assert_eq!(
            event.buffer_status,
            VsockBufferStatus {
                buffer_allocation: 50,
                forward_count: 0,
            }
        );
        println!("Guest sending");
        socket
            .send(host_address, guest_port, "Hello from guest".as_bytes())
            .unwrap();
        println!("Guest waiting to receive.");
        let event = socket.wait_for_event().unwrap();
        assert_eq!(event.source, host_address);
        assert_eq!(
            event.destination,
            VsockAddr {
                cid: guest_cid,
                port: guest_port,
            }
        );
        assert_eq!(
            event.event_type,
            VsockEventType::Received {
                length: hello_from_host.len()
            }
        );
        assert_eq!(
            event.buffer_status,
            VsockBufferStatus {
                buffer_allocation: 50,
                forward_count: hello_from_guest.len() as u32,
            }
        );
        println!("Guest getting received data.");
        let mut buffer = [0u8; 64];
        assert_eq!(
            socket.recv(host_address, guest_port, &mut buffer).unwrap(),
            hello_from_host.len()
        );
        assert_eq!(
            &buffer[0..hello_from_host.len()],
            hello_from_host.as_bytes()
        );
        socket.shutdown(host_address, guest_port).unwrap();

        handle.join().unwrap();
    }

    #[test]
    fn incoming_connection() {
        let host_cid = 2;
        let guest_cid = 66;
        let host_port = 1234;
        let guest_port = 4321;
        let wrong_guest_port = 4444;
        let host_address = VsockAddr {
            cid: host_cid,
            port: host_port,
        };

        let config_space = VirtioVsockConfig {
            guest_cid_low: ReadOnly::new(66),
            guest_cid_high: ReadOnly::new(0),
        };
        let state = Arc::new(Mutex::new(State::new(
            vec![
                QueueStatus::default(),
                QueueStatus::default(),
                QueueStatus::default(),
            ],
            config_space,
        )));
        let transport = FakeTransport {
            device_type: DeviceType::Socket,
            max_queue_size: 32,
            device_features: 0,
            state: state.clone(),
        };
        let mut socket = VsockConnectionManager::new(
            VirtIOSocket::<FakeHal, FakeTransport<VirtioVsockConfig>, SpinLockFactory>::new(
                transport,
            )
            .unwrap(),
        );

        socket.listen(guest_port);

        // Start a thread to simulate the device.
        let handle = thread::spawn(move || {
            // Send a connection request for a port the guest isn't listening on.
            println!("Host sending connection request to wrong port");
            state.lock().unwrap().write_to_queue::<QUEUE_SIZE>(
                RX_QUEUE_IDX,
                VirtioVsockHdr {
                    op: VirtioVsockOp::Request.into(),
                    src_cid: host_cid.into(),
                    dst_cid: guest_cid.into(),
                    src_port: host_port.into(),
                    dst_port: wrong_guest_port.into(),
                    len: 0.into(),
                    socket_type: SocketType::Stream.into(),
                    flags: 0.into(),
                    buf_alloc: 50.into(),
                    fwd_cnt: 0.into(),
                }
                .as_bytes(),
            );

            // Expect a rejection.
            println!("Host waiting for rejection");
            State::wait_until_queue_notified(&state, TX_QUEUE_IDX);
            assert_eq!(
                VirtioVsockHdr::read_from_bytes(
                    state
                        .lock()
                        .unwrap()
                        .read_from_queue::<QUEUE_SIZE>(TX_QUEUE_IDX)
                        .as_slice()
                )
                .unwrap(),
                VirtioVsockHdr {
                    op: VirtioVsockOp::Rst.into(),
                    src_cid: guest_cid.into(),
                    dst_cid: host_cid.into(),
                    src_port: wrong_guest_port.into(),
                    dst_port: host_port.into(),
                    len: 0.into(),
                    socket_type: SocketType::Stream.into(),
                    flags: 0.into(),
                    buf_alloc: 1024.into(),
                    fwd_cnt: 0.into(),
                }
            );

            // Send a connection request for a port the guest is listening on.
            println!("Host sending connection request to right port");
            state.lock().unwrap().write_to_queue::<QUEUE_SIZE>(
                RX_QUEUE_IDX,
                VirtioVsockHdr {
                    op: VirtioVsockOp::Request.into(),
                    src_cid: host_cid.into(),
                    dst_cid: guest_cid.into(),
                    src_port: host_port.into(),
                    dst_port: guest_port.into(),
                    len: 0.into(),
                    socket_type: SocketType::Stream.into(),
                    flags: 0.into(),
                    buf_alloc: 50.into(),
                    fwd_cnt: 0.into(),
                }
                .as_bytes(),
            );

            // Expect a response.
            println!("Host waiting for response");
            State::wait_until_queue_notified(&state, TX_QUEUE_IDX);
            assert_eq!(
                VirtioVsockHdr::read_from_bytes(
                    state
                        .lock()
                        .unwrap()
                        .read_from_queue::<QUEUE_SIZE>(TX_QUEUE_IDX)
                        .as_slice()
                )
                .unwrap(),
                VirtioVsockHdr {
                    op: VirtioVsockOp::Response.into(),
                    src_cid: guest_cid.into(),
                    dst_cid: host_cid.into(),
                    src_port: guest_port.into(),
                    dst_port: host_port.into(),
                    len: 0.into(),
                    socket_type: SocketType::Stream.into(),
                    flags: 0.into(),
                    buf_alloc: 1024.into(),
                    fwd_cnt: 0.into(),
                }
            );

            println!("Host finished");
        });

        // Expect an incoming connection.
        println!("Guest expecting incoming connection.");
        let event = socket.wait_for_event().unwrap();
        assert_eq!(event.source, host_address);
        assert_eq!(
            event.destination,
            VsockAddr {
                cid: guest_cid,
                port: guest_port,
            }
        );
        assert_eq!(event.event_type, VsockEventType::ConnectionRequest);
        assert_eq!(
            event.buffer_status,
            VsockBufferStatus {
                buffer_allocation: 50,
                forward_count: 0,
            }
        );

        handle.join().unwrap();
    }
}
