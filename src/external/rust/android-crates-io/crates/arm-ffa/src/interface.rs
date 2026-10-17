// SPDX-FileCopyrightText: Copyright The arm-ffa Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! FF-A interface message types and their register-level serialization.

use crate::{
    Error, FfaError, FuncId, UuidHelper, Version, VersionOut,
    interface_args::{
        ConsoleLogChars, ConsoleLogChars32, ConsoleLogChars64, DirectMsg2Args, DirectMsgArgs,
        Feature, MemAddr, MemOpBuf, MsgSend2Flags, MsgWaitFlags, RxTxAddr, SecondaryEpRegisterAddr,
        SuccessArgs, TargetInfo, VersionFlags, WarmBootType,
    },
    memory_management,
    notification::{NotificationBindFlags, NotificationGetFlags, NotificationSetFlags},
    partition_info::PartitionInfoGetFlags,
};
use uuid::Uuid;

/// FF-A "message types", the terminology used by the spec is "interfaces".
///
/// The interfaces are used by FF-A components for communication at an FF-A instance. The spec also
/// describes the valid FF-A instances and conduits for each interface.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Interface {
    Error {
        target_info: TargetInfo,
        error_code: FfaError,
        error_arg: u32,
        is_32bit: bool,
    },
    Success {
        target_info: TargetInfo,
        args: SuccessArgs,
    },
    Interrupt {
        target_info: TargetInfo,
        interrupt_id: u32,
        is_32bit: bool,
    },
    Version {
        input_version: Version,
        flags: VersionFlags,
    },
    VersionOut {
        output_version: VersionOut,
    },
    Features {
        feat_id: Feature,
        input_properties: u32,
    },
    RxAcquire {
        vm_id: u16,
    },
    RxRelease {
        vm_id: u16,
    },
    RxTxMap {
        addr: RxTxAddr,
        page_cnt: u32,
    },
    RxTxUnmap {
        id: u16,
    },
    PartitionInfoGet {
        uuid: Uuid,
        flags: PartitionInfoGetFlags,
    },
    PartitionInfoGetRegs {
        uuid: Uuid,
        start_index: u16,
        info_tag: u16,
    },
    IdGet,
    SpmIdGet,
    MsgWait {
        flags: MsgWaitFlags,
        is_32bit: bool,
    },
    Yield {
        is_32bit: bool,
    },
    Run {
        target_info: TargetInfo,
        is_32bit: bool,
    },
    NormalWorldResume {
        is_32bit: bool,
    },
    SecondaryEpRegister {
        entrypoint: SecondaryEpRegisterAddr,
    },
    MsgSend2 {
        sender_vm_id: u16,
        flags: MsgSend2Flags,
    },
    MsgSendDirectReq {
        src_id: u16,
        dst_id: u16,
        args: DirectMsgArgs,
    },
    MsgSendDirectResp {
        src_id: u16,
        dst_id: u16,
        args: DirectMsgArgs,
    },
    MsgSendDirectReq2 {
        src_id: u16,
        dst_id: u16,
        uuid: Uuid,
        args: DirectMsg2Args,
    },
    MsgSendDirectResp2 {
        src_id: u16,
        dst_id: u16,
        args: DirectMsg2Args,
    },
    MemDonate {
        total_len: u32,
        frag_len: u32,
        buf: Option<MemOpBuf>,
    },
    MemLend {
        total_len: u32,
        frag_len: u32,
        buf: Option<MemOpBuf>,
    },
    MemShare {
        total_len: u32,
        frag_len: u32,
        buf: Option<MemOpBuf>,
    },
    MemRetrieveReq {
        total_len: u32,
        frag_len: u32,
        buf: Option<MemOpBuf>,
    },
    MemRetrieveResp {
        total_len: u32,
        frag_len: u32,
    },
    MemRelinquish,
    MemReclaim {
        handle: memory_management::Handle,
        flags: memory_management::MemReclaimFlags,
    },
    MemPermGet {
        addr: MemAddr,
        /// The actual number of pages queried by the call.  It is calculated by adding one to the
        /// corresponding register's value, i.e. zero in the register means one page. For FF-A v1.2
        /// and lower the register value MBZ, so the page count is always 1. For higher versions the
        /// page count can be any nonzero value.
        page_cnt: u32,
    },
    MemPermSet {
        addr: MemAddr,
        page_cnt: u32,
        mem_perm: memory_management::MemPermissionsGetSet,
    },
    MemOpPause {
        handle: memory_management::Handle,
    },
    MemOpResume {
        handle: memory_management::Handle,
    },
    MemFragRx {
        handle: memory_management::Handle,
        frag_offset: u32,
        endpoint_id: u16,
    },
    MemFragTx {
        handle: memory_management::Handle,
        frag_len: u32,
        endpoint_id: u16,
    },
    ConsoleLog {
        chars: ConsoleLogChars,
    },
    NotificationBitmapCreate {
        vm_id: u16,
        vcpu_cnt: u32,
    },
    NotificationBitmapDestroy {
        vm_id: u16,
    },
    NotificationBind {
        sender_id: u16,
        receiver_id: u16,
        flags: NotificationBindFlags,
        bitmap: u64,
    },
    NotificationUnbind {
        sender_id: u16,
        receiver_id: u16,
        bitmap: u64,
    },
    NotificationSet {
        sender_id: u16,
        receiver_id: u16,
        flags: NotificationSetFlags,
        bitmap: u64,
    },
    NotificationGet {
        vcpu_id: u16,
        endpoint_id: u16,
        flags: NotificationGetFlags,
    },
    NotificationInfoGet {
        is_32bit: bool,
    },
    El3IntrHandle,
}

impl Interface {
    /// Returns the function ID for the call, if it has one.
    pub fn function_id(&self) -> Option<FuncId> {
        match self {
            Interface::Error { is_32bit, .. } => Some(if *is_32bit {
                FuncId::Error32
            } else {
                FuncId::Error64
            }),
            Interface::Success { args, .. } => match args {
                SuccessArgs::Args32(..) => Some(FuncId::Success32),
                SuccessArgs::Args64(..) => Some(FuncId::Success64),
            },
            Interface::Interrupt { is_32bit, .. } => Some(if *is_32bit {
                FuncId::Interrupt32
            } else {
                FuncId::Interrupt64
            }),
            Interface::Version { .. } => Some(FuncId::Version),
            Interface::VersionOut { .. } => None,
            Interface::Features { .. } => Some(FuncId::Features),
            Interface::RxAcquire { .. } => Some(FuncId::RxAcquire),
            Interface::RxRelease { .. } => Some(FuncId::RxRelease),
            Interface::RxTxMap { addr, .. } => match addr {
                RxTxAddr::Addr32 { .. } => Some(FuncId::RxTxMap32),
                RxTxAddr::Addr64 { .. } => Some(FuncId::RxTxMap64),
            },
            Interface::RxTxUnmap { .. } => Some(FuncId::RxTxUnmap),
            Interface::PartitionInfoGet { .. } => Some(FuncId::PartitionInfoGet),
            Interface::PartitionInfoGetRegs { .. } => Some(FuncId::PartitionInfoGetRegs),
            Interface::IdGet => Some(FuncId::IdGet),
            Interface::SpmIdGet => Some(FuncId::SpmIdGet),
            Interface::MsgWait { is_32bit, .. } => Some(if *is_32bit {
                FuncId::MsgWait32
            } else {
                FuncId::MsgWait64
            }),
            Interface::Yield { is_32bit } => Some(if *is_32bit {
                FuncId::Yield32
            } else {
                FuncId::Yield64
            }),
            Interface::Run { is_32bit, .. } => Some(if *is_32bit {
                FuncId::Run32
            } else {
                FuncId::Run64
            }),
            Interface::NormalWorldResume { is_32bit } => Some(if *is_32bit {
                FuncId::NormalWorldResume32
            } else {
                FuncId::NormalWorldResume64
            }),
            Interface::SecondaryEpRegister { entrypoint } => match entrypoint {
                SecondaryEpRegisterAddr::Addr32 { .. } => Some(FuncId::SecondaryEpRegister32),
                SecondaryEpRegisterAddr::Addr64 { .. } => Some(FuncId::SecondaryEpRegister64),
            },
            Interface::MsgSend2 { .. } => Some(FuncId::MsgSend2),
            Interface::MsgSendDirectReq { args, .. } => match args {
                DirectMsgArgs::Args32(_) => Some(FuncId::MsgSendDirectReq32),
                DirectMsgArgs::Args64(_) => Some(FuncId::MsgSendDirectReq64),
                DirectMsgArgs::VersionReq { .. } => Some(FuncId::MsgSendDirectReq32),
                DirectMsgArgs::PowerPsciReq32 { .. } => Some(FuncId::MsgSendDirectReq32),
                DirectMsgArgs::PowerPsciReq64 { .. } => Some(FuncId::MsgSendDirectReq64),
                DirectMsgArgs::PowerWarmBootReq { .. } => Some(FuncId::MsgSendDirectReq32),
                DirectMsgArgs::VmCreated { .. } => Some(FuncId::MsgSendDirectReq32),
                DirectMsgArgs::VmDestructed { .. } => Some(FuncId::MsgSendDirectReq32),
                _ => panic!("Invalid direct request arguments: {:#?}", args),
            },
            Interface::MsgSendDirectResp { args, .. } => match args {
                DirectMsgArgs::Args32(_) => Some(FuncId::MsgSendDirectResp32),
                DirectMsgArgs::Args64(_) => Some(FuncId::MsgSendDirectResp64),
                DirectMsgArgs::VersionResp { .. } => Some(FuncId::MsgSendDirectResp32),
                DirectMsgArgs::PowerPsciResp { .. } => Some(FuncId::MsgSendDirectResp32),
                DirectMsgArgs::VmCreatedAck { .. } => Some(FuncId::MsgSendDirectResp32),
                DirectMsgArgs::VmDestructedAck { .. } => Some(FuncId::MsgSendDirectResp32),
                _ => panic!("Invalid direct response arguments: {:#?}", args),
            },
            Interface::MsgSendDirectReq2 { .. } => Some(FuncId::MsgSendDirectReq64_2),
            Interface::MsgSendDirectResp2 { .. } => Some(FuncId::MsgSendDirectResp64_2),
            Interface::MemDonate { buf, .. } => match buf {
                Some(MemOpBuf::Buf64 { .. }) => Some(FuncId::MemDonate64),
                _ => Some(FuncId::MemDonate32),
            },
            Interface::MemLend { buf, .. } => match buf {
                Some(MemOpBuf::Buf64 { .. }) => Some(FuncId::MemLend64),
                _ => Some(FuncId::MemLend32),
            },
            Interface::MemShare { buf, .. } => match buf {
                Some(MemOpBuf::Buf64 { .. }) => Some(FuncId::MemShare64),
                _ => Some(FuncId::MemShare32),
            },
            Interface::MemRetrieveReq { buf, .. } => match buf {
                Some(MemOpBuf::Buf64 { .. }) => Some(FuncId::MemRetrieveReq64),
                _ => Some(FuncId::MemRetrieveReq32),
            },
            Interface::MemRetrieveResp { .. } => Some(FuncId::MemRetrieveResp),
            Interface::MemRelinquish => Some(FuncId::MemRelinquish),
            Interface::MemReclaim { .. } => Some(FuncId::MemReclaim),
            Interface::MemPermGet { addr, .. } => match addr {
                MemAddr::Addr32(_) => Some(FuncId::MemPermGet32),
                MemAddr::Addr64(_) => Some(FuncId::MemPermGet64),
            },
            Interface::MemPermSet { addr, .. } => match addr {
                MemAddr::Addr32(_) => Some(FuncId::MemPermSet32),
                MemAddr::Addr64(_) => Some(FuncId::MemPermSet64),
            },
            Interface::MemOpPause { .. } => Some(FuncId::MemOpPause),
            Interface::MemOpResume { .. } => Some(FuncId::MemOpResume),
            Interface::MemFragRx { .. } => Some(FuncId::MemFragRx),
            Interface::MemFragTx { .. } => Some(FuncId::MemFragTx),
            Interface::ConsoleLog { chars, .. } => match chars {
                ConsoleLogChars::Chars32(_) => Some(FuncId::ConsoleLog32),
                ConsoleLogChars::Chars64(_) => Some(FuncId::ConsoleLog64),
            },
            Interface::NotificationBitmapCreate { .. } => Some(FuncId::NotificationBitmapCreate),
            Interface::NotificationBitmapDestroy { .. } => Some(FuncId::NotificationBitmapDestroy),
            Interface::NotificationBind { .. } => Some(FuncId::NotificationBind),
            Interface::NotificationUnbind { .. } => Some(FuncId::NotificationUnbind),
            Interface::NotificationSet { .. } => Some(FuncId::NotificationSet),
            Interface::NotificationGet { .. } => Some(FuncId::NotificationGet),
            Interface::NotificationInfoGet { is_32bit } => match is_32bit {
                true => Some(FuncId::NotificationInfoGet32),
                false => Some(FuncId::NotificationInfoGet64),
            },
            Interface::El3IntrHandle => Some(FuncId::El3IntrHandle),
        }
    }

