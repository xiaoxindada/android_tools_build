use crate::{
    error::{Error, Result},
    header::DoipHeader,
    payload::DoipPayload,
};

/// The decoded struct of a `DoIP` packet.
///
/// Each `DoIP` packet contains a header which describes the message, this is outlined
/// in `DoipHeader`.
///
/// Some Payload Types available in `DoIP` require a payload which is covered by
/// `DoipPayload`.
#[cfg(not(feature = "std"))]
#[derive(Debug, PartialEq, Clone)]
pub struct DoipMessage<const N: usize> {
    /// Defined by `DoipHeader`, the header supplies the information for programs
    /// to understand the payload.
    pub header: DoipHeader,

    /// Takes any struct implementing `DoipPayload`.
    pub payload: DoipPayload<N>,
}

#[cfg(not(feature = "std"))]
impl<const N: usize> TryFrom<DoipMessage<N>> for [u8; N] {
    type Error = Error;

    fn try_from(value: DoipMessage<N>) -> Result<Self> {
        let mut buffer = [0u8; N];

        let payload_len = value.header.payload_length;

        let header: [u8; 8] = value.header.into();
        buffer[0..8].copy_from_slice(&header);

        if N < 8 + (payload_len as usize) {
            return Err(Error::BufferTooSmall { size: N });
        }

        let payload: [u8; N] = value.payload.into();
        buffer[8..].copy_from_slice(&payload);

        Ok(buffer)
    }
}

/// The decoded struct of a `DoIP` packet.
///
/// Each `DoIP` packet contains a header which describes the message, this is outlined
/// in `DoipHeader`.
///
/// Some Payload Types available in `DoIP` require a payload which is covered by
/// `DoipPayload`.
#[cfg(feature = "std")]
#[derive(Debug, PartialEq, Clone)]
pub struct DoipMessage {
    /// Defined by `DoipHeader`, the header supplies the information for programs
    /// to understand the payload.
    pub header: DoipHeader,

    /// Takes any struct implementing `DoipPayload`.
    pub payload: DoipPayload,
}

#[cfg(feature = "std")]
impl TryFrom<DoipMessage> for Vec<u8> {
    type Error = Error;

    fn try_from(value: DoipMessage) -> Result<Self> {
        let mut buffer = Vec::<u8>::new();

        let header: [u8; 8] = value.header.into();
        buffer[0..8].copy_from_slice(&header);

        let payload: Vec<u8> = value.payload.into();
        buffer[8..].copy_from_slice(&payload);

        Ok(buffer)
    }
}
