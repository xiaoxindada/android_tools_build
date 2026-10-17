// SPDX-FileCopyrightText: Copyright The arm-ffa Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

#![allow(non_camel_case_types)]

use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

/// Table 5.8: Boot information descriptor
#[derive(Default, FromBytes, IntoBytes, KnownLayout, Immutable)]
#[repr(C, packed)]
pub(crate) struct boot_info_descriptor {
    /// Offset 0, length 16: Name of boot information passed to the consumer.
    pub(crate) name: [u8; 16],
    /// Offset 16, length 1: Type of boot information passed to the consumer.
    /// - Bit\[7\]: Boot information type.
    ///   + b’0: Standard boot information.
    ///   + b’1: Implementation defined boot information.
    /// - Bits\[6:0\]: Boot information identifier.
    ///   + Standard boot information (bit\[7\] = b’0).
    ///     * 0: Flattened device tree (FDT).
    ///     * 1: Hand-Off Block (HOB) List.
    ///     * All other identifiers are reserved.
    ///   + Implementation defined identifiers (bit\[7\] = b’ 1).
    ///     * Identifier is defined by the implementation.
    pub(crate) typ: u8,
    /// Offset 17, length 1: Reserved (MBZ)
    pub(crate) reserved: u8,
    /// Offset 18, length 2: Flags to describe properties of boot information associated with this
    /// descriptor.
    /// - Bits\[15:4\]: Reserved (MBZ).
    /// - Bits\[3:2\]: Format of Contents field.
    ///   + b’0: Address of boot information identified by the Name and Type fields.
    ///   + b’1: Value of boot information identified by the Name and Type fields.
    ///   + All other bit encodings are reserved for future use.
    /// - Bits\[1:0\]: Format of Name field.
    ///   + b’0: Null terminated string.
    ///   + b’1: UUID encoded in little-endian byte order.
    ///   + All other bit encodings are reserved for future use.
    pub(crate) flags: u16,
    /// Offset 20, length 4: Size (in bytes) of boot information identified by the Name and Type
    /// fields
    pub(crate) size: u32,
    /// Offset 24, length 8: Value or address of boot information identified by the Name and Type
    /// fields.
    ///
    /// If in the Flags field, bit\[3:2\] = b'0,
    /// * The address has the same attributes as the boot information blob address described in
    ///   5.4.3 Boot information address.
    /// * Size field contains the length (in bytes) of boot information at the specified address.
    ///
    /// If in the Flags field, bit\[3:2\] = b’1,
    /// * Size field contains the exact size of the value specified in this field.
    /// * Size is >=1 bytes and <= 8 bytes.
    pub(crate) contents: u64,
}

/// Table 5.9: Boot information header
#[derive(Default, FromBytes, IntoBytes, KnownLayout, Immutable)]
#[repr(C, packed)]
pub(crate) struct boot_info_header {
    /// Offset 0, length 4: Hexadecimal value 0x0FFA to identify the header
    pub(crate) signature: u32,
    /// Offset 4, length 4: Version of the boot information blob encoded as in FFA_VERSION_GET
    pub(crate) version: u32,
    /// Offset 8, length 4: Size of boot information blob spanning contiguous memory
    pub(crate) boot_info_blob_size: u32,
    /// Offset 12, length 4: Size of each boot information descriptor in the array
    pub(crate) boot_info_desc_size: u32,
    /// Offset 16, length 4: Count of boot information descriptors in the array
    pub(crate) boot_info_desc_count: u32,
    /// Offset 20, length 4: Offset to array of boot information descriptors
    pub(crate) boot_info_array_offset: u32,
    /// Offset 24, length 8: Reserved (MBZ)
    pub(crate) reserved: u64,
}