    /// Returns true if this is a 32-bit call, or false if it is a 64-bit call.
    pub fn is_32bit(&self) -> bool {
        if matches!(self, Self::VersionOut { .. }) {
            return true;
        }

        self.function_id().unwrap().is_32bit()
    }

    /// Returns the FF-A version that has introduced the function ID.
    pub fn minimum_ffa_version(&self) -> Version {
        if matches!(self, Self::VersionOut { .. }) {
            return Version(1, 0);
        }

        self.function_id().unwrap().minimum_ffa_version()
    }

    /// Parse interface from register contents. The caller must ensure that the `regs` argument has
    /// the correct length: at least 8 registers for SMC32 calls and at least 18 for SMC64 calls.
    pub fn from_regs(version: Version, regs: &[u64]) -> Result<Self, Error> {
        let func_id = FuncId::try_from(regs[0] as u32)?;

        if version < func_id.minimum_ffa_version() {
            return Err(Error::InvalidVersionForFunctionId(version, func_id));
        }

        if func_id.is_32bit() {
            if regs.len() < 8 {
                return Err(Error::InvalidRegisterCount {
                    expected: 8,
                    actual: regs.len(),
                });
            }

            Interface::unpack_regs8(version, func_id, regs.first_chunk().unwrap())
        } else {
            if regs.len() < 18 {
                return Err(Error::InvalidRegisterCount {
                    expected: 18,
                    actual: regs.len(),
                });
            }

            match func_id {
                FuncId::ConsoleLog64
                | FuncId::Success64
                | FuncId::MsgSendDirectReq64
                | FuncId::MsgSendDirectResp64
                | FuncId::MsgSendDirectReq64_2
                | FuncId::MsgSendDirectResp64_2 => {
                    Interface::unpack_regs18(version, func_id, regs.first_chunk().unwrap())
                }
                _ => Interface::unpack_regs8(version, func_id, regs.first_chunk().unwrap()),
            }
        }
    }

