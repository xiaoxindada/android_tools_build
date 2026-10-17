// SPDX-FileCopyrightText: Copyright The arm-ffa Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

#![allow(non_camel_case_types)]

use crate::ffa_v1_1;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

/// Table 6.1: Partition information descriptor
/// Table 6.2: Partition properties descriptor
/// The following changes are introduced by FF-A v1.2 to the partition properties field:
/// - bit\[10:9\]: Has the following encoding if Bits\[5:4\] = b’00. Reserved (MBZ) otherwise.
///   + bit\[9\] has the following encoding:
///     * b’0: Cannot receive Direct requests via the FFA_MSG_SEND_DIRECT_REQ2 ABI.
///     * b’1: Can receive Direct requests via the FFA_MSG_SEND_DIRECT_REQ2 ABI.
///   + bit\[10\] has the following encoding:
///     * b’0: Cannot send Direct requests via the FFA_MSG_SEND_DIRECT_REQ2 ABI.
///     * b’1: Can send Direct requests via the FFA_MSG_SEND_DIRECT_REQ2 ABI.
/// - bit\[31:11\]: Reserved (MBZ).
///
/// This doesn't change the descriptor format so we can just use an alias.
#[allow(unused)]
pub(crate) type partition_info_descriptor = ffa_v1_1::partition_info_descriptor;

/// FF-A Memory Management Protocol Table 1.16: Endpoint memory access descriptor
#[derive(Default, FromBytes, IntoBytes, KnownLayout, Immutable)]
#[repr(C, packed)]
#[allow(unused)]
pub(crate) struct endpoint_memory_access_descriptor {
    /// Offset 0, length 4: Memory access permissions descriptor as specified in Table 10.15
    pub(crate) access_perm_desc: ffa_v1_1::memory_access_permission_descriptor,
    /// Offset 4, length 4: Offset to the composite memory region descriptor to which the endpoint
    /// access permissions apply. Offset must be calculated from the base address of the data
    /// structure this descriptor is included in. An offset value of 0 indicates that the endpoint
    /// access permissions apply to a memory region description identified by the Handle parameter
    /// specified in the data structure that includes this one.
    pub(crate) composite_offset: u32,
    /// Offset 8, length 16: Implementation defined information
    pub(crate) impdef_info: [u8; 16],
    /// Offset 24, length 8: Reserved (MBZ)
    pub(crate) reserved: u64,
}
