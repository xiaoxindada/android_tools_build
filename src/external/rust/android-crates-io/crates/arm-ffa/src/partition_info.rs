// SPDX-FileCopyrightText: Copyright The arm-ffa Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Implementation of FF-A partition discovery data structures.

use thiserror::Error;
use uuid::Uuid;
use zerocopy::{FromBytes, IntoBytes, transmute};

// This module uses FF-A v1.1 types by default.
// FF-A v1.2 specified some previously reserved bits in the partition info properties field, but
// this doesn't change the descriptor format.
use crate::{
    UuidHelper, Version, ffa_v1_1::partition_info_descriptor, interface_args::SuccessArgs,
};

// Sanity check to catch if the descriptor format is changed.
const _: () = assert!(
    size_of::<crate::ffa_v1_1::partition_info_descriptor>()
        == size_of::<crate::ffa_v1_2::partition_info_descriptor>()
);

/// Rich error types returned by this module. Should be converted to [`crate::FfaError`] when used
/// with the `FFA_ERROR` interface.
#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
pub enum Error {
    #[error("Invalid buffer size")]
    InvalidBufferSize,
    #[error("Malformed descriptor")]
    MalformedDescriptor,
    #[error("Invalid FF-A Partition Info Get Flag {0}")]
    InvalidPartitionInfoGetFlag(u32),
    #[error("Invalid Partition Info Get Regs response")]
    InvalidPartitionInfoGetRegsResponse,
}

impl From<Error> for crate::FfaError {
    fn from(_value: Error) -> Self {
        Self::InvalidParameters
    }
}

/// Type of partition identified by the partition ID.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PartitionIdType {
    /// Partition ID is a PE endpoint ID. Contains the number of execution contexts implemented by
    /// this partition.
    PeEndpoint { execution_ctx_count: u16 },
    /// Partition ID is a SEPID for an independent peripheral device.
    SepidIndep,
    /// Partition ID is a SEPID for an dependent peripheral device. Contains the ID of the proxy
    /// endpoint for a dependent peripheral device.
    SepidDep { proxy_endpoint_id: u16 },
    /// Partition ID is an auxiliary ID.
    Aux,
}

impl PartitionIdType {
    const SHIFT: usize = 4;
    const MASK: u32 = 0b11;
    const PE_ENDPOINT: u32 = 0b00;
    const SEPID_INDEP: u32 = 0b01;
    const SEPID_DEP: u32 = 0b10;
    const AUX: u32 = 0b11;
}

/// Properties of a partition.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PartitionProperties {
    /// The partition supports receipt of direct requests.
    pub support_direct_req_rec: bool,
    /// The partition can send direct requests.
    pub support_direct_req_send: bool,
    /// The partition supports receipt of direct requests via the FFA_MSG_SEND_DIRECT_REQ2 ABI.
    /// Added in FF-A v1.2
    pub support_direct_req2_rec: Option<bool>,
    /// The partition can send direct requests via the FFA_MSG_SEND_DIRECT_REQ2 ABI.
    /// Added in FF-A v1.2
    pub support_direct_req2_send: Option<bool>,
    /// The partition can send and receive indirect messages.
    pub support_indirect_msg: bool,
    /// The partition supports receipt of notifications.
    pub support_notif_rec: bool,
    /// The partition must be informed about each VM that is created by the Hypervisor.
    pub subscribe_vm_created: bool,
    /// The partition must be informed about each VM that is destroyed by the Hypervisor.
    pub subscribe_vm_destroyed: bool,
    /// The partition runs in the AArch64 execution state.
    pub is_aarch64: bool,
}

impl PartitionProperties {
    const SUPPORT_DIRECT_REQ_REC_SHIFT: usize = 0;
    const SUPPORT_DIRECT_REQ_SEND_SHIFT: usize = 1;
    const SUPPORT_INDIRECT_MSG_SHIFT: usize = 2;
    const SUPPORT_NOTIF_REC_SHIFT: usize = 3;
    const SUBSCRIBE_VM_CREATED_SHIFT: usize = 6;
    const SUBSCRIBE_VM_DESTROYED_SHIFT: usize = 7;
    const IS_AARCH64_SHIFT: usize = 8;
    const SUPPORT_DIRECT_REQ2_REC_SHIFT: usize = 9;
    const SUPPORT_DIRECT_REQ2_SEND_SHIFT: usize = 10;
}