    fn unpack_regs8(version: Version, func_id: FuncId, regs: &[u64; 8]) -> Result<Self, Error> {
        let msg = match func_id {
            FuncId::Error32 | FuncId::Error64 => Self::Error {
                is_32bit: matches!(func_id, FuncId::Error32),
                target_info: (regs[1] as u32).into(),
                error_code: FfaError::try_from(regs[2] as i32)?,
                error_arg: regs[3] as u32,
            },
            FuncId::Success32 => Self::Success {
                target_info: (regs[1] as u32).into(),
                args: SuccessArgs::Args32([
                    regs[2] as u32,
                    regs[3] as u32,
                    regs[4] as u32,
                    regs[5] as u32,
                    regs[6] as u32,
                    regs[7] as u32,
                ]),
            },
            FuncId::Interrupt32 | FuncId::Interrupt64 => Self::Interrupt {
                is_32bit: matches!(func_id, FuncId::Interrupt32),
                target_info: (regs[1] as u32).into(),
                interrupt_id: regs[2] as u32,
            },
            FuncId::Version => Self::Version {
                input_version: (regs[1] as u32).try_into()?,
                flags: (regs[2] as u32).try_into()?,
            },
            FuncId::Features => Self::Features {
                feat_id: (regs[1] as u32).into(),
                input_properties: regs[2] as u32,
            },
            FuncId::RxAcquire => Self::RxAcquire {
                vm_id: regs[1] as u16,
            },
            FuncId::RxRelease => Self::RxRelease {
                vm_id: regs[1] as u16,
            },
            FuncId::RxTxMap32 => {
                let addr = RxTxAddr::Addr32 {
                    rx: regs[2] as u32,
                    tx: regs[1] as u32,
                };
                let page_cnt = regs[3] as u32;

                Self::RxTxMap { addr, page_cnt }
            }
            FuncId::RxTxMap64 => {
                let addr = RxTxAddr::Addr64 {
                    rx: regs[2],
                    tx: regs[1],
                };
                let page_cnt = regs[3] as u32;

                Self::RxTxMap { addr, page_cnt }
            }
            FuncId::RxTxUnmap => Self::RxTxUnmap {
                id: (regs[1] >> 16) as u16,
            },
            FuncId::PartitionInfoGet => {
                let uuid_words = [
                    regs[1] as u32,
                    regs[2] as u32,
                    regs[3] as u32,
                    regs[4] as u32,
                ];

                Self::PartitionInfoGet {
                    uuid: UuidHelper::from_u32_regs(uuid_words),
                    flags: PartitionInfoGetFlags::try_from(regs[5] as u32)?,
                }
            }
            FuncId::PartitionInfoGetRegs => {
                // Bits[15:0]: Start index
                let start_index = (regs[3] & 0xffff) as u16;
                let info_tag = ((regs[3] >> 16) & 0xffff) as u16;
                Self::PartitionInfoGetRegs {
                    uuid: UuidHelper::from_u64_regs([regs[1], regs[2]]),
                    start_index,
                    info_tag: if start_index == 0 && info_tag != 0 {
                        return Err(Error::InvalidInformationTag(info_tag));
                    } else {
                        info_tag
                    },
                }
            }
            FuncId::IdGet => Self::IdGet,
            FuncId::SpmIdGet => Self::SpmIdGet,
            FuncId::MsgWait32 | FuncId::MsgWait64 => Self::MsgWait {
                is_32bit: matches!(func_id, FuncId::MsgWait32),
                flags: MsgWaitFlags::try_from(regs[2] as u32)?,
            },
            FuncId::Yield32 | FuncId::Yield64 => Self::Yield {
                is_32bit: matches!(func_id, FuncId::Yield32),
            },
            FuncId::Run32 | FuncId::Run64 => Self::Run {
                is_32bit: matches!(func_id, FuncId::Run32),
                target_info: (regs[1] as u32).into(),
            },
            FuncId::NormalWorldResume32 | FuncId::NormalWorldResume64 => Self::NormalWorldResume {
                is_32bit: matches!(func_id, FuncId::NormalWorldResume32),
            },
            FuncId::SecondaryEpRegister32 => Self::SecondaryEpRegister {
                entrypoint: SecondaryEpRegisterAddr::Addr32(regs[1] as u32),
            },
            FuncId::SecondaryEpRegister64 => Self::SecondaryEpRegister {
                entrypoint: SecondaryEpRegisterAddr::Addr64(regs[1]),
            },
            FuncId::MsgSend2 => Self::MsgSend2 {
                sender_vm_id: (regs[1] >> 16) as u16,
                flags: (regs[2] as u32).try_into()?,
            },
            FuncId::MsgSendDirectReq32 => Self::MsgSendDirectReq {
                src_id: (regs[1] >> 16) as u16,
                dst_id: regs[1] as u16,
                args: if (regs[2] as u32 & DirectMsgArgs::FWK_MSG_BITS) != 0 {
                    match regs[2] as u32 {
                        DirectMsgArgs::VERSION_REQ => DirectMsgArgs::VersionReq {
                            version: Version::try_from(regs[3] as u32)?,
                            flags: VersionFlags::try_from(regs[4] as u32)?,
                        },
                        DirectMsgArgs::POWER_PSCI_REQ => DirectMsgArgs::PowerPsciReq32 {
                            params: [
                                regs[3] as u32,
                                regs[4] as u32,
                                regs[5] as u32,
                                regs[6] as u32,
                            ],
                        },
                        DirectMsgArgs::POWER_WARM_BOOT_REQ => DirectMsgArgs::PowerWarmBootReq {
                            boot_type: WarmBootType::try_from(regs[3] as u32)?,
                        },
                        DirectMsgArgs::VM_CREATED => DirectMsgArgs::VmCreated {
                            handle: memory_management::Handle::from([
                                regs[3] as u32,
                                regs[4] as u32,
                            ]),
                            vm_id: regs[5] as u16,
                        },
                        DirectMsgArgs::VM_DESTRUCTED => DirectMsgArgs::VmDestructed {
                            handle: memory_management::Handle::from([
                                regs[3] as u32,
                                regs[4] as u32,
                            ]),
                            vm_id: regs[5] as u16,
                        },
                        _ => return Err(Error::UnrecognisedFwkMsg(regs[2] as u32)),
                    }
                } else {
                    DirectMsgArgs::Args32([
                        regs[3] as u32,
                        regs[4] as u32,
                        regs[5] as u32,
                        regs[6] as u32,
                        regs[7] as u32,
                    ])
                },
            },
            FuncId::MsgSendDirectResp32 => Self::MsgSendDirectResp {
                src_id: (regs[1] >> 16) as u16,
                dst_id: regs[1] as u16,
                args: if (regs[2] as u32 & DirectMsgArgs::FWK_MSG_BITS) != 0 {
                    match regs[2] as u32 {
                        DirectMsgArgs::VERSION_RESP => {
                            if regs[3] as i32 == VersionOut::SMCCC_NOT_SUPPORTED {
                                DirectMsgArgs::VersionResp { version: None }
                            } else {
                                DirectMsgArgs::VersionResp {
                                    version: Some(Version::try_from(regs[3] as u32)?),
                                }
                            }
                        }
                        DirectMsgArgs::POWER_PSCI_RESP => DirectMsgArgs::PowerPsciResp {
                            psci_status: regs[3] as i32,
                        },
                        DirectMsgArgs::VM_CREATED_ACK => DirectMsgArgs::VmCreatedAck {
                            sp_status: (regs[3] as i32).try_into()?,
                        },
                        DirectMsgArgs::VM_DESTRUCTED_ACK => DirectMsgArgs::VmDestructedAck {
                            sp_status: (regs[3] as i32).try_into()?,
                        },
                        _ => return Err(Error::UnrecognisedFwkMsg(regs[2] as u32)),
                    }
                } else {
                    DirectMsgArgs::Args32([
                        regs[3] as u32,
                        regs[4] as u32,
                        regs[5] as u32,
                        regs[6] as u32,
                        regs[7] as u32,
                    ])
                },
            },
            FuncId::MemDonate32 => Self::MemDonate {
                total_len: regs[1] as u32,
                frag_len: regs[2] as u32,
                buf: if regs[3] != 0 && regs[4] != 0 {
                    Some(MemOpBuf::Buf32 {
                        addr: regs[3] as u32,
                        page_cnt: regs[4] as u32,
                    })
                } else {
                    None
                },
            },
            FuncId::MemDonate64 => Self::MemDonate {
                total_len: regs[1] as u32,
                frag_len: regs[2] as u32,
                buf: if regs[3] != 0 && regs[4] != 0 {
                    Some(MemOpBuf::Buf64 {
                        addr: regs[3],
                        page_cnt: regs[4] as u32,
                    })
                } else {
                    None
                },
            },
            FuncId::MemLend32 => Self::MemLend {
                total_len: regs[1] as u32,
                frag_len: regs[2] as u32,
                buf: if regs[3] != 0 && regs[4] != 0 {
                    Some(MemOpBuf::Buf32 {
                        addr: regs[3] as u32,
                        page_cnt: regs[4] as u32,
                    })
                } else {
                    None
                },
            },
            FuncId::MemLend64 => Self::MemLend {
                total_len: regs[1] as u32,
                frag_len: regs[2] as u32,
                buf: if regs[3] != 0 && regs[4] != 0 {
                    Some(MemOpBuf::Buf64 {
                        addr: regs[3],
                        page_cnt: regs[4] as u32,
                    })
                } else {
                    None
                },
            },
            FuncId::MemShare32 => Self::MemShare {
                total_len: regs[1] as u32,
                frag_len: regs[2] as u32,
                buf: if regs[3] != 0 && regs[4] != 0 {
                    Some(MemOpBuf::Buf32 {
                        addr: regs[3] as u32,
                        page_cnt: regs[4] as u32,
                    })
                } else {
                    None
                },
            },
            FuncId::MemShare64 => Self::MemShare {
                total_len: regs[1] as u32,
                frag_len: regs[2] as u32,
                buf: if regs[3] != 0 && regs[4] != 0 {
                    Some(MemOpBuf::Buf64 {
                        addr: regs[3],
                        page_cnt: regs[4] as u32,
                    })
                } else {
                    None
                },
            },
            FuncId::MemRetrieveReq32 => Self::MemRetrieveReq {
                total_len: regs[1] as u32,
                frag_len: regs[2] as u32,
                buf: if regs[3] != 0 && regs[4] != 0 {
                    Some(MemOpBuf::Buf32 {
                        addr: regs[3] as u32,
                        page_cnt: regs[4] as u32,
                    })
                } else {
                    None
                },
            },
            FuncId::MemRetrieveReq64 => Self::MemRetrieveReq {
                total_len: regs[1] as u32,
                frag_len: regs[2] as u32,
                buf: if regs[3] != 0 && regs[4] != 0 {
                    Some(MemOpBuf::Buf64 {
                        addr: regs[3],
                        page_cnt: regs[4] as u32,
                    })
                } else {
                    None
                },
            },
            FuncId::MemRetrieveResp => Self::MemRetrieveResp {
                total_len: regs[1] as u32,
                frag_len: regs[2] as u32,
            },
            FuncId::MemRelinquish => Self::MemRelinquish,
            FuncId::MemReclaim => Self::MemReclaim {
                handle: memory_management::Handle::from([regs[1] as u32, regs[2] as u32]),
                flags: (regs[3] as u32).try_into()?,
            },
            FuncId::MemPermGet32 => {
                if (version <= Version(1, 2) && regs[2] != 0)
                    || (regs[2] as u32).checked_add(1).is_none()
                {
                    return Err(Error::MemoryManagementError(
                        memory_management::Error::InvalidPageCount,
                    ));
                }

                Self::MemPermGet {
                    addr: MemAddr::Addr32(regs[1] as u32),
                    page_cnt: regs[2] as u32 + 1,
                }
            }
            FuncId::MemPermGet64 => {
                if (version <= Version(1, 2) && regs[2] != 0)
                    || (regs[2] as u32).checked_add(1).is_none()
                {
                    return Err(Error::MemoryManagementError(
                        memory_management::Error::InvalidPageCount,
                    ));
                }

                Self::MemPermGet {
                    addr: MemAddr::Addr64(regs[1]),
                    page_cnt: regs[2] as u32 + 1,
                }
            }
            FuncId::MemPermSet32 => Self::MemPermSet {
                addr: MemAddr::Addr32(regs[1] as u32),
                page_cnt: regs[2] as u32,
                mem_perm: (regs[3] as u32).try_into()?,
            },
            FuncId::MemPermSet64 => Self::MemPermSet {
                addr: MemAddr::Addr64(regs[1]),
                page_cnt: regs[2] as u32,
                mem_perm: (regs[3] as u32).try_into()?,
            },
            FuncId::MemOpPause => Self::MemOpPause {
                handle: memory_management::Handle::from([regs[1] as u32, regs[2] as u32]),
            },
            FuncId::MemOpResume => Self::MemOpResume {
                handle: memory_management::Handle::from([regs[1] as u32, regs[2] as u32]),
            },
            FuncId::MemFragRx => Self::MemFragRx {
                handle: memory_management::Handle::from([regs[1] as u32, regs[2] as u32]),
                frag_offset: regs[3] as u32,
                endpoint_id: (regs[4] >> 16) as u16,
            },
            FuncId::MemFragTx => Self::MemFragTx {
                handle: memory_management::Handle::from([regs[1] as u32, regs[2] as u32]),
                frag_len: regs[3] as u32,
                endpoint_id: (regs[4] >> 16) as u16,
            },
            FuncId::ConsoleLog32 => {
                let char_cnt = regs[1] as u8;
                if char_cnt > ConsoleLogChars32::MAX_LENGTH {
                    return Err(Error::InvalidCharacterCount(char_cnt));
                }

                Self::ConsoleLog {
                    chars: ConsoleLogChars::Chars32(ConsoleLogChars32 {
                        char_cnt,
                        char_lists: [
                            regs[2] as u32,
                            regs[3] as u32,
                            regs[4] as u32,
                            regs[5] as u32,
                            regs[6] as u32,
                            regs[7] as u32,
                        ],
                    }),
                }
            }
            FuncId::NotificationBitmapCreate => {
                let tentative_vm_id = regs[1] as u32;
                if (tentative_vm_id >> 16) != 0 {
                    return Err(Error::InvalidVmId(tentative_vm_id));
                }
                Self::NotificationBitmapCreate {
                    vm_id: tentative_vm_id as u16,
                    vcpu_cnt: regs[2] as u32,
                }
            }
            FuncId::NotificationBitmapDestroy => {
                let tentative_vm_id = regs[1] as u32;
                if (tentative_vm_id >> 16) != 0 {
                    return Err(Error::InvalidVmId(tentative_vm_id));
                }
                Self::NotificationBitmapDestroy {
                    vm_id: tentative_vm_id as u16,
                }
            }
            FuncId::NotificationBind => Self::NotificationBind {
                sender_id: (regs[1] >> 16) as u16,
                receiver_id: regs[1] as u16,
                flags: (regs[2] as u32).into(),
                bitmap: (regs[4] << 32) | (regs[3] & 0xffff_ffff),
            },
            FuncId::NotificationUnbind => Self::NotificationUnbind {
                sender_id: (regs[1] >> 16) as u16,
                receiver_id: regs[1] as u16,
                bitmap: (regs[4] << 32) | (regs[3] & 0xffff_ffff),
            },
            FuncId::NotificationSet => Self::NotificationSet {
                sender_id: (regs[1] >> 16) as u16,
                receiver_id: regs[1] as u16,
                flags: (regs[2] as u32).try_into()?,
                bitmap: (regs[4] << 32) | (regs[3] & 0xffff_ffff),
            },
            FuncId::NotificationGet => Self::NotificationGet {
                vcpu_id: (regs[1] >> 16) as u16,
                endpoint_id: regs[1] as u16,
                flags: (regs[2] as u32).into(),
            },
            FuncId::NotificationInfoGet32 => Self::NotificationInfoGet { is_32bit: true },
            FuncId::NotificationInfoGet64 => Self::NotificationInfoGet { is_32bit: false },
            FuncId::El3IntrHandle => Self::El3IntrHandle,
            _ => panic!(
                "Invalid number of registers (8) for function {:#x?}",
                func_id
            ),
        };

        Ok(msg)
    }

