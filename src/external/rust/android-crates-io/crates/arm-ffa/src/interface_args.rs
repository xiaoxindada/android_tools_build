// SPDX-FileCopyrightText: Copyright The arm-ffa Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Data structures for describing the parameters of the various FF-A interfaces.

use crate::{Error, FfaError, FuncId, Version, memory_management};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use zerocopy::{FromBytes, Immutable, IntoBytes};

/// Endpoint ID and vCPU ID pair, used by `FFA_ERROR`, `FFA_INTERRUPT` and `FFA_RUN` interfaces.
#[derive(Debug, Default, Eq, PartialEq, Clone, Copy)]
pub struct TargetInfo {
    pub endpoint_id: u16,
    pub vcpu_id: u16,
}

impl From<u32> for TargetInfo {
    fn from(value: u32) -> Self {
        Self {
            endpoint_id: (value >> 16) as u16,
            vcpu_id: value as u16,
        }
    }
}

impl From<TargetInfo> for u32 {
    fn from(value: TargetInfo) -> Self {
        ((value.endpoint_id as u32) << 16) | value.vcpu_id as u32
    }
}

/// Generic arguments of the `FFA_SUCCESS` interface. The interpretation of the arguments depends on
/// the interface that initiated the request. The application code has knowledge of the request, so
/// it has to convert `SuccessArgs` into/from a specific success args structure that matches the
/// request.
///
/// The current specialized success arguments types are:
/// * `FFA_FEATURES` - [`SuccessArgsFeatures`]
/// * `FFA_ID_GET` - [`SuccessArgsIdGet`]
/// * `FFA_SPM_ID_GET` - [`SuccessArgsSpmIdGet`]
/// * `FFA_PARTITION_INFO_GET` - [`crate::partition_info::SuccessArgsPartitionInfoGet`]
/// * `FFA_PARTITION_INFO_GET_REGS` - [`crate::partition_info::SuccessArgsPartitionInfoGetRegs`]
/// * `FFA_NOTIFICATION_GET` - [`crate::notification::SuccessArgsNotificationGet`]
/// * `FFA_NOTIFICATION_INFO_GET_32` - [`crate::notification::SuccessArgsNotificationInfoGet32`]
/// * `FFA_NOTIFICATION_INFO_GET_64` - [`crate::notification::SuccessArgsNotificationInfoGet64`]
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum SuccessArgs {
    Args32([u32; 6]),
    Args64([u64; 16]),
}

impl SuccessArgs {
    pub(crate) fn try_get_args32(self) -> Result<[u32; 6], Error> {
        match self {
            SuccessArgs::Args32(args) => Ok(args),
            SuccessArgs::Args64(_) => Err(Error::InvalidSuccessArgsVariant),
        }
    }

    pub(crate) fn try_get_args64(self) -> Result<[u64; 16], Error> {
        match self {
            SuccessArgs::Args64(args) => Ok(args),
            SuccessArgs::Args32(_) => Err(Error::InvalidSuccessArgsVariant),
        }
    }
}

/// Entrypoint address argument for `FFA_SECONDARY_EP_REGISTER` interface.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum SecondaryEpRegisterAddr {
    Addr32(u32),
    Addr64(u64),
}

/// Feature IDs used by the `FFA_FEATURES` interface.
#[derive(Clone, Copy, Debug, Eq, IntoPrimitive, PartialEq, TryFromPrimitive)]
#[num_enum(error_type(name = Error, constructor = Error::UnrecognisedFeatureId))]
#[repr(u8)]
pub enum FeatureId {
    NotificationPendingInterrupt = 0x1,
    ScheduleReceiverInterrupt = 0x2,
    ManagedExitInterrupt = 0x3,
}

/// Arguments for the `FFA_FEATURES` interface.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Feature {
    FuncId(FuncId),
    FeatureId(FeatureId),
    Unknown(u32),
}

