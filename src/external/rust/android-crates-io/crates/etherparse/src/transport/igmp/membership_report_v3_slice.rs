use crate::{igmp::*, *};

/// Zero-copy slice of an IGMPv3 "Membership Report" (type `0x22`)
/// including the list of group records.
///
/// ```text
/// 0                   1                   2                   3
/// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+  -
/// |  Type = 0x22  |    Reserved   |           Checksum            |  | part of header &
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+  | this type
/// |             Flags             |  Number of Group Records (M)  |  ↓
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+  -
/// |                                                               |  |
/// .                                                               .  |
/// .                        Group Record [1]                       .  |
/// .                                                               .  |
/// |                                                               |  |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+  |
/// |                                                               |  |
/// .                                                               .  |
/// .                        Group Record [2]                       .  | part of payload
/// .                                                               .  |
/// |                                                               |  |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+  |
/// |                               .                               |  |
/// .                               .                               .  |
/// |                               .                               |  |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+  |
/// |                                                               |  |
/// .                                                               .  |
/// .                        Group Record [M]                       .  |
/// .                                                               .  |
/// |                                                               |  ↓
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+  -
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MembershipReportV3Slice<'a> {
    slice: &'a [u8],
}

impl<'a> MembershipReportV3Slice<'a> {
    /// Creates a slice from bytes without checking the length.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that `slice` is at least
    /// [`MembershipReportV3Header::LEN`] (8) bytes long.
    #[inline]
    pub(crate) unsafe fn from_slice_unchecked(slice: &'a [u8]) -> MembershipReportV3Slice<'a> {
        debug_assert!(slice.len() >= MembershipReportV3Header::LEN);
        MembershipReportV3Slice { slice }
    }

    /// Returns the "checksum" value stored in the IGMP header.
    #[inline]
    pub fn checksum(&self) -> u16 {
        // SAFETY: from_slice_unchecked guarantees at least 8 bytes.
        unsafe { get_unchecked_be_u16(self.slice.as_ptr().add(2)) }
    }

    /// Additional flags (bytes 4-5 of the header).
    #[inline]
    pub fn flags(&self) -> [u8; 2] {
        // SAFETY: from_slice_unchecked guarantees at least 8 bytes.
        unsafe { [*self.slice.get_unchecked(4), *self.slice.get_unchecked(5)] }
    }

    /// Number of group records declared in the report header.
    #[inline]
    pub fn num_of_records(&self) -> u16 {
        // SAFETY: from_slice_unchecked guarantees at least 8 bytes.
        unsafe { get_unchecked_be_u16(self.slice.as_ptr().add(6)) }
    }

    /// Decodes the fixed fields into an owned [`MembershipReportV3Header`].
    #[inline]
    pub fn to_header(&self) -> MembershipReportV3Header {
        MembershipReportV3Header {
            flags: self.flags(),
            num_of_records: self.num_of_records(),
        }
    }

    /// Returns an iterator over the group records.
    ///
    /// The iterator yields at most [`MembershipReportV3Slice::num_of_records`]
    /// items and stops early with an [`err::LenError`] if the payload is
    /// shorter than the records claim.
    #[inline]
    pub fn group_records(&self) -> ReportGroupRecordV3SliceIter<'a> {
        ReportGroupRecordV3SliceIter::new(self.payload(), self.num_of_records())
    }

    /// Returns the bytes after the 8-byte header (the group records).
    #[inline]
    pub fn payload(&self) -> &'a [u8] {
        // SAFETY: from_slice_unchecked guarantees at least 8 bytes.
        unsafe {
            core::slice::from_raw_parts(
                self.slice.as_ptr().add(MembershipReportV3Header::LEN),
                self.slice.len() - MembershipReportV3Header::LEN,
            )
        }
    }

    /// Returns the slice containing the entire IGMP message.
    #[inline]
    pub fn slice(&self) -> &'a [u8] {
        self.slice
    }
}