fn create_partition_properties(
    version: Version,
    id_type: PartitionIdType,
    properties: PartitionProperties,
) -> (u32, u16) {
    let exec_ctx_count_or_proxy_id = match id_type {
        PartitionIdType::PeEndpoint {
            execution_ctx_count,
        } => execution_ctx_count,
        PartitionIdType::SepidIndep => 0,
        PartitionIdType::SepidDep { proxy_endpoint_id } => proxy_endpoint_id,
        PartitionIdType::Aux => 0,
    };

    let mut prop_bits = match id_type {
        PartitionIdType::PeEndpoint { .. } => {
            let mut p = PartitionIdType::PE_ENDPOINT << PartitionIdType::SHIFT;

            if properties.support_direct_req_rec {
                p |= 1 << PartitionProperties::SUPPORT_DIRECT_REQ_REC_SHIFT;
                if properties.subscribe_vm_created {
                    // TODO: how to handle if ABI is invoked at NS phys instance?
                    p |= 1 << PartitionProperties::SUBSCRIBE_VM_CREATED_SHIFT
                }
                if properties.subscribe_vm_destroyed {
                    // TODO: how to handle if ABI is invoked at NS phys instance?
                    p |= 1 << PartitionProperties::SUBSCRIBE_VM_DESTROYED_SHIFT
                }
            }

            if properties.support_direct_req_send {
                p |= 1 << PartitionProperties::SUPPORT_DIRECT_REQ_SEND_SHIFT
            }

            // For v1.2 and later it's mandatory to specify these properties
            if version >= Version(1, 2) {
                if properties.support_direct_req2_rec.unwrap() {
                    p |= 1 << PartitionProperties::SUPPORT_DIRECT_REQ2_REC_SHIFT
                }

                if properties.support_direct_req2_send.unwrap() {
                    p |= 1 << PartitionProperties::SUPPORT_DIRECT_REQ2_SEND_SHIFT
                }
            }

            if properties.support_indirect_msg {
                p |= 1 << PartitionProperties::SUPPORT_INDIRECT_MSG_SHIFT
            }

            if properties.support_notif_rec {
                p |= 1 << PartitionProperties::SUPPORT_NOTIF_REC_SHIFT
            }

            p
        }
        PartitionIdType::SepidIndep => PartitionIdType::SEPID_INDEP << PartitionIdType::SHIFT,
        PartitionIdType::SepidDep { .. } => PartitionIdType::SEPID_DEP << PartitionIdType::SHIFT,
        PartitionIdType::Aux => PartitionIdType::AUX << PartitionIdType::SHIFT,
    };

    if properties.is_aarch64 {
        prop_bits |= 1 << PartitionProperties::IS_AARCH64_SHIFT
    }

    (prop_bits, exec_ctx_count_or_proxy_id)
}