impl From<u32> for Feature {
    fn from(value: u32) -> Self {
        // Bit[31] is set for all valid FF-A function IDs so we don't have to check it separately
        if let Ok(func_id) = value.try_into() {
            Self::FuncId(func_id)
        } else if let Ok(feat_id) = (value as u8).try_into() {
            Self::FeatureId(feat_id)
        } else {
            Self::Unknown(value)
        }
    }
}

impl From<Feature> for u32 {
    fn from(value: Feature) -> Self {
        match value {
            Feature::FuncId(func_id) => (1 << 31) | func_id as u32,
            Feature::FeatureId(feature_id) => feature_id as u32,
            Feature::Unknown(id) => id,
        }
    }
}

/// `FFA_FEATURES` specific success argument structure. This type needs further specialization based
/// on 'FF-A function ID or Feature ID' field of the preceeding `FFA_FEATURES` request.
#[derive(Debug, Eq, Default, PartialEq, Clone, Copy)]
pub struct SuccessArgsFeatures {
    pub properties: [u32; 2],
}

impl From<SuccessArgsFeatures> for SuccessArgs {
    fn from(value: SuccessArgsFeatures) -> Self {
        Self::Args32([value.properties[0], value.properties[1], 0, 0, 0, 0])
    }
}

impl TryFrom<SuccessArgs> for SuccessArgsFeatures {
    type Error = Error;

    fn try_from(value: SuccessArgs) -> Result<Self, Self::Error> {
        let args = value.try_get_args32()?;

        Ok(Self {
            properties: [args[0], args[1]],
        })
    }
}

/// Version query types for the `FFA_VERSION` interface.
#[derive(Clone, Copy, Debug, Eq, IntoPrimitive, PartialEq, TryFromPrimitive)]
#[num_enum(error_type(name = Error, constructor = Error::InvalidVersionQueryType))]
#[repr(u8)]
pub enum VersionQueryType {
    /// Request to negotiate version specified in Input version number.
    Negotiate = 0b00,
    /// Request to query whether the callee implements a version that is compatible with the version
    /// specified in the Input version number by the caller.
    QueryCompatibility = 0b01,
    /// Request to query the negotiated version for the caller.
    QueryNegotiated = 0b10,
}

/// Flags for the `FFA_VERSION` interface.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VersionFlags {
    pub query_type: VersionQueryType,
}

impl VersionFlags {
    const QUERY_TYPE_MASK: u32 = 0b11;
    const MBZ_BITS: u32 = 0xffff_fffc;
}

impl TryFrom<u32> for VersionFlags {
    type Error = Error;

    fn try_from(val: u32) -> Result<Self, Self::Error> {
        if (val & Self::MBZ_BITS) != 0 {
            Err(Error::InvalidVersionFlags(val))
        } else {
            Ok(Self {
                query_type: VersionQueryType::try_from((val & Self::QUERY_TYPE_MASK) as u8)?,
            })
        }
    }
}

impl From<VersionFlags> for u32 {
    fn from(flags: VersionFlags) -> Self {
        flags.query_type as u32
    }
}

/// RXTX buffer descriptor, used by `FFA_RXTX_MAP`.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum RxTxAddr {
    Addr32 { rx: u32, tx: u32 },
    Addr64 { rx: u64, tx: u64 },
}

/// `FFA_ID_GET` specific success argument structure.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct SuccessArgsIdGet {
    pub id: u16,
}

impl From<SuccessArgsIdGet> for SuccessArgs {
    fn from(value: SuccessArgsIdGet) -> Self {
        SuccessArgs::Args32([value.id as u32, 0, 0, 0, 0, 0])
    }
}

impl TryFrom<SuccessArgs> for SuccessArgsIdGet {
    type Error = Error;

    fn try_from(value: SuccessArgs) -> Result<Self, Self::Error> {
        let args = value.try_get_args32()?;
        Ok(Self { id: args[0] as u16 })
    }
}

/// `FFA_SPM_ID_GET` specific success argument structure.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct SuccessArgsSpmIdGet {
    pub id: u16,
}

impl From<SuccessArgsSpmIdGet> for SuccessArgs {
    fn from(value: SuccessArgsSpmIdGet) -> Self {
        SuccessArgs::Args32([value.id as u32, 0, 0, 0, 0, 0])
    }
}

