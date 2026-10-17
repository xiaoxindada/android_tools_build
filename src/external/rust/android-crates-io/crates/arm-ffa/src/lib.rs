// SPDX-FileCopyrightText: Copyright The arm-ffa Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

#![cfg_attr(not(test), no_std)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![deny(unsafe_op_in_unsafe_fn)]
#![doc = include_str!("../README.md")]

pub mod boot_info;
mod ffa_v1_1;
mod ffa_v1_2;
pub mod interface;
pub mod interface_args;
pub mod memory_management;
pub mod notification;
pub mod partition_info;

use core::fmt::{self, Debug, Display, Formatter};
pub use interface::Interface;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use thiserror::Error;
pub use uuid::Uuid;

/// Constant for 4K page size. On many occasions the FF-A spec defines memory size as count of 4K
/// pages, regardless of the current translation granule.
pub const FFA_PAGE_SIZE_4K: usize = 4096;

/// Rich error types returned by this module. Should be converted to [`crate::FfaError`] when used
/// with the `FFA_ERROR` interface.
#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
pub enum Error {
    #[error("Unrecognised FF-A function ID {0}")]
    UnrecognisedFunctionId(u32),
    #[error("Unrecognised FF-A feature ID {0}")]
    UnrecognisedFeatureId(u8),
    #[error("Unrecognised FF-A error code {0}")]
    UnrecognisedErrorCode(i32),
    #[error("Unrecognised FF-A Framework Message {0}")]
    UnrecognisedFwkMsg(u32),
    #[error("Invalid FF-A Msg Wait Flag {0}")]
    InvalidMsgWaitFlag(u32),
    #[error("Invalid FF-A Msg Send2 Flag {0}")]
    InvalidMsgSend2Flag(u32),
    #[error("Unrecognised VM availability status {0}")]
    UnrecognisedVmAvailabilityStatus(i32),
    #[error("Unrecognised FF-A Warm Boot Type {0}")]
    UnrecognisedWarmBootType(u32),
    #[error("Invalid version {0}")]
    InvalidVersion(u32),
    #[error("Invalid Information Tag {0}")]
    InvalidInformationTag(u16),
    #[error("Invalid Vm ID")]
    InvalidVmId(u32),
    #[error("Invalid success argument variant")]
    InvalidSuccessArgsVariant,
    #[error("Invalid FF-A version {0} for function ID {1:?}")]
    InvalidVersionForFunctionId(Version, FuncId),
    #[error("Invalid character count {0}")]
    InvalidCharacterCount(u8),
    #[error("Invalid register count: expected {expected}, actual {actual}")]
    InvalidRegisterCount { expected: usize, actual: usize },
    #[error("Invalid version query type {0}")]
    InvalidVersionQueryType(u8),
    #[error("Invalid FF-A version flag {0}")]
    InvalidVersionFlags(u32),
    #[error("Memory management error")]
    MemoryManagementError(#[from] memory_management::Error),
    #[error("Notification error")]
    NotificationError(#[from] notification::Error),
    #[error("Partition info error")]
    PartitionInfoError(#[from] partition_info::Error),
}

impl From<Error> for FfaError {
    fn from(value: Error) -> Self {
        match value {
            Error::UnrecognisedFunctionId(_)
            | Error::UnrecognisedFeatureId(_)
            | Error::InvalidVersionForFunctionId(..)
            | Error::InvalidRegisterCount { .. } => Self::NotSupported,
            Error::InvalidInformationTag(_) => Self::Retry,
            Error::UnrecognisedErrorCode(_)
            | Error::UnrecognisedFwkMsg(_)
            | Error::InvalidVersion(_)
            | Error::InvalidMsgWaitFlag(_)
            | Error::InvalidMsgSend2Flag(_)
            | Error::UnrecognisedVmAvailabilityStatus(_)
            | Error::InvalidVmId(_)
            | Error::UnrecognisedWarmBootType(_)
            | Error::InvalidSuccessArgsVariant
            | Error::InvalidCharacterCount(_)
            | Error::InvalidVersionQueryType(_)
            | Error::InvalidVersionFlags(_) => Self::InvalidParameters,
            Error::MemoryManagementError(_)
            | Error::NotificationError(_)
            | Error::PartitionInfoError(_) => value.into(),
        }
    }
}

/// An FF-A instance is a valid combination of two FF-A components at an exception level boundary.
#[derive(PartialEq, Clone, Copy)]
pub enum Instance {
    /// The instance between the SPMC and SPMD.
    SecurePhysical,
    /// The instance between the SPMC and a physical SP (contains the SP's endpoint ID).
    SecureVirtual(u16),
}

/// Function IDs of the various FF-A interfaces.
#[derive(Clone, Copy, Debug, Eq, IntoPrimitive, PartialEq, TryFromPrimitive)]
#[num_enum(error_type(name = Error, constructor = Error::UnrecognisedFunctionId))]
#[repr(u32)]
pub enum FuncId {
    Error32 = 0x84000060,
    Error64 = 0xc4000060,
    Success32 = 0x84000061,
    Success64 = 0xc4000061,
    Interrupt32 = 0x84000062,
    Interrupt64 = 0xc4000062,
    Version = 0x84000063,
    Features = 0x84000064,
    RxAcquire = 0x84000084,
    RxRelease = 0x84000065,
    RxTxMap32 = 0x84000066,
    RxTxMap64 = 0xc4000066,
    RxTxUnmap = 0x84000067,
    PartitionInfoGet = 0x84000068,
    PartitionInfoGetRegs = 0xc400008b,
    IdGet = 0x84000069,
    SpmIdGet = 0x84000085,
    ConsoleLog32 = 0x8400008a,
    ConsoleLog64 = 0xc400008a,
    MsgWait32 = 0x8400006b,
    MsgWait64 = 0xc400006b,
    Yield32 = 0x8400006c,
    Yield64 = 0xc400006c,
    Run32 = 0x8400006d,
    Run64 = 0xc400006d,
    NormalWorldResume32 = 0x8400007c,
    NormalWorldResume64 = 0xc400007c,
    MsgSend2 = 0x84000086,
    MsgSendDirectReq32 = 0x8400006f,
    MsgSendDirectReq64 = 0xc400006f,
    MsgSendDirectReq64_2 = 0xc400008d,
    MsgSendDirectResp32 = 0x84000070,
    MsgSendDirectResp64 = 0xc4000070,
    MsgSendDirectResp64_2 = 0xc400008e,
    NotificationBitmapCreate = 0x8400007d,
    NotificationBitmapDestroy = 0x8400007e,
    NotificationBind = 0x8400007f,
    NotificationUnbind = 0x84000080,
    NotificationSet = 0x84000081,
    NotificationGet = 0x84000082,
    NotificationInfoGet32 = 0x84000083,
    NotificationInfoGet64 = 0xc4000083,
    El3IntrHandle = 0x8400008c,
    SecondaryEpRegister32 = 0x84000087,
    SecondaryEpRegister64 = 0xc4000087,
    MemDonate32 = 0x84000071,
    MemDonate64 = 0xc4000071,
    MemLend32 = 0x84000072,
    MemLend64 = 0xc4000072,
    MemShare32 = 0x84000073,
    MemShare64 = 0xc4000073,
    MemRetrieveReq32 = 0x84000074,
    MemRetrieveReq64 = 0xc4000074,
    MemRetrieveResp = 0x84000075,
    MemRelinquish = 0x84000076,
    MemReclaim = 0x84000077,
    MemPermGet32 = 0x84000088,
    MemPermGet64 = 0xc4000088,
    MemPermSet32 = 0x84000089,
    MemPermSet64 = 0xc4000089,
    MemOpPause = 0x84000078,
    MemOpResume = 0x84000079,
    MemFragRx = 0x8400007a,
    MemFragTx = 0x8400007b,
}

impl FuncId {
    /// Returns true if this is a 32-bit call, or false if it is a 64-bit call.
    pub fn is_32bit(&self) -> bool {
        u32::from(*self) & (1 << 30) == 0
    }

    /// Returns the FF-A version that has introduced the function ID.
    pub fn minimum_ffa_version(&self) -> Version {
        match self {
            FuncId::Error32
            | FuncId::Success32
            | FuncId::Success64
            | FuncId::Interrupt32
            | FuncId::Version
            | FuncId::Features
            | FuncId::RxRelease
            | FuncId::RxTxMap32
            | FuncId::RxTxMap64
            | FuncId::RxTxUnmap
            | FuncId::PartitionInfoGet
            | FuncId::IdGet
            | FuncId::MsgWait32
            | FuncId::Yield32
            | FuncId::Run32
            | FuncId::NormalWorldResume32
            | FuncId::MsgSendDirectReq32
            | FuncId::MsgSendDirectReq64
            | FuncId::MsgSendDirectResp32
            | FuncId::MsgSendDirectResp64
            | FuncId::MemDonate32
            | FuncId::MemDonate64
            | FuncId::MemLend32
            | FuncId::MemLend64
            | FuncId::MemShare32
            | FuncId::MemShare64
            | FuncId::MemRetrieveReq32
            | FuncId::MemRetrieveReq64
            | FuncId::MemRetrieveResp
            | FuncId::MemRelinquish
            | FuncId::MemReclaim
            | FuncId::MemOpPause
            | FuncId::MemOpResume
            | FuncId::MemFragRx
            | FuncId::MemFragTx => Version(1, 0),

            FuncId::RxAcquire
            | FuncId::SpmIdGet
            | FuncId::MsgSend2
            | FuncId::MemPermGet32
            | FuncId::MemPermGet64
            | FuncId::MemPermSet32
            | FuncId::MemPermSet64
            | FuncId::NotificationBitmapCreate
            | FuncId::NotificationBitmapDestroy
            | FuncId::NotificationBind
            | FuncId::NotificationUnbind
            | FuncId::NotificationSet
            | FuncId::NotificationGet
            | FuncId::NotificationInfoGet32
            | FuncId::NotificationInfoGet64
            | FuncId::SecondaryEpRegister32
            | FuncId::SecondaryEpRegister64 => Version(1, 1),

            FuncId::PartitionInfoGetRegs
            | FuncId::ConsoleLog32
            | FuncId::ConsoleLog64
            | FuncId::MsgSendDirectReq64_2
            | FuncId::MsgSendDirectResp64_2
            | FuncId::El3IntrHandle => Version(1, 2),

            FuncId::Error64
            | FuncId::Interrupt64
            | FuncId::MsgWait64
            | FuncId::Yield64
            | FuncId::Run64
            | FuncId::NormalWorldResume64 => Version(1, 3),
        }
    }
}

/// Error status codes used by the `FFA_ERROR` interface.
#[derive(Clone, Copy, Debug, Eq, Error, IntoPrimitive, PartialEq, TryFromPrimitive)]
#[num_enum(error_type(name = Error, constructor = Error::UnrecognisedErrorCode))]
#[repr(i32)]
pub enum FfaError {
    #[error("Not supported")]
    NotSupported = -1,
    #[error("Invalid parameters")]
    InvalidParameters = -2,
    #[error("No memory")]
    NoMemory = -3,
    #[error("Busy")]
    Busy = -4,
    #[error("Interrupted")]
    Interrupted = -5,
    #[error("Denied")]
    Denied = -6,
    #[error("Retry")]
    Retry = -7,
    #[error("Aborted")]
    Aborted = -8,
    #[error("No data")]
    NoData = -9,
}

/// Collection of helper functions for converting between `Uuid` type and its representations in
/// various FF-A containers.
pub struct UuidHelper;

impl UuidHelper {
    /// Converts byte array into `Uuid`.
    /// Example:
    /// * Input `[a1, a2, a3, a4, b1, b2, c1, c2, d1, d2, d3, d4, d5, d6, d7, d8]`
    /// * Output: `a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8`
    pub fn from_bytes(value: [u8; 16]) -> Uuid {
        Uuid::from_bytes(value)
    }

    /// Converts `Uuid` into byte array.
    /// Example:
    /// * Input: `a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8`
    /// * Output `[a1, a2, a3, a4, b1, b2, c1, c2, d1, d2, d3, d4, d5, d6, d7, d8]`
    pub fn to_bytes(value: Uuid) -> [u8; 16] {
        value.into_bytes()
    }

    /// Creates `Uuid` from four 32 bit register values.
    /// Example:
    /// * Input `[a4a3a2a1, c2c1b2b1, d4d3d2d1, d8d7d6d5]`
    /// * Output: `a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8`
    pub fn from_u32_regs(value: [u32; 4]) -> Uuid {
        Uuid::from_u128_le(
            value[0] as u128
                | (value[1] as u128) << 32
                | (value[2] as u128) << 64
                | (value[3] as u128) << 96,
        )
    }

    /// Converts `Uuid` into four 32 bit register values.
    /// Example:
    /// * Input: `a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8`
    /// * Output `[a4a3a2a1, c2c1b2b1, d4d3d2d1, d8d7d6d5]`
    pub fn to_u32_regs(value: Uuid) -> [u32; 4] {
        let bits = value.to_u128_le();

        [
            bits as u32,
            (bits >> 32) as u32,
            (bits >> 64) as u32,
            (bits >> 96) as u32,
        ]
    }

    /// Creates `Uuid` from a 64 bit register pair.
    /// Example:
    /// * Input `[c2c1b2b1a4a3a2a1, d8d7d6d5d4d3d2d1]`
    /// * Output: `a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8`
    pub fn from_u64_regs(value: [u64; 2]) -> Uuid {
        Uuid::from_u128_le(value[0] as u128 | (value[1] as u128) << 64)
    }

    /// Converts `Uuid` into a 64 bit register pair.
    /// Example:
    /// * Input `[c2c1b2b1a4a3a2a1, d8d7d6d5d4d3d2d1]`
    /// * Output: `a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8`
    pub fn to_u64_regs(value: Uuid) -> [u64; 2] {
        let bits = value.to_u128_le();
        [bits as u64, (bits >> 64) as u64]
    }
}

/// Version number of the FF-A implementation, `.0` is the major, `.1` is minor the version.
#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct Version(pub u16, pub u16);

impl Version {
    // The FF-A spec mandates that bit[31] of a version number must be 0
    const MBZ_BITS: u32 = 1 << 31;

    /// The encoding used if no version is negotiated between a caller and the callee.
    pub const NULL: Version = Version(0, 0);

    /// Returns whether the caller's version (self) is compatible with the callee's version (input
    /// parameter)
    pub fn is_compatible_to(&self, callee_version: Version) -> bool {
        self.0 == callee_version.0 && self.1 <= callee_version.1
    }
}

impl TryFrom<u32> for Version {
    type Error = Error;

    fn try_from(val: u32) -> Result<Self, Self::Error> {
        if (val & Self::MBZ_BITS) != 0 {
            Err(Error::InvalidVersion(val))
        } else {
            Ok(Self((val >> 16) as u16, val as u16))
        }
    }
}

impl From<Version> for u32 {
    fn from(v: Version) -> Self {
        let v_u32 = ((v.0 as u32) << 16) | v.1 as u32;
        assert!(v_u32 & Version::MBZ_BITS == 0);
        v_u32
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.0, self.1)
    }
}

impl Debug for Version {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(self, f)
    }
}

/// Enum for storing the response of an FFA_VERSION request. It can either contain a `Version` or
/// a `NOT_SUPPORTED` error code.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum VersionOut {
    Version(Version),
    NotSupported,
    InvalidParameter,
}

