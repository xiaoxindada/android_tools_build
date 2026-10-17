use crate::{igmp::*, *};

/// Zero-copy slice of an IGMPv2 "Membership Report" (type `0x16`).
///
/// ```text
/// 0                   1                   2                   3
/// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |  Type = 0x16  |    Unused     |           Checksum            |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                         Group Address                         |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MembershipReportV2Slice<'a> {
    slice: &'a [u8],
}

impl<'a> MembershipReportV2Slice<'a> {
    /// Creates a slice from bytes without checking the length.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that `slice` is at least
    /// [`MembershipReportV2Type::LEN`] (8) bytes long.
    #[inline]
    pub(crate) unsafe fn from_slice_unchecked(slice: &'a [u8]) -> MembershipReportV2Slice<'a> {
        debug_assert!(slice.len() >= MembershipReportV2Type::LEN);
        MembershipReportV2Slice { slice }
    }

    /// Returns the "checksum" value stored in the IGMP header.
    #[inline]
    pub fn checksum(&self) -> u16 {
        // SAFETY: from_slice_unchecked guarantees at least 8 bytes.
        unsafe { get_unchecked_be_u16(self.slice.as_ptr().add(2)) }
    }

    /// The IP multicast group address of the group being reported.
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

    /// Decodes the fixed fields into an owned [`MembershipReportV2Type`].
    #[inline]
    pub fn to_header(&self) -> MembershipReportV2Type {
        MembershipReportV2Type {
            group_address: self.group_address(),
        }
    }

    /// Returns the bytes after the 8-byte header (usually empty).
    #[inline]
    pub fn payload(&self) -> &'a [u8] {
        // SAFETY: from_slice_unchecked guarantees at least 8 bytes.
        unsafe {
            core::slice::from_raw_parts(
                self.slice.as_ptr().add(MembershipReportV2Type::LEN),
                self.slice.len() - MembershipReportV2Type::LEN,
            )
        }
    }

    /// Returns the slice containing the entire IGMP message.
    #[inline]
    pub fn slice(&self) -> &'a [u8] {
        self.slice
    }
}