impl TryFrom<SuccessArgs> for SuccessArgsSpmIdGet {
    type Error = Error;

    fn try_from(value: SuccessArgs) -> Result<Self, Self::Error> {
        let args = value.try_get_args32()?;
        Ok(Self { id: args[0] as u16 })
    }
}

/// Flags field of the `FFA_MSG_SEND2` interface.
#[derive(Debug, Eq, Default, PartialEq, Clone, Copy)]
pub struct MsgSend2Flags {
    pub delay_schedule_receiver: bool,
}

impl MsgSend2Flags {
    const DELAY_SCHEDULE_RECEIVER: u32 = 1 << 1;
    const MBZ_BITS: u32 = 0xffff_fffd;
}

impl TryFrom<u32> for MsgSend2Flags {
    type Error = Error;

    fn try_from(val: u32) -> Result<Self, Self::Error> {
        if (val & Self::MBZ_BITS) != 0 {
            Err(Error::InvalidMsgSend2Flag(val))
        } else {
            Ok(MsgSend2Flags {
                delay_schedule_receiver: val & Self::DELAY_SCHEDULE_RECEIVER != 0,
            })
        }
    }
}

impl From<MsgSend2Flags> for u32 {
    fn from(flags: MsgSend2Flags) -> Self {
        let mut bits: u32 = 0;
        if flags.delay_schedule_receiver {
            bits |= MsgSend2Flags::DELAY_SCHEDULE_RECEIVER;
        }
        bits
    }
}

/// Composite type for capturing success and error return codes for the VM availability messages.
///
/// Error codes are handled by the `FfaError` type. Having a separate type for errors helps using
/// `Result<(), FfaError>`. If a single type would include both success and error values,
/// then `Err(FfaError::Success)` would be incomprehensible.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum VmAvailabilityStatus {
    Success,
    Error(FfaError),
}

impl TryFrom<i32> for VmAvailabilityStatus {
    type Error = Error;
    fn try_from(value: i32) -> Result<Self, <Self as TryFrom<i32>>::Error> {
        Ok(match value {
            0 => Self::Success,
            error_code => Self::Error(FfaError::try_from(error_code)?),
        })
    }
}

impl From<VmAvailabilityStatus> for i32 {
    fn from(value: VmAvailabilityStatus) -> Self {
        match value {
            VmAvailabilityStatus::Success => 0,
            VmAvailabilityStatus::Error(error_code) => error_code.into(),
        }
    }
}

/// Arguments for the Power Warm Boot `FFA_MSG_SEND_DIRECT_REQ` interface.
#[derive(Clone, Copy, Debug, Eq, IntoPrimitive, PartialEq, TryFromPrimitive)]
#[num_enum(error_type(name = Error, constructor = Error::UnrecognisedWarmBootType))]
#[repr(u32)]
pub enum WarmBootType {
    ExitFromSuspendToRam = 0,
    // Corresponds to an exit from any low power state shallower than suspend to RAM.
    ExitFromLowPower = 1,
}

impl WarmBootType {
    #[allow(non_upper_case_globals)]
    #[deprecated = "Ambiguous name. Please use ExitFromSuspendToRam."]
    pub const ExitFromSuspend: WarmBootType = WarmBootType::ExitFromSuspendToRam;
}