impl VersionOut {
    /// SMCCC return code: The call is not supported by the implementation.
    const SMCCC_NOT_SUPPORTED: i32 = -1;
    /// SMCCC return code: One of the call parameters has a non-supported value.
    const SMCCC_INVALID_PARAMETER: i32 = -3;
}

impl TryFrom<u32> for VersionOut {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value == i32::from(FfaError::NotSupported) as u32 {
            Ok(Self::NotSupported)
        } else {
            Ok(Self::Version(Version::try_from(value)?))
        }
    }
}

impl From<VersionOut> for u32 {
    fn from(value: VersionOut) -> Self {
        // Note: in case of error we return the SMCCC error codes, not the FF-A ones
        match value {
            VersionOut::Version(version) => version.into(),
            VersionOut::NotSupported => VersionOut::SMCCC_NOT_SUPPORTED as u32,
            VersionOut::InvalidParameter => VersionOut::SMCCC_INVALID_PARAMETER as u32,
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use uuid::uuid;

    macro_rules! test_regs_serde {
        ($value:expr, $bytes:expr) => {
            let mut regs = [0u64; 18];
            let mut bytes = [0u64; 18];

            let b: &[u64] = &$bytes;
            bytes[0..(b.len())].copy_from_slice(&b);

            $value.to_regs(Version(1, 2), &mut regs);
            assert_eq!(regs, bytes);

            assert_eq!(Interface::from_regs(Version(1, 2), &bytes), Ok($value));
        };
    }
    pub(crate) use test_regs_serde;

    macro_rules! test_args_serde {
        ($args:expr, $sa:expr) => {
            assert_eq!($args.try_into(), Ok($sa));
            assert_eq!($sa.try_into(), Ok($args));
        };
        ($args:expr, $sa:expr, $flags:expr) => {
            assert_eq!($args.try_into(), Ok($sa));
            assert_eq!(($flags, $sa).try_into(), Ok($args));
        };
    }
    pub(crate) use test_args_serde;

    #[test]
    fn ffa_uuid_helpers() {
        const UUID: Uuid = uuid!("a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8");

        let bytes = [
            0xa1, 0xa2, 0xa3, 0xa4, 0xb1, 0xb2, 0xc1, 0xc2, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6,
            0xd7, 0xd8,
        ];

        assert_eq!(UUID, UuidHelper::from_bytes(bytes));
        assert_eq!(bytes, UuidHelper::to_bytes(UUID));

        let words = [0xa4a3a2a1, 0xc2c1b2b1, 0xd4d3d2d1, 0xd8d7d6d5];
        assert_eq!(UUID, UuidHelper::from_u32_regs(words));
        assert_eq!(words, UuidHelper::to_u32_regs(UUID));

        let pair = [0xc2c1b2b1a4a3a2a1, 0xd8d7d6d5d4d3d2d1];
        assert_eq!(UUID, UuidHelper::from_u64_regs(pair));
        assert_eq!(pair, UuidHelper::to_u64_regs(UUID));
    }
}