    fn unpack_regs18(version: Version, func_id: FuncId, regs: &[u64; 18]) -> Result<Self, Error> {
        assert!(version >= Version(1, 2));

        let msg = match func_id {
            FuncId::Success64 => Self::Success {
                target_info: (regs[1] as u32).into(),
                args: SuccessArgs::Args64(regs[2..18].try_into().unwrap()),
            },
            FuncId::MsgSendDirectReq64 => Self::MsgSendDirectReq {
                src_id: (regs[1] >> 16) as u16,
                dst_id: regs[1] as u16,
                args: if (regs[2] & DirectMsgArgs::FWK_MSG_BITS as u64) != 0 {
                    match regs[2] as u32 {
                        DirectMsgArgs::POWER_PSCI_REQ => DirectMsgArgs::PowerPsciReq64 {
                            params: regs[3..7].try_into().unwrap(),
                        },
                        _ => return Err(Error::UnrecognisedFwkMsg(regs[2] as u32)),
                    }
                } else {
                    DirectMsgArgs::Args64(regs[3..18].try_into().unwrap())
                },
            },
            FuncId::MsgSendDirectResp64 => Self::MsgSendDirectResp {
                src_id: (regs[1] >> 16) as u16,
                dst_id: regs[1] as u16,
                args: if (regs[2] & DirectMsgArgs::FWK_MSG_BITS as u64) != 0 {
                    return Err(Error::UnrecognisedFwkMsg(regs[2] as u32));
                } else {
                    DirectMsgArgs::Args64(regs[3..18].try_into().unwrap())
                },
            },
            FuncId::MsgSendDirectReq64_2 => Self::MsgSendDirectReq2 {
                src_id: (regs[1] >> 16) as u16,
                dst_id: regs[1] as u16,
                uuid: UuidHelper::from_u64_regs([regs[2], regs[3]]),
                args: DirectMsg2Args(regs[4..18].try_into().unwrap()),
            },
            FuncId::MsgSendDirectResp64_2 => Self::MsgSendDirectResp2 {
                src_id: (regs[1] >> 16) as u16,
                dst_id: regs[1] as u16,
                args: DirectMsg2Args(regs[4..18].try_into().unwrap()),
            },
            FuncId::ConsoleLog64 => {
                let char_cnt = regs[1] as u8;
                if char_cnt > ConsoleLogChars64::MAX_LENGTH {
                    return Err(Error::InvalidCharacterCount(char_cnt));
                }

                Self::ConsoleLog {
                    chars: ConsoleLogChars::Chars64(ConsoleLogChars64 {
                        char_cnt,
                        char_lists: regs[2..18].try_into().unwrap(),
                    }),
                }
            }

            _ => panic!(
                "Invalid number of registers (18) for function {:#x?}",
                func_id
            ),
        };

        Ok(msg)
    }

    /// Create register contents for an interface. The caller must ensure that the `regs` argument
    /// has the correct length: at least 8 registers for SMC32 calls and at least 18 for SMC64 calls
    pub fn to_regs(&self, version: Version, regs: &mut [u64]) {
        if self.is_32bit() {
            self.pack_regs8(version, regs.first_chunk_mut::<8>().unwrap());
        } else {
            match self {
                Interface::ConsoleLog {
                    chars: ConsoleLogChars::Chars64(_),
                    ..
                }
                | Interface::Success {
                    args: SuccessArgs::Args64(_),
                    ..
                }
                | Interface::MsgSendDirectReq {
                    args: DirectMsgArgs::Args64(_),
                    ..
                }
                | Interface::MsgSendDirectResp {
                    args: DirectMsgArgs::Args64(_),
                    ..
                }
                | Interface::MsgSendDirectReq2 { .. }
                | Interface::MsgSendDirectResp2 { .. } => {
                    self.pack_regs18(version, regs.first_chunk_mut::<18>().unwrap());
                }
                _ => {
                    self.pack_regs8(version, regs.first_chunk_mut::<8>().unwrap());
                    regs[8..18].fill(0);
                }
            }
        }
    }

    fn pack_regs8(&self, version: Version, a: &mut [u64; 8]) {
        a.fill(0);

        if let Some(function_id) = self.function_id() {
            assert!(function_id.minimum_ffa_version() <= version);

            a[0] = function_id as u64;
        }

        match *self {
            Interface::Error {
                target_info,
                error_code,
                error_arg,
                ..
            } => {
                a[1] = u32::from(target_info).into();
                a[2] = (error_code as u32).into();
                a[3] = error_arg.into();
            }
            Interface::Success { target_info, args } => {
                a[1] = u32::from(target_info).into();
                match args {
                    SuccessArgs::Args32(regs) => {
                        a[2] = regs[0].into();
                        a[3] = regs[1].into();
                        a[4] = regs[2].into();
                        a[5] = regs[3].into();
                        a[6] = regs[4].into();
                        a[7] = regs[5].into();
                    }
                    _ => panic!("{:#x?} requires 18 registers", args),
                }
            }
            Interface::Interrupt {
                target_info,
                interrupt_id,
                ..
            } => {
                a[1] = u32::from(target_info).into();
                a[2] = interrupt_id.into();
            }
            Interface::Version {
                input_version,
                flags,
            } => {
                a[1] = u32::from(input_version).into();
                a[2] = u32::from(flags).into();
            }
            Interface::VersionOut { output_version } => {
                a[0] = u32::from(output_version).into();
            }
            Interface::Features {
                feat_id,
                input_properties,
            } => {
                a[1] = u32::from(feat_id).into();
                a[2] = input_properties.into();
            }
            Interface::RxAcquire { vm_id } => {
                a[1] = vm_id.into();
            }
            Interface::RxRelease { vm_id } => {
                a[1] = vm_id.into();
            }
            Interface::RxTxMap { addr, page_cnt } => {
                match addr {
                    RxTxAddr::Addr32 { rx, tx } => {
                        a[1] = tx.into();
                        a[2] = rx.into();
                    }
                    RxTxAddr::Addr64 { rx, tx } => {
                        a[1] = tx;
                        a[2] = rx;
                    }
                }
                a[3] = page_cnt.into();
            }
            Interface::RxTxUnmap { id } => {
                a[1] = (u32::from(id) << 16).into();
            }
            Interface::PartitionInfoGet { uuid, flags } => {
                let uuid_words: [u32; 4] = UuidHelper::to_u32_regs(uuid);

                a[1] = uuid_words[0].into();
                a[2] = uuid_words[1].into();
                a[3] = uuid_words[2].into();
                a[4] = uuid_words[3].into();
                a[5] = u32::from(flags).into();
            }
            Interface::PartitionInfoGetRegs {
                uuid,
                start_index,
                info_tag,
            } => {
                if start_index == 0 && info_tag != 0 {
                    panic!("Information Tag MBZ if start index is 0: {:#x?}", self);
                }
                [a[1], a[2]] = UuidHelper::to_u64_regs(uuid);
                a[3] = (u64::from(info_tag) << 16) | u64::from(start_index);
            }
            Interface::MsgWait { flags, .. } => {
                a[2] = u32::from(flags).into();
            }
            Interface::IdGet | Interface::SpmIdGet | Interface::Yield { .. } => {}
            Interface::Run { target_info, .. } => {
                a[1] = u32::from(target_info).into();
            }
            Interface::NormalWorldResume { .. } => {}
            Interface::SecondaryEpRegister { entrypoint } => match entrypoint {
                SecondaryEpRegisterAddr::Addr32(addr) => a[1] = addr as u64,
                SecondaryEpRegisterAddr::Addr64(addr) => a[1] = addr,
            },
            Interface::MsgSend2 {
                sender_vm_id,
                flags,
            } => {
                a[1] = (sender_vm_id as u64) << 16;
                a[2] = u32::from(flags).into();
            }
            Interface::MsgSendDirectReq {
                src_id,
                dst_id,
                args,
            } => {
                a[1] = ((src_id as u64) << 16) | dst_id as u64;
                match args {
                    DirectMsgArgs::Args32(args) => {
                        a[3] = args[0].into();
                        a[4] = args[1].into();
                        a[5] = args[2].into();
                        a[6] = args[3].into();
                        a[7] = args[4].into();
                    }
                    DirectMsgArgs::VersionReq { version, flags } => {
                        a[2] = DirectMsgArgs::VERSION_REQ.into();
                        a[3] = u32::from(version).into();
                        a[4] = u32::from(flags).into();
                    }
                    DirectMsgArgs::PowerPsciReq32 { params } => {
                        a[2] = DirectMsgArgs::POWER_PSCI_REQ.into();
                        a[3] = params[0].into();
                        a[4] = params[1].into();
                        a[5] = params[2].into();
                        a[6] = params[3].into();
                    }
                    DirectMsgArgs::PowerPsciReq64 { params } => {
                        a[2] = DirectMsgArgs::POWER_PSCI_REQ.into();
                        a[3] = params[0];
                        a[4] = params[1];
                        a[5] = params[2];
                        a[6] = params[3];
                    }
                    DirectMsgArgs::PowerWarmBootReq { boot_type } => {
                        a[2] = DirectMsgArgs::POWER_WARM_BOOT_REQ.into();
                        a[3] = u32::from(boot_type).into();
                    }
                    DirectMsgArgs::VmCreated { handle, vm_id } => {
                        a[2] = DirectMsgArgs::VM_CREATED.into();
                        let handle_regs: [u32; 2] = handle.into();
                        a[3] = handle_regs[0].into();
                        a[4] = handle_regs[1].into();
                        a[5] = vm_id.into();
                    }
                    DirectMsgArgs::VmDestructed { handle, vm_id } => {
                        a[2] = DirectMsgArgs::VM_DESTRUCTED.into();
                        let handle_regs: [u32; 2] = handle.into();
                        a[3] = handle_regs[0].into();
                        a[4] = handle_regs[1].into();
                        a[5] = vm_id.into();
                    }
                    _ => panic!("Malformed MsgSendDirectReq interface"),
                }
            }
            Interface::MsgSendDirectResp {
                src_id,
                dst_id,
                args,
            } => {
                a[1] = ((src_id as u64) << 16) | dst_id as u64;
                match args {
                    DirectMsgArgs::Args32(args) => {
                        a[3] = args[0].into();
                        a[4] = args[1].into();
                        a[5] = args[2].into();
                        a[6] = args[3].into();
                        a[7] = args[4].into();
                    }
                    DirectMsgArgs::VersionResp { version } => {
                        a[2] = DirectMsgArgs::VERSION_RESP.into();
                        match version {
                            None => a[3] = (i32::from(FfaError::NotSupported) as u32).into(),
                            Some(ver) => a[3] = u32::from(ver).into(),
                        }
                    }
                    DirectMsgArgs::PowerPsciResp { psci_status } => {
                        a[2] = DirectMsgArgs::POWER_PSCI_RESP.into();
                        a[3] = (psci_status as u32).into();
                    }
                    DirectMsgArgs::VmCreatedAck { sp_status } => {
                        a[2] = DirectMsgArgs::VM_CREATED_ACK.into();
                        a[3] = (i32::from(sp_status) as u32).into();
                    }
                    DirectMsgArgs::VmDestructedAck { sp_status } => {
                        a[2] = DirectMsgArgs::VM_DESTRUCTED_ACK.into();
                        a[3] = (i32::from(sp_status) as u32).into();
                    }
                    _ => panic!("Malformed MsgSendDirectResp interface"),
                }
            }
            Interface::MemDonate {
                total_len,
                frag_len,
                buf,
            } => {
                a[1] = total_len.into();
                a[2] = frag_len.into();
                (a[3], a[4]) = match buf {
                    Some(MemOpBuf::Buf32 { addr, page_cnt }) => (addr.into(), page_cnt.into()),
                    Some(MemOpBuf::Buf64 { addr, page_cnt }) => (addr, page_cnt.into()),
                    None => (0, 0),
                };
            }
            Interface::MemLend {
                total_len,
                frag_len,
                buf,
            } => {
                a[1] = total_len.into();
                a[2] = frag_len.into();
                (a[3], a[4]) = match buf {
                    Some(MemOpBuf::Buf32 { addr, page_cnt }) => (addr.into(), page_cnt.into()),
                    Some(MemOpBuf::Buf64 { addr, page_cnt }) => (addr, page_cnt.into()),
                    None => (0, 0),
                };
            }
            Interface::MemShare {
                total_len,
                frag_len,
                buf,
            } => {
                a[1] = total_len.into();
                a[2] = frag_len.into();
                (a[3], a[4]) = match buf {
                    Some(MemOpBuf::Buf32 { addr, page_cnt }) => (addr.into(), page_cnt.into()),
                    Some(MemOpBuf::Buf64 { addr, page_cnt }) => (addr, page_cnt.into()),
                    None => (0, 0),
                };
            }
            Interface::MemRetrieveReq {
                total_len,
                frag_len,
                buf,
            } => {
                a[1] = total_len.into();
                a[2] = frag_len.into();
                (a[3], a[4]) = match buf {
                    Some(MemOpBuf::Buf32 { addr, page_cnt }) => (addr.into(), page_cnt.into()),
                    Some(MemOpBuf::Buf64 { addr, page_cnt }) => (addr, page_cnt.into()),
                    None => (0, 0),
                };
            }
            Interface::MemRetrieveResp {
                total_len,
                frag_len,
            } => {
                a[1] = total_len.into();
                a[2] = frag_len.into();
            }
            Interface::MemRelinquish => {}
            Interface::MemReclaim { handle, flags } => {
                let handle_regs: [u32; 2] = handle.into();
                a[1] = handle_regs[0].into();
                a[2] = handle_regs[1].into();
                a[3] = u32::from(flags).into();
            }
            Interface::MemPermGet { addr, page_cnt } => {
                a[1] = match addr {
                    MemAddr::Addr32(addr) => addr.into(),
                    MemAddr::Addr64(addr) => addr,
                };
                a[2] = if version <= Version(1, 2) {
                    assert_eq!(page_cnt, 1);
                    0
                } else {
                    assert_ne!(page_cnt, 0);
                    (page_cnt - 1).into()
                }
            }
            Interface::MemPermSet {
                addr,
                page_cnt,
                mem_perm,
            } => {
                a[1] = match addr {
                    MemAddr::Addr32(addr) => addr.into(),
                    MemAddr::Addr64(addr) => addr,
                };
                a[2] = page_cnt.into();
                a[3] = u32::from(mem_perm).into();
            }
            Interface::MemOpPause { handle } => {
                let handle_regs: [u32; 2] = handle.into();
                a[1] = handle_regs[0].into();
                a[2] = handle_regs[1].into();
            }
            Interface::MemOpResume { handle } => {
                let handle_regs: [u32; 2] = handle.into();
                a[1] = handle_regs[0].into();
                a[2] = handle_regs[1].into();
            }
            Interface::MemFragRx {
                handle,
                frag_offset,
                endpoint_id,
            } => {
                let handle_regs: [u32; 2] = handle.into();
                a[1] = handle_regs[0].into();
                a[2] = handle_regs[1].into();
                a[3] = frag_offset.into();
                a[4] = (u32::from(endpoint_id) << 16).into();
            }
            Interface::MemFragTx {
                handle,
                frag_len,
                endpoint_id,
            } => {
                let handle_regs: [u32; 2] = handle.into();
                a[1] = handle_regs[0].into();
                a[2] = handle_regs[1].into();
                a[3] = frag_len.into();
                a[4] = (u32::from(endpoint_id) << 16).into();
            }
            Interface::ConsoleLog { chars } => match chars {
                ConsoleLogChars::Chars32(ConsoleLogChars32 {
                    char_cnt,
                    char_lists,
                }) => {
                    a[1] = char_cnt.into();
                    a[2] = char_lists[0].into();
                    a[3] = char_lists[1].into();
                    a[4] = char_lists[2].into();
                    a[5] = char_lists[3].into();
                    a[6] = char_lists[4].into();
                    a[7] = char_lists[5].into();
                }
                _ => panic!("{:#x?} requires 18 registers", chars),
            },
            Interface::NotificationBitmapCreate { vm_id, vcpu_cnt } => {
                a[1] = vm_id.into();
                a[2] = vcpu_cnt.into();
            }
            Interface::NotificationBitmapDestroy { vm_id } => {
                a[1] = vm_id.into();
            }
            Interface::NotificationBind {
                sender_id,
                receiver_id,
                flags,
                bitmap,
            } => {
                a[1] = (u64::from(sender_id) << 16) | u64::from(receiver_id);
                a[2] = u32::from(flags).into();
                a[3] = bitmap & 0xffff_ffff;
                a[4] = bitmap >> 32;
            }
            Interface::NotificationUnbind {
                sender_id,
                receiver_id,
                bitmap,
            } => {
                a[1] = (u64::from(sender_id) << 16) | u64::from(receiver_id);
                a[3] = bitmap & 0xffff_ffff;
                a[4] = bitmap >> 32;
            }
            Interface::NotificationSet {
                sender_id,
                receiver_id,
                flags,
                bitmap,
            } => {
                a[1] = (u64::from(sender_id) << 16) | u64::from(receiver_id);
                a[2] = u32::from(flags).into();
                a[3] = bitmap & 0xffff_ffff;
                a[4] = bitmap >> 32;
            }
            Interface::NotificationGet {
                vcpu_id,
                endpoint_id,
                flags,
            } => {
                a[1] = (u64::from(vcpu_id) << 16) | u64::from(endpoint_id);
                a[2] = u32::from(flags).into();
            }
            Interface::NotificationInfoGet { .. } => {}
            Interface::El3IntrHandle => {}
            _ => panic!("{:#x?} requires 18 registers", self),
        }
    }

