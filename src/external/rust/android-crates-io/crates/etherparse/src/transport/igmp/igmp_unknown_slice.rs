use crate::{igmp::*, *};

/// Zero-copy slice of an IGMP message with a type unknown to etherparse.
///
/// See [`UnknownHeader`] for the field layout.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IgmpUnknownSlice<'a> {
    slice: &'a [u8],
}

impl<'a> IgmpUnknownSlice<'a> {
    /// Creates a slice from bytes without checking the length.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that `slice` is at least
    /// [`UnknownHeader::LEN`] (8) bytes long.
    #[inline]
    pub(crate) unsafe fn from_slice_unchecked(slice: &'a [u8]) -> IgmpUnknownSlice<'a> {
        debug_assert!(slice.len() >= UnknownHeader::LEN);
        IgmpUnknownSlice { slice }
    }

    /// Returns the "type" byte value in the IGMP header.
    #[inline]
    pub fn type_u8(&self) -> u8 {
        // SAFETY: from_slice_unchecked guarantees at least 8 bytes.
        unsafe { *self.slice.get_unchecked(0) }
    }

    /// Returns the raw byte value after the type value.
    #[inline]
    pub fn raw_byte_1(&self) -> u8 {
        // SAFETY: from_slice_unchecked guarantees at least 8 bytes.
        unsafe { *self.slice.get_unchecked(1) }
    }

    /// Returns the "checksum" value stored in the IGMP header.
    #[inline]
    pub fn checksum(&self) -> u16 {
        // SAFETY: from_slice_unchecked guarantees at least 8 bytes.
        unsafe { get_unchecked_be_u16(self.slice.as_ptr().add(2)) }
    }

    /// Returns the raw byte values after the checksum (bytes 4-7).
    #[inline]
    pub fn raw_bytes_4_7(&self) -> [u8; 4] {
        // SAFETY: from_slice_unchecked guarantees at least 8 bytes.
        unsafe {
            [
                *self.slice.get_unchecked(4),
                *self.slice.get_unchecked(5),
                *self.slice.get_unchecked(6),
                *self.slice.get_unchecked(7),
            ]
        }
    }

    /// Decodes the fixed fields into an owned [`UnknownHeader`].
    #[inline]
    pub fn to_header(&self) -> UnknownHeader {
        UnknownHeader {
            igmp_type: self.type_u8(),
            raw_byte_1: self.raw_byte_1(),
            raw_bytes_4_7: self.raw_bytes_4_7(),
        }
    }

    /// Returns the bytes after the 8-byte header.
    #[inline]
    pub fn payload(&self) -> &'a [u8] {
        // SAFETY: from_slice_unchecked guarantees at least 8 bytes.
        unsafe {
            core::slice::from_raw_parts(
                self.slice.as_ptr().add(UnknownHeader::LEN),
                self.slice.len() - UnknownHeader::LEN,
            )
        }
    }

    /// Returns the slice containing the entire IGMP message.
    #[inline]
    pub fn slice(&self) -> &'a [u8] {
        self.slice
    }
}