/// Arguments for the `FFA_MSG_SEND_DIRECT_{REQ,RESP}` interfaces.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum DirectMsgArgs {
    Args32([u32; 5]),
    Args64([u64; 15]),
    /// Message for forwarding FFA_VERSION call from Normal world to the SPMC
    VersionReq {
        version: Version,
        flags: VersionFlags,
    },
    /// Response message to forwarded FFA_VERSION call from the Normal world
    /// Contains the version returned by the SPMC or None
    VersionResp {
        version: Option<Version>,
    },
    /// Message for a power management operation initiated by a PSCI function
    PowerPsciReq32 {
        // params[i]: Input parameter in w[i] in PSCI function invocation at EL3.
        // params[0]: Function ID.
        params: [u32; 4],
    },
    /// Message for a power management operation initiated by a PSCI function
    PowerPsciReq64 {
        // params[i]: Input parameter in x[i] in PSCI function invocation at EL3.
        // params[0]: Function ID.
        params: [u64; 4],
    },
    /// Message for a warm boot
    PowerWarmBootReq {
        boot_type: WarmBootType,
    },
    /// Response message to indicate return status of the last power management request message
    /// Return error code SUCCESS or DENIED as defined in PSCI spec. Caller is left to do the
    /// parsing of the return status.
    PowerPsciResp {
        psci_status: i32,
    },
    /// Message to signal creation of a VM
    VmCreated {
        // Globally unique Handle to identify a memory region that contains IMPLEMENTATION DEFINED
        // information associated with the created VM.
        // The invalid memory region handle must be specified by the Hypervisor if this field is not
        //  used.
        handle: memory_management::Handle,
        vm_id: u16,
    },
    /// Message to acknowledge creation of a VM
    VmCreatedAck {
        sp_status: VmAvailabilityStatus,
    },
    /// Message to signal destruction of a VM
    VmDestructed {
        // Globally unique Handle to identify a memory region that contains IMPLEMENTATION DEFINED
        // information associated with the created VM.
        // The invalid memory region handle must be specified by the Hypervisor if this field is not
        //  used.
        handle: memory_management::Handle,
        vm_id: u16,
    },
    /// Message to acknowledge destruction of a VM
    VmDestructedAck {
        sp_status: VmAvailabilityStatus,
    },
}

impl DirectMsgArgs {
    // Flags for the `FFA_MSG_SEND_DIRECT_{REQ,RESP}` interfaces.

    pub(crate) const FWK_MSG_BITS: u32 = 1 << 31;
    pub(crate) const VERSION_REQ: u32 = DirectMsgArgs::FWK_MSG_BITS | 0b1000;
    pub(crate) const VERSION_RESP: u32 = DirectMsgArgs::FWK_MSG_BITS | 0b1001;
    pub(crate) const POWER_PSCI_REQ: u32 = DirectMsgArgs::FWK_MSG_BITS;
    pub(crate) const POWER_WARM_BOOT_REQ: u32 = DirectMsgArgs::FWK_MSG_BITS | 0b0001;
    pub(crate) const POWER_PSCI_RESP: u32 = DirectMsgArgs::FWK_MSG_BITS | 0b0010;
    pub(crate) const VM_CREATED: u32 = DirectMsgArgs::FWK_MSG_BITS | 0b0100;
    pub(crate) const VM_CREATED_ACK: u32 = DirectMsgArgs::FWK_MSG_BITS | 0b0101;
    pub(crate) const VM_DESTRUCTED: u32 = DirectMsgArgs::FWK_MSG_BITS | 0b0110;
    pub(crate) const VM_DESTRUCTED_ACK: u32 = DirectMsgArgs::FWK_MSG_BITS | 0b0111;
}

/// Arguments for the `FFA_MSG_SEND_DIRECT_{REQ,RESP}2` interfaces.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct DirectMsg2Args(pub [u64; 14]);

/// Flags field of the `FFA_MSG_WAIT` interface.
#[derive(Debug, Default, Eq, PartialEq, Clone, Copy)]
pub struct MsgWaitFlags {
    pub retain_rx_buffer: bool,
}

impl MsgWaitFlags {
    const RETAIN_RX_BUFFER: u32 = 0x01;
    const MBZ_BITS: u32 = 0xfffe;
}

impl TryFrom<u32> for MsgWaitFlags {
    type Error = Error;

    fn try_from(val: u32) -> Result<Self, Self::Error> {
        if (val & Self::MBZ_BITS) != 0 {
            Err(Error::InvalidMsgWaitFlag(val))
        } else {
            Ok(MsgWaitFlags {
                retain_rx_buffer: val & Self::RETAIN_RX_BUFFER != 0,
            })
        }
    }
}