    fn pack_regs18(&self, version: Version, a: &mut [u64; 18]) {
        a.fill(0);

        if let Some(function_id) = self.function_id() {
            assert!(function_id.minimum_ffa_version() <= version);

            a[0] = function_id as u64;
        }

        match *self {
            Interface::Success { target_info, args } => {
                a[1] = u32::from(target_info).into();
                match args {
                    SuccessArgs::Args64(regs) => a[2..18].copy_from_slice(&regs[..16]),
                    _ => panic!("{:#x?} requires 8 registers", args),
                }
            }
            Interface::MsgSendDirectReq {
                src_id,
                dst_id,
                args,
            }
            | Interface::MsgSendDirectResp {
                src_id,
                dst_id,
                args,
            } => {
                a[1] = ((src_id as u64) << 16) | dst_id as u64;
                match args {
                    DirectMsgArgs::Args64(args) => a[3..18].copy_from_slice(&args.map(u64::from)),
                    _ => panic!("Malformed MsgSendDirectReq/Resp interface"),
                }
            }

            Interface::MsgSendDirectReq2 {
                src_id,
                dst_id,
                uuid,
                args,
            } => {
                a[1] = ((src_id as u64) << 16) | dst_id as u64;
                [a[2], a[3]] = UuidHelper::to_u64_regs(uuid);
                a[4..18].copy_from_slice(&args.0[..14]);
            }
            Interface::MsgSendDirectResp2 {
                src_id,
                dst_id,
                args,
            } => {
                a[1] = ((src_id as u64) << 16) | dst_id as u64;
                a[2] = 0;
                a[3] = 0;
                a[4..18].copy_from_slice(&args.0[..14]);
            }
            Interface::ConsoleLog { chars: char_lists } => match char_lists {
                ConsoleLogChars::Chars64(ConsoleLogChars64 {
                    char_cnt,
                    char_lists,
                }) => {
                    a[1] = char_cnt.into();
                    a[2..18].copy_from_slice(&char_lists[..16])
                }
                _ => panic!("{:#x?} requires 8 registers", char_lists),
            },
            _ => panic!("{:#x?} requires 8 registers", self),
        }
    }

    /// Helper function to create an `FFA_SUCCESS` interface without any arguments.
    pub fn success32_noargs() -> Self {
        Self::Success {
            target_info: TargetInfo::default(),
            args: SuccessArgs::Args32([0; 6]),
        }
    }

