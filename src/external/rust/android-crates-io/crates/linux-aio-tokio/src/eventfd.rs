#![allow(clippy::enum_variant_names)]

use std::fs::File;
use std::io::{self, Read, Write};
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::pin::Pin;
use std::task::{Context, Poll};
use std::{fmt, mem, slice};

use libc::eventfd;
use thiserror::Error;
use tokio::io::Ready;
use tokio::io::unix::AsyncFd;
use tokio_stream::Stream;

#[derive(Error, Debug)]
pub enum EventFdError {
    #[error("error creating EventFd: `{0}`")]
    CreateError(#[source] io::Error),
    #[error("Poll error: `{0}`")]
    PollError(#[source] io::Error),
    #[error("Read error: `{0}`")]
    ReadError(#[source] io::Error),
}

pub struct EventFdInner {
    pub inner: File,
}

impl AsRawFd for EventFdInner {
    fn as_raw_fd(&self) -> RawFd {
        self.inner.as_raw_fd()
    }
}

/// Tokio-aware EventFd implementation
pub struct EventFd {
    evented: AsyncFd<EventFdInner>,
}

impl fmt::Debug for EventFd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("EventFd").finish()
    }
}

impl Stream for EventFd {
    type Item = Result<u64, EventFdError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();

        loop {
            let mut guard = match this.evented.poll_read_ready_mut(cx) {
                Poll::Ready(Ok(g)) => g,
                Poll::Ready(Err(e)) => return Poll::Ready(Some(Err(EventFdError::PollError(e)))),
                Poll::Pending => return Poll::Pending,
            };

            let mut result = 0u64;
            let result_ptr = &mut result as *mut u64 as *mut u8;

            match guard
                .get_inner_mut()
                .inner
                .read(unsafe { slice::from_raw_parts_mut(result_ptr, 8) })
            {
                Ok(rc) => {
                    if rc != mem::size_of::<u64>() {
                        panic!(
                            "Reading from an eventfd should transfer exactly {} bytes",
                            mem::size_of::<u64>()
                        )
                    }

                    assert_ne!(result, 0);
                    return Poll::Ready(Some(Ok(result)));
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    guard.clear_ready_matching(Ready::READABLE);
                }
                Err(e) => return Poll::Ready(Some(Err(EventFdError::ReadError(e)))),
            }
        }
    }
}

impl AsRawFd for EventFd {
    fn as_raw_fd(&self) -> RawFd {
        self.evented.get_ref().inner.as_raw_fd()
    }
}

impl EventFd {
    /// Create EventFd  with `init` permits.
    pub fn new(init: usize, semaphore: bool) -> Result<EventFd, EventFdError> {
        let flags = if semaphore {
            libc::O_CLOEXEC | libc::EFD_NONBLOCK | libc::EFD_SEMAPHORE
        } else {
            libc::O_CLOEXEC | libc::EFD_NONBLOCK
        };

        let fd = unsafe { eventfd(init as libc::c_uint, flags) };

        if fd < 0 {
            return Err(EventFdError::CreateError(io::Error::last_os_error()));
        }

        Ok(EventFd {
            evented: AsyncFd::new(EventFdInner {
                inner: unsafe { File::from_raw_fd(fd) },
            })
            .map_err(EventFdError::PollError)?,
        })
    }

    /// Receive the next value from the eventfd, waiting for readability.
    pub async fn recv(&mut self) -> Result<u64, EventFdError> {
        loop {
            let mut guard = self
                .evented
                .readable_mut()
                .await
                .map_err(EventFdError::PollError)?;

            let mut result = 0u64;
            let result_ptr = &mut result as *mut u64 as *mut u8;

            match guard
                .get_inner_mut()
                .inner
                .read(unsafe { slice::from_raw_parts_mut(result_ptr, 8) })
            {
                Ok(rc) => {
                    if rc != mem::size_of::<u64>() {
                        panic!(
                            "Reading from an eventfd should transfer exactly {} bytes",
                            mem::size_of::<u64>()
                        )
                    }

                    assert_ne!(result, 0);
                    return Ok(result);
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    guard.clear_ready_matching(Ready::READABLE);
                }
                Err(e) => return Err(EventFdError::ReadError(e)),
            }
        }
    }

    /// Send a value into the eventfd, waiting for writability.
    pub async fn send_value(&mut self, value: u64) -> Result<(), EventFdError> {
        let bytes = value.to_ne_bytes();

        loop {
            let mut guard = self
                .evented
                .writable_mut()
                .await
                .map_err(EventFdError::PollError)?;

            match guard.get_inner_mut().inner.write(&bytes) {
                Ok(rc) => {
                    assert_eq!(8, rc);
                    return Ok(());
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    guard.clear_ready_matching(Ready::WRITABLE);
                }
                Err(e) => return Err(EventFdError::ReadError(e)),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn non_semaphore() {
        let init = 5;
        let increment: u64 = 10;

        let mut efd = EventFd::new(init, false).unwrap();

        assert_eq!(init as u64, efd.recv().await.unwrap());

        efd.send_value(increment).await.unwrap();
        assert_eq!(increment, efd.recv().await.unwrap());

        efd.send_value(increment).await.unwrap();
        efd.send_value(increment).await.unwrap();
        assert_eq!(2 * increment, efd.recv().await.unwrap());
    }

    #[tokio::test]
    async fn semaphore() {
        let init = 2;
        let increment: u64 = 10;

        let mut efd = EventFd::new(init, true).unwrap();

        efd.send_value(increment).await.unwrap();
        for _ in 0..(increment as usize + init) {
            assert_eq!(1, efd.recv().await.unwrap());
        }

        efd.send_value(increment).await.unwrap();
        for _ in 0..(increment as usize) {
            assert_eq!(1, efd.recv().await.unwrap());
        }
    }
}
