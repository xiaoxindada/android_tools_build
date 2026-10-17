use crate::{igmp::*, *};

/// Zero-copy slice of an IGMPv1/IGMPv2 "Membership Query" (type `0x11`,
/// exactly 8 octets long).
///
/// ```text
/// 0                   1                   2                   3
/// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |  Type = 0x11  | Max Resp Time |           Checksum            |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                         Group Address                         |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MembershipQuerySlice<'a> {
    slice: &'a [u8],
}

impl<'a> MembershipQuerySlice<'a> {
    /// Creates a slice from bytes without checking that they contain a
    /// valid IGMPv1/v2 query.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that `slice` is at least
    /// [`MembershipQueryType::LEN`] (8) bytes long.
    #[inline]
    pub(crate) unsafe fn from_slice_unchecked(slice: &'a [u8]) -> MembershipQuerySlice<'a> {
        debug_assert!(slice.len() >= MembershipQueryType::LEN);
        MembershipQuerySlice { slice }
    }

    /// The maximum response time (0 for IGMPv1, non-zero for IGMPv2).
    #[inline]
    pub fn max_response_time(&self) -> u8 {
        // SAFETY: from_slice_unchecked guarantees at least 8 bytes.
        unsafe { *self.slice.get_unchecked(1) }
    }

    /// Returns the "checksum" value stored in the IGMP header.
    #[inline]
    pub fn checksum(&self) -> u16 {
        // SAFETY: from_slice_unchecked guarantees at least 8 bytes.
        unsafe { get_unchecked_be_u16(self.slice.as_ptr().add(2)) }
    }

    /// The group address being queried.
    #[inline]
    pub fn group_address(&self) -> GroupAddress {
        // SAFETY: from_slice_unchecked guarantees at least 8 bytes.
        GroupAddress::new(unsafe {
            [
                *self.slice.get_unchecked(4),
                *self.slice.get_unchecked(5),
                *self.slice.get_unchecked(6),
                *self.slice.get_unchecked(7),
            ]
        })
    }

    /// Decodes the fixed fields into an owned [`MembershipQueryType`].
    #[inline]
    pub fn to_header(&self) -> MembershipQueryType {
        MembershipQueryType {
            max_response_time: self.max_response_time(),
            group_address: self.group_address(),
        }
    }

    /// Returns the slice containing the entire IGMP message.
    #[inline]
    pub fn slice(&self) -> &'a [u8] {
        self.slice
    }
}