    /// Helper function to create an `FFA_ERROR` interface with an error code.
    pub fn error(error_code: FfaError, is_32bit: bool) -> Self {
        Self::Error {
            target_info: TargetInfo::default(),
            error_code,
            error_arg: 0,
            is_32bit,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        interface_args::{
            FeatureId, LogChars, SuccessArgsFeatures, SuccessArgsIdGet, SuccessArgsSpmIdGet,
            VersionQueryType, VmAvailabilityStatus,
        },
        memory_management::Handle,
        notification::{
            NotificationGetFlags, SuccessArgsNotificationGet, SuccessArgsNotificationInfoGet,
        },
        partition_info::{SuccessArgsPartitionInfoGet, SuccessArgsPartitionInfoGetRegs},
        tests::{test_args_serde, test_regs_serde},
    };
    use uuid::uuid;

    const fn error_code(code: i32) -> u64 {
        (code as u32) as u64
    }

    #[test]
    fn part_info_get_regs() {
        let uuid = Uuid::parse_str("a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8").unwrap();
        let uuid_bytes = uuid.as_bytes();
        let test_info_tag = 0b1101_1101;
        let test_start_index = 0b1101;
        let start_index_and_tag = (test_info_tag << 16) | test_start_index;
        let version = Version(1, 2);

        // From spec:
        // Bytes[0...7] of UUID with byte 0 in the low-order bits.
        let reg_x1 = ((uuid_bytes[7] as u64) << 56)
            | ((uuid_bytes[6] as u64) << 48)
            | ((uuid_bytes[5] as u64) << 40)
            | ((uuid_bytes[4] as u64) << 32)
            | ((uuid_bytes[3] as u64) << 24)
            | ((uuid_bytes[2] as u64) << 16)
            | ((uuid_bytes[1] as u64) << 8)
            | (uuid_bytes[0] as u64);

        // From spec:
        // Bytes[8...15] of UUID with byte 8 in the low-order bits.
        let reg_x2 = ((uuid_bytes[15] as u64) << 56)
            | ((uuid_bytes[14] as u64) << 48)
            | ((uuid_bytes[13] as u64) << 40)
            | ((uuid_bytes[12] as u64) << 32)
            | ((uuid_bytes[11] as u64) << 24)
            | ((uuid_bytes[10] as u64) << 16)
            | ((uuid_bytes[9] as u64) << 8)
            | (uuid_bytes[8] as u64);

        // First, test for wrong tag:
        {
            let mut regs = [0u64; 18];
            regs[0] = FuncId::PartitionInfoGetRegs as u64;
            regs[1] = reg_x1;
            regs[2] = reg_x2;
            regs[3] = test_info_tag << 16;

            assert!(Interface::from_regs(version, &regs).is_err_and(
                |e| e == Error::InvalidInformationTag(test_info_tag.try_into().unwrap())
            ));
        }

        // Test for regs -> Interface -> regs
        {
            let mut orig_regs = [0u64; 18];
            orig_regs[0] = FuncId::PartitionInfoGetRegs as u64;
            orig_regs[1] = reg_x1;
            orig_regs[2] = reg_x2;
            orig_regs[3] = start_index_and_tag;

            let mut test_regs = orig_regs;
            let interface = Interface::from_regs(version, &test_regs).unwrap();
            match &interface {
                Interface::PartitionInfoGetRegs {
                    info_tag,
                    start_index,
                    uuid: int_uuid,
                } => {
                    assert_eq!(u64::from(*info_tag), test_info_tag);
                    assert_eq!(u64::from(*start_index), test_start_index);
                    assert_eq!(*int_uuid, uuid);
                }
                _ => panic!("Expecting Interface::PartitionInfoGetRegs!"),
            }
            test_regs.fill(0);
            interface.to_regs(version, &mut test_regs);
            assert_eq!(orig_regs, test_regs);
        }

        // Test for Interface -> regs -> Interface
        {
            let interface = Interface::PartitionInfoGetRegs {
                info_tag: test_info_tag.try_into().unwrap(),
                start_index: test_start_index.try_into().unwrap(),
                uuid,
            };

            let mut regs: [u64; 18] = [0; 18];
            interface.to_regs(version, &mut regs);

            assert_eq!(Some(FuncId::PartitionInfoGetRegs), interface.function_id());
            assert_eq!(regs[0], interface.function_id().unwrap() as u64);
            assert_eq!(regs[1], reg_x1);
            assert_eq!(regs[2], reg_x2);
            assert_eq!(regs[3], (test_info_tag << 16) | test_start_index);

            assert_eq!(Interface::from_regs(version, &regs).unwrap(), interface);
        }
    }

    #[test]
    fn msg_send_direct_req2() {
        let uuid = Uuid::parse_str("a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8").unwrap();
        let uuid_bytes = uuid.as_bytes();

        // From spec:
        // Bytes[0...7] of UUID with byte 0 in the low-order bits.
        let reg_x2 = ((uuid_bytes[7] as u64) << 56)
            | ((uuid_bytes[6] as u64) << 48)
            | ((uuid_bytes[5] as u64) << 40)
            | ((uuid_bytes[4] as u64) << 32)
            | ((uuid_bytes[3] as u64) << 24)
            | ((uuid_bytes[2] as u64) << 16)
            | ((uuid_bytes[1] as u64) << 8)
            | (uuid_bytes[0] as u64);

        // From spec:
        // Bytes[8...15] of UUID with byte 8 in the low-order bits.
        let reg_x3 = ((uuid_bytes[15] as u64) << 56)
            | ((uuid_bytes[14] as u64) << 48)
            | ((uuid_bytes[13] as u64) << 40)
            | ((uuid_bytes[12] as u64) << 32)
            | ((uuid_bytes[11] as u64) << 24)
            | ((uuid_bytes[10] as u64) << 16)
            | ((uuid_bytes[9] as u64) << 8)
            | (uuid_bytes[8] as u64);

        let test_sender = 0b1101_1101;
        let test_receiver = 0b1101;
        let test_sender_receiver = (test_sender << 16) | test_receiver;
        let version = Version(1, 2);

        // Test for regs -> Interface -> regs
        {
            let mut orig_regs = [0u64; 18];
            orig_regs[0] = FuncId::MsgSendDirectReq64_2 as u64;
            orig_regs[1] = test_sender_receiver;
            orig_regs[2] = reg_x2;
            orig_regs[3] = reg_x3;

            let mut test_regs = orig_regs;
            let interface = Interface::from_regs(version, &test_regs).unwrap();
            match &interface {
                Interface::MsgSendDirectReq2 {
                    dst_id,
                    src_id,
                    args: _,
                    uuid: int_uuid,
                } => {
                    assert_eq!(u64::from(*src_id), test_sender);
                    assert_eq!(u64::from(*dst_id), test_receiver);
                    assert_eq!(*int_uuid, uuid);
                }
                _ => panic!("Expecting Interface::MsgSendDirectReq2!"),
            }
            test_regs.fill(0);
            interface.to_regs(version, &mut test_regs);
            assert_eq!(orig_regs, test_regs);
        }

        // Test for Interface -> regs -> Interface
        {
            let rest_of_regs: [u64; 14] = [0; 14];

            let interface = Interface::MsgSendDirectReq2 {
                src_id: test_sender.try_into().unwrap(),
                dst_id: test_receiver.try_into().unwrap(),
                uuid,
                args: DirectMsg2Args(rest_of_regs),
            };

            let mut regs: [u64; 18] = [0; 18];
            interface.to_regs(version, &mut regs);

            assert_eq!(Some(FuncId::MsgSendDirectReq64_2), interface.function_id());
            assert_eq!(regs[0], interface.function_id().unwrap() as u64);
            assert_eq!(regs[1], test_sender_receiver);
            assert_eq!(regs[2], reg_x2);
            assert_eq!(regs[3], reg_x3);
            assert_eq!(regs[4], 0);

            assert_eq!(Interface::from_regs(version, &regs).unwrap(), interface);
        }
    }

    #[test]
    fn is_32bit() {
        let interface_64 = Interface::MsgSendDirectReq {
            src_id: 0,
            dst_id: 1,
            args: DirectMsgArgs::Args64([0; 15]),
        };
        assert!(!interface_64.is_32bit());

        let interface_32 = Interface::MsgSendDirectReq {
            src_id: 0,
            dst_id: 1,
            args: DirectMsgArgs::Args32([0, 0, 0, 0, 0]),
        };
        assert!(interface_32.is_32bit());
    }

    #[test]
    fn mem_perm_get_pack() {
        let mut expected_regs = [0u64; 18];
        let mut out_regs = [0u64; 18];

        expected_regs[0] = u32::from(FuncId::MemPermGet32).into();
        expected_regs[1] = 0xabcd;
        expected_regs[2] = 5;

        Interface::MemPermGet {
            addr: MemAddr::Addr32(0xabcd),
            page_cnt: 6,
        }
        .to_regs(Version(1, 3), &mut out_regs);

        assert_eq!(expected_regs, out_regs);

        expected_regs[2] = 0;

        Interface::MemPermGet {
            addr: MemAddr::Addr32(0xabcd),
            page_cnt: 1,
        }
        .to_regs(Version(1, 2), &mut out_regs);

        assert_eq!(expected_regs, out_regs);
    }

    #[test]
    #[should_panic]
    fn mem_perm_get_pack_fail1() {
        let mut out_regs = [0u64; 18];
        Interface::MemPermGet {
            addr: MemAddr::Addr32(0xabcd),
            page_cnt: 2,
        }
        .to_regs(Version(1, 2), &mut out_regs);
    }

    #[test]
    #[should_panic]
    fn mem_perm_get_pack_fail2() {
        let mut out_regs = [0u64; 18];
        Interface::MemPermGet {
            addr: MemAddr::Addr32(0xabcd),
            page_cnt: 0,
        }
        .to_regs(Version(1, 3), &mut out_regs);
    }

    #[test]
    fn mem_perm_get_unpack() {
        let mut in_regs = [0u64; 18];

        in_regs[0] = u32::from(FuncId::MemPermGet32).into();
        in_regs[1] = 0xabcd;
        in_regs[2] = 5;

        assert_eq!(
            Interface::from_regs(Version(1, 3), &in_regs),
            Ok(Interface::MemPermGet {
                addr: MemAddr::Addr32(0xabcd),
                page_cnt: 6,
            }),
        );

        assert_eq!(
            Interface::from_regs(Version(1, 2), &in_regs),
            Err(Error::MemoryManagementError(
                memory_management::Error::InvalidPageCount
            )),
        );

        in_regs[2] = 0;

        assert_eq!(
            Interface::from_regs(Version(1, 2), &in_regs),
            Ok(Interface::MemPermGet {
                addr: MemAddr::Addr32(0xabcd),
                page_cnt: 1,
            }),
        );

        in_regs[2] = u32::MAX.into();

        assert_eq!(
            Interface::from_regs(Version(1, 3), &in_regs),
            Err(Error::MemoryManagementError(
                memory_management::Error::InvalidPageCount
            )),
        );
    }

    #[test]
    fn ffa_error_serde() {
        test_regs_serde!(
            Interface::Error {
                target_info: TargetInfo {
                    endpoint_id: 0x1234,
                    vcpu_id: 0xabcd
                },
                error_code: FfaError::Aborted,
                error_arg: 0xdead_beef,
                is_32bit: true,
            },
            [0x84000060, 0x1234_abcd, error_code(-8), 0xdead_beef]
        );
    }

    #[test]
    fn ffa_success_serde() {
        test_regs_serde!(
            Interface::Success {
                target_info: TargetInfo {
                    endpoint_id: 0x1234,
                    vcpu_id: 0xabcd
                },
                args: SuccessArgs::Args32([1, 2, 3, 4, 5, 6])
            },
            [0x84000061, 0x1234_abcd, 1, 2, 3, 4, 5, 6]
        );
        test_regs_serde!(
            Interface::Success {
                target_info: TargetInfo {
                    endpoint_id: 0x1234,
                    vcpu_id: 0xabcd
                },
                args: SuccessArgs::Args64([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16])
            },
            [
                0xC4000061,
                0x1234_abcd,
                1,
                2,
                3,
                4,
                5,
                6,
                7,
                8,
                9,
                10,
                11,
                12,
                13,
                14,
                15,
                16
            ]
        );
    }

    #[test]
    fn ffa_interrupt_serde() {
        test_regs_serde!(
            Interface::Interrupt {
                target_info: TargetInfo {
                    endpoint_id: 0x1234,
                    vcpu_id: 0xabcd
                },
                interrupt_id: 0xdead_beef,
                is_32bit: true,
            },
            [0x84000062, 0x1234_abcd, 0xdead_beef]
        );
    }

    #[test]
    fn ffa_version_serde() {
        test_regs_serde!(
            Interface::Version {
                input_version: Version(1, 3),
                flags: VersionFlags {
                    query_type: VersionQueryType::QueryCompatibility
                }
            },
            [0x84000063, 0x0001_0003, 0x0000_0001]
        );
    }

    #[test]
    fn ffa_feature_serde() {
        test_regs_serde!(
            Interface::Features {
                feat_id: Feature::FeatureId(FeatureId::NotificationPendingInterrupt),
                input_properties: 0
            },
            [0x84000064, 0x1]
        );
        test_regs_serde!(
            Interface::Features {
                feat_id: Feature::FeatureId(FeatureId::ScheduleReceiverInterrupt),
                input_properties: 0
            },
            [0x84000064, 0x2]
        );
        test_regs_serde!(
            Interface::Features {
                feat_id: Feature::FeatureId(FeatureId::ManagedExitInterrupt),
                input_properties: 0
            },
            [0x84000064, 0x3]
        );
        test_regs_serde!(
            Interface::Features {
                feat_id: Feature::FuncId(FuncId::Features),
                input_properties: 32
            },
            [0x84000064, 0x84000064, 32]
        );
        test_args_serde!(
            SuccessArgs::Args32([8, 8, 0, 0, 0, 0]),
            SuccessArgsFeatures { properties: [8, 8] }
        );
    }

    #[test]
    fn ffa_rx_acquire_serde() {
        test_regs_serde!(Interface::RxAcquire { vm_id: 0xbeef }, [0x84000084, 0xbeef]);
    }

    #[test]
    fn ffa_rx_release_serde() {
        test_regs_serde!(Interface::RxRelease { vm_id: 0xbeef }, [0x84000065, 0xbeef]);
    }

    #[test]
    fn ffa_rxtx_map_serde() {
        test_regs_serde!(
            Interface::RxTxMap {
                addr: RxTxAddr::Addr32 {
                    rx: 0xbeef,
                    tx: 0xfeed_dead
                },
                page_cnt: 0x1234_abcd
            },
            [0x84000066, 0xfeed_dead, 0xbeef, 0x1234_abcd]
        );
        test_regs_serde!(
            Interface::RxTxMap {
                addr: RxTxAddr::Addr64 {
                    rx: 0xdead_1234_beef,
                    tx: 0xaaaa_bbbb_feed_dead
                },
                page_cnt: 0x1234_abcd
            },
            [
                0xC4000066,
                0xaaaa_bbbb_feed_dead,
                0xdead_1234_beef,
                0x1234_abcd
            ]
        );
    }

    #[test]
    fn ffa_rxtx_unmap_serde() {
        test_regs_serde!(
            Interface::RxTxUnmap { id: 0x1234 },
            [0x84000067, 0x1234_0000]
        );
    }

    #[test]
    fn ffa_partition_info_get_serde() {
        test_regs_serde!(
            Interface::PartitionInfoGet {
                uuid: uuid!("12345678-abcd-ef12-3456-7890abcdef00"),
                flags: PartitionInfoGetFlags { count_only: false }
            },
            [0x84000068, 0x78563412, 0x12efcdab, 0x90785634, 0x00efcdab]
        );
        test_args_serde!(
            SuccessArgsPartitionInfoGet {
                count: 0x1234_5678,
                size: Some(0xabcd_beef)
            },
            SuccessArgs::Args32([0x1234_5678, 0xabcd_beef, 0, 0, 0, 0]),
            PartitionInfoGetFlags { count_only: false }
        );
        test_regs_serde!(
            Interface::PartitionInfoGet {
                uuid: uuid!("12345678-abcd-ef12-3456-7890abcdef00"),
                flags: PartitionInfoGetFlags { count_only: true }
            },
            [
                0x84000068, 0x78563412, 0x12efcdab, 0x90785634, 0x00efcdab, 0b1
            ]
        );
        test_args_serde!(
            SuccessArgsPartitionInfoGet {
                count: 0x1234_5678,
                size: None
            },
            SuccessArgs::Args32([0x1234_5678, 0, 0, 0, 0, 0]),
            PartitionInfoGetFlags { count_only: true }
        );
    }

    #[test]
    fn ffa_partition_info_get_regs_serde() {
        test_regs_serde!(
            Interface::PartitionInfoGetRegs {
                uuid: uuid!("12345678-abcd-ef12-3456-7890abcdef00"),
                start_index: 0xfeed,
                info_tag: 0xbeef
            },
            [
                0xC400008B,
                0x12ef_cdab_7856_3412,
                0x00ef_cdab_9078_5634,
                0xbeef_feed
            ]
        );
        test_args_serde!(
            SuccessArgs::Args64([
                0x0018_2222_0002_0004,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0
            ]),
            SuccessArgsPartitionInfoGetRegs {
                last_index: 4,
                current_index: 2,
                info_tag: 0x2222,
                descriptor_data: [0; 120]
            }
        );
    }

    #[test]
    fn ffa_id_get_serde() {
        test_regs_serde!(Interface::IdGet, [0x84000069]);
        test_args_serde!(
            SuccessArgs::Args32([0x1234, 0, 0, 0, 0, 0]),
            SuccessArgsIdGet { id: 0x1234 }
        );
    }

    #[test]
    fn ffa_spm_id_get_serde() {
        test_regs_serde!(Interface::SpmIdGet, [0x84000085]);
        test_args_serde!(
            SuccessArgs::Args32([0x1234, 0, 0, 0, 0, 0]),
            SuccessArgsSpmIdGet { id: 0x1234 }
        );
    }

    #[test]
    fn ffa_console_log_serde() {
        test_regs_serde!(
            Interface::ConsoleLog {
                chars: ConsoleLogChars::Chars32(LogChars {
                    char_cnt: 8,
                    char_lists: [0x6566_6768, 0x6970_7172, 0, 0, 0, 0,]
                })
            },
            [0x8400008A, 8, 0x6566_6768, 0x6970_7172]
        );
        test_regs_serde!(
            Interface::ConsoleLog {
                chars: ConsoleLogChars::Chars64(LogChars {
                    char_cnt: 8,
                    char_lists: [
                        0x6566_6768_6970_7172,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0
                    ]
                })
            },
            [0xC400008A, 8, 0x6566_6768_6970_7172]
        );
    }

    #[test]
    fn ffa_msg_send2_serde() {
        test_regs_serde!(
            Interface::MsgSend2 {
                sender_vm_id: 0xfeed,
                flags: MsgSend2Flags {
                    delay_schedule_receiver: true
                }
            },
            [0x84000086, 0xfeed_0000, 0b10]
        );
    }

    #[test]
    fn ffa_msg_send_direct_req_serde() {
        test_regs_serde!(
            Interface::MsgSendDirectReq {
                src_id: 0x8005,
                dst_id: 0x8003,
                args: DirectMsgArgs::Args32([1, 2, 3, 4, 5])
            },
            [0x8400006F, 0x8005_8003, 0x0, 1, 2, 3, 4, 5]
        );

        test_regs_serde!(
            Interface::MsgSendDirectReq {
                src_id: 0x8005,
                dst_id: 0x8003,
                args: DirectMsgArgs::Args64([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15])
            },
            [
                0xC400006F,
                0x8005_8003,
                0x0,
                1,
                2,
                3,
                4,
                5,
                6,
                7,
                8,
                9,
                10,
                11,
                12,
                13,
                14,
                15
            ]
        );
    }

    #[test]
    fn ffa_msg_send_direct_resp_serde() {
        test_regs_serde!(
            Interface::MsgSendDirectResp {
                src_id: 0x8005,
                dst_id: 0x8003,
                args: DirectMsgArgs::Args32([1, 2, 3, 4, 5])
            },
            [0x84000070, 0x8005_8003, 0x0, 1, 2, 3, 4, 5]
        );

        test_regs_serde!(
            Interface::MsgSendDirectResp {
                src_id: 0x8005,
                dst_id: 0x8003,
                args: DirectMsgArgs::Args64([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15])
            },
            [
                0xC4000070,
                0x8005_8003,
                0x0,
                1,
                2,
                3,
                4,
                5,
                6,
                7,
                8,
                9,
                10,
                11,
                12,
                13,
                14,
                15
            ]
        );
    }

    #[test]
    fn ffa_psci_req_serde() {
        test_regs_serde!(
            Interface::MsgSendDirectReq {
                src_id: 0xdead,
                dst_id: 0xbeef,
                args: DirectMsgArgs::PowerPsciReq32 {
                    params: [1, 2, 3, 4]
                }
            },
            [0x8400006F, 0xdead_beef, 0x8000_0000, 1, 2, 3, 4]
        );
        test_regs_serde!(
            Interface::MsgSendDirectReq {
                src_id: 0xdead,
                dst_id: 0xbeef,
                args: DirectMsgArgs::PowerPsciReq64 {
                    params: [0x1234_5678_90ab_cdef, 2, 3, 4]
                }
            },
            [
                0xC400006F,
                0xdead_beef,
                0x8000_0000,
                0x1234_5678_90ab_cdef,
                2,
                3,
                4
            ]
        );
    }

    #[test]
    fn ffa_power_warm_boot_req_serde() {
        test_regs_serde!(
            Interface::MsgSendDirectReq {
                src_id: 0xdead,
                dst_id: 0xbeef,
                args: DirectMsgArgs::PowerWarmBootReq {
                    boot_type: WarmBootType::ExitFromLowPower
                }
            },
            [0x8400006F, 0xdead_beef, 0x80000001, 0b1]
        );
        test_regs_serde!(
            Interface::MsgSendDirectReq {
                src_id: 0xdead,
                dst_id: 0xbeef,
                args: DirectMsgArgs::PowerWarmBootReq {
                    boot_type: WarmBootType::ExitFromSuspendToRam
                }
            },
            [0x8400006F, 0xdead_beef, 0x80000001, 0b0]
        );
    }

    #[test]
    fn ffa_power_resp_serde() {
        test_regs_serde!(
            Interface::MsgSendDirectResp {
                src_id: 0xdead,
                dst_id: 0xbeef,
                args: DirectMsgArgs::PowerPsciResp {
                    psci_status: 0x1234
                }
            },
            [0x84000070, 0xdead_beef, 0x8000_0002, 0x1234]
        );
    }

    #[test]
    fn ffa_vm_creation_req() {
        test_regs_serde!(
            Interface::MsgSendDirectReq {
                src_id: 0xdead,
                dst_id: 0xbeef,
                args: DirectMsgArgs::VmCreated {
                    handle: Handle(0x1234_5678_90ab_cdef),
                    vm_id: 0x1234
                }
            },
            [
                0x8400006F,
                0xdead_beef,
                0x8000_0004,
                0x90ab_cdef,
                0x1234_5678,
                0x1234
            ]
        );
    }

    #[test]
    fn ffa_vm_creation_resp() {
        test_regs_serde!(
            Interface::MsgSendDirectResp {
                src_id: 0xdead,
                dst_id: 0xbeef,
                args: DirectMsgArgs::VmCreatedAck {
                    sp_status: VmAvailabilityStatus::Success
                }
            },
            [0x84000070, 0xdead_beef, 0x8000_0005]
        );
        test_regs_serde!(
            Interface::MsgSendDirectResp {
                src_id: 0xdead,
                dst_id: 0xbeef,
                args: DirectMsgArgs::VmCreatedAck {
                    sp_status: VmAvailabilityStatus::Error(FfaError::Retry)
                }
            },
            [0x84000070, 0xdead_beef, 0x8000_0005, error_code(-7)]
        );
    }

    #[test]
    fn ffa_vm_destruction_req() {
        test_regs_serde!(
            Interface::MsgSendDirectReq {
                src_id: 0xdead,
                dst_id: 0xbeef,
                args: DirectMsgArgs::VmDestructed {
                    handle: Handle(0x1234_5678_90ab_cdef),
                    vm_id: 0x1234
                }
            },
            [
                0x8400006F,
                0xdead_beef,
                0x8000_0006,
                0x90ab_cdef,
                0x1234_5678,
                0x1234
            ]
        );
    }

    #[test]
    fn ffa_vm_destruction_resp() {
        test_regs_serde!(
            Interface::MsgSendDirectResp {
                src_id: 0xdead,
                dst_id: 0xbeef,
                args: DirectMsgArgs::VmDestructedAck {
                    sp_status: VmAvailabilityStatus::Success
                }
            },
            [0x84000070, 0xdead_beef, 0x8000_0007]
        );
        test_regs_serde!(
            Interface::MsgSendDirectResp {
                src_id: 0xdead,
                dst_id: 0xbeef,
                args: DirectMsgArgs::VmDestructedAck {
                    sp_status: VmAvailabilityStatus::Error(FfaError::Denied)
                }
            },
            [0x84000070, 0xdead_beef, 0x8000_0007, error_code(-6)]
        );
    }

    #[test]
    fn ffa_version_req() {
        test_regs_serde!(
            Interface::MsgSendDirectReq {
                src_id: 0xdead,
                dst_id: 0xbeef,
                args: DirectMsgArgs::VersionReq {
                    version: Version(1, 3),
                    flags: VersionFlags {
                        query_type: VersionQueryType::QueryCompatibility
                    }
                }
            },
            [
                0x8400006F,
                0xdead_beef,
                0x8000_0008,
                0x0001_0003,
                0x0000_0001
            ]
        );
    }

    #[test]
    fn ffa_version_resp() {
        test_regs_serde!(
            Interface::MsgSendDirectResp {
                src_id: 0xdead,
                dst_id: 0xbeef,
                args: DirectMsgArgs::VersionResp {
                    version: Some(Version(1, 2))
                }
            },
            [0x84000070, 0xdead_beef, 0x8000_0009, 0x0001_0002]
        );
        test_regs_serde!(
            Interface::MsgSendDirectResp {
                src_id: 0xdead,
                dst_id: 0xbeef,
                args: DirectMsgArgs::VersionResp { version: None }
            },
            [0x84000070, 0xdead_beef, 0x8000_0009, u32::MAX as u64]
        );
    }

    #[test]
    fn ffa_msg_send_direct_req2_serde() {
        test_regs_serde!(
            Interface::MsgSendDirectReq2 {
                src_id: 0x1234,
                dst_id: 0xdcba,
                uuid: uuid!("12345678-abcd-ef12-3456-7890abcdef00"),
                args: DirectMsg2Args([4; 14])
            },
            [
                0xC400008D,
                0x1234_dcba,
                0x12ef_cdab_7856_3412,
                0x00ef_cdab_9078_5634,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
            ]
        );
    }

    #[test]
    fn ffa_msg_send_direct_resp2_serde() {
        test_regs_serde!(
            Interface::MsgSendDirectResp2 {
                src_id: 0xaaaa,
                dst_id: 0xbbbb,
                args: DirectMsg2Args([8; 14])
            },
            [
                0xC400008E,
                0xaaaa_bbbb,
                0,
                0,
                8,
                8,
                8,
                8,
                8,
                8,
                8,
                8,
                8,
                8,
                8,
                8,
                8,
                8
            ]
        );
    }

    #[test]
    fn ffa_msg_wait_serde() {
        test_regs_serde!(
            Interface::MsgWait {
                flags: MsgWaitFlags {
                    retain_rx_buffer: true
                },
                is_32bit: true,
            },
            [0x8400006B, 0, 0b1]
        );
    }

    #[test]
    fn ffa_yield_serde() {
        test_regs_serde!(Interface::Yield { is_32bit: true }, [0x8400006C]);
    }

    #[test]
    fn ffa_run_serde() {
        test_regs_serde!(
            Interface::Run {
                target_info: TargetInfo {
                    endpoint_id: 0xaaaa,
                    vcpu_id: 0x1234
                },
                is_32bit: true,
            },
            [0x8400006D, 0xaaaa_1234]
        );
    }

    #[test]
    fn ffa_normal_world_resume_serde() {
        test_regs_serde!(
            Interface::NormalWorldResume { is_32bit: true },
            [0x8400007C]
        );
    }

    #[test]
    fn ffa_notification_bitmap_create_serde() {
        test_regs_serde!(
            Interface::NotificationBitmapCreate {
                vm_id: 0xabcd,
                vcpu_cnt: 16
            },
            [0x8400007D, 0xabcd, 16]
        );
    }

    #[test]
    fn ffa_notification_bitmap_destroy_serde() {
        test_regs_serde!(
            Interface::NotificationBitmapDestroy { vm_id: 0xabcd },
            [0x8400007E, 0xabcd]
        );
    }

    #[test]
    fn ffa_notification_bind_serde() {
        test_regs_serde!(
            Interface::NotificationBind {
                sender_id: 0xdead,
                receiver_id: 0xbeef,
                flags: NotificationBindFlags {
                    per_vcpu_notification: true
                },
                bitmap: 0x1234_abcd_5678_def0
            },
            [0x8400007F, 0xdead_beef, 0b1, 0x5678_def0, 0x1234_abcd]
        );
    }

    #[test]
    fn ffa_notification_unbind_serde() {
        test_regs_serde!(
            Interface::NotificationUnbind {
                sender_id: 0xaaaa,
                receiver_id: 0xbbbb,
                bitmap: 0x1234_abcd_5678_def0
            },
            [0x84000080, 0xaaaa_bbbb, 0, 0x5678_def0, 0x1234_abcd]
        );
    }

    #[test]
    fn ffa_notification_set_serde() {
        test_regs_serde!(
            Interface::NotificationSet {
                sender_id: 0xaaaa,
                receiver_id: 0xbbbb,
                flags: NotificationSetFlags {
                    delay_schedule_receiver: true,
                    vcpu_id: Some(7)
                },
                bitmap: 0x1234_abcd_5678_def0
            },
            [
                0x84000081,
                0xaaaa_bbbb,
                0x0007_0003,
                0x5678_def0,
                0x1234_abcd
            ]
        );
        test_regs_serde!(
            Interface::NotificationSet {
                sender_id: 0xaaaa,
                receiver_id: 0xbbbb,
                flags: NotificationSetFlags {
                    delay_schedule_receiver: false,
                    vcpu_id: None
                },
                bitmap: 0x1234_abcd_5678_def0
            },
            [0x84000081, 0xaaaa_bbbb, 0, 0x5678_def0, 0x1234_abcd]
        );
    }

    #[test]
    fn ffa_notification_get_serde() {
        test_regs_serde!(
            Interface::NotificationGet {
                vcpu_id: 13,
                endpoint_id: 0x1234,
                flags: NotificationGetFlags {
                    sp_bitmap_id: false,
                    vm_bitmap_id: true,
                    spm_bitmap_id: true,
                    hyp_bitmap_id: false
                }
            },
            [0x84000082, 0x000d_1234, 0b0110]
        );
        test_regs_serde!(
            Interface::NotificationGet {
                vcpu_id: 13,
                endpoint_id: 0x1234,
                flags: NotificationGetFlags {
                    sp_bitmap_id: false,
                    vm_bitmap_id: false,
                    spm_bitmap_id: false,
                    hyp_bitmap_id: false
                }
            },
            [0x84000082, 0x000d_1234, 0b0000]
        );
        test_regs_serde!(
            Interface::NotificationGet {
                vcpu_id: 13,
                endpoint_id: 0x1234,
                flags: NotificationGetFlags {
                    sp_bitmap_id: true,
                    vm_bitmap_id: true,
                    spm_bitmap_id: true,
                    hyp_bitmap_id: true
                }
            },
            [0x84000082, 0x000d_1234, 0b1111]
        );

        test_args_serde!(
            SuccessArgsNotificationGet {
                sp_notifications: None,
                vm_notifications: None,
                spm_notifications: None,
                hypervisor_notifications: None
            },
            SuccessArgs::Args32([0, 0, 0, 0, 0, 0]),
            NotificationGetFlags {
                sp_bitmap_id: false,
                vm_bitmap_id: false,
                spm_bitmap_id: false,
                hyp_bitmap_id: false
            }
        );
        test_args_serde!(
            SuccessArgsNotificationGet {
                sp_notifications: None,
                vm_notifications: Some(0xdead_beef_1234_1234),
                spm_notifications: None,
                hypervisor_notifications: Some(0x1234_5678)
            },
            SuccessArgs::Args32([0, 0, 0x1234_1234, 0xdead_beef, 0, 0x1234_5678]),
            NotificationGetFlags {
                sp_bitmap_id: false,
                vm_bitmap_id: true,
                spm_bitmap_id: false,
                hyp_bitmap_id: true
            }
        );

        test_args_serde!(
            SuccessArgsNotificationGet {
                sp_notifications: Some(0x1000),
                vm_notifications: Some(0xdead_beef_1234_1234),
                spm_notifications: Some(0x2000),
                hypervisor_notifications: Some(0x1234_5678)
            },
            SuccessArgs::Args32([0x1000, 0, 0x1234_1234, 0xdead_beef, 0x2000, 0x1234_5678]),
            NotificationGetFlags {
                sp_bitmap_id: true,
                vm_bitmap_id: true,
                spm_bitmap_id: true,
                hyp_bitmap_id: true
            }
        );
    }

    #[test]
    fn ffa_notification_info_get_serde() {
        test_regs_serde!(
            Interface::NotificationInfoGet { is_32bit: true },
            [0x84000083]
        );
        test_regs_serde!(
            Interface::NotificationInfoGet { is_32bit: false },
            [0xC4000083]
        );
        test_args_serde!(
            SuccessArgs::Args32([0b1001_0001_0000_0001, 0xbbbb_cccc, 0xaaaa, 0, 0, 0]),
            SuccessArgsNotificationInfoGet {
                more_pending_notifications: true,
                list_count: 2,
                id_counts: [1, 2, 0, 0, 0, 0, 0, 0, 0, 0],
                ids: [0xcccc, 0xbbbb, 0xaaaa, 0, 0, 0, 0, 0, 0, 0]
            }
        );
    }

    #[test]
    fn ffa_el3_intr_handle_serde() {
        test_regs_serde!(Interface::El3IntrHandle, [0x8400008C]);
    }

    #[test]
    fn ffa_secondary_ep_regs32() {
        test_regs_serde!(
            Interface::SecondaryEpRegister {
                entrypoint: SecondaryEpRegisterAddr::Addr32(0xdead_beef)
            },
            [0x84000087, 0xdead_beef]
        );
    }

    #[test]
    fn ffa_secondary_ep_regs64() {
        test_regs_serde!(
            Interface::SecondaryEpRegister {
                entrypoint: SecondaryEpRegisterAddr::Addr64(0x1234_5678_90ab_cdef)
            },
            [0xC4000087, 0x1234_5678_90ab_cdef]
        );
    }
}
