use crate::{igmp::*, *};

/// A zero-copy slice of an IGMP network packet, decoded into one variant
/// per message type.
///
/// This mirrors [`IgmpType`] but keeps the variable-length parts (source
/// address lists & group records) as zero-copy slices instead of copying
/// them. Match on the variant to get typed, compile-time-checked access
/// to the message specific accessors (e.g. `group_records` is only
/// reachable on [`IgmpSlice::MembershipReportV3`]).
///
/// # Important: Caller must trim to IGMP message length
///
/// For `0x11` "Membership Query" messages, the IGMP version is
/// determined by message length per [RFC 9776 §7.1](
/// https://datatracker.ietf.org/doc/html/rfc9776#section-7.1):
///
/// * IGMPv1/v2 Query: length = 8 octets
/// * IGMPv3 Query: length >= 12 octets
///
/// The caller **must** trim the input slice to the exact IGMP message
/// boundary (typically derived from the IP payload length) before
/// calling [`IgmpSlice::from_slice`]. If extra trailing bytes are
/// present, a query may be misidentified as IGMPv3.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IgmpSlice<'a> {
    /// Membership Query message (IGMPv1 & IGMPv2, type `0x11`, 8 octets).
    MembershipQuery(MembershipQuerySlice<'a>),

    /// Membership Query message (IGMPv3, type `0x11`, >= 12 octets) with sources.
    MembershipQueryWithSources(MembershipQueryWithSourcesSlice<'a>),

    /// Membership Report message (IGMPv1, type `0x12`).
    MembershipReportV1(MembershipReportV1Slice<'a>),

    /// Membership Report message (IGMPv2, type `0x16`).
    MembershipReportV2(MembershipReportV2Slice<'a>),

    /// Membership Report message (IGMPv3, type `0x22`) with group records.
    MembershipReportV3(MembershipReportV3Slice<'a>),

    /// Leave Group message (IGMPv2, type `0x17`).
    LeaveGroup(LeaveGroupSlice<'a>),

    /// Unknown type of IGMP message.
    Unknown(IgmpUnknownSlice<'a>),
}

impl<'a> IgmpSlice<'a> {
    /// Creates a slice containing an IGMP packet.
    ///
    /// # Errors
    ///
    /// The function will return an `Err` [`err::LenError`] if the given
    /// slice is too small to contain a valid IGMP header (minimum 8
    /// bytes), or has a length of 9-11 bytes for a `0x11` Membership
    /// Query (which is invalid per RFC 9776).
    #[inline]
    pub fn from_slice(slice: &'a [u8]) -> Result<IgmpSlice<'a>, err::LenError> {
        // Ensure the slice is large enough for the minimum IGMP header.
        if slice.len() < IgmpHeader::MIN_LEN {
            return Err(err::LenError {
                required_len: IgmpHeader::MIN_LEN,
                len: slice.len(),
                len_source: LenSource::Slice,
                layer: err::Layer::Igmp,
                layer_start_offset: 0,
            });
        }

        // SAFETY: length checked above to be >= IgmpHeader::MIN_LEN (8).
        let type_u8 = unsafe { *slice.get_unchecked(0) };
        Ok(match type_u8 {
            IGMP_TYPE_MEMBERSHIP_QUERY => {
                if slice.len() == MembershipQueryType::LEN {
                    // A query of exactly 8 bytes is an IGMPv1/v2 query.
                    // SAFETY: length is exactly MembershipQueryType::LEN (8).
                    IgmpSlice::MembershipQuery(unsafe {
                        MembershipQuerySlice::from_slice_unchecked(slice)
                    })
                } else if slice.len() >= MembershipQueryWithSourcesHeader::LEN {
                    // A query of at least 12 bytes is an IGMPv3 query.
                    // Validate that all declared source addresses (4 bytes
                    // each) are actually present in the payload.
                    // SAFETY: length checked above to be >= 12, so bytes 10..12 exist.
                    let num_of_sources =
                        usize::from(unsafe { get_unchecked_be_u16(slice.as_ptr().add(10)) });
                    let required_len = MembershipQueryWithSourcesHeader::LEN + num_of_sources * 4;
                    if slice.len() < required_len {
                        return Err(err::LenError {
                            required_len,
                            len: slice.len(),
                            len_source: LenSource::Slice,
                            layer: err::Layer::Igmp,
                            layer_start_offset: 0,
                        });
                    }
                    // SAFETY: length checked to be >= MembershipQueryWithSourcesHeader::LEN (12).
                    IgmpSlice::MembershipQueryWithSources(unsafe {
                        MembershipQueryWithSourcesSlice::from_slice_unchecked(slice)
                    })
                } else {
                    // A query with a length of 9-11 bytes is invalid per RFC 9776.
                    return Err(err::LenError {
                        required_len: MembershipQueryWithSourcesHeader::LEN,
                        len: slice.len(),
                        len_source: LenSource::Slice,
                        layer: err::Layer::Igmp,
                        layer_start_offset: 0,
                    });
                }
            }
            // SAFETY: length checked above to be >= 8.
            IGMPV1_TYPE_MEMBERSHIP_REPORT => IgmpSlice::MembershipReportV1(unsafe {
                MembershipReportV1Slice::from_slice_unchecked(slice)
            }),
            // SAFETY: length checked above to be >= 8.
            IGMPV2_TYPE_MEMBERSHIP_REPORT => IgmpSlice::MembershipReportV2(unsafe {
                MembershipReportV2Slice::from_slice_unchecked(slice)
            }),
            // The group records of a v3 report are variable length and are
            // NOT validated here. Use `MembershipReportV3Slice::group_records`
            // to iterate them; the iterator reports length errors lazily.
            // SAFETY: length checked above to be >= 8.
            IGMPV3_TYPE_MEMBERSHIP_REPORT => IgmpSlice::MembershipReportV3(unsafe {
                MembershipReportV3Slice::from_slice_unchecked(slice)
            }),
            // SAFETY: length checked above to be >= 8.
            IGMPV2_TYPE_LEAVE_GROUP => {
                IgmpSlice::LeaveGroup(unsafe { LeaveGroupSlice::from_slice_unchecked(slice) })
            }
            // SAFETY: length checked above to be >= 8.
            _ => IgmpSlice::Unknown(unsafe { IgmpUnknownSlice::from_slice_unchecked(slice) }),
        })
    }

    /// Decode the header values into an [`IgmpHeader`] struct.
    #[inline]
    pub fn header(&self) -> IgmpHeader {
        // `from_slice` already validated the slice, so this cannot fail.
        let (header, _) = IgmpHeader::from_slice(self.slice()).unwrap();
        header
    }

    /// Number of bytes/octets that will be converted into an
    /// [`IgmpHeader`] when [`IgmpSlice::header`] gets called.
    #[inline]
    pub fn header_len(&self) -> usize {
        match self {
            IgmpSlice::MembershipQueryWithSources(_) => MembershipQueryWithSourcesHeader::LEN,
            _ => IgmpHeader::MIN_LEN,
        }
    }

    /// Decode the header values (excluding the checksum) into an [`IgmpType`] enum.
    #[inline]
    pub fn igmp_type(&self) -> IgmpType {
        self.header().igmp_type
    }

    /// Returns the "type" byte value in the IGMP header.
    #[inline]
    pub fn type_u8(&self) -> u8 {
        // SAFETY: from_slice guarantees at least 8 bytes.
        unsafe { *self.slice().get_unchecked(0) }
    }

    /// Returns the second byte of the IGMP header.
    ///
    /// The meaning of this byte depends on the message type:
    /// - Membership Query: Max Response Time (v1: 0, v2: non-zero)
    /// - Membership Report V3: Reserved (0)
    /// - All other types: unused/reserved
    #[inline]
    pub fn max_resp_code_or_reserved(&self) -> u8 {
        // SAFETY: from_slice guarantees at least 8 bytes.
        unsafe { *self.slice().get_unchecked(1) }
    }

    /// Returns the "checksum" value in the IGMP header.
    #[inline]
    pub fn checksum(&self) -> u16 {
        // SAFETY: from_slice guarantees at least 8 bytes.
        unsafe { get_unchecked_be_u16(self.slice().as_ptr().add(2)) }
    }

    /// Returns the bytes from position 4 through 7 in the IGMP header.
    ///
    /// For most message types this is the Group Address. For IGMPv3
    /// Membership Reports, bytes 4-5 are flags and bytes 6-7 are the
    /// Number of Group Records.
    #[inline]
    pub fn bytes4to7(&self) -> [u8; 4] {
        // SAFETY: from_slice guarantees at least 8 bytes.
        let slice = self.slice();
        unsafe {
            [
                *slice.get_unchecked(4),
                *slice.get_unchecked(5),
                *slice.get_unchecked(6),
                *slice.get_unchecked(7),
            ]
        }
    }

    /// Returns a slice to the bytes not covered by `.header()`.
    ///
    /// The contents of the payload depend on the message type:
    ///
    /// | Message Type | Payload Content |
    /// |---|---|
    /// | [`IgmpSlice::MembershipQuery`] (v1/v2) | Nothing (empty) |
    /// | [`IgmpSlice::MembershipQueryWithSources`] (v3) | Source Address list |
    /// | [`IgmpSlice::MembershipReportV1`] | Nothing (empty, unless trailing data) |
    /// | [`IgmpSlice::MembershipReportV2`] | Nothing (empty, unless trailing data) |
    /// | [`IgmpSlice::MembershipReportV3`] | Group Records |
    /// | [`IgmpSlice::LeaveGroup`] | Nothing (empty, unless trailing data) |
    /// | [`IgmpSlice::Unknown`] | Everything after the 8th byte |
    #[inline]
    pub fn payload(&self) -> &'a [u8] {
        match self {
            IgmpSlice::MembershipQuery(s) => {
                // v1/v2 queries are always exactly 8 bytes long.
                let slice = s.slice();
                // SAFETY: from_slice guarantees at least 8 bytes.
                unsafe {
                    core::slice::from_raw_parts(
                        slice.as_ptr().add(MembershipQueryType::LEN),
                        slice.len() - MembershipQueryType::LEN,
                    )
                }
            }
            IgmpSlice::MembershipQueryWithSources(s) => s.payload(),
            IgmpSlice::MembershipReportV1(s) => s.payload(),
            IgmpSlice::MembershipReportV2(s) => s.payload(),
            IgmpSlice::MembershipReportV3(s) => s.payload(),
            IgmpSlice::LeaveGroup(s) => s.payload(),
            IgmpSlice::Unknown(s) => s.payload(),
        }
    }

    /// Returns the slice containing the entire IGMP packet.
    #[inline]
    pub fn slice(&self) -> &'a [u8] {
        match self {
            IgmpSlice::MembershipQuery(s) => s.slice(),
            IgmpSlice::MembershipQueryWithSources(s) => s.slice(),
            IgmpSlice::MembershipReportV1(s) => s.slice(),
            IgmpSlice::MembershipReportV2(s) => s.slice(),
            IgmpSlice::MembershipReportV3(s) => s.slice(),
            IgmpSlice::LeaveGroup(s) => s.slice(),
            IgmpSlice::Unknown(s) => s.slice(),
        }
    }

    /// Verifies the checksum of the IGMP message.
    ///
    /// Unlike ICMPv6 (and TCP/UDP), IGMP does not use an IP pseudo
    /// header. Per RFC 1112, RFC 2236 and RFC 9776 the checksum is the
    /// 16-bit one's complement of the one's complement sum of the whole
    /// IGMP message (header + payload). This is why no IP addresses are
    /// required here (in contrast to [`crate::Icmpv6Slice::is_checksum_valid`]).
    ///
    /// Returns `true` if the checksum stored in the message matches the
    /// one calculated over the entire slice.
    pub fn is_checksum_valid(&self) -> bool {
        checksum::Sum16BitWords::new()
            .add_slice(self.slice())
            .ones_complement()
            == 0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use alloc::{format, vec};
    use proptest::prelude::*;

    #[test]
    fn from_slice_too_small() {
        for bad_len in 0..IgmpHeader::MIN_LEN {
            let bytes = [0u8; 8];
            assert_eq!(
                IgmpSlice::from_slice(&bytes[..bad_len]).unwrap_err(),
                err::LenError {
                    required_len: IgmpHeader::MIN_LEN,
                    len: bad_len,
                    len_source: LenSource::Slice,
                    layer: err::Layer::Igmp,
                    layer_start_offset: 0,
                }
            );
        }
    }

    #[test]
    fn from_slice_query_invalid_length() {
        // 9-11 bytes with type 0x11 should fail
        for bad_len in 9..12 {
            let mut bytes = [0u8; 12];
            bytes[0] = IGMP_TYPE_MEMBERSHIP_QUERY;
            assert!(IgmpSlice::from_slice(&bytes[..bad_len]).is_err());
        }
    }

    #[test]
    fn from_slice_v1_query() {
        // 8 bytes, type 0x11, max_resp_time = 0 => v1 query
        let mut bytes = [0u8; 8];
        bytes[0] = IGMP_TYPE_MEMBERSHIP_QUERY;
        bytes[4] = 224;
        bytes[7] = 1;

        let slice = IgmpSlice::from_slice(&bytes).unwrap();
        assert_eq!(slice.type_u8(), IGMP_TYPE_MEMBERSHIP_QUERY);
        assert_eq!(slice.max_resp_code_or_reserved(), 0);
        assert_eq!(slice.header_len(), 8);
        assert_eq!(slice.payload(), &[]);

        match slice.igmp_type() {
            IgmpType::MembershipQuery(q) => {
                assert_eq!(q.max_response_time, 0);
                assert_eq!(q.group_address.octets, [224, 0, 0, 1]);
            }
            _ => panic!("expected MembershipQuery"),
        }
    }

    #[test]
    fn from_slice_v2_query() {
        // 8 bytes, type 0x11, max_resp_time != 0 => v2 query
        let mut bytes = [0u8; 8];
        bytes[0] = IGMP_TYPE_MEMBERSHIP_QUERY;
        bytes[1] = 100; // max_resp_time
        bytes[4] = 224;
        bytes[7] = 1;

        let slice = IgmpSlice::from_slice(&bytes).unwrap();
        assert_eq!(slice.max_resp_code_or_reserved(), 100);
        assert_eq!(slice.header_len(), 8);

        match slice.igmp_type() {
            IgmpType::MembershipQuery(q) => {
                assert_eq!(q.max_response_time, 100);
            }
            _ => panic!("expected MembershipQuery"),
        }
    }

    #[test]
    fn from_slice_v3_query() {
        // >= 12 bytes, type 0x11 => v3 query
        let mut bytes = [0u8; 16];
        bytes[0] = IGMP_TYPE_MEMBERSHIP_QUERY;
        bytes[1] = 50; // max_resp_code
        bytes[4] = 224;
        bytes[7] = 1;
        bytes[8] = 0x0A; // flags|S|QRV
        bytes[9] = 125; // QQIC
        bytes[10] = 0;
        bytes[11] = 1; // 1 source

        // 12 bytes header + 4 bytes payload (1 source address)
        let slice = IgmpSlice::from_slice(&bytes[..16]).unwrap();
        assert_eq!(slice.header_len(), MembershipQueryWithSourcesHeader::LEN);
        assert_eq!(slice.payload().len(), 4); // 16 - 12

        match slice.igmp_type() {
            IgmpType::MembershipQueryWithSources(q) => {
                assert_eq!(q.max_response_code.0, 50);
                assert_eq!(q.group_address.octets, [224, 0, 0, 1]);
                assert_eq!(q.raw_byte_8, 0x0A);
                assert_eq!(q.qqic, 125);
                assert_eq!(q.num_of_sources, 1);
            }
            _ => panic!("expected MembershipQueryWithSources"),
        }
    }

    #[test]
    fn from_slice_v1_report() {
        let mut bytes = [0u8; 8];
        bytes[0] = IGMPV1_TYPE_MEMBERSHIP_REPORT;
        bytes[4] = 224;
        bytes[7] = 1;

        let slice = IgmpSlice::from_slice(&bytes).unwrap();
        assert_eq!(slice.type_u8(), IGMPV1_TYPE_MEMBERSHIP_REPORT);
        assert_eq!(slice.header_len(), 8);
        assert_eq!(slice.payload(), &[]);

        match slice.igmp_type() {
            IgmpType::MembershipReportV1(r) => {
                assert_eq!(r.group_address.octets, [224, 0, 0, 1]);
            }
            _ => panic!("expected MembershipReportV1"),
        }
    }

    #[test]
    fn from_slice_v2_report() {
        let mut bytes = [0u8; 8];
        bytes[0] = IGMPV2_TYPE_MEMBERSHIP_REPORT;
        bytes[4] = 224;
        bytes[7] = 2;

        let slice = IgmpSlice::from_slice(&bytes).unwrap();
        assert_eq!(slice.type_u8(), IGMPV2_TYPE_MEMBERSHIP_REPORT);

        match slice.igmp_type() {
            IgmpType::MembershipReportV2(r) => {
                assert_eq!(r.group_address.octets, [224, 0, 0, 2]);
            }
            _ => panic!("expected MembershipReportV2"),
        }
    }

    #[test]
    fn from_slice_v3_report() {
        // type 0x22, 8-byte header + group record payload
        let mut bytes = vec![0u8; 16];
        bytes[0] = IGMPV3_TYPE_MEMBERSHIP_REPORT;
        bytes[1] = 0; // reserved
                      // bytes[2..4] = checksum (0)
        bytes[4] = 0; // flags[0]
        bytes[5] = 0; // flags[1]
        bytes[6] = 0; // num_of_records high
        bytes[7] = 1; // num_of_records low = 1
                      // group record (8 bytes)
        bytes[8] = 1; // record type (MODE_IS_INCLUDE)
        bytes[9] = 0; // aux data len
        bytes[10] = 0; // num sources high
        bytes[11] = 0; // num sources low
        bytes[12] = 224;
        bytes[13] = 0;
        bytes[14] = 0;
        bytes[15] = 1;

        let slice = IgmpSlice::from_slice(&bytes).unwrap();
        assert_eq!(slice.type_u8(), IGMPV3_TYPE_MEMBERSHIP_REPORT);
        assert_eq!(slice.header_len(), 8);
        assert_eq!(slice.payload().len(), 8);

        match slice.igmp_type() {
            IgmpType::MembershipReportV3(r) => {
                assert_eq!(r.num_of_records, 1);
                assert_eq!(r.flags, [0, 0]);
            }
            _ => panic!("expected MembershipReportV3"),
        }
    }

    #[test]
    fn from_slice_leave_group() {
        let mut bytes = [0u8; 8];
        bytes[0] = IGMPV2_TYPE_LEAVE_GROUP;
        bytes[4] = 224;
        bytes[7] = 1;

        let slice = IgmpSlice::from_slice(&bytes).unwrap();
        assert_eq!(slice.type_u8(), IGMPV2_TYPE_LEAVE_GROUP);

        match slice.igmp_type() {
            IgmpType::LeaveGroup(l) => {
                assert_eq!(l.group_address.octets, [224, 0, 0, 1]);
            }
            _ => panic!("expected LeaveGroup"),
        }
    }

    #[test]
    fn from_slice_unknown_type() {
        let mut bytes = [0u8; 8];
        bytes[0] = 0xFF;
        bytes[1] = 0xAB;
        bytes[4] = 1;
        bytes[5] = 2;
        bytes[6] = 3;
        bytes[7] = 4;

        let slice = IgmpSlice::from_slice(&bytes).unwrap();
        assert_eq!(slice.type_u8(), 0xFF);

        match slice.igmp_type() {
            IgmpType::Unknown(u) => {
                assert_eq!(u.igmp_type, 0xFF);
                assert_eq!(u.raw_byte_1, 0xAB);
                assert_eq!(u.raw_bytes_4_7, [1, 2, 3, 4]);
            }
            _ => panic!("expected Unknown"),
        }
    }

    #[test]
    fn from_slice_with_trailing_payload() {
        // v1 report with trailing data
        let mut bytes = [0u8; 12];
        bytes[0] = IGMPV1_TYPE_MEMBERSHIP_REPORT;
        bytes[4] = 224;
        bytes[8] = 0xDE;
        bytes[9] = 0xAD;
        bytes[10] = 0xBE;
        bytes[11] = 0xEF;

        let slice = IgmpSlice::from_slice(&bytes).unwrap();
        assert_eq!(slice.header_len(), 8);
        assert_eq!(slice.payload(), &[0xDE, 0xAD, 0xBE, 0xEF]);
    }

    proptest! {
        #[test]
        fn header_roundtrip(bytes in proptest::collection::vec(any::<u8>(), 8..=8)) {
            // Avoid type 0x11 (query) to sidestep the length-based version detection
            let mut bytes = bytes;
            if bytes[0] == IGMP_TYPE_MEMBERSHIP_QUERY {
                bytes[0] = 0xFF;
            }
            let slice = IgmpSlice::from_slice(&bytes).unwrap();
            let header = slice.header();
            assert_eq!(header.checksum, slice.checksum());
        }
    }

    proptest! {
        #[test]
        fn type_u8_accessor(bytes in any::<[u8; 8]>()) {
            // Avoid 0x11 with exactly 8 bytes -> fine, but avoid invalid 9-11
            let slice_result = IgmpSlice::from_slice(&bytes);
            if let Ok(slice) = slice_result {
                assert_eq!(bytes[0], slice.type_u8());
            }
        }
    }

    proptest! {
        #[test]
        fn checksum_accessor(bytes in any::<[u8; 8]>()) {
            if let Ok(slice) = IgmpSlice::from_slice(&bytes) {
                assert_eq!(
                    u16::from_be_bytes([bytes[2], bytes[3]]),
                    slice.checksum()
                );
            }
        }
    }

    proptest! {
        #[test]
        fn bytes4to7_accessor(bytes in any::<[u8; 8]>()) {
            if let Ok(slice) = IgmpSlice::from_slice(&bytes) {
                assert_eq!(
                    [bytes[4], bytes[5], bytes[6], bytes[7]],
                    slice.bytes4to7()
                );
            }
        }
    }

    proptest! {
        #[test]
        fn slice_accessor(bytes in proptest::collection::vec(any::<u8>(), 8..64)) {
            let mut bytes = bytes;
            // Avoid query type to prevent 9-11 byte rejection & source
            // address length validation.
            if bytes[0] == IGMP_TYPE_MEMBERSHIP_QUERY {
                bytes[0] = 0xFF;
            }
            let igmp_slice = IgmpSlice::from_slice(&bytes).unwrap();
            assert_eq!(&bytes[..], igmp_slice.slice());
        }
    }

    proptest! {
        #[test]
        fn clone_eq(bytes in any::<[u8; 12]>()) {
            // Use v3 query type so 12 bytes is valid
            let mut bytes = bytes;
            bytes[0] = IGMP_TYPE_MEMBERSHIP_QUERY;
            // Zero the source count so the 12 byte slice (no sources) is valid.
            bytes[10] = 0;
            bytes[11] = 0;
            let slice = IgmpSlice::from_slice(&bytes).unwrap();
            assert_eq!(slice, slice.clone());
        }
    }

    proptest! {
        #[test]
        fn debug_fmt(bytes in any::<[u8; 8]>()) {
            let mut bytes = bytes;
            if bytes[0] == IGMP_TYPE_MEMBERSHIP_QUERY {
                bytes[0] = 0xFF;
            }
            let slice = IgmpSlice::from_slice(&bytes).unwrap();
            let dbg = format!("{:?}", slice);
            assert!(dbg.contains("Slice"));
        }
    }

    proptest! {
        /// Round-trip an IGMP header + arbitrary payload through the
        /// slice and confirm the decoded header & payload match.
        #[test]
        fn from_slice_roundtrip(
            group in any::<[u8; 4]>(),
            checksum in any::<u16>(),
            payload in proptest::collection::vec(any::<u8>(), 0..64),
        ) {
            let header = IgmpHeader {
                igmp_type: IgmpType::MembershipReportV2(MembershipReportV2Type {
                    group_address: group.into(),
                }),
                checksum,
            };
            let mut bytes = header.to_bytes().to_vec();
            bytes.extend_from_slice(&payload);

            let slice = IgmpSlice::from_slice(&bytes).unwrap();
            assert_eq!(slice.header(), header);
            assert_eq!(slice.header_len(), IgmpHeader::MIN_LEN);
            assert_eq!(slice.payload(), &payload[..]);
            assert_eq!(slice.slice(), &bytes[..]);
        }
    }

    proptest! {
        /// A checksum written by `IgmpHeader::with_checksum` must be
        /// accepted by `IgmpSlice::is_checksum_valid`, and any single
        /// bit flip must invalidate it.
        #[test]
        fn is_checksum_valid_proptest(
            group in any::<[u8; 4]>(),
            payload in proptest::collection::vec(any::<u8>(), 0..64),
            corrupt_idx in any::<u8>(),
        ) {
            let header = IgmpHeader::with_checksum(
                IgmpType::MembershipReportV2(MembershipReportV2Type {
                    group_address: group.into(),
                }),
                &payload,
            );
            let mut bytes = header.to_bytes().to_vec();
            bytes.extend_from_slice(&payload);

            assert!(IgmpSlice::from_slice(&bytes).unwrap().is_checksum_valid());

            // corrupt a single byte -> checksum must fail
            let idx = usize::from(corrupt_idx) % bytes.len();
            bytes[idx] = bytes[idx].wrapping_add(1);
            assert!(!IgmpSlice::from_slice(&bytes).unwrap().is_checksum_valid());
        }
    }

    proptest! {
        /// Reject slices shorter than the minimum IGMP header length.
        #[test]
        fn from_slice_too_short(len in 0usize..8) {
            let bytes = [0u8; 8];
            assert_eq!(
                IgmpSlice::from_slice(&bytes[..len]).unwrap_err(),
                err::LenError {
                    required_len: IgmpHeader::MIN_LEN,
                    len,
                    len_source: LenSource::Slice,
                    layer: err::Layer::Igmp,
                    layer_start_offset: 0,
                }
            );
        }
    }

    #[test]
    fn payload_v3_query_sources() {
        // 12-byte header + 8 bytes (2 source addresses)
        let mut bytes = [0u8; 20];
        bytes[0] = IGMP_TYPE_MEMBERSHIP_QUERY;
        bytes[1] = 10; // max_resp_code
        bytes[10] = 0;
        bytes[11] = 2; // 2 sources
                       // source 1: 10.0.0.1
        bytes[12] = 10;
        bytes[15] = 1;
        // source 2: 10.0.0.2
        bytes[16] = 10;
        bytes[19] = 2;

        let slice = IgmpSlice::from_slice(&bytes).unwrap();
        assert_eq!(slice.header_len(), 12);
        let payload = slice.payload();
        assert_eq!(payload.len(), 8);
        assert_eq!(payload[0], 10); // first byte of source 1
        assert_eq!(payload[3], 1);
        assert_eq!(payload[4], 10); // first byte of source 2
        assert_eq!(payload[7], 2);
    }

    #[test]
    fn group_records_v3_report() {
        // type 0x22, 8-byte header + 2 group records
        let mut bytes = vec![0u8; 8];
        bytes[0] = IGMPV3_TYPE_MEMBERSHIP_REPORT;
        bytes[7] = 2; // num_of_records = 2
                      // group record 1 (0 sources, 0 aux)
        bytes.extend_from_slice(&[1, 0, 0, 0, 224, 0, 0, 1]);
        // group record 2 (1 source, 0 aux)
        bytes.extend_from_slice(&[2, 0, 0, 1, 224, 0, 0, 2, 10, 0, 0, 5]);

        let slice = IgmpSlice::from_slice(&bytes).unwrap();
        let report = match slice {
            IgmpSlice::MembershipReportV3(r) => r,
            other => panic!("expected MembershipReportV3, got {:?}", other),
        };
        let records: alloc::vec::Vec<_> = report.group_records().collect::<Result<_, _>>().unwrap();
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].multicast_address(), [224, 0, 0, 1]);
        assert_eq!(records[0].num_of_sources(), 0);
        assert_eq!(records[1].multicast_address(), [224, 0, 0, 2]);
        assert_eq!(records[1].source_addrs_bytes(), &[10, 0, 0, 5]);
    }

    #[test]
    fn group_records_only_for_v3_report() {
        // a v2 report is not decoded as the v3 report variant
        let mut bytes = [0u8; 8];
        bytes[0] = IGMPV2_TYPE_MEMBERSHIP_REPORT;
        let slice = IgmpSlice::from_slice(&bytes).unwrap();
        assert!(matches!(slice, IgmpSlice::MembershipReportV2(_)));
    }

    #[test]
    fn query_source_addrs_bytes_v3() {
        // 12-byte header + 2 source addresses
        let mut bytes = [0u8; 20];
        bytes[0] = IGMP_TYPE_MEMBERSHIP_QUERY;
        bytes[11] = 2; // 2 sources
        bytes[12] = 10;
        bytes[15] = 1;
        bytes[16] = 10;
        bytes[19] = 2;

        let slice = IgmpSlice::from_slice(&bytes).unwrap();
        let query = match slice {
            IgmpSlice::MembershipQueryWithSources(q) => q,
            other => panic!("expected MembershipQueryWithSources, got {:?}", other),
        };
        assert_eq!(query.source_addrs_bytes(), &[10, 0, 0, 1, 10, 0, 0, 2][..]);
    }

    #[test]
    fn from_slice_v3_query_too_short_sources() {
        // Declares 5 sources but only carries room for 1 -> from_slice
        // must reject the slice as too short.
        let mut bytes = [0u8; 16];
        bytes[0] = IGMP_TYPE_MEMBERSHIP_QUERY;
        bytes[11] = 5; // claims 5 sources (needs 12 + 5*4 = 32 bytes)
        bytes[12] = 10;
        bytes[15] = 1;

        assert_eq!(
            IgmpSlice::from_slice(&bytes).unwrap_err(),
            err::LenError {
                required_len: MembershipQueryWithSourcesHeader::LEN + 5 * 4,
                len: 16,
                len_source: LenSource::Slice,
                layer: err::Layer::Igmp,
                layer_start_offset: 0,
            }
        );
    }

    #[test]
    fn from_slice_v3_query_exact_sources() {
        // Exactly enough room for the declared sources must be accepted.
        let mut bytes = [0u8; 20];
        bytes[0] = IGMP_TYPE_MEMBERSHIP_QUERY;
        bytes[11] = 2; // 2 sources (needs 12 + 2*4 = 20 bytes)

        let slice = IgmpSlice::from_slice(&bytes).unwrap();
        assert!(matches!(slice, IgmpSlice::MembershipQueryWithSources(_)));

        // One byte short must be rejected.
        assert_eq!(
            IgmpSlice::from_slice(&bytes[..19]).unwrap_err(),
            err::LenError {
                required_len: MembershipQueryWithSourcesHeader::LEN + 2 * 4,
                len: 19,
                len_source: LenSource::Slice,
                layer: err::Layer::Igmp,
                layer_start_offset: 0,
            }
        );
    }

    #[test]
    fn from_slice_v3_report_records_not_validated() {
        // Declares 2 records but only provides 1 -> from_slice still
        // succeeds; the group records are validated lazily by the iterator.
        let mut bytes = alloc::vec![0u8; 8];
        bytes[0] = IGMPV3_TYPE_MEMBERSHIP_REPORT;
        bytes[7] = 2; // num_of_records = 2
                      // record 1: type 1, 0 aux, 1 source (8 + 4 = 12 bytes)
        bytes.extend_from_slice(&[1, 0, 0, 1, 224, 0, 0, 1, 10, 0, 0, 5]);
        // record 2 is missing entirely

        let slice = IgmpSlice::from_slice(&bytes).unwrap();
        assert!(matches!(slice, IgmpSlice::MembershipReportV3(_)));
    }

    #[test]
    fn group_records_iter_stops_on_missing_record() {
        // Declares 2 records but only provides 1 -> from_slice succeeds,
        // the iterator yields the first record then a length error.
        let mut bytes = alloc::vec![0u8; 8];
        bytes[0] = IGMPV3_TYPE_MEMBERSHIP_REPORT;
        bytes[7] = 2; // num_of_records = 2
                      // record 1: type 1, 0 aux, 1 source (8 + 4 = 12 bytes)
        bytes.extend_from_slice(&[1, 0, 0, 1, 224, 0, 0, 1, 10, 0, 0, 5]);
        // record 2 is missing entirely

        let report = match IgmpSlice::from_slice(&bytes).unwrap() {
            IgmpSlice::MembershipReportV3(r) => r,
            other => panic!("expected MembershipReportV3, got {:?}", other),
        };
        let mut iter = report.group_records();
        // first record parses fine
        assert!(iter.next().unwrap().is_ok());
        // second record is missing entirely -> error
        assert_eq!(
            iter.next().unwrap().unwrap_err(),
            err::LenError {
                required_len: ReportGroupRecordV3Header::LEN,
                len: 0,
                len_source: LenSource::Slice,
                layer: err::Layer::Igmp,
                layer_start_offset: 0,
            }
        );
        // iteration stops after the error
        assert!(iter.next().is_none());
    }

    #[test]
    fn group_records_iter_stops_on_short_record() {
        // A single record that declares more sources than it carries.
        let mut bytes = alloc::vec![0u8; 8];
        bytes[0] = IGMPV3_TYPE_MEMBERSHIP_REPORT;
        bytes[7] = 1; // 1 record
                      // record: declares 2 sources but provides only 1 (needs 8 + 2*4 = 16).
        bytes.extend_from_slice(&[1, 0, 0, 2, 224, 0, 0, 1, 10, 0, 0, 5]);

        let report = match IgmpSlice::from_slice(&bytes).unwrap() {
            IgmpSlice::MembershipReportV3(r) => r,
            other => panic!("expected MembershipReportV3, got {:?}", other),
        };
        let mut iter = report.group_records();
        assert_eq!(
            iter.next().unwrap().unwrap_err(),
            err::LenError {
                required_len: ReportGroupRecordV3Header::LEN + 2 * 4,
                len: 12, // bytes available for the record
                len_source: LenSource::Slice,
                layer: err::Layer::Igmp,
                layer_start_offset: 0,
            }
        );
        assert!(iter.next().is_none());
    }

    #[test]
    fn group_records_iter_all_records() {
        // Two well formed records are both yielded successfully.
        let mut bytes = alloc::vec![0u8; 8];
        bytes[0] = IGMPV3_TYPE_MEMBERSHIP_REPORT;
        bytes[7] = 2;
        bytes.extend_from_slice(&[1, 0, 0, 0, 224, 0, 0, 1]); // record 1 (no sources)
        bytes.extend_from_slice(&[2, 0, 0, 1, 224, 0, 0, 2, 10, 0, 0, 5]); // record 2 (1 source)

        let report = match IgmpSlice::from_slice(&bytes).unwrap() {
            IgmpSlice::MembershipReportV3(r) => r,
            other => panic!("expected MembershipReportV3, got {:?}", other),
        };
        let records: alloc::vec::Vec<_> = report.group_records().collect::<Result<_, _>>().unwrap();
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].multicast_address(), [224, 0, 0, 1]);
        assert_eq!(records[1].multicast_address(), [224, 0, 0, 2]);
    }

    #[test]
    fn query_source_addrs_bytes_only_for_v3_query() {
        // v1/v2 query (8 bytes) has no source list -> not the v3 variant
        let mut bytes = [0u8; 8];
        bytes[0] = IGMP_TYPE_MEMBERSHIP_QUERY;
        let slice = IgmpSlice::from_slice(&bytes).unwrap();
        assert!(matches!(slice, IgmpSlice::MembershipQuery(_)));

        // non-query type
        let mut bytes = [0u8; 8];
        bytes[0] = IGMPV3_TYPE_MEMBERSHIP_REPORT;
        let slice = IgmpSlice::from_slice(&bytes).unwrap();
        assert!(matches!(slice, IgmpSlice::MembershipReportV3(_)));
    }

    #[test]
    fn query_source_addresses_iter() {
        let mut bytes = [0u8; 20];
        bytes[0] = IGMP_TYPE_MEMBERSHIP_QUERY;
        bytes[11] = 2;
        bytes[12] = 10;
        bytes[15] = 1;
        bytes[16] = 10;
        bytes[19] = 2;

        let slice = IgmpSlice::from_slice(&bytes).unwrap();
        let query = match slice {
            IgmpSlice::MembershipQueryWithSources(q) => q,
            other => panic!("expected MembershipQueryWithSources, got {:?}", other),
        };
        let addrs: alloc::vec::Vec<_> = query.source_addresses().collect();
        assert_eq!(addrs, alloc::vec![[10, 0, 0, 1], [10, 0, 0, 2]]);
    }

    #[test]
    fn is_checksum_valid() {
        // Build a valid IGMPv2 membership report using the header's
        // checksum calculation and confirm the slice validates it.
        let header = IgmpHeader::with_checksum(
            IgmpType::MembershipReportV2(MembershipReportV2Type {
                group_address: [224, 0, 0, 1].into(),
            }),
            &[],
        );
        let bytes = header.to_bytes();
        let slice = IgmpSlice::from_slice(bytes.as_slice()).unwrap();
        assert!(slice.is_checksum_valid());

        // Corrupt a byte -> checksum must fail.
        let mut corrupted = bytes.to_vec();
        corrupted[4] ^= 0xFF;
        let slice = IgmpSlice::from_slice(&corrupted).unwrap();
        assert!(!slice.is_checksum_valid());
    }

    #[test]
    fn is_checksum_valid_v3_report_with_payload() {
        let group_record = [1u8, 0, 0, 0, 224, 0, 0, 1];
        let header = IgmpHeader::with_checksum(
            IgmpType::MembershipReportV3(MembershipReportV3Header {
                flags: [0, 0],
                num_of_records: 1,
            }),
            &group_record,
        );
        let mut bytes = header.to_bytes().to_vec();
        bytes.extend_from_slice(&group_record);

        let slice = IgmpSlice::from_slice(&bytes).unwrap();
        assert!(slice.is_checksum_valid());
    }

    #[test]
    fn packet_builder_roundtrip() {
        use crate::*;

        let builder = PacketBuilder::ethernet2([1, 2, 3, 4, 5, 6], [7, 8, 9, 10, 11, 12])
            .ipv4([192, 168, 1, 1], [192, 168, 1, 2], 20)
            .igmp(IgmpType::MembershipReportV2(MembershipReportV2Type {
                group_address: [224, 0, 0, 1].into(),
            }));

        let mut buffer = alloc::vec::Vec::with_capacity(builder.size(0));
        builder.write(&mut buffer, &[]).unwrap();

        // parse back via SlicedPacket
        let sliced = SlicedPacket::from_ethernet(&buffer).unwrap();
        let igmp = match sliced.transport.unwrap() {
            TransportSlice::Igmp(igmp) => igmp,
            other => panic!("unexpected transport: {:?}", other),
        };
        assert_eq!(igmp.type_u8(), IGMPV2_TYPE_MEMBERSHIP_REPORT);
        // checksum must have been calculated by the builder
        assert!(igmp.is_checksum_valid());
        assert_eq!(
            igmp.header().igmp_type,
            IgmpType::MembershipReportV2(MembershipReportV2Type {
                group_address: [224, 0, 0, 1].into(),
            })
        );

        // parse back via PacketHeaders as well
        let headers = PacketHeaders::from_ethernet_slice(&buffer).unwrap();
        assert!(matches!(headers.transport, Some(TransportHeader::Igmp(_))));
    }
}