/// Table 13.37: Partition information descriptor
#[derive(Default, FromBytes, IntoBytes, KnownLayout, Immutable)]
#[repr(C, packed)]
pub(crate) struct partition_info_descriptor {
    /// Offset 0, length 2: 16-bit ID of the partition, stream or auxiliary endpoint.
    pub(crate) partition_id: u16,
    /// Offset 2, length 2:
    /// - Number of execution contexts implemented by this partition if Bit\[5:4\] = b’00 in the
    ///   Partition properties field.
    /// - ID of the proxy endpoint for a dependent peripheral device if Bit\[5:4\] = b’10 in the
    ///   Partition properties field.
    /// - Reserved and MBZ for all other encodings of the Partition properties field.
    pub(crate) exec_ctx_count_or_proxy_id: u16,
    /// Offset 4, length 4: Flags to determine partition properties.
    /// - Bit\[3:0\] has the following encoding if Bit\[5:4\] = b’00. It is Reserved and MBZ otherwise.
    ///   + Bit\[0\] has the following encoding:
    ///     * b’0: Does not support receipt of direct requests
    ///     * b’1: Supports receipt of direct requests. Count of execution contexts must be either 1
    ///       or equal to the number of PEs in the system.
    ///   + bit\[1\] has the following encoding:
    ///     * b’0: Cannot send direct requests.
    ///     * b’1: Can send direct requests.
    ///   + bit\[2\] has the following encoding:
    ///     * b’0: Cannot send and receive indirect messages.
    ///     * b’1: Can send and receive indirect messages.
    ///   + bit\[3\] has the following encoding:
    ///     * b’0: Does not support receipt of notifications.
    ///     * b’1: Supports receipt of notifications.
    /// - bit\[5:4\] has the following encoding:
    ///   + b’00: Partition ID is a PE endpoint ID.
    ///   + b’01: Partition ID is a SEPID for an independent peripheral device.
    ///   + b’10: Partition ID is a SEPID for an dependent peripheral device.
    ///   + b’11: Partition ID is an auxiliary ID.
    /// - bit\[6\] has the following encoding:
    ///   + b’0: Partition must not be informed about each VM that is created by the Hypervisor.
    ///   + b’1: Partition must be informed about each VM that is created by the Hypervisor.
    ///   + bit\[6\] is used only if the following conditions are true. It is Reserved (MBZ) in all
    ///     other scenarios.
    ///     * This ABI is invoked at the Non-secure physical FF-A instance.
    ///     * The partition is an SP that supports receipt of direct requests i.e. Bit\[0\] = b’1.
    /// - bit\[7\] has the following encoding:
    ///   + b’0: Partition must not be informed about each VM that is destroyed by the Hypervisor.
    ///   + b’1: Partition must be informed about each VM that is destroyed by the Hypervisor.
    ///   + bit\[7\] is used only if the following conditions are true. It is Reserved (MBZ) in all
    ///     other scenarios.
    ///     * This ABI is invoked at the Non-secure physical FF-A instance.
    ///     * The partition is an SP that supports receipt of direct requests i.e. Bit\[0\] = b’1.
    /// - bit\[8\] has the following encoding:
    ///   + b’0: Partition runs in the AArch32 execution state.
    ///   + b’1: Partition runs in the AArch64 execution state.
    /// - bit\[31:9\]: Reserved (MBZ).
    pub(crate) partition_props: u32,
    /// Offset 8, length 16:
    /// - UUID of the partition, stream or auxiliary endpoint if the Nil UUID was specified in w1-w4
    ///   as an input parameter.
    /// - This field is reserved and MBZ if a non-Nil UUID was was specified in w1-w4 as an input
    ///   parameter.
    pub(crate) uuid: [u8; 16],
}

/// Table 10.13: Composite memory region descriptor
#[derive(Default, FromBytes, IntoBytes, KnownLayout, Immutable)]
#[repr(C, packed)]
pub(crate) struct composite_memory_region_descriptor {
    /// Offset 0, length 4: Size of the memory region described as the count of 4K pages. Must be
    /// equal to the sum of page counts specified in each constituent memory region descriptor.
    pub(crate) total_page_count: u32,
    /// Offset 4, length 4: Count of address ranges specified using constituent memory region
    /// descriptors
    pub(crate) address_range_count: u32,
    /// Offset 8, length 8: Reserved (MBZ)
    pub(crate) reserved: u64,
}