fn parse_partition_properties(
    version: Version,
    prop_bits: u32,
    id_type: u16,
) -> (PartitionIdType, PartitionProperties) {
    let part_id_type = match (prop_bits >> PartitionIdType::SHIFT) & PartitionIdType::MASK {
        PartitionIdType::PE_ENDPOINT => PartitionIdType::PeEndpoint {
            execution_ctx_count: id_type,
        },
        PartitionIdType::SEPID_INDEP => PartitionIdType::SepidIndep,
        PartitionIdType::SEPID_DEP => PartitionIdType::SepidDep {
            proxy_endpoint_id: id_type,
        },
        PartitionIdType::AUX => PartitionIdType::Aux,
        _ => panic!(), // The match is exhaustive for a 2-bit value
    };

    let mut part_props = PartitionProperties::default();

    if matches!(part_id_type, PartitionIdType::PeEndpoint { .. }) {
        if (prop_bits >> PartitionProperties::SUPPORT_DIRECT_REQ_REC_SHIFT) & 0b1 == 1 {
            part_props.support_direct_req_rec = true;

            if (prop_bits >> PartitionProperties::SUBSCRIBE_VM_CREATED_SHIFT) & 0b1 == 1 {
                part_props.subscribe_vm_created = true;
            }

            if (prop_bits >> PartitionProperties::SUBSCRIBE_VM_DESTROYED_SHIFT) & 0b1 == 1 {
                part_props.subscribe_vm_destroyed = true;
            }
        }

        if (prop_bits >> PartitionProperties::SUPPORT_DIRECT_REQ_SEND_SHIFT) & 0b1 == 1 {
            part_props.support_direct_req_send = true;
        }

        if version >= Version(1, 2) {
            part_props.support_direct_req2_rec =
                Some((prop_bits >> PartitionProperties::SUPPORT_DIRECT_REQ2_REC_SHIFT) & 0b1 == 1);
            part_props.support_direct_req2_send =
                Some((prop_bits >> PartitionProperties::SUPPORT_DIRECT_REQ2_SEND_SHIFT) & 0b1 == 1);
        }

        if (prop_bits >> PartitionProperties::SUPPORT_INDIRECT_MSG_SHIFT) & 0b1 == 1 {
            part_props.support_indirect_msg = true;
        }

        if (prop_bits >> PartitionProperties::SUPPORT_NOTIF_REC_SHIFT) & 0b1 == 1 {
            part_props.support_notif_rec = true;
        }
    }

    if (prop_bits >> PartitionProperties::IS_AARCH64_SHIFT) & 0b1 == 1 {
        part_props.is_aarch64 = true;
    }

    (part_id_type, part_props)
}

/// Partition information descriptor, returned by the `FFA_PARTITION_INFO_GET` interface.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PartitionInfo {
    pub uuid: Uuid,
    pub partition_id: u16,
    pub partition_id_type: PartitionIdType,
    pub props: PartitionProperties,
}

impl PartitionInfo {
    pub const DESC_SIZE: usize = size_of::<partition_info_descriptor>();

    /// Serialize a list of partition information descriptors into a buffer. The `fill_uuid`
    /// parameter controls whether the UUID field of the descriptor will be filled.
    pub fn pack(version: Version, descriptors: &[PartitionInfo], buf: &mut [u8], fill_uuid: bool) {
        assert!((Version(1, 1)..=Version(1, 2)).contains(&version));

        let mut offset = 0;

        for desc in descriptors {
            let mut desc_raw = partition_info_descriptor {
                partition_id: desc.partition_id,
                ..Default::default()
            };

            (
                desc_raw.partition_props,
                desc_raw.exec_ctx_count_or_proxy_id,
            ) = create_partition_properties(version, desc.partition_id_type, desc.props);

            if fill_uuid {
                desc_raw
                    .uuid
                    .copy_from_slice(&UuidHelper::to_bytes(desc.uuid));
            }

            desc_raw.write_to_prefix(&mut buf[offset..]).unwrap();
            offset += Self::DESC_SIZE;
        }
    }
}

/// Flags of the `FFA_PARTITION_INFO_GET` interface.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct PartitionInfoGetFlags {
    pub count_only: bool,
}

impl PartitionInfoGetFlags {
    const RETURN_INFORMATION_TYPE_FLAG: u32 = 1 << 0;
    const MBZ_BITS: u32 = 0xffff_fffe;
}

impl TryFrom<u32> for PartitionInfoGetFlags {
    type Error = Error;

    fn try_from(val: u32) -> Result<Self, Self::Error> {
        if (val & Self::MBZ_BITS) != 0 {
            Err(Error::InvalidPartitionInfoGetFlag(val))
        } else {
            Ok(Self {
                count_only: val & Self::RETURN_INFORMATION_TYPE_FLAG != 0,
            })
        }
    }
}

impl From<PartitionInfoGetFlags> for u32 {
    fn from(flags: PartitionInfoGetFlags) -> Self {
        let mut bits: u32 = 0;
        if flags.count_only {
            bits |= PartitionInfoGetFlags::RETURN_INFORMATION_TYPE_FLAG;
        }
        bits
    }
}