impl From<MsgWaitFlags> for u32 {
    fn from(flags: MsgWaitFlags) -> Self {
        let mut bits: u32 = 0;
        if flags.retain_rx_buffer {
            bits |= MsgWaitFlags::RETAIN_RX_BUFFER;
        }
        bits
    }
}

/// Descriptor for a dynamically allocated memory buffer that contains the memory transaction
/// descriptor.
///
/// Used by `FFA_MEM_{DONATE,LEND,SHARE,RETRIEVE_REQ}` interfaces, only when the TX buffer is not
/// used to transmit the transaction descriptor.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum MemOpBuf {
    Buf32 { addr: u32, page_cnt: u32 },
    Buf64 { addr: u64, page_cnt: u32 },
}

/// Memory address argument for `FFA_MEM_PERM_{GET,SET}` interfaces.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum MemAddr {
    Addr32(u32),
    Addr64(u64),
}

impl MemAddr {
    /// Returns the contained address.
    pub fn address(&self) -> u64 {
        match self {
            MemAddr::Addr32(a) => (*a).into(),
            MemAddr::Addr64(a) => *a,
        }
    }
}

/// Argument for the `FFA_CONSOLE_LOG` interface.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum ConsoleLogChars {
    Chars32(ConsoleLogChars32),
    Chars64(ConsoleLogChars64),
}

/// Generic type for storing `FFA_CONSOLE_LOG` character payload and its length in bytes.
#[derive(Debug, Default, Eq, PartialEq, Clone, Copy)]
pub struct LogChars<T>
where
    T: IntoBytes + FromBytes + Immutable,
{
    pub(crate) char_cnt: u8,
    pub(crate) char_lists: T,
}

impl<T> LogChars<T>
where
    T: IntoBytes + FromBytes + Immutable,
{
    pub(crate) const MAX_LENGTH: u8 = core::mem::size_of::<T>() as u8;

    /// Returns true if there are no characters in the structure.
    pub fn empty(&self) -> bool {
        self.char_cnt == 0
    }

    /// Returns true if the structure is full.
    pub fn full(&self) -> bool {
        self.char_cnt as usize >= core::mem::size_of::<T>()
    }

    /// Returns the payload bytes.
    pub fn bytes(&self) -> &[u8] {
        &self.char_lists.as_bytes()[..self.char_cnt as usize]
    }

    /// Append byte slice to the end of the characters.
    pub fn push(&mut self, source: &[u8]) -> usize {
        let empty_area = &mut self.char_lists.as_mut_bytes()[self.char_cnt.into()..];
        let len = empty_area.len().min(source.len());

        empty_area[..len].copy_from_slice(&source[..len]);
        self.char_cnt += len as u8;

        len
    }
}

/// Specialized type for 32-bit `FFA_CONSOLE_LOG` payload.
pub type ConsoleLogChars32 = LogChars<[u32; 6]>;

/// Specialized type for 64-bit `FFA_CONSOLE_LOG` payload.
pub type ConsoleLogChars64 = LogChars<[u64; 16]>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn log_chars_empty() {
        assert!(
            ConsoleLogChars64 {
                char_cnt: 0,
                char_lists: [0; 16]
            }
            .empty()
        )
    }

    #[test]
    fn log_chars_push() {
        let mut console = ConsoleLogChars64 {
            char_cnt: 0,
            char_lists: [0; 16],
        };

        assert_eq!(console.push("hello world!".as_bytes()), 12);

        assert_eq!(console.char_cnt, 12);
        assert_eq!(&console.bytes()[0..12], "hello world!".as_bytes());
        assert!(!console.empty());
    }

    #[test]
    fn log_chars_full() {
        let mut console = ConsoleLogChars64 {
            char_cnt: 0,
            char_lists: [0; 16],
        };

        assert_eq!(console.push(&[97; 128]), 128);

        assert!(console.full());
    }

    #[test]
    fn success_args_invalid_variants() {
        assert!(SuccessArgs::Args32([0; 6]).try_get_args64().is_err());
        assert!(SuccessArgs::Args64([0; 16]).try_get_args32().is_err());
    }
}