/// Table 10.14: Constituent memory region descriptor
#[derive(Default, FromBytes, IntoBytes, KnownLayout, Immutable)]
#[repr(C, packed)]
pub(crate) struct constituent_memory_region_descriptor {
    /// Offset 0, length 8: Base VA, PA or IPA of constituent memory region aligned to the page size
    /// (4K) granularity
    pub(crate) address: u64,
    /// Offset 8, length 4: Number of 4K pages in constituent memory region
    pub(crate) page_count: u32,
    /// Offset 12, length 4: Reserved (MBZ)
    pub(crate) reserved: u32,
}

/// Table 10.15: Memory access permissions descriptor
#[derive(Default, FromBytes, IntoBytes, KnownLayout, Immutable)]
#[repr(C, packed)]
pub(crate) struct memory_access_permission_descriptor {
    /// Offset 0, length 2: 16-bit ID of endpoint to which the memory access permissions apply
    pub(crate) endpoint_id: u16,
    /// Offset 2, length 1: Permissions used to access a memory region
    pub(crate) memory_access_permissions: u8,
    /// Offset 3, length 1: ABI specific flags
    pub(crate) flags: u8,
}

/// Table 10.16: Endpoint memory access descriptor
#[derive(Default, FromBytes, IntoBytes, KnownLayout, Immutable)]
#[repr(C, packed)]
pub(crate) struct endpoint_memory_access_descriptor {
    /// Offset 0, length 4: Memory access permissions descriptor as specified in Table 10.15
    pub(crate) access_perm_desc: memory_access_permission_descriptor,
    /// Offset 4, length 4: Offset to the composite memory region descriptor to which the endpoint
    /// access permissions apply. Offset must be calculated from the base address of the data
    /// structure this descriptor is included in. An offset value of 0 indicates that the endpoint
    /// access permissions apply to a memory region description identified by the Handle parameter
    /// specified in the data structure that includes this one.
    pub(crate) composite_offset: u32,
    /// Offset 8, length 8: Reserved (MBZ)
    pub(crate) reserved: u64,
}

/// Table 10.20: Memory transaction descriptor
#[derive(Default, FromBytes, IntoBytes, KnownLayout, Immutable)]
#[repr(C, packed)]
pub(crate) struct memory_transaction_descriptor {
    /// Offset 0, length 2: ID of the Owner endpoint
    pub(crate) sender_endpoint_id: u16,
    /// Offset 2, length 2: Memory region attributes
    pub(crate) memory_region_attributes: u16,
    /// Offset 4, length 4: Flags
    pub(crate) flags: u32,
    /// Offset 8, length 8: Globally unique Handle to identify a memory region
    pub(crate) handle: u64,
    /// Offset 16, length 8: Tag
    pub(crate) tag: u64,
    /// Offset 24, length 4: Size of each endpoint memory access descriptor in the array
    pub(crate) endpoint_mem_access_desc_size: u32,
    /// Offset 28, length 4: Count of endpoint memory access descriptors
    pub(crate) endpoint_mem_access_desc_count: u32,
    /// Offset 32, length 4: 16-byte aligned offset from the base address of this descriptor to the
    /// first element of the Endpoint memory access descriptor array
    pub(crate) endpoint_mem_access_desc_array_offset: u32,
    /// Offset 36, length 12: Reserved (MBZ)
    pub(crate) reserved1: u32,
    pub(crate) reserved2: u64,
}

/// Table 16.25: Descriptor to relinquish a memory region
#[derive(Default, FromBytes, IntoBytes, KnownLayout, Immutable)]
#[repr(C, packed)]
pub(crate) struct memory_relinquish_descriptor {
    /// Offset 0, length 8: Globally unique Handle to identify a memory region
    pub(crate) handle: u64,
    /// Offset 8, length 4: Flags
    pub(crate) flags: u32,
    /// Offset 12, length 4: Count of endpoint ID entries in the Endpoint array
    pub(crate) endpoint_count: u32,
}