/// `FFA_PARTITION_INFO_GET` specific success args structure.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct SuccessArgsPartitionInfoGet {
    pub count: u32,
    pub size: Option<u32>,
}

impl From<SuccessArgsPartitionInfoGet> for SuccessArgs {
    fn from(value: SuccessArgsPartitionInfoGet) -> Self {
        SuccessArgs::Args32([value.count, value.size.unwrap_or(0), 0, 0, 0, 0])
    }
}

impl TryFrom<(PartitionInfoGetFlags, SuccessArgs)> for SuccessArgsPartitionInfoGet {
    type Error = crate::Error;

    fn try_from(value: (PartitionInfoGetFlags, SuccessArgs)) -> Result<Self, Self::Error> {
        let (flags, value) = value;
        let args = value.try_get_args32()?;

        let size = if !flags.count_only {
            Some(args[1])
        } else {
            None
        };

        Ok(Self {
            count: args[0],
            size,
        })
    }
}

/// `FFA_PARTITION_INFO_GET_REGS` specific success args structure.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct SuccessArgsPartitionInfoGetRegs {
    pub last_index: u16,
    pub current_index: u16,
    pub info_tag: u16,
    pub descriptor_data: [u8; Self::DESCRIPTOR_REG_COUNT * 8],
}

impl SuccessArgsPartitionInfoGetRegs {
    const DESCRIPTOR_REG_COUNT: usize = 15;
    const LAST_INDEX_SHIFT: usize = 0;
    const CURRENT_INDEX_SHIFT: usize = 16;
    const INFO_TAG_SHIFT: usize = 32;
    const SIZE_SHIFT: usize = 48;
}

impl From<SuccessArgsPartitionInfoGetRegs> for SuccessArgs {
    fn from(value: SuccessArgsPartitionInfoGetRegs) -> Self {
        let mut args = [0; 16];

        args[0] = ((value.last_index as u64) << SuccessArgsPartitionInfoGetRegs::LAST_INDEX_SHIFT)
            | ((value.current_index as u64)
                << SuccessArgsPartitionInfoGetRegs::CURRENT_INDEX_SHIFT)
            | ((value.info_tag as u64) << SuccessArgsPartitionInfoGetRegs::INFO_TAG_SHIFT)
            | ((PartitionInfo::DESC_SIZE as u64) << SuccessArgsPartitionInfoGetRegs::SIZE_SHIFT);

        let descriptor_regs: [u64; SuccessArgsPartitionInfoGetRegs::DESCRIPTOR_REG_COUNT] =
            transmute!(value.descriptor_data);
        args[1..].copy_from_slice(&descriptor_regs);

        Self::Args64(args)
    }
}

impl TryFrom<SuccessArgs> for SuccessArgsPartitionInfoGetRegs {
    type Error = crate::Error;

    fn try_from(value: SuccessArgs) -> Result<Self, Self::Error> {
        let args = value.try_get_args64()?;

        // Validate size
        let size = (args[0] >> Self::SIZE_SHIFT) as u16;
        if size as usize != PartitionInfo::DESC_SIZE {
            return Err(Error::InvalidPartitionInfoGetRegsResponse.into());
        }

        // Validate inidices
        let last_index = (args[0] >> Self::LAST_INDEX_SHIFT) as u16;
        let current_index = (args[0] >> Self::CURRENT_INDEX_SHIFT) as u16;
        if last_index < current_index {
            return Err(Error::InvalidPartitionInfoGetRegsResponse.into());
        }

        let info_tag = (args[0] >> Self::INFO_TAG_SHIFT) as u16;

        // Convert registers into byte array
        let descriptor_regs: [u64; SuccessArgsPartitionInfoGetRegs::DESCRIPTOR_REG_COUNT] =
            args[1..].try_into().unwrap();
        let descriptor_data = transmute!(descriptor_regs);

        Ok(Self {
            last_index,
            current_index,
            info_tag,
            descriptor_data,
        })
    }
}

