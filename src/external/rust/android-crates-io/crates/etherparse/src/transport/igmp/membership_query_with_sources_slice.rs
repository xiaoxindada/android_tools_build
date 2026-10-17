use crate::{igmp::*, *};

/// Zero-copy slice of an IGMPv3 "Membership Query" (type `0x11`, at
/// least 12 octets long) including the source address list.
///
/// ```text
/// 0                   1                   2                   3
/// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+  -
/// |  Type = 0x11  | Max Resp Code |           Checksum            |  |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+  | part of header and
/// |                         Group Address                         |  | this type
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+  |
/// | Flags |S| QRV |     QQIC      |     Number of Sources (N)     |  ↓
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+  -
/// |                       Source Address [1]                      |  |
/// +-                                                             -+  |
/// |                       Source Address [2]                      |  |
/// +-                              .                              -+  | part of payload
/// .                               .                               .  |
/// .                               .                               .  |
/// +-                                                             -+  |
/// |                       Source Address [N]                      |  ↓
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+  -
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MembershipQueryWithSourcesSlice<'a> {
    slice: &'a [u8],
}

impl<'a> MembershipQueryWithSourcesSlice<'a> {
    /// Creates a slice from bytes without checking that they contain a
    /// valid IGMPv3 query.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that `slice` contains the complete
    /// declared payload:
    ///
    /// * at least [`MembershipQueryWithSourcesHeader::LEN`] (12) bytes for
    ///   the header, and
    /// * additionally `num_of_sources * 4` bytes for the source address
    ///   list, where `num_of_sources` is the `u16` read from bytes 10..12
    ///   of the header.
    ///
    /// In other words `slice.len()` must be at least
    /// `12 + num_of_sources * 4` so that all source addresses accessed by
    /// [`MembershipQueryWithSourcesSlice::source_addrs_bytes`] are in
    /// bounds.
    #[inline]
    pub(crate) unsafe fn from_slice_unchecked(
        slice: &'a [u8],
    ) -> MembershipQueryWithSourcesSlice<'a> {
        debug_assert!(slice.len() >= MembershipQueryWithSourcesHeader::LEN);
        // The slice must also be long enough to hold all declared source
        // addresses (4 bytes each). `IgmpSlice::from_slice` guarantees this.
        debug_assert!(
            slice.len()
                >= MembershipQueryWithSourcesHeader::LEN
                    + usize::from(u16::from_be_bytes([slice[10], slice[11]])) * 4
        );
        MembershipQueryWithSourcesSlice { slice }
    }

    /// The Max Resp Code field.
    #[inline]
    pub fn max_response_code(&self) -> MaxResponseCode {
        // SAFETY: from_slice_unchecked guarantees at least 12 bytes.
        MaxResponseCode(unsafe { *self.slice.get_unchecked(1) })
    }

    /// Returns the "checksum" value stored in the IGMP header.
    #[inline]
    pub fn checksum(&self) -> u16 {
        // SAFETY: from_slice_unchecked guarantees at least 12 bytes.
        unsafe { get_unchecked_be_u16(self.slice.as_ptr().add(2)) }
    }

    /// The group address being queried.
    #[inline]
    pub fn group_address(&self) -> GroupAddress {
        // SAFETY: from_slice_unchecked guarantees at least 12 bytes.
        GroupAddress::new(unsafe {
            [
                *self.slice.get_unchecked(4),
                *self.slice.get_unchecked(5),
                *self.slice.get_unchecked(6),
                *self.slice.get_unchecked(7),
            ]
        })
    }

    /// Raw byte containing "flags", "s" & "QRV".
    #[inline]
    pub fn raw_byte_8(&self) -> u8 {
        // SAFETY: from_slice_unchecked guarantees at least 12 bytes.
        unsafe { *self.slice.get_unchecked(8) }
    }

    /// Extracts the "flags" from the `raw_byte_8` field.
    #[inline]
    pub fn flags(&self) -> u8 {
        (self.raw_byte_8() & MembershipQueryWithSourcesHeader::RAW_BYTE_8_MASK_FLAGS)
            >> MembershipQueryWithSourcesHeader::RAW_BYTE_8_OFFSET_FLAGS
    }

    /// Extracts the S flag (Suppress Router-Side Processing).
    #[inline]
    pub fn s_flag(&self) -> bool {
        0 != (self.raw_byte_8() & MembershipQueryWithSourcesHeader::RAW_BYTE_8_MASK_S_FLAG)
    }

    /// Extracts the QRV (Querier's Robustness Variable).
    #[inline]
    pub fn qrv(&self) -> Qrv {
        // SAFETY: the value is guaranteed to be within range after the mask.
        unsafe {
            Qrv::new_unchecked(
                self.raw_byte_8() & MembershipQueryWithSourcesHeader::RAW_BYTE_8_MASK_QRV,
            )
        }
    }

    /// QQIC (Querier's Query Interval Code).
    #[inline]
    pub fn qqic(&self) -> u8 {
        // SAFETY: from_slice_unchecked guarantees at least 12 bytes.
        unsafe { *self.slice.get_unchecked(9) }
    }

    /// Number of source addresses declared in the query header.
    #[inline]
    pub fn num_of_sources(&self) -> u16 {
        // SAFETY: from_slice_unchecked guarantees at least 12 bytes.
        unsafe { get_unchecked_be_u16(self.slice.as_ptr().add(10)) }
    }

    /// Decodes the fixed fields into an owned [`MembershipQueryWithSourcesHeader`].
    #[inline]
    pub fn to_header(&self) -> MembershipQueryWithSourcesHeader {
        MembershipQueryWithSourcesHeader {
            max_response_code: self.max_response_code(),
            group_address: self.group_address(),
            raw_byte_8: self.raw_byte_8(),
            qqic: self.qqic(),
            num_of_sources: self.num_of_sources(),
        }
    }

    /// Returns the raw source address bytes.
    ///
    /// The returned slice contains `num_of_sources * 4` bytes (each 4
    /// consecutive bytes are one IPv4 source address).
    #[inline]
    pub fn source_addrs_bytes(&self) -> &'a [u8] {
        let payload = self.payload();
        let len = usize::from(self.num_of_sources()) * 4;
        // SAFETY: `IgmpSlice::from_slice` guarantees the payload contains
        // all declared source addresses (num_of_sources * 4 bytes).
        unsafe { core::slice::from_raw_parts(payload.as_ptr(), len) }
    }

    /// Returns an iterator over the source addresses as `[u8; 4]` arrays.
    #[inline]
    pub fn source_addresses(&self) -> impl ExactSizeIterator<Item = [u8; 4]> + 'a {
        self.source_addrs_bytes()
            .chunks_exact(4)
            .map(|c| [c[0], c[1], c[2], c[3]])
    }

    /// Returns the bytes after the 12-byte header (the source address list).
    #[inline]
    pub fn payload(&self) -> &'a [u8] {
        // SAFETY: from_slice_unchecked guarantees at least 12 bytes.
        unsafe {
            core::slice::from_raw_parts(
                self.slice
                    .as_ptr()
                    .add(MembershipQueryWithSourcesHeader::LEN),
                self.slice.len() - MembershipQueryWithSourcesHeader::LEN,
            )
        }
    }

    /// Returns the slice containing the entire IGMP message.
    #[inline]
    pub fn slice(&self) -> &'a [u8] {
        self.slice
    }
}