/// Iterator of partition information descriptors.
pub struct PartitionInfoIterator<'a> {
    version: Version,
    buf: &'a [u8],
    offset: usize,
    count: usize,
}

impl<'a> PartitionInfoIterator<'a> {
    /// Create an iterator of partition information descriptors from a buffer.
    pub fn new(version: Version, buf: &'a [u8], count: usize) -> Result<Self, Error> {
        assert!((Version(1, 1)..=Version(1, 2)).contains(&version));

        let Some(total_size) = count.checked_mul(PartitionInfo::DESC_SIZE) else {
            return Err(Error::InvalidBufferSize);
        };

        if buf.len() < total_size {
            return Err(Error::InvalidBufferSize);
        }

        Ok(Self {
            version,
            buf,
            offset: 0,
            count,
        })
    }
}

impl Iterator for PartitionInfoIterator<'_> {
    type Item = Result<PartitionInfo, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count > 0 {
            let offset = self.offset;
            self.offset += PartitionInfo::DESC_SIZE;
            self.count -= 1;

            let Ok(desc_raw) = partition_info_descriptor::ref_from_bytes(
                &self.buf[offset..offset + PartitionInfo::DESC_SIZE],
            ) else {
                return Some(Err(Error::MalformedDescriptor));
            };

            let partition_id = desc_raw.partition_id;

            let (partition_id_type, props) = parse_partition_properties(
                self.version,
                desc_raw.partition_props,
                desc_raw.exec_ctx_count_or_proxy_id,
            );

            let uuid = UuidHelper::from_bytes(desc_raw.uuid);

            let desc = PartitionInfo {
                uuid,
                partition_id,
                partition_id_type,
                props,
            };

            return Some(Ok(desc));
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::uuid;

    // TODO: add tests with a known correct partition info blob

    #[test]
    fn part_info() {
        let desc1 = PartitionInfo {
            uuid: uuid!("12345678-1234-1234-1234-123456789abc"),
            partition_id: 0x8001,
            partition_id_type: PartitionIdType::PeEndpoint {
                execution_ctx_count: 1,
            },
            props: PartitionProperties {
                support_direct_req_rec: true,
                support_direct_req_send: true,
                support_indirect_msg: false,
                support_notif_rec: false,
                subscribe_vm_created: true,
                subscribe_vm_destroyed: true,
                is_aarch64: true,
                support_direct_req2_rec: Some(true),
                support_direct_req2_send: Some(true),
            },
        };

        let desc2 = PartitionInfo {
            uuid: uuid!("abcdef00-abcd-dcba-1234-abcdef012345"),
            partition_id: 0x8002,
            partition_id_type: PartitionIdType::SepidIndep,
            props: PartitionProperties {
                support_direct_req_rec: false,
                support_direct_req_send: false,
                support_indirect_msg: false,
                support_notif_rec: false,
                subscribe_vm_created: false,
                subscribe_vm_destroyed: false,
                is_aarch64: true,
                support_direct_req2_rec: None,
                support_direct_req2_send: None,
            },
        };

        let mut buf = [0u8; 0xff];
        PartitionInfo::pack(Version(1, 2), &[desc1, desc2], &mut buf, true);

        let mut descriptors = PartitionInfoIterator::new(Version(1, 2), &buf, 2).unwrap();
        let desc1_check = descriptors.next().unwrap().unwrap();
        let desc2_check = descriptors.next().unwrap().unwrap();

        assert_eq!(desc1, desc1_check);
        assert_eq!(desc2, desc2_check);

        let expected = [
            // Desc1
            0x01, 0x80, 0x01, 0x00, 0xc3, 0x07, 0x00, 0x00, 0x12, 0x34, 0x56, 0x78, 0x12, 0x34,
            0x12, 0x34, 0x12, 0x34, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, // End of Desc1
            // Desc2
            0x02, 0x80, 0x00, 0x00, 0x10, 0x01, 0x00, 0x00, 0xab, 0xcd, 0xef, 0x00, 0xab, 0xcd,
            0xdc, 0xba, 0x12, 0x34, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, // End of Desc2
        ];

        assert_eq!(buf[0..expected.len()], expected);
    }
}
