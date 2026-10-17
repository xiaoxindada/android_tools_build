// SPDX-FileCopyrightText: Copyright The arm-sysregs Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Arm CPU system registers.

// This file is generated, do not edit manually.

use bitflags::bitflags;

bitflags! {
    /// `APIAKeyHi_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ApiakeyhiEl1: u64 {
    }
}

impl ApiakeyhiEl1 {
    /// Offset of the `APIAKeyHi` field.
    pub const APIAKEYHI_SHIFT: u32 = 0;
    /// Mask for the `APIAKeyHi` field.
    pub const APIAKEYHI_MASK: u64 =
        0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `APIAKeyHi` field.
    pub const fn apiakeyhi(self) -> u64 {
        (self.bits() >> Self::APIAKEYHI_SHIFT) & Self::APIAKEYHI_MASK
    }

    /// Sets the value of the `APIAKeyHi` field.
    pub const fn set_apiakeyhi(&mut self, value: u64) {
        let offset = Self::APIAKEYHI_SHIFT;
        assert!(value & Self::APIAKEYHI_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::APIAKEYHI_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `APIAKeyHi` field set to the given value.
    pub const fn with_apiakeyhi(mut self, value: u64) -> Self {
        self.set_apiakeyhi(value);
        self
    }
}

bitflags! {
    /// `APIAKeyLo_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ApiakeyloEl1: u64 {
    }
}

impl ApiakeyloEl1 {
    /// Offset of the `APIAKeyLo` field.
    pub const APIAKEYLO_SHIFT: u32 = 0;
    /// Mask for the `APIAKeyLo` field.
    pub const APIAKEYLO_MASK: u64 =
        0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `APIAKeyLo` field.
    pub const fn apiakeylo(self) -> u64 {
        (self.bits() >> Self::APIAKEYLO_SHIFT) & Self::APIAKEYLO_MASK
    }

    /// Sets the value of the `APIAKeyLo` field.
    pub const fn set_apiakeylo(&mut self, value: u64) {
        let offset = Self::APIAKEYLO_SHIFT;
        assert!(value & Self::APIAKEYLO_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::APIAKEYLO_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `APIAKeyLo` field set to the given value.
    pub const fn with_apiakeylo(mut self, value: u64) -> Self {
        self.set_apiakeylo(value);
        self
    }
}

bitflags! {
    /// `CCSIDR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CcsidrEl1: u64 {
    }
}

impl CcsidrEl1 {
    /// Offset of the `LineSize` field.
    pub const LINESIZE_SHIFT: u32 = 0;
    /// Mask for the `LineSize` field.
    pub const LINESIZE_MASK: u64 = 0b111;

    /// Returns the value of the `LineSize` field.
    pub const fn linesize(self) -> u8 {
        ((self.bits() >> Self::LINESIZE_SHIFT) & Self::LINESIZE_MASK) as u8
    }

    /// Sets the value of the `LineSize` field.
    pub const fn set_linesize(&mut self, value: u8) {
        let offset = Self::LINESIZE_SHIFT;
        assert!(value & (Self::LINESIZE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::LINESIZE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `LineSize` field set to the given value.
    pub const fn with_linesize(mut self, value: u8) -> Self {
        self.set_linesize(value);
        self
    }
}

bitflags! {
    /// `CLIDR_EL1` system register value.
    ///
    /// Cache Level ID.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ClidrEl1: u64 {
    }
}

impl ClidrEl1 {
    /// Offset of the `Ctype<n>` field.
    pub const CTYPE_SHIFT: u32 = 0;
    /// Mask for the `Ctype<n>` field.
    pub const CTYPE_MASK: u64 = 0b111;
    /// Offset of the `LoUIS` field.
    pub const LOUIS_SHIFT: u32 = 21;
    /// Mask for the `LoUIS` field.
    pub const LOUIS_MASK: u64 = 0b111;
    /// Offset of the `LoC` field.
    pub const LOC_SHIFT: u32 = 24;
    /// Mask for the `LoC` field.
    pub const LOC_MASK: u64 = 0b111;
    /// Offset of the `LoUU` field.
    pub const LOUU_SHIFT: u32 = 27;
    /// Mask for the `LoUU` field.
    pub const LOUU_MASK: u64 = 0b111;
    /// Offset of the `ICB` field.
    pub const ICB_SHIFT: u32 = 30;
    /// Mask for the `ICB` field.
    pub const ICB_MASK: u64 = 0b111;
    /// Offset of the `Ttype<n>` field.
    pub const TTYPE_SHIFT: u32 = 33;
    /// Mask for the `Ttype<n>` field.
    pub const TTYPE_MASK: u64 = 0b11;

    /// Returns the value of the given `Ctype<n>` field.
    pub fn ctype(self, n: u32) -> arm_sysregs_common::types::CacheType {
        assert!(n >= 1 && n < 8);
        arm_sysregs_common::types::CacheType::try_from(
            ((self.bits() >> (Self::CTYPE_SHIFT + (n - 1) * 3)) & Self::CTYPE_MASK) as u8,
        )
        .unwrap()
    }

    /// Sets the value of the `Ctype<n>` field.
    pub fn set_ctype(&mut self, n: u32, value: arm_sysregs_common::types::CacheType) {
        assert!(n >= 1 && n < 8);
        let offset = Self::CTYPE_SHIFT + (n - 1) * 3;
        let value: u8 = value.into();
        assert!(value & (Self::CTYPE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CTYPE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `Ctype<n>` field set to the given value.
    pub fn with_ctype(mut self, n: u32, value: arm_sysregs_common::types::CacheType) -> Self {
        self.set_ctype(n, value);
        self
    }

    /// Returns the value of the `LoUIS` field.
    ///
    /// Level of Unification Inner Shareable for the cache hierarchy.
    pub const fn louis(self) -> u8 {
        ((self.bits() >> Self::LOUIS_SHIFT) & Self::LOUIS_MASK) as u8
    }

    /// Sets the value of the `LoUIS` field.
    ///
    /// Level of Unification Inner Shareable for the cache hierarchy.
    pub const fn set_louis(&mut self, value: u8) {
        let offset = Self::LOUIS_SHIFT;
        assert!(value & (Self::LOUIS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::LOUIS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `LoUIS` field set to the given value.
    ///
    /// Level of Unification Inner Shareable for the cache hierarchy.
    pub const fn with_louis(mut self, value: u8) -> Self {
        self.set_louis(value);
        self
    }

    /// Returns the value of the `LoC` field.
    ///
    /// Level of Coherence for the cache hierarchy.
    pub const fn loc(self) -> u8 {
        ((self.bits() >> Self::LOC_SHIFT) & Self::LOC_MASK) as u8
    }

    /// Sets the value of the `LoC` field.
    ///
    /// Level of Coherence for the cache hierarchy.
    pub const fn set_loc(&mut self, value: u8) {
        let offset = Self::LOC_SHIFT;
        assert!(value & (Self::LOC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::LOC_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `LoC` field set to the given value.
    ///
    /// Level of Coherence for the cache hierarchy.
    pub const fn with_loc(mut self, value: u8) -> Self {
        self.set_loc(value);
        self
    }

    /// Returns the value of the `LoUU` field.
    ///
    /// Level of Unification Uniprocessor for the cache hierarchy.
    pub const fn louu(self) -> u8 {
        ((self.bits() >> Self::LOUU_SHIFT) & Self::LOUU_MASK) as u8
    }

    /// Sets the value of the `LoUU` field.
    ///
    /// Level of Unification Uniprocessor for the cache hierarchy.
    pub const fn set_louu(&mut self, value: u8) {
        let offset = Self::LOUU_SHIFT;
        assert!(value & (Self::LOUU_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::LOUU_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `LoUU` field set to the given value.
    ///
    /// Level of Unification Uniprocessor for the cache hierarchy.
    pub const fn with_louu(mut self, value: u8) -> Self {
        self.set_louu(value);
        self
    }

    /// Returns the value of the `ICB` field.
    ///
    /// Inner cache boundary level.
    pub const fn icb(self) -> u8 {
        ((self.bits() >> Self::ICB_SHIFT) & Self::ICB_MASK) as u8
    }

    /// Sets the value of the `ICB` field.
    ///
    /// Inner cache boundary level.
    pub const fn set_icb(&mut self, value: u8) {
        let offset = Self::ICB_SHIFT;
        assert!(value & (Self::ICB_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ICB_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `ICB` field set to the given value.
    ///
    /// Inner cache boundary level.
    pub const fn with_icb(mut self, value: u8) -> Self {
        self.set_icb(value);
        self
    }

    /// Returns the value of the given `Ttype<n>` field.
    pub const fn ttype(self, n: u32) -> u8 {
        assert!(n >= 1 && n < 8);
        ((self.bits() >> (Self::TTYPE_SHIFT + (n - 1) * 2)) & Self::TTYPE_MASK) as u8
    }

    /// Sets the value of the `Ttype<n>` field.
    pub const fn set_ttype(&mut self, n: u32, value: u8) {
        assert!(n >= 1 && n < 8);
        let offset = Self::TTYPE_SHIFT + (n - 1) * 2;
        assert!(value & (Self::TTYPE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TTYPE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `Ttype<n>` field set to the given value.
    pub const fn with_ttype(mut self, n: u32, value: u8) -> Self {
        self.set_ttype(n, value);
        self
    }
}

bitflags! {
    /// `CNTKCTL_EL1` system register value.
    ///
    /// Counter-timer Kernel Control Register
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CntkctlEl1: u64 {
        /// `EL0PCTEN` bit.
        const EL0PCTEN = 1 << 0;
        /// `EL0VCTEN` bit.
        const EL0VCTEN = 1 << 1;
        /// `EVNTEN` bit.
        const EVNTEN = 1 << 2;
        /// `EVNTDIR` bit.
        const EVNTDIR = 1 << 3;
        /// `EL0VTEN` bit.
        const EL0VTEN = 1 << 8;
        /// `EL0PTEN` bit.
        const EL0PTEN = 1 << 9;
        /// `EL1PCTEN` bit.
        const EL1PCTEN = 1 << 10;
        /// `EL1PTEN` bit.
        const EL1PTEN = 1 << 11;
        /// `ECV` bit.
        const ECV = 1 << 12;
        /// `EL1TVT` bit.
        const EL1TVT = 1 << 13;
        /// `EL1TVCT` bit.
        const EL1TVCT = 1 << 14;
        /// `EL1NVPCT` bit.
        const EL1NVPCT = 1 << 15;
        /// `EL1NVVCT` bit.
        const EL1NVVCT = 1 << 16;
        /// `EVNTIS` bit.
        const EVNTIS = 1 << 17;
        /// `CNTVMASK` bit.
        const CNTVMASK = 1 << 18;
        /// `CNTPMASK` bit.
        const CNTPMASK = 1 << 19;
    }
}

impl CntkctlEl1 {
    /// Offset of the `EL0PCTEN` field.
    pub const EL0PCTEN_SHIFT: u32 = 0;
    /// Offset of the `EL0VCTEN` field.
    pub const EL0VCTEN_SHIFT: u32 = 1;
    /// Offset of the `EVNTEN` field.
    pub const EVNTEN_SHIFT: u32 = 2;
    /// Offset of the `EVNTDIR` field.
    pub const EVNTDIR_SHIFT: u32 = 3;
    /// Offset of the `EVNTI` field.
    pub const EVNTI_SHIFT: u32 = 4;
    /// Mask for the `EVNTI` field.
    pub const EVNTI_MASK: u64 = 0b1111;
    /// Offset of the `EL0VTEN` field.
    pub const EL0VTEN_SHIFT: u32 = 8;
    /// Offset of the `EL0PTEN` field.
    pub const EL0PTEN_SHIFT: u32 = 9;
    /// Offset of the `EL1PCTEN` field.
    pub const EL1PCTEN_SHIFT: u32 = 10;
    /// Offset of the `EL1PTEN` field.
    pub const EL1PTEN_SHIFT: u32 = 11;
    /// Offset of the `ECV` field.
    pub const ECV_SHIFT: u32 = 12;
    /// Offset of the `EL1TVT` field.
    pub const EL1TVT_SHIFT: u32 = 13;
    /// Offset of the `EL1TVCT` field.
    pub const EL1TVCT_SHIFT: u32 = 14;
    /// Offset of the `EL1NVPCT` field.
    pub const EL1NVPCT_SHIFT: u32 = 15;
    /// Offset of the `EL1NVVCT` field.
    pub const EL1NVVCT_SHIFT: u32 = 16;
    /// Offset of the `EVNTIS` field.
    pub const EVNTIS_SHIFT: u32 = 17;
    /// Offset of the `CNTVMASK` field.
    pub const CNTVMASK_SHIFT: u32 = 18;
    /// Offset of the `CNTPMASK` field.
    pub const CNTPMASK_SHIFT: u32 = 19;

    /// Returns the value of the `EVNTI` field.
    pub const fn evnti(self) -> u8 {
        ((self.bits() >> Self::EVNTI_SHIFT) & Self::EVNTI_MASK) as u8
    }

    /// Sets the value of the `EVNTI` field.
    pub const fn set_evnti(&mut self, value: u8) {
        let offset = Self::EVNTI_SHIFT;
        assert!(value & (Self::EVNTI_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EVNTI_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `EVNTI` field set to the given value.
    pub const fn with_evnti(mut self, value: u8) -> Self {
        self.set_evnti(value);
        self
    }
}

bitflags! {
    /// `CNTPS_CTL_EL1` system register value.
    ///
    /// Counter-timer Physical Secure Timer Control Register
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CntpsCtlEl1: u64 {
        /// `ENABLE` bit.
        const ENABLE = 1 << 0;
        /// `IMASK` bit.
        const IMASK = 1 << 1;
        /// `ISTATUS` bit.
        const ISTATUS = 1 << 2;
    }
}

impl CntpsCtlEl1 {
    /// Offset of the `ENABLE` field.
    pub const ENABLE_SHIFT: u32 = 0;
    /// Offset of the `IMASK` field.
    pub const IMASK_SHIFT: u32 = 1;
    /// Offset of the `ISTATUS` field.
    pub const ISTATUS_SHIFT: u32 = 2;
}

bitflags! {
    /// `CNTPS_CVAL_EL1` system register value.
    ///
    /// Counter-timer Physical Secure Timer CompareValue Register
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CntpsCvalEl1: u64 {
    }
}

impl CntpsCvalEl1 {
    /// Offset of the `CompareValue` field.
    pub const COMPAREVALUE_SHIFT: u32 = 0;
    /// Mask for the `CompareValue` field.
    pub const COMPAREVALUE_MASK: u64 =
        0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `CompareValue` field.
    pub const fn comparevalue(self) -> u64 {
        (self.bits() >> Self::COMPAREVALUE_SHIFT) & Self::COMPAREVALUE_MASK
    }

    /// Sets the value of the `CompareValue` field.
    pub const fn set_comparevalue(&mut self, value: u64) {
        let offset = Self::COMPAREVALUE_SHIFT;
        assert!(value & Self::COMPAREVALUE_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::COMPAREVALUE_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `CompareValue` field set to the given value.
    pub const fn with_comparevalue(mut self, value: u64) -> Self {
        self.set_comparevalue(value);
        self
    }
}

bitflags! {
    /// `CNTPS_TVAL_EL1` system register value.
    ///
    /// Counter-timer Physical Secure Timer TimerValue Register
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CntpsTvalEl1: u64 {
    }
}

impl CntpsTvalEl1 {
    /// Offset of the `TimerValue` field.
    pub const TIMERVALUE_SHIFT: u32 = 0;
    /// Mask for the `TimerValue` field.
    pub const TIMERVALUE_MASK: u64 = 0b1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `TimerValue` field.
    pub const fn timervalue(self) -> u32 {
        ((self.bits() >> Self::TIMERVALUE_SHIFT) & Self::TIMERVALUE_MASK) as u32
    }

    /// Sets the value of the `TimerValue` field.
    pub const fn set_timervalue(&mut self, value: u32) {
        let offset = Self::TIMERVALUE_SHIFT;
        assert!(value & (Self::TIMERVALUE_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TIMERVALUE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `TimerValue` field set to the given value.
    pub const fn with_timervalue(mut self, value: u32) -> Self {
        self.set_timervalue(value);
        self
    }
}

bitflags! {
    /// `CONTEXTIDR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ContextidrEl1: u64 {
    }
}

impl ContextidrEl1 {
    /// Offset of the `PROCID` field.
    pub const PROCID_SHIFT: u32 = 0;
    /// Mask for the `PROCID` field.
    pub const PROCID_MASK: u64 = 0b1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `PROCID` field.
    pub const fn procid(self) -> u32 {
        ((self.bits() >> Self::PROCID_SHIFT) & Self::PROCID_MASK) as u32
    }

    /// Sets the value of the `PROCID` field.
    pub const fn set_procid(&mut self, value: u32) {
        let offset = Self::PROCID_SHIFT;
        assert!(value & (Self::PROCID_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PROCID_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PROCID` field set to the given value.
    pub const fn with_procid(mut self, value: u32) -> Self {
        self.set_procid(value);
        self
    }
}

bitflags! {
    /// `CPACR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CpacrEl1: u64 {
        /// `TTA` bit.
        const TTA = 1 << 28;
        /// `E0POE` bit.
        const E0POE = 1 << 29;
        /// `TAM` bit.
        const TAM = 1 << 30;
        /// `TCPAC` bit.
        const TCPAC = 1 << 31;
        /// `E0TP0E` bit.
        const E0TP0E = 1 << 32;
        /// `E0TP1E` bit.
        const E0TP1E = 1 << 33;
    }
}

impl CpacrEl1 {
    /// Offset of the `ZEN` field.
    pub const ZEN_SHIFT: u32 = 16;
    /// Mask for the `ZEN` field.
    pub const ZEN_MASK: u64 = 0b11;
    /// Offset of the `FPEN` field.
    pub const FPEN_SHIFT: u32 = 20;
    /// Mask for the `FPEN` field.
    pub const FPEN_MASK: u64 = 0b11;
    /// Offset of the `SMEN` field.
    pub const SMEN_SHIFT: u32 = 24;
    /// Mask for the `SMEN` field.
    pub const SMEN_MASK: u64 = 0b11;
    /// Offset of the `TTA` field.
    pub const TTA_SHIFT: u32 = 28;
    /// Offset of the `E0POE` field.
    pub const E0POE_SHIFT: u32 = 29;
    /// Offset of the `TAM` field.
    pub const TAM_SHIFT: u32 = 30;
    /// Offset of the `TCPAC` field.
    pub const TCPAC_SHIFT: u32 = 31;
    /// Offset of the `E0TP0E` field.
    pub const E0TP0E_SHIFT: u32 = 32;
    /// Offset of the `E0TP1E` field.
    pub const E0TP1E_SHIFT: u32 = 33;

    /// Returns the value of the `ZEN` field.
    pub const fn zen(self) -> u8 {
        ((self.bits() >> Self::ZEN_SHIFT) & Self::ZEN_MASK) as u8
    }

    /// Sets the value of the `ZEN` field.
    pub const fn set_zen(&mut self, value: u8) {
        let offset = Self::ZEN_SHIFT;
        assert!(value & (Self::ZEN_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ZEN_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `ZEN` field set to the given value.
    pub const fn with_zen(mut self, value: u8) -> Self {
        self.set_zen(value);
        self
    }

    /// Returns the value of the `FPEN` field.
    pub const fn fpen(self) -> u8 {
        ((self.bits() >> Self::FPEN_SHIFT) & Self::FPEN_MASK) as u8
    }

    /// Sets the value of the `FPEN` field.
    pub const fn set_fpen(&mut self, value: u8) {
        let offset = Self::FPEN_SHIFT;
        assert!(value & (Self::FPEN_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::FPEN_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `FPEN` field set to the given value.
    pub const fn with_fpen(mut self, value: u8) -> Self {
        self.set_fpen(value);
        self
    }

    /// Returns the value of the `SMEN` field.
    pub const fn smen(self) -> u8 {
        ((self.bits() >> Self::SMEN_SHIFT) & Self::SMEN_MASK) as u8
    }

    /// Sets the value of the `SMEN` field.
    pub const fn set_smen(&mut self, value: u8) {
        let offset = Self::SMEN_SHIFT;
        assert!(value & (Self::SMEN_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SMEN_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `SMEN` field set to the given value.
    pub const fn with_smen(mut self, value: u8) -> Self {
        self.set_smen(value);
        self
    }
}

bitflags! {
    /// `CSSELR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CsselrEl1: u64 {
        /// Instruction not Data bit.
        const IND = 1 << 0;
        /// Allocation Tag not Data bit, only valid if FEAT_MTE2 is implemented.
        const TND = 1 << 4;
    }
}

impl CsselrEl1 {
    /// Offset of the `InD` field.
    pub const IND_SHIFT: u32 = 0;
    /// Offset of the `Level` field.
    pub const LEVEL_SHIFT: u32 = 1;
    /// Mask for the `Level` field.
    pub const LEVEL_MASK: u64 = 0b111;
    /// Offset of the `TnD` field.
    pub const TND_SHIFT: u32 = 4;

    /// Returns the value of the `Level` field.
    pub const fn level(self) -> u8 {
        ((self.bits() >> Self::LEVEL_SHIFT) & Self::LEVEL_MASK) as u8
    }

    /// Sets the value of the `Level` field.
    pub const fn set_level(&mut self, value: u8) {
        let offset = Self::LEVEL_SHIFT;
        assert!(value & (Self::LEVEL_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::LEVEL_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `Level` field set to the given value.
    pub const fn with_level(mut self, value: u8) -> Self {
        self.set_level(value);
        self
    }
}

bitflags! {
    /// `DISR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct DisrEl1: u64 {
        /// `WnR` bit.
        const WNR = 1 << 6;
        /// `WnRV` bit.
        const WNRV = 1 << 7;
        /// `EA` bit.
        const EA = 1 << 9;
        /// `IDS` bit.
        const IDS = 1 << 24;
        /// `A` bit.
        const A = 1 << 31;
    }
}

impl DisrEl1 {
    /// Offset of the `DFSC` field.
    pub const DFSC_SHIFT: u32 = 0;
    /// Mask for the `DFSC` field.
    pub const DFSC_MASK: u64 = 0b11_1111;
    /// Offset of the `WnR` field.
    pub const WNR_SHIFT: u32 = 6;
    /// Offset of the `WnRV` field.
    pub const WNRV_SHIFT: u32 = 7;
    /// Offset of the `EA` field.
    pub const EA_SHIFT: u32 = 9;
    /// Offset of the `AET` field.
    pub const AET_SHIFT: u32 = 10;
    /// Mask for the `AET` field.
    pub const AET_MASK: u64 = 0b111;
    /// Offset of the `WU` field.
    pub const WU_SHIFT: u32 = 16;
    /// Mask for the `WU` field.
    pub const WU_MASK: u64 = 0b11;
    /// Offset of the `IDS` field.
    pub const IDS_SHIFT: u32 = 24;
    /// Offset of the `A` field.
    pub const A_SHIFT: u32 = 31;

    /// Returns the value of the `DFSC` field.
    pub const fn dfsc(self) -> u8 {
        ((self.bits() >> Self::DFSC_SHIFT) & Self::DFSC_MASK) as u8
    }

    /// Sets the value of the `DFSC` field.
    pub const fn set_dfsc(&mut self, value: u8) {
        let offset = Self::DFSC_SHIFT;
        assert!(value & (Self::DFSC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::DFSC_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `DFSC` field set to the given value.
    pub const fn with_dfsc(mut self, value: u8) -> Self {
        self.set_dfsc(value);
        self
    }

    /// Returns the value of the `AET` field.
    pub const fn aet(self) -> u8 {
        ((self.bits() >> Self::AET_SHIFT) & Self::AET_MASK) as u8
    }

    /// Sets the value of the `AET` field.
    pub const fn set_aet(&mut self, value: u8) {
        let offset = Self::AET_SHIFT;
        assert!(value & (Self::AET_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::AET_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `AET` field set to the given value.
    pub const fn with_aet(mut self, value: u8) -> Self {
        self.set_aet(value);
        self
    }

    /// Returns the value of the `WU` field.
    pub const fn wu(self) -> u8 {
        ((self.bits() >> Self::WU_SHIFT) & Self::WU_MASK) as u8
    }

    /// Sets the value of the `WU` field.
    pub const fn set_wu(&mut self, value: u8) {
        let offset = Self::WU_SHIFT;
        assert!(value & (Self::WU_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::WU_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `WU` field set to the given value.
    pub const fn with_wu(mut self, value: u8) -> Self {
        self.set_wu(value);
        self
    }
}

bitflags! {
    /// `ELR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ElrEl1: u64 {
    }
}

impl ElrEl1 {
    /// Offset of the `ADDR` field.
    pub const ADDR_SHIFT: u32 = 0;
    /// Mask for the `ADDR` field.
    pub const ADDR_MASK: u64 =
        0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `ADDR` field.
    pub const fn addr(self) -> u64 {
        (self.bits() >> Self::ADDR_SHIFT) & Self::ADDR_MASK
    }

    /// Sets the value of the `ADDR` field.
    pub const fn set_addr(&mut self, value: u64) {
        let offset = Self::ADDR_SHIFT;
        assert!(value & Self::ADDR_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ADDR_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `ADDR` field set to the given value.
    pub const fn with_addr(mut self, value: u64) -> Self {
        self.set_addr(value);
        self
    }
}

bitflags! {
    /// `ESR_EL1` system register value.
    #[derive(Clone, Copy, Eq, Default, PartialEq)]
    #[repr(transparent)]
    pub struct EsrEl1: u64 {
        /// `IL` bit.
        const IL = 1 << 25;
    }
}

impl EsrEl1 {
    /// Offset of the `ISS` field.
    pub const ISS_SHIFT: u32 = 0;
    /// Mask for the `ISS` field.
    pub const ISS_MASK: u64 = 0b1_1111_1111_1111_1111_1111_1111;
    /// Offset of the `IL` field.
    pub const IL_SHIFT: u32 = 25;
    /// Offset of the `EC` field.
    pub const EC_SHIFT: u32 = 26;
    /// Mask for the `EC` field.
    pub const EC_MASK: u64 = 0b11_1111;
    /// Offset of the `ISS2` field.
    pub const ISS2_SHIFT: u32 = 32;
    /// Mask for the `ISS2` field.
    pub const ISS2_MASK: u64 = 0b1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `ISS` field.
    pub const fn iss(self) -> u32 {
        ((self.bits() >> Self::ISS_SHIFT) & Self::ISS_MASK) as u32
    }

    /// Sets the value of the `ISS` field.
    pub const fn set_iss(&mut self, value: u32) {
        let offset = Self::ISS_SHIFT;
        assert!(value & (Self::ISS_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ISS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `ISS` field set to the given value.
    pub const fn with_iss(mut self, value: u32) -> Self {
        self.set_iss(value);
        self
    }

    /// Returns the value of the `EC` field.
    pub const fn ec(self) -> u8 {
        ((self.bits() >> Self::EC_SHIFT) & Self::EC_MASK) as u8
    }

    /// Sets the value of the `EC` field.
    pub const fn set_ec(&mut self, value: u8) {
        let offset = Self::EC_SHIFT;
        assert!(value & (Self::EC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EC_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `EC` field set to the given value.
    pub const fn with_ec(mut self, value: u8) -> Self {
        self.set_ec(value);
        self
    }

    /// Returns the value of the `ISS2` field.
    pub const fn iss2(self) -> u32 {
        ((self.bits() >> Self::ISS2_SHIFT) & Self::ISS2_MASK) as u32
    }

    /// Sets the value of the `ISS2` field.
    pub const fn set_iss2(&mut self, value: u32) {
        let offset = Self::ISS2_SHIFT;
        assert!(value & (Self::ISS2_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ISS2_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `ISS2` field set to the given value.
    pub const fn with_iss2(mut self, value: u32) -> Self {
        self.set_iss2(value);
        self
    }
}

bitflags! {
    /// `FAR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct FarEl1: u64 {
    }
}

impl FarEl1 {
    /// Offset of the `VA` field.
    pub const VA_SHIFT: u32 = 0;
    /// Mask for the `VA` field.
    pub const VA_MASK: u64 =
        0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `VA` field.
    pub const fn va(self) -> u64 {
        (self.bits() >> Self::VA_SHIFT) & Self::VA_MASK
    }

    /// Sets the value of the `VA` field.
    pub const fn set_va(&mut self, value: u64) {
        let offset = Self::VA_SHIFT;
        assert!(value & Self::VA_MASK == value);
        *self =
            Self::from_bits_retain((self.bits() & !(Self::VA_MASK << offset)) | (value << offset));
    }

    /// Returns a copy with the `VA` field set to the given value.
    pub const fn with_va(mut self, value: u64) -> Self {
        self.set_va(value);
        self
    }
}

bitflags! {
    /// `GCR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct GcrEl1: u64 {
        /// `RRND` bit.
        const RRND = 1 << 16;
    }
}

impl GcrEl1 {
    /// Offset of the `Exclude` field.
    pub const EXCLUDE_SHIFT: u32 = 0;
    /// Mask for the `Exclude` field.
    pub const EXCLUDE_MASK: u64 = 0b1111_1111_1111_1111;
    /// Offset of the `RRND` field.
    pub const RRND_SHIFT: u32 = 16;

    /// Returns the value of the `Exclude` field.
    pub const fn exclude(self) -> u16 {
        ((self.bits() >> Self::EXCLUDE_SHIFT) & Self::EXCLUDE_MASK) as u16
    }

    /// Sets the value of the `Exclude` field.
    pub const fn set_exclude(&mut self, value: u16) {
        let offset = Self::EXCLUDE_SHIFT;
        assert!(value & (Self::EXCLUDE_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EXCLUDE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `Exclude` field set to the given value.
    pub const fn with_exclude(mut self, value: u16) -> Self {
        self.set_exclude(value);
        self
    }
}

bitflags! {
    /// `GCSCRE0_EL1` system register value.
    ///
    /// Guarded Control Stack Control register.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Gcscre0El1: u64 {
        /// `PCRSEL` bit.
        const PCRSEL = 1 << 0;
        /// `RVCHKEN` bit.
        const RVCHKEN = 1 << 5;
        /// `PUSHMEn` bit.
        const PUSHMEN = 1 << 8;
        /// `STREn` bit.
        const STREN = 1 << 9;
        /// `nTR` bit.
        const NTR = 1 << 10;
    }
}

impl Gcscre0El1 {
    /// Offset of the `PCRSEL` field.
    pub const PCRSEL_SHIFT: u32 = 0;
    /// Offset of the `RVCHKEN` field.
    pub const RVCHKEN_SHIFT: u32 = 5;
    /// Offset of the `PUSHMEn` field.
    pub const PUSHMEN_SHIFT: u32 = 8;
    /// Offset of the `STREn` field.
    pub const STREN_SHIFT: u32 = 9;
    /// Offset of the `nTR` field.
    pub const NTR_SHIFT: u32 = 10;
}

bitflags! {
    /// `GCSCR_EL1` system register value.
    ///
    /// Guarded Control Stack Control register.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct GcscrEl1: u64 {
        /// `PCRSEL` bit.
        const PCRSEL = 1 << 0;
        /// `RVCHKEN` bit.
        const RVCHKEN = 1 << 5;
        /// Exception state lock enable.
        const EXLOCKEN = 1 << 6;
        /// `PUSHMEn` bit.
        const PUSHMEN = 1 << 8;
        /// `STREn` bit.
        const STREN = 1 << 9;
    }
}

impl GcscrEl1 {
    /// Offset of the `PCRSEL` field.
    pub const PCRSEL_SHIFT: u32 = 0;
    /// Offset of the `RVCHKEN` field.
    pub const RVCHKEN_SHIFT: u32 = 5;
    /// Offset of the `EXLOCKEN` field.
    pub const EXLOCKEN_SHIFT: u32 = 6;
    /// Offset of the `PUSHMEn` field.
    pub const PUSHMEN_SHIFT: u32 = 8;
    /// Offset of the `STREn` field.
    pub const STREN_SHIFT: u32 = 9;
}

bitflags! {
    /// `GCSPR_EL1` system register value.
    ///
    /// Guarded Control Stack Pointer register.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct GcsprEl1: u64 {
    }
}

impl GcsprEl1 {
    /// Offset of the `PTR[63:3]` field.
    pub const PTR_63_3_SHIFT: u32 = 3;
    /// Mask for the `PTR[63:3]` field.
    pub const PTR_63_3_MASK: u64 =
        0b1_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `PTR[63:3]` field.
    pub const fn ptr_63_3(self) -> u64 {
        (self.bits() >> Self::PTR_63_3_SHIFT) & Self::PTR_63_3_MASK
    }

    /// Sets the value of the `PTR[63:3]` field.
    pub const fn set_ptr_63_3(&mut self, value: u64) {
        let offset = Self::PTR_63_3_SHIFT;
        assert!(value & Self::PTR_63_3_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PTR_63_3_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `PTR[63:3]` field set to the given value.
    pub const fn with_ptr_63_3(mut self, value: u64) -> Self {
        self.set_ptr_63_3(value);
        self
    }
}

bitflags! {
    /// `ICC_AP1R0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccAp1r0El1: u64 {
        /// `NMI` bit.
        const NMI = 1 << 63;
    }
}

impl IccAp1r0El1 {
    /// Offset of the `NMI` field.
    pub const NMI_SHIFT: u32 = 63;
}

bitflags! {
    /// `ICC_ASGI1R_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccAsgi1rEl1: u64 {
        /// `IRM` bit.
        const IRM = 1 << 40;
    }
}

impl IccAsgi1rEl1 {
    /// Offset of the `TargetList` field.
    pub const TARGETLIST_SHIFT: u32 = 0;
    /// Mask for the `TargetList` field.
    pub const TARGETLIST_MASK: u64 = 0b1111_1111_1111_1111;
    /// Offset of the `Aff1` field.
    pub const AFF1_SHIFT: u32 = 16;
    /// Mask for the `Aff1` field.
    pub const AFF1_MASK: u64 = 0b1111_1111;
    /// Offset of the `INTID` field.
    pub const INTID_SHIFT: u32 = 24;
    /// Mask for the `INTID` field.
    pub const INTID_MASK: u64 = 0b1111;
    /// Offset of the `Aff2` field.
    pub const AFF2_SHIFT: u32 = 32;
    /// Mask for the `Aff2` field.
    pub const AFF2_MASK: u64 = 0b1111_1111;
    /// Offset of the `IRM` field.
    pub const IRM_SHIFT: u32 = 40;
    /// Offset of the `RS` field.
    pub const RS_SHIFT: u32 = 44;
    /// Mask for the `RS` field.
    pub const RS_MASK: u64 = 0b1111;
    /// Offset of the `Aff3` field.
    pub const AFF3_SHIFT: u32 = 48;
    /// Mask for the `Aff3` field.
    pub const AFF3_MASK: u64 = 0b1111_1111;

    /// Returns the value of the `TargetList` field.
    pub const fn targetlist(self) -> u16 {
        ((self.bits() >> Self::TARGETLIST_SHIFT) & Self::TARGETLIST_MASK) as u16
    }

    /// Sets the value of the `TargetList` field.
    pub const fn set_targetlist(&mut self, value: u16) {
        let offset = Self::TARGETLIST_SHIFT;
        assert!(value & (Self::TARGETLIST_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TARGETLIST_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `TargetList` field set to the given value.
    pub const fn with_targetlist(mut self, value: u16) -> Self {
        self.set_targetlist(value);
        self
    }

    /// Returns the value of the `Aff1` field.
    pub const fn aff1(self) -> u8 {
        ((self.bits() >> Self::AFF1_SHIFT) & Self::AFF1_MASK) as u8
    }

    /// Sets the value of the `Aff1` field.
    pub const fn set_aff1(&mut self, value: u8) {
        let offset = Self::AFF1_SHIFT;
        assert!(value & (Self::AFF1_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::AFF1_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `Aff1` field set to the given value.
    pub const fn with_aff1(mut self, value: u8) -> Self {
        self.set_aff1(value);
        self
    }

    /// Returns the value of the `INTID` field.
    pub const fn intid(self) -> u8 {
        ((self.bits() >> Self::INTID_SHIFT) & Self::INTID_MASK) as u8
    }

    /// Sets the value of the `INTID` field.
    pub const fn set_intid(&mut self, value: u8) {
        let offset = Self::INTID_SHIFT;
        assert!(value & (Self::INTID_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::INTID_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `INTID` field set to the given value.
    pub const fn with_intid(mut self, value: u8) -> Self {
        self.set_intid(value);
        self
    }

    /// Returns the value of the `Aff2` field.
    pub const fn aff2(self) -> u8 {
        ((self.bits() >> Self::AFF2_SHIFT) & Self::AFF2_MASK) as u8
    }

    /// Sets the value of the `Aff2` field.
    pub const fn set_aff2(&mut self, value: u8) {
        let offset = Self::AFF2_SHIFT;
        assert!(value & (Self::AFF2_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::AFF2_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `Aff2` field set to the given value.
    pub const fn with_aff2(mut self, value: u8) -> Self {
        self.set_aff2(value);
        self
    }

    /// Returns the value of the `RS` field.
    pub const fn rs(self) -> u8 {
        ((self.bits() >> Self::RS_SHIFT) & Self::RS_MASK) as u8
    }

    /// Sets the value of the `RS` field.
    pub const fn set_rs(&mut self, value: u8) {
        let offset = Self::RS_SHIFT;
        assert!(value & (Self::RS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::RS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `RS` field set to the given value.
    pub const fn with_rs(mut self, value: u8) -> Self {
        self.set_rs(value);
        self
    }

    /// Returns the value of the `Aff3` field.
    pub const fn aff3(self) -> u8 {
        ((self.bits() >> Self::AFF3_SHIFT) & Self::AFF3_MASK) as u8
    }

    /// Sets the value of the `Aff3` field.
    pub const fn set_aff3(&mut self, value: u8) {
        let offset = Self::AFF3_SHIFT;
        assert!(value & (Self::AFF3_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::AFF3_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `Aff3` field set to the given value.
    pub const fn with_aff3(mut self, value: u8) -> Self {
        self.set_aff3(value);
        self
    }
}

bitflags! {
    /// `ICC_BPR0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccBpr0El1: u64 {
    }
}

impl IccBpr0El1 {
    /// Offset of the `BinaryPoint` field.
    pub const BINARYPOINT_SHIFT: u32 = 0;
    /// Mask for the `BinaryPoint` field.
    pub const BINARYPOINT_MASK: u64 = 0b111;

    /// Returns the value of the `BinaryPoint` field.
    pub const fn binarypoint(self) -> u8 {
        ((self.bits() >> Self::BINARYPOINT_SHIFT) & Self::BINARYPOINT_MASK) as u8
    }

    /// Sets the value of the `BinaryPoint` field.
    pub const fn set_binarypoint(&mut self, value: u8) {
        let offset = Self::BINARYPOINT_SHIFT;
        assert!(value & (Self::BINARYPOINT_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BINARYPOINT_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `BinaryPoint` field set to the given value.
    pub const fn with_binarypoint(mut self, value: u8) -> Self {
        self.set_binarypoint(value);
        self
    }
}

/// `ICC_BPR1_EL1` system register value.
pub type IccBpr1El1 = IccBpr0El1;

bitflags! {
    /// `ICC_CTLR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccCtlrEl1: u64 {
        /// `CBPR` bit.
        const CBPR = 1 << 0;
        /// `EOImode` bit.
        const EOIMODE = 1 << 1;
        /// `PMHE` bit.
        const PMHE = 1 << 6;
        /// `SEIS` bit.
        const SEIS = 1 << 14;
        /// `A3V` bit.
        const A3V = 1 << 15;
        /// `RSS` bit.
        const RSS = 1 << 18;
        /// `ExtRange` bit.
        const EXTRANGE = 1 << 19;
    }
}

impl IccCtlrEl1 {
    /// Offset of the `CBPR` field.
    pub const CBPR_SHIFT: u32 = 0;
    /// Offset of the `EOImode` field.
    pub const EOIMODE_SHIFT: u32 = 1;
    /// Offset of the `PMHE` field.
    pub const PMHE_SHIFT: u32 = 6;
    /// Offset of the `PRIbits` field.
    pub const PRIBITS_SHIFT: u32 = 8;
    /// Mask for the `PRIbits` field.
    pub const PRIBITS_MASK: u64 = 0b111;
    /// Offset of the `IDbits` field.
    pub const IDBITS_SHIFT: u32 = 11;
    /// Mask for the `IDbits` field.
    pub const IDBITS_MASK: u64 = 0b111;
    /// Offset of the `SEIS` field.
    pub const SEIS_SHIFT: u32 = 14;
    /// Offset of the `A3V` field.
    pub const A3V_SHIFT: u32 = 15;
    /// Offset of the `RSS` field.
    pub const RSS_SHIFT: u32 = 18;
    /// Offset of the `ExtRange` field.
    pub const EXTRANGE_SHIFT: u32 = 19;

    /// Returns the value of the `PRIbits` field.
    pub const fn pribits(self) -> u8 {
        ((self.bits() >> Self::PRIBITS_SHIFT) & Self::PRIBITS_MASK) as u8
    }

    /// Sets the value of the `PRIbits` field.
    pub const fn set_pribits(&mut self, value: u8) {
        let offset = Self::PRIBITS_SHIFT;
        assert!(value & (Self::PRIBITS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PRIBITS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PRIbits` field set to the given value.
    pub const fn with_pribits(mut self, value: u8) -> Self {
        self.set_pribits(value);
        self
    }

    /// Returns the value of the `IDbits` field.
    pub const fn idbits(self) -> u8 {
        ((self.bits() >> Self::IDBITS_SHIFT) & Self::IDBITS_MASK) as u8
    }

    /// Sets the value of the `IDbits` field.
    pub const fn set_idbits(&mut self, value: u8) {
        let offset = Self::IDBITS_SHIFT;
        assert!(value & (Self::IDBITS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::IDBITS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `IDbits` field set to the given value.
    pub const fn with_idbits(mut self, value: u8) -> Self {
        self.set_idbits(value);
        self
    }
}

bitflags! {
    /// `ICC_DIR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccDirEl1: u64 {
    }
}

impl IccDirEl1 {
    /// Offset of the `INTID` field.
    pub const INTID_SHIFT: u32 = 0;
    /// Mask for the `INTID` field.
    pub const INTID_MASK: u64 = 0b1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `INTID` field.
    pub const fn intid(self) -> u32 {
        ((self.bits() >> Self::INTID_SHIFT) & Self::INTID_MASK) as u32
    }

    /// Sets the value of the `INTID` field.
    pub const fn set_intid(&mut self, value: u32) {
        let offset = Self::INTID_SHIFT;
        assert!(value & (Self::INTID_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::INTID_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `INTID` field set to the given value.
    pub const fn with_intid(mut self, value: u32) -> Self {
        self.set_intid(value);
        self
    }
}

/// `ICC_EOIR0_EL1` system register value.
pub type IccEoir0El1 = IccDirEl1;

/// `ICC_EOIR1_EL1` system register value.
pub type IccEoir1El1 = IccDirEl1;

bitflags! {
    /// `ICC_HPPIR0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccHppir0El1: u64 {
    }
}

impl IccHppir0El1 {
    /// Offset of the `INTID` field.
    pub const INTID_SHIFT: u32 = 0;
    /// Mask for the `INTID` field.
    pub const INTID_MASK: u64 = 0b1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `INTID` field.
    pub const fn intid(self) -> u32 {
        ((self.bits() >> Self::INTID_SHIFT) & Self::INTID_MASK) as u32
    }

    /// Sets the value of the `INTID` field.
    pub const fn set_intid(&mut self, value: u32) {
        let offset = Self::INTID_SHIFT;
        assert!(value & (Self::INTID_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::INTID_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `INTID` field set to the given value.
    pub const fn with_intid(mut self, value: u32) -> Self {
        self.set_intid(value);
        self
    }
}

/// `ICC_HPPIR1_EL1` system register value.
pub type IccHppir1El1 = IccHppir0El1;

/// `ICC_IAR0_EL1` system register value.
pub type IccIar0El1 = IccHppir0El1;

/// `ICC_IAR1_EL1` system register value.
pub type IccIar1El1 = IccHppir0El1;

bitflags! {
    /// `ICC_IGRPEN0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccIgrpen0El1: u64 {
        /// `Enable` bit.
        const ENABLE = 1 << 0;
    }
}

impl IccIgrpen0El1 {
    /// Offset of the `Enable` field.
    pub const ENABLE_SHIFT: u32 = 0;
}

/// `ICC_IGRPEN1_EL1` system register value.
pub type IccIgrpen1El1 = IccIgrpen0El1;

/// `ICC_NMIAR1_EL1` system register value.
pub type IccNmiar1El1 = IccHppir0El1;

bitflags! {
    /// `ICC_PMR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccPmrEl1: u64 {
    }
}

impl IccPmrEl1 {
    /// Offset of the `Priority` field.
    pub const PRIORITY_SHIFT: u32 = 0;
    /// Mask for the `Priority` field.
    pub const PRIORITY_MASK: u64 = 0b1111_1111;

    /// Returns the value of the `Priority` field.
    pub const fn priority(self) -> u8 {
        ((self.bits() >> Self::PRIORITY_SHIFT) & Self::PRIORITY_MASK) as u8
    }

    /// Sets the value of the `Priority` field.
    pub const fn set_priority(&mut self, value: u8) {
        let offset = Self::PRIORITY_SHIFT;
        assert!(value & (Self::PRIORITY_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PRIORITY_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `Priority` field set to the given value.
    pub const fn with_priority(mut self, value: u8) -> Self {
        self.set_priority(value);
        self
    }
}

bitflags! {
    /// `ICC_RPR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccRprEl1: u64 {
        /// `NMI_NS` bit.
        const NMI_NS = 1 << 62;
        /// `NMI` bit.
        const NMI = 1 << 63;
    }
}

impl IccRprEl1 {
    /// Offset of the `Priority` field.
    pub const PRIORITY_SHIFT: u32 = 0;
    /// Mask for the `Priority` field.
    pub const PRIORITY_MASK: u64 = 0b1111_1111;
    /// Offset of the `NMI_NS` field.
    pub const NMI_NS_SHIFT: u32 = 62;
    /// Offset of the `NMI` field.
    pub const NMI_SHIFT: u32 = 63;

    /// Returns the value of the `Priority` field.
    pub const fn priority(self) -> u8 {
        ((self.bits() >> Self::PRIORITY_SHIFT) & Self::PRIORITY_MASK) as u8
    }

    /// Sets the value of the `Priority` field.
    pub const fn set_priority(&mut self, value: u8) {
        let offset = Self::PRIORITY_SHIFT;
        assert!(value & (Self::PRIORITY_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PRIORITY_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `Priority` field set to the given value.
    pub const fn with_priority(mut self, value: u8) -> Self {
        self.set_priority(value);
        self
    }
}

/// `ICC_SGI0R_EL1` system register value.
pub type IccSgi0rEl1 = IccAsgi1rEl1;

/// `ICC_SGI1R_EL1` system register value.
pub type IccSgi1rEl1 = IccAsgi1rEl1;

bitflags! {
    /// `ICC_SRE_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccSreEl1: u64 {
        /// Enable the system register interface.
        const SRE = 1 << 0;
        /// Disable FIQ bypass.
        const DFB = 1 << 1;
        /// Disable IRQ bypass.
        const DIB = 1 << 2;
    }
}

impl IccSreEl1 {
    /// Offset of the `SRE` field.
    pub const SRE_SHIFT: u32 = 0;
    /// Offset of the `DFB` field.
    pub const DFB_SHIFT: u32 = 1;
    /// Offset of the `DIB` field.
    pub const DIB_SHIFT: u32 = 2;
}

bitflags! {
    /// `ID_AA64DFR0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdAa64dfr0El1: u64 {
    }
}

impl IdAa64dfr0El1 {
    /// Offset of the `DebugVer` field.
    pub const DEBUGVER_SHIFT: u32 = 0;
    /// Mask for the `DebugVer` field.
    pub const DEBUGVER_MASK: u64 = 0b1111;
    /// Offset of the `TraceVer` field.
    pub const TRACEVER_SHIFT: u32 = 4;
    /// Mask for the `TraceVer` field.
    pub const TRACEVER_MASK: u64 = 0b1111;
    /// Offset of the `PMUVer` field.
    pub const PMUVER_SHIFT: u32 = 8;
    /// Mask for the `PMUVer` field.
    pub const PMUVER_MASK: u64 = 0b1111;
    /// Offset of the `BRPs` field.
    pub const BRPS_SHIFT: u32 = 12;
    /// Mask for the `BRPs` field.
    pub const BRPS_MASK: u64 = 0b1111;
    /// Offset of the `PMSS` field.
    pub const PMSS_SHIFT: u32 = 16;
    /// Mask for the `PMSS` field.
    pub const PMSS_MASK: u64 = 0b1111;
    /// Offset of the `WRPs` field.
    pub const WRPS_SHIFT: u32 = 20;
    /// Mask for the `WRPs` field.
    pub const WRPS_MASK: u64 = 0b1111;
    /// Offset of the `CTX_CMPs` field.
    pub const CTX_CMPS_SHIFT: u32 = 28;
    /// Mask for the `CTX_CMPs` field.
    pub const CTX_CMPS_MASK: u64 = 0b1111;
    /// Offset of the `PMSVer` field.
    pub const PMSVER_SHIFT: u32 = 32;
    /// Mask for the `PMSVer` field.
    pub const PMSVER_MASK: u64 = 0b1111;
    /// Offset of the `DoubleLock` field.
    pub const DOUBLELOCK_SHIFT: u32 = 36;
    /// Mask for the `DoubleLock` field.
    pub const DOUBLELOCK_MASK: u64 = 0b1111;
    /// Offset of the `TraceFilt` field.
    pub const TRACEFILT_SHIFT: u32 = 40;
    /// Mask for the `TraceFilt` field.
    pub const TRACEFILT_MASK: u64 = 0b1111;
    /// Offset of the `TraceBuffer` field.
    pub const TRACEBUFFER_SHIFT: u32 = 44;
    /// Mask for the `TraceBuffer` field.
    pub const TRACEBUFFER_MASK: u64 = 0b1111;
    /// Offset of the `MTPMU` field.
    pub const MTPMU_SHIFT: u32 = 48;
    /// Mask for the `MTPMU` field.
    pub const MTPMU_MASK: u64 = 0b1111;
    /// Offset of the `BRBE` field.
    pub const BRBE_SHIFT: u32 = 52;
    /// Mask for the `BRBE` field.
    pub const BRBE_MASK: u64 = 0b1111;
    /// Offset of the `ExtTrcBuff` field.
    pub const EXTTRCBUFF_SHIFT: u32 = 56;
    /// Mask for the `ExtTrcBuff` field.
    pub const EXTTRCBUFF_MASK: u64 = 0b1111;
    /// Offset of the `HPMN0` field.
    pub const HPMN0_SHIFT: u32 = 60;
    /// Mask for the `HPMN0` field.
    pub const HPMN0_MASK: u64 = 0b1111;

    /// Returns the value of the `DebugVer` field.
    pub const fn debugver(self) -> u8 {
        ((self.bits() >> Self::DEBUGVER_SHIFT) & Self::DEBUGVER_MASK) as u8
    }

    /// Sets the value of the `DebugVer` field.
    pub const fn set_debugver(&mut self, value: u8) {
        let offset = Self::DEBUGVER_SHIFT;
        assert!(value & (Self::DEBUGVER_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::DEBUGVER_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `DebugVer` field set to the given value.
    pub const fn with_debugver(mut self, value: u8) -> Self {
        self.set_debugver(value);
        self
    }

    /// Returns the value of the `TraceVer` field.
    pub const fn tracever(self) -> u8 {
        ((self.bits() >> Self::TRACEVER_SHIFT) & Self::TRACEVER_MASK) as u8
    }

    /// Sets the value of the `TraceVer` field.
    pub const fn set_tracever(&mut self, value: u8) {
        let offset = Self::TRACEVER_SHIFT;
        assert!(value & (Self::TRACEVER_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TRACEVER_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `TraceVer` field set to the given value.
    pub const fn with_tracever(mut self, value: u8) -> Self {
        self.set_tracever(value);
        self
    }

    /// Returns the value of the `PMUVer` field.
    pub const fn pmuver(self) -> u8 {
        ((self.bits() >> Self::PMUVER_SHIFT) & Self::PMUVER_MASK) as u8
    }

    /// Sets the value of the `PMUVer` field.
    pub const fn set_pmuver(&mut self, value: u8) {
        let offset = Self::PMUVER_SHIFT;
        assert!(value & (Self::PMUVER_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PMUVER_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PMUVer` field set to the given value.
    pub const fn with_pmuver(mut self, value: u8) -> Self {
        self.set_pmuver(value);
        self
    }

    /// Returns the value of the `BRPs` field.
    pub const fn brps(self) -> u8 {
        ((self.bits() >> Self::BRPS_SHIFT) & Self::BRPS_MASK) as u8
    }

    /// Sets the value of the `BRPs` field.
    pub const fn set_brps(&mut self, value: u8) {
        let offset = Self::BRPS_SHIFT;
        assert!(value & (Self::BRPS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BRPS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `BRPs` field set to the given value.
    pub const fn with_brps(mut self, value: u8) -> Self {
        self.set_brps(value);
        self
    }

    /// Returns the value of the `PMSS` field.
    pub const fn pmss(self) -> u8 {
        ((self.bits() >> Self::PMSS_SHIFT) & Self::PMSS_MASK) as u8
    }

    /// Sets the value of the `PMSS` field.
    pub const fn set_pmss(&mut self, value: u8) {
        let offset = Self::PMSS_SHIFT;
        assert!(value & (Self::PMSS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PMSS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PMSS` field set to the given value.
    pub const fn with_pmss(mut self, value: u8) -> Self {
        self.set_pmss(value);
        self
    }

    /// Returns the value of the `WRPs` field.
    pub const fn wrps(self) -> u8 {
        ((self.bits() >> Self::WRPS_SHIFT) & Self::WRPS_MASK) as u8
    }

    /// Sets the value of the `WRPs` field.
    pub const fn set_wrps(&mut self, value: u8) {
        let offset = Self::WRPS_SHIFT;
        assert!(value & (Self::WRPS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::WRPS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `WRPs` field set to the given value.
    pub const fn with_wrps(mut self, value: u8) -> Self {
        self.set_wrps(value);
        self
    }

    /// Returns the value of the `CTX_CMPs` field.
    pub const fn ctx_cmps(self) -> u8 {
        ((self.bits() >> Self::CTX_CMPS_SHIFT) & Self::CTX_CMPS_MASK) as u8
    }

    /// Sets the value of the `CTX_CMPs` field.
    pub const fn set_ctx_cmps(&mut self, value: u8) {
        let offset = Self::CTX_CMPS_SHIFT;
        assert!(value & (Self::CTX_CMPS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CTX_CMPS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `CTX_CMPs` field set to the given value.
    pub const fn with_ctx_cmps(mut self, value: u8) -> Self {
        self.set_ctx_cmps(value);
        self
    }

    /// Returns the value of the `PMSVer` field.
    pub const fn pmsver(self) -> u8 {
        ((self.bits() >> Self::PMSVER_SHIFT) & Self::PMSVER_MASK) as u8
    }

    /// Sets the value of the `PMSVer` field.
    pub const fn set_pmsver(&mut self, value: u8) {
        let offset = Self::PMSVER_SHIFT;
        assert!(value & (Self::PMSVER_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PMSVER_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PMSVer` field set to the given value.
    pub const fn with_pmsver(mut self, value: u8) -> Self {
        self.set_pmsver(value);
        self
    }

    /// Returns the value of the `DoubleLock` field.
    pub const fn doublelock(self) -> u8 {
        ((self.bits() >> Self::DOUBLELOCK_SHIFT) & Self::DOUBLELOCK_MASK) as u8
    }

    /// Sets the value of the `DoubleLock` field.
    pub const fn set_doublelock(&mut self, value: u8) {
        let offset = Self::DOUBLELOCK_SHIFT;
        assert!(value & (Self::DOUBLELOCK_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::DOUBLELOCK_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `DoubleLock` field set to the given value.
    pub const fn with_doublelock(mut self, value: u8) -> Self {
        self.set_doublelock(value);
        self
    }

    /// Returns the value of the `TraceFilt` field.
    pub const fn tracefilt(self) -> u8 {
        ((self.bits() >> Self::TRACEFILT_SHIFT) & Self::TRACEFILT_MASK) as u8
    }

    /// Sets the value of the `TraceFilt` field.
    pub const fn set_tracefilt(&mut self, value: u8) {
        let offset = Self::TRACEFILT_SHIFT;
        assert!(value & (Self::TRACEFILT_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TRACEFILT_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `TraceFilt` field set to the given value.
    pub const fn with_tracefilt(mut self, value: u8) -> Self {
        self.set_tracefilt(value);
        self
    }

    /// Returns the value of the `TraceBuffer` field.
    pub const fn tracebuffer(self) -> u8 {
        ((self.bits() >> Self::TRACEBUFFER_SHIFT) & Self::TRACEBUFFER_MASK) as u8
    }

    /// Sets the value of the `TraceBuffer` field.
    pub const fn set_tracebuffer(&mut self, value: u8) {
        let offset = Self::TRACEBUFFER_SHIFT;
        assert!(value & (Self::TRACEBUFFER_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TRACEBUFFER_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `TraceBuffer` field set to the given value.
    pub const fn with_tracebuffer(mut self, value: u8) -> Self {
        self.set_tracebuffer(value);
        self
    }

    /// Returns the value of the `MTPMU` field.
    pub const fn mtpmu(self) -> u8 {
        ((self.bits() >> Self::MTPMU_SHIFT) & Self::MTPMU_MASK) as u8
    }

    /// Sets the value of the `MTPMU` field.
    pub const fn set_mtpmu(&mut self, value: u8) {
        let offset = Self::MTPMU_SHIFT;
        assert!(value & (Self::MTPMU_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::MTPMU_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `MTPMU` field set to the given value.
    pub const fn with_mtpmu(mut self, value: u8) -> Self {
        self.set_mtpmu(value);
        self
    }

    /// Returns the value of the `BRBE` field.
    pub const fn brbe(self) -> u8 {
        ((self.bits() >> Self::BRBE_SHIFT) & Self::BRBE_MASK) as u8
    }

    /// Sets the value of the `BRBE` field.
    pub const fn set_brbe(&mut self, value: u8) {
        let offset = Self::BRBE_SHIFT;
        assert!(value & (Self::BRBE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BRBE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `BRBE` field set to the given value.
    pub const fn with_brbe(mut self, value: u8) -> Self {
        self.set_brbe(value);
        self
    }

    /// Returns the value of the `ExtTrcBuff` field.
    pub const fn exttrcbuff(self) -> u8 {
        ((self.bits() >> Self::EXTTRCBUFF_SHIFT) & Self::EXTTRCBUFF_MASK) as u8
    }

    /// Sets the value of the `ExtTrcBuff` field.
    pub const fn set_exttrcbuff(&mut self, value: u8) {
        let offset = Self::EXTTRCBUFF_SHIFT;
        assert!(value & (Self::EXTTRCBUFF_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EXTTRCBUFF_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `ExtTrcBuff` field set to the given value.
    pub const fn with_exttrcbuff(mut self, value: u8) -> Self {
        self.set_exttrcbuff(value);
        self
    }

    /// Returns the value of the `HPMN0` field.
    pub const fn hpmn0(self) -> u8 {
        ((self.bits() >> Self::HPMN0_SHIFT) & Self::HPMN0_MASK) as u8
    }

    /// Sets the value of the `HPMN0` field.
    pub const fn set_hpmn0(&mut self, value: u8) {
        let offset = Self::HPMN0_SHIFT;
        assert!(value & (Self::HPMN0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::HPMN0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `HPMN0` field set to the given value.
    pub const fn with_hpmn0(mut self, value: u8) -> Self {
        self.set_hpmn0(value);
        self
    }
}

bitflags! {
    /// `ID_AA64DFR1_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdAa64dfr1El1: u64 {
    }
}

impl IdAa64dfr1El1 {
    /// Offset of the `SYSPMUID` field.
    pub const SYSPMUID_SHIFT: u32 = 0;
    /// Mask for the `SYSPMUID` field.
    pub const SYSPMUID_MASK: u64 = 0b1111_1111;
    /// Offset of the `BRPs` field.
    pub const BRPS_SHIFT: u32 = 8;
    /// Mask for the `BRPs` field.
    pub const BRPS_MASK: u64 = 0b1111_1111;
    /// Offset of the `WRPs` field.
    pub const WRPS_SHIFT: u32 = 16;
    /// Mask for the `WRPs` field.
    pub const WRPS_MASK: u64 = 0b1111_1111;
    /// Offset of the `CTX_CMPs` field.
    pub const CTX_CMPS_SHIFT: u32 = 24;
    /// Mask for the `CTX_CMPs` field.
    pub const CTX_CMPS_MASK: u64 = 0b1111_1111;
    /// Offset of the `SPMU` field.
    pub const SPMU_SHIFT: u32 = 32;
    /// Mask for the `SPMU` field.
    pub const SPMU_MASK: u64 = 0b1111;
    /// Offset of the `PMICNTR` field.
    pub const PMICNTR_SHIFT: u32 = 36;
    /// Mask for the `PMICNTR` field.
    pub const PMICNTR_MASK: u64 = 0b1111;
    /// Offset of the `ABLE` field.
    pub const ABLE_SHIFT: u32 = 40;
    /// Mask for the `ABLE` field.
    pub const ABLE_MASK: u64 = 0b1111;
    /// Offset of the `ITE` field.
    pub const ITE_SHIFT: u32 = 44;
    /// Mask for the `ITE` field.
    pub const ITE_MASK: u64 = 0b1111;
    /// Offset of the `EBEP` field.
    pub const EBEP_SHIFT: u32 = 48;
    /// Mask for the `EBEP` field.
    pub const EBEP_MASK: u64 = 0b1111;
    /// Offset of the `DPFZS` field.
    pub const DPFZS_SHIFT: u32 = 52;
    /// Mask for the `DPFZS` field.
    pub const DPFZS_MASK: u64 = 0b1111;
    /// Offset of the `ABL_CMPs` field.
    pub const ABL_CMPS_SHIFT: u32 = 56;
    /// Mask for the `ABL_CMPs` field.
    pub const ABL_CMPS_MASK: u64 = 0b1111_1111;

    /// Returns the value of the `SYSPMUID` field.
    pub const fn syspmuid(self) -> u8 {
        ((self.bits() >> Self::SYSPMUID_SHIFT) & Self::SYSPMUID_MASK) as u8
    }

    /// Sets the value of the `SYSPMUID` field.
    pub const fn set_syspmuid(&mut self, value: u8) {
        let offset = Self::SYSPMUID_SHIFT;
        assert!(value & (Self::SYSPMUID_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SYSPMUID_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `SYSPMUID` field set to the given value.
    pub const fn with_syspmuid(mut self, value: u8) -> Self {
        self.set_syspmuid(value);
        self
    }

    /// Returns the value of the `BRPs` field.
    pub const fn brps(self) -> u8 {
        ((self.bits() >> Self::BRPS_SHIFT) & Self::BRPS_MASK) as u8
    }

    /// Sets the value of the `BRPs` field.
    pub const fn set_brps(&mut self, value: u8) {
        let offset = Self::BRPS_SHIFT;
        assert!(value & (Self::BRPS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BRPS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `BRPs` field set to the given value.
    pub const fn with_brps(mut self, value: u8) -> Self {
        self.set_brps(value);
        self
    }

    /// Returns the value of the `WRPs` field.
    pub const fn wrps(self) -> u8 {
        ((self.bits() >> Self::WRPS_SHIFT) & Self::WRPS_MASK) as u8
    }

    /// Sets the value of the `WRPs` field.
    pub const fn set_wrps(&mut self, value: u8) {
        let offset = Self::WRPS_SHIFT;
        assert!(value & (Self::WRPS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::WRPS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `WRPs` field set to the given value.
    pub const fn with_wrps(mut self, value: u8) -> Self {
        self.set_wrps(value);
        self
    }

    /// Returns the value of the `CTX_CMPs` field.
    pub const fn ctx_cmps(self) -> u8 {
        ((self.bits() >> Self::CTX_CMPS_SHIFT) & Self::CTX_CMPS_MASK) as u8
    }

    /// Sets the value of the `CTX_CMPs` field.
    pub const fn set_ctx_cmps(&mut self, value: u8) {
        let offset = Self::CTX_CMPS_SHIFT;
        assert!(value & (Self::CTX_CMPS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CTX_CMPS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `CTX_CMPs` field set to the given value.
    pub const fn with_ctx_cmps(mut self, value: u8) -> Self {
        self.set_ctx_cmps(value);
        self
    }

    /// Returns the value of the `SPMU` field.
    pub const fn spmu(self) -> u8 {
        ((self.bits() >> Self::SPMU_SHIFT) & Self::SPMU_MASK) as u8
    }

    /// Sets the value of the `SPMU` field.
    pub const fn set_spmu(&mut self, value: u8) {
        let offset = Self::SPMU_SHIFT;
        assert!(value & (Self::SPMU_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SPMU_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `SPMU` field set to the given value.
    pub const fn with_spmu(mut self, value: u8) -> Self {
        self.set_spmu(value);
        self
    }

    /// Returns the value of the `PMICNTR` field.
    pub const fn pmicntr(self) -> u8 {
        ((self.bits() >> Self::PMICNTR_SHIFT) & Self::PMICNTR_MASK) as u8
    }

    /// Sets the value of the `PMICNTR` field.
    pub const fn set_pmicntr(&mut self, value: u8) {
        let offset = Self::PMICNTR_SHIFT;
        assert!(value & (Self::PMICNTR_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PMICNTR_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PMICNTR` field set to the given value.
    pub const fn with_pmicntr(mut self, value: u8) -> Self {
        self.set_pmicntr(value);
        self
    }

    /// Returns the value of the `ABLE` field.
    pub const fn able(self) -> u8 {
        ((self.bits() >> Self::ABLE_SHIFT) & Self::ABLE_MASK) as u8
    }

    /// Sets the value of the `ABLE` field.
    pub const fn set_able(&mut self, value: u8) {
        let offset = Self::ABLE_SHIFT;
        assert!(value & (Self::ABLE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ABLE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `ABLE` field set to the given value.
    pub const fn with_able(mut self, value: u8) -> Self {
        self.set_able(value);
        self
    }

    /// Returns the value of the `ITE` field.
    pub const fn ite(self) -> u8 {
        ((self.bits() >> Self::ITE_SHIFT) & Self::ITE_MASK) as u8
    }

    /// Sets the value of the `ITE` field.
    pub const fn set_ite(&mut self, value: u8) {
        let offset = Self::ITE_SHIFT;
        assert!(value & (Self::ITE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ITE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `ITE` field set to the given value.
    pub const fn with_ite(mut self, value: u8) -> Self {
        self.set_ite(value);
        self
    }

    /// Returns the value of the `EBEP` field.
    pub const fn ebep(self) -> u8 {
        ((self.bits() >> Self::EBEP_SHIFT) & Self::EBEP_MASK) as u8
    }

    /// Sets the value of the `EBEP` field.
    pub const fn set_ebep(&mut self, value: u8) {
        let offset = Self::EBEP_SHIFT;
        assert!(value & (Self::EBEP_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EBEP_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `EBEP` field set to the given value.
    pub const fn with_ebep(mut self, value: u8) -> Self {
        self.set_ebep(value);
        self
    }

    /// Returns the value of the `DPFZS` field.
    pub const fn dpfzs(self) -> u8 {
        ((self.bits() >> Self::DPFZS_SHIFT) & Self::DPFZS_MASK) as u8
    }

    /// Sets the value of the `DPFZS` field.
    pub const fn set_dpfzs(&mut self, value: u8) {
        let offset = Self::DPFZS_SHIFT;
        assert!(value & (Self::DPFZS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::DPFZS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `DPFZS` field set to the given value.
    pub const fn with_dpfzs(mut self, value: u8) -> Self {
        self.set_dpfzs(value);
        self
    }

    /// Returns the value of the `ABL_CMPs` field.
    pub const fn abl_cmps(self) -> u8 {
        ((self.bits() >> Self::ABL_CMPS_SHIFT) & Self::ABL_CMPS_MASK) as u8
    }

    /// Sets the value of the `ABL_CMPs` field.
    pub const fn set_abl_cmps(&mut self, value: u8) {
        let offset = Self::ABL_CMPS_SHIFT;
        assert!(value & (Self::ABL_CMPS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ABL_CMPS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `ABL_CMPs` field set to the given value.
    pub const fn with_abl_cmps(mut self, value: u8) -> Self {
        self.set_abl_cmps(value);
        self
    }
}

bitflags! {
    /// `ID_AA64ISAR1_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdAa64isar1El1: u64 {
    }
}

impl IdAa64isar1El1 {
    /// Offset of the `DPB` field.
    pub const DPB_SHIFT: u32 = 0;
    /// Mask for the `DPB` field.
    pub const DPB_MASK: u64 = 0b1111;
    /// Offset of the `APA` field.
    pub const APA_SHIFT: u32 = 4;
    /// Mask for the `APA` field.
    pub const APA_MASK: u64 = 0b1111;
    /// Offset of the `API` field.
    pub const API_SHIFT: u32 = 8;
    /// Mask for the `API` field.
    pub const API_MASK: u64 = 0b1111;
    /// Offset of the `JSCVT` field.
    pub const JSCVT_SHIFT: u32 = 12;
    /// Mask for the `JSCVT` field.
    pub const JSCVT_MASK: u64 = 0b1111;
    /// Offset of the `FCMA` field.
    pub const FCMA_SHIFT: u32 = 16;
    /// Mask for the `FCMA` field.
    pub const FCMA_MASK: u64 = 0b1111;
    /// Offset of the `LRCPC` field.
    pub const LRCPC_SHIFT: u32 = 20;
    /// Mask for the `LRCPC` field.
    pub const LRCPC_MASK: u64 = 0b1111;
    /// Offset of the `GPA` field.
    pub const GPA_SHIFT: u32 = 24;
    /// Mask for the `GPA` field.
    pub const GPA_MASK: u64 = 0b1111;
    /// Offset of the `GPI` field.
    pub const GPI_SHIFT: u32 = 28;
    /// Mask for the `GPI` field.
    pub const GPI_MASK: u64 = 0b1111;
    /// Offset of the `FRINTTS` field.
    pub const FRINTTS_SHIFT: u32 = 32;
    /// Mask for the `FRINTTS` field.
    pub const FRINTTS_MASK: u64 = 0b1111;
    /// Offset of the `SB` field.
    pub const SB_SHIFT: u32 = 36;
    /// Mask for the `SB` field.
    pub const SB_MASK: u64 = 0b1111;
    /// Offset of the `SPECRES` field.
    pub const SPECRES_SHIFT: u32 = 40;
    /// Mask for the `SPECRES` field.
    pub const SPECRES_MASK: u64 = 0b1111;
    /// Offset of the `BF16` field.
    pub const BF16_SHIFT: u32 = 44;
    /// Mask for the `BF16` field.
    pub const BF16_MASK: u64 = 0b1111;
    /// Offset of the `DGH` field.
    pub const DGH_SHIFT: u32 = 48;
    /// Mask for the `DGH` field.
    pub const DGH_MASK: u64 = 0b1111;
    /// Offset of the `I8MM` field.
    pub const I8MM_SHIFT: u32 = 52;
    /// Mask for the `I8MM` field.
    pub const I8MM_MASK: u64 = 0b1111;
    /// Offset of the `XS` field.
    pub const XS_SHIFT: u32 = 56;
    /// Mask for the `XS` field.
    pub const XS_MASK: u64 = 0b1111;
    /// Offset of the `LS64` field.
    pub const LS64_SHIFT: u32 = 60;
    /// Mask for the `LS64` field.
    pub const LS64_MASK: u64 = 0b1111;

    /// Returns the value of the `DPB` field.
    pub const fn dpb(self) -> u8 {
        ((self.bits() >> Self::DPB_SHIFT) & Self::DPB_MASK) as u8
    }

    /// Sets the value of the `DPB` field.
    pub const fn set_dpb(&mut self, value: u8) {
        let offset = Self::DPB_SHIFT;
        assert!(value & (Self::DPB_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::DPB_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `DPB` field set to the given value.
    pub const fn with_dpb(mut self, value: u8) -> Self {
        self.set_dpb(value);
        self
    }

    /// Returns the value of the `APA` field.
    pub const fn apa(self) -> u8 {
        ((self.bits() >> Self::APA_SHIFT) & Self::APA_MASK) as u8
    }

    /// Sets the value of the `APA` field.
    pub const fn set_apa(&mut self, value: u8) {
        let offset = Self::APA_SHIFT;
        assert!(value & (Self::APA_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::APA_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `APA` field set to the given value.
    pub const fn with_apa(mut self, value: u8) -> Self {
        self.set_apa(value);
        self
    }

    /// Returns the value of the `API` field.
    pub const fn api(self) -> u8 {
        ((self.bits() >> Self::API_SHIFT) & Self::API_MASK) as u8
    }

    /// Sets the value of the `API` field.
    pub const fn set_api(&mut self, value: u8) {
        let offset = Self::API_SHIFT;
        assert!(value & (Self::API_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::API_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `API` field set to the given value.
    pub const fn with_api(mut self, value: u8) -> Self {
        self.set_api(value);
        self
    }

    /// Returns the value of the `JSCVT` field.
    pub const fn jscvt(self) -> u8 {
        ((self.bits() >> Self::JSCVT_SHIFT) & Self::JSCVT_MASK) as u8
    }

    /// Sets the value of the `JSCVT` field.
    pub const fn set_jscvt(&mut self, value: u8) {
        let offset = Self::JSCVT_SHIFT;
        assert!(value & (Self::JSCVT_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::JSCVT_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `JSCVT` field set to the given value.
    pub const fn with_jscvt(mut self, value: u8) -> Self {
        self.set_jscvt(value);
        self
    }

    /// Returns the value of the `FCMA` field.
    pub const fn fcma(self) -> u8 {
        ((self.bits() >> Self::FCMA_SHIFT) & Self::FCMA_MASK) as u8
    }

    /// Sets the value of the `FCMA` field.
    pub const fn set_fcma(&mut self, value: u8) {
        let offset = Self::FCMA_SHIFT;
        assert!(value & (Self::FCMA_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::FCMA_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `FCMA` field set to the given value.
    pub const fn with_fcma(mut self, value: u8) -> Self {
        self.set_fcma(value);
        self
    }

    /// Returns the value of the `LRCPC` field.
    pub const fn lrcpc(self) -> u8 {
        ((self.bits() >> Self::LRCPC_SHIFT) & Self::LRCPC_MASK) as u8
    }

    /// Sets the value of the `LRCPC` field.
    pub const fn set_lrcpc(&mut self, value: u8) {
        let offset = Self::LRCPC_SHIFT;
        assert!(value & (Self::LRCPC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::LRCPC_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `LRCPC` field set to the given value.
    pub const fn with_lrcpc(mut self, value: u8) -> Self {
        self.set_lrcpc(value);
        self
    }

    /// Returns the value of the `GPA` field.
    pub const fn gpa(self) -> u8 {
        ((self.bits() >> Self::GPA_SHIFT) & Self::GPA_MASK) as u8
    }

    /// Sets the value of the `GPA` field.
    pub const fn set_gpa(&mut self, value: u8) {
        let offset = Self::GPA_SHIFT;
        assert!(value & (Self::GPA_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::GPA_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `GPA` field set to the given value.
    pub const fn with_gpa(mut self, value: u8) -> Self {
        self.set_gpa(value);
        self
    }

    /// Returns the value of the `GPI` field.
    pub const fn gpi(self) -> u8 {
        ((self.bits() >> Self::GPI_SHIFT) & Self::GPI_MASK) as u8
    }

    /// Sets the value of the `GPI` field.
    pub const fn set_gpi(&mut self, value: u8) {
        let offset = Self::GPI_SHIFT;
        assert!(value & (Self::GPI_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::GPI_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `GPI` field set to the given value.
    pub const fn with_gpi(mut self, value: u8) -> Self {
        self.set_gpi(value);
        self
    }

    /// Returns the value of the `FRINTTS` field.
    pub const fn frintts(self) -> u8 {
        ((self.bits() >> Self::FRINTTS_SHIFT) & Self::FRINTTS_MASK) as u8
    }

    /// Sets the value of the `FRINTTS` field.
    pub const fn set_frintts(&mut self, value: u8) {
        let offset = Self::FRINTTS_SHIFT;
        assert!(value & (Self::FRINTTS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::FRINTTS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `FRINTTS` field set to the given value.
    pub const fn with_frintts(mut self, value: u8) -> Self {
        self.set_frintts(value);
        self
    }

    /// Returns the value of the `SB` field.
    pub const fn sb(self) -> u8 {
        ((self.bits() >> Self::SB_SHIFT) & Self::SB_MASK) as u8
    }

    /// Sets the value of the `SB` field.
    pub const fn set_sb(&mut self, value: u8) {
        let offset = Self::SB_SHIFT;
        assert!(value & (Self::SB_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SB_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `SB` field set to the given value.
    pub const fn with_sb(mut self, value: u8) -> Self {
        self.set_sb(value);
        self
    }

    /// Returns the value of the `SPECRES` field.
    pub const fn specres(self) -> u8 {
        ((self.bits() >> Self::SPECRES_SHIFT) & Self::SPECRES_MASK) as u8
    }

    /// Sets the value of the `SPECRES` field.
    pub const fn set_specres(&mut self, value: u8) {
        let offset = Self::SPECRES_SHIFT;
        assert!(value & (Self::SPECRES_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SPECRES_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `SPECRES` field set to the given value.
    pub const fn with_specres(mut self, value: u8) -> Self {
        self.set_specres(value);
        self
    }

    /// Returns the value of the `BF16` field.
    pub const fn bf16(self) -> u8 {
        ((self.bits() >> Self::BF16_SHIFT) & Self::BF16_MASK) as u8
    }

    /// Sets the value of the `BF16` field.
    pub const fn set_bf16(&mut self, value: u8) {
        let offset = Self::BF16_SHIFT;
        assert!(value & (Self::BF16_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BF16_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `BF16` field set to the given value.
    pub const fn with_bf16(mut self, value: u8) -> Self {
        self.set_bf16(value);
        self
    }

    /// Returns the value of the `DGH` field.
    pub const fn dgh(self) -> u8 {
        ((self.bits() >> Self::DGH_SHIFT) & Self::DGH_MASK) as u8
    }

    /// Sets the value of the `DGH` field.
    pub const fn set_dgh(&mut self, value: u8) {
        let offset = Self::DGH_SHIFT;
        assert!(value & (Self::DGH_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::DGH_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `DGH` field set to the given value.
    pub const fn with_dgh(mut self, value: u8) -> Self {
        self.set_dgh(value);
        self
    }

    /// Returns the value of the `I8MM` field.
    pub const fn i8mm(self) -> u8 {
        ((self.bits() >> Self::I8MM_SHIFT) & Self::I8MM_MASK) as u8
    }

    /// Sets the value of the `I8MM` field.
    pub const fn set_i8mm(&mut self, value: u8) {
        let offset = Self::I8MM_SHIFT;
        assert!(value & (Self::I8MM_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::I8MM_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `I8MM` field set to the given value.
    pub const fn with_i8mm(mut self, value: u8) -> Self {
        self.set_i8mm(value);
        self
    }

    /// Returns the value of the `XS` field.
    pub const fn xs(self) -> u8 {
        ((self.bits() >> Self::XS_SHIFT) & Self::XS_MASK) as u8
    }

    /// Sets the value of the `XS` field.
    pub const fn set_xs(&mut self, value: u8) {
        let offset = Self::XS_SHIFT;
        assert!(value & (Self::XS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::XS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `XS` field set to the given value.
    pub const fn with_xs(mut self, value: u8) -> Self {
        self.set_xs(value);
        self
    }

    /// Returns the value of the `LS64` field.
    pub const fn ls64(self) -> u8 {
        ((self.bits() >> Self::LS64_SHIFT) & Self::LS64_MASK) as u8
    }

    /// Sets the value of the `LS64` field.
    pub const fn set_ls64(&mut self, value: u8) {
        let offset = Self::LS64_SHIFT;
        assert!(value & (Self::LS64_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::LS64_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `LS64` field set to the given value.
    pub const fn with_ls64(mut self, value: u8) -> Self {
        self.set_ls64(value);
        self
    }
}

bitflags! {
    /// `ID_AA64ISAR2_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdAa64isar2El1: u64 {
    }
}

impl IdAa64isar2El1 {
    /// Offset of the `WFxT` field.
    pub const WFXT_SHIFT: u32 = 0;
    /// Mask for the `WFxT` field.
    pub const WFXT_MASK: u64 = 0b1111;
    /// Offset of the `RPRES` field.
    pub const RPRES_SHIFT: u32 = 4;
    /// Mask for the `RPRES` field.
    pub const RPRES_MASK: u64 = 0b1111;
    /// Offset of the `GPA3` field.
    pub const GPA3_SHIFT: u32 = 8;
    /// Mask for the `GPA3` field.
    pub const GPA3_MASK: u64 = 0b1111;
    /// Offset of the `APA3` field.
    pub const APA3_SHIFT: u32 = 12;
    /// Mask for the `APA3` field.
    pub const APA3_MASK: u64 = 0b1111;
    /// Offset of the `MOPS` field.
    pub const MOPS_SHIFT: u32 = 16;
    /// Mask for the `MOPS` field.
    pub const MOPS_MASK: u64 = 0b1111;
    /// Offset of the `BC` field.
    pub const BC_SHIFT: u32 = 20;
    /// Mask for the `BC` field.
    pub const BC_MASK: u64 = 0b1111;
    /// Offset of the `PAC_frac` field.
    pub const PAC_FRAC_SHIFT: u32 = 24;
    /// Mask for the `PAC_frac` field.
    pub const PAC_FRAC_MASK: u64 = 0b1111;
    /// Offset of the `CLRBHB` field.
    pub const CLRBHB_SHIFT: u32 = 28;
    /// Mask for the `CLRBHB` field.
    pub const CLRBHB_MASK: u64 = 0b1111;
    /// Offset of the `SYSREG_128` field.
    pub const SYSREG_128_SHIFT: u32 = 32;
    /// Mask for the `SYSREG_128` field.
    pub const SYSREG_128_MASK: u64 = 0b1111;
    /// Offset of the `SYSINSTR_128` field.
    pub const SYSINSTR_128_SHIFT: u32 = 36;
    /// Mask for the `SYSINSTR_128` field.
    pub const SYSINSTR_128_MASK: u64 = 0b1111;
    /// Offset of the `PRFMSLC` field.
    pub const PRFMSLC_SHIFT: u32 = 40;
    /// Mask for the `PRFMSLC` field.
    pub const PRFMSLC_MASK: u64 = 0b1111;
    /// Offset of the `PCDPHINT` field.
    pub const PCDPHINT_SHIFT: u32 = 44;
    /// Mask for the `PCDPHINT` field.
    pub const PCDPHINT_MASK: u64 = 0b1111;
    /// Offset of the `RPRFM` field.
    pub const RPRFM_SHIFT: u32 = 48;
    /// Mask for the `RPRFM` field.
    pub const RPRFM_MASK: u64 = 0b1111;
    /// Offset of the `CSSC` field.
    pub const CSSC_SHIFT: u32 = 52;
    /// Mask for the `CSSC` field.
    pub const CSSC_MASK: u64 = 0b1111;
    /// Offset of the `LUT` field.
    pub const LUT_SHIFT: u32 = 56;
    /// Mask for the `LUT` field.
    pub const LUT_MASK: u64 = 0b1111;
    /// Offset of the `ATS1A` field.
    pub const ATS1A_SHIFT: u32 = 60;
    /// Mask for the `ATS1A` field.
    pub const ATS1A_MASK: u64 = 0b1111;

    /// Returns the value of the `WFxT` field.
    pub const fn wfxt(self) -> u8 {
        ((self.bits() >> Self::WFXT_SHIFT) & Self::WFXT_MASK) as u8
    }

    /// Sets the value of the `WFxT` field.
    pub const fn set_wfxt(&mut self, value: u8) {
        let offset = Self::WFXT_SHIFT;
        assert!(value & (Self::WFXT_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::WFXT_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `WFxT` field set to the given value.
    pub const fn with_wfxt(mut self, value: u8) -> Self {
        self.set_wfxt(value);
        self
    }

    /// Returns the value of the `RPRES` field.
    pub const fn rpres(self) -> u8 {
        ((self.bits() >> Self::RPRES_SHIFT) & Self::RPRES_MASK) as u8
    }

    /// Sets the value of the `RPRES` field.
    pub const fn set_rpres(&mut self, value: u8) {
        let offset = Self::RPRES_SHIFT;
        assert!(value & (Self::RPRES_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::RPRES_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `RPRES` field set to the given value.
    pub const fn with_rpres(mut self, value: u8) -> Self {
        self.set_rpres(value);
        self
    }

    /// Returns the value of the `GPA3` field.
    pub const fn gpa3(self) -> u8 {
        ((self.bits() >> Self::GPA3_SHIFT) & Self::GPA3_MASK) as u8
    }

    /// Sets the value of the `GPA3` field.
    pub const fn set_gpa3(&mut self, value: u8) {
        let offset = Self::GPA3_SHIFT;
        assert!(value & (Self::GPA3_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::GPA3_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `GPA3` field set to the given value.
    pub const fn with_gpa3(mut self, value: u8) -> Self {
        self.set_gpa3(value);
        self
    }

    /// Returns the value of the `APA3` field.
    pub const fn apa3(self) -> u8 {
        ((self.bits() >> Self::APA3_SHIFT) & Self::APA3_MASK) as u8
    }

    /// Sets the value of the `APA3` field.
    pub const fn set_apa3(&mut self, value: u8) {
        let offset = Self::APA3_SHIFT;
        assert!(value & (Self::APA3_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::APA3_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `APA3` field set to the given value.
    pub const fn with_apa3(mut self, value: u8) -> Self {
        self.set_apa3(value);
        self
    }

    /// Returns the value of the `MOPS` field.
    pub const fn mops(self) -> u8 {
        ((self.bits() >> Self::MOPS_SHIFT) & Self::MOPS_MASK) as u8
    }

    /// Sets the value of the `MOPS` field.
    pub const fn set_mops(&mut self, value: u8) {
        let offset = Self::MOPS_SHIFT;
        assert!(value & (Self::MOPS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::MOPS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `MOPS` field set to the given value.
    pub const fn with_mops(mut self, value: u8) -> Self {
        self.set_mops(value);
        self
    }

    /// Returns the value of the `BC` field.
    pub const fn bc(self) -> u8 {
        ((self.bits() >> Self::BC_SHIFT) & Self::BC_MASK) as u8
    }

    /// Sets the value of the `BC` field.
    pub const fn set_bc(&mut self, value: u8) {
        let offset = Self::BC_SHIFT;
        assert!(value & (Self::BC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BC_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `BC` field set to the given value.
    pub const fn with_bc(mut self, value: u8) -> Self {
        self.set_bc(value);
        self
    }

    /// Returns the value of the `PAC_frac` field.
    pub const fn pac_frac(self) -> u8 {
        ((self.bits() >> Self::PAC_FRAC_SHIFT) & Self::PAC_FRAC_MASK) as u8
    }

    /// Sets the value of the `PAC_frac` field.
    pub const fn set_pac_frac(&mut self, value: u8) {
        let offset = Self::PAC_FRAC_SHIFT;
        assert!(value & (Self::PAC_FRAC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PAC_FRAC_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PAC_frac` field set to the given value.
    pub const fn with_pac_frac(mut self, value: u8) -> Self {
        self.set_pac_frac(value);
        self
    }

    /// Returns the value of the `CLRBHB` field.
    pub const fn clrbhb(self) -> u8 {
        ((self.bits() >> Self::CLRBHB_SHIFT) & Self::CLRBHB_MASK) as u8
    }

    /// Sets the value of the `CLRBHB` field.
    pub const fn set_clrbhb(&mut self, value: u8) {
        let offset = Self::CLRBHB_SHIFT;
        assert!(value & (Self::CLRBHB_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CLRBHB_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `CLRBHB` field set to the given value.
    pub const fn with_clrbhb(mut self, value: u8) -> Self {
        self.set_clrbhb(value);
        self
    }

    /// Returns the value of the `SYSREG_128` field.
    pub const fn sysreg_128(self) -> u8 {
        ((self.bits() >> Self::SYSREG_128_SHIFT) & Self::SYSREG_128_MASK) as u8
    }

    /// Sets the value of the `SYSREG_128` field.
    pub const fn set_sysreg_128(&mut self, value: u8) {
        let offset = Self::SYSREG_128_SHIFT;
        assert!(value & (Self::SYSREG_128_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SYSREG_128_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `SYSREG_128` field set to the given value.
    pub const fn with_sysreg_128(mut self, value: u8) -> Self {
        self.set_sysreg_128(value);
        self
    }

    /// Returns the value of the `SYSINSTR_128` field.
    pub const fn sysinstr_128(self) -> u8 {
        ((self.bits() >> Self::SYSINSTR_128_SHIFT) & Self::SYSINSTR_128_MASK) as u8
    }

    /// Sets the value of the `SYSINSTR_128` field.
    pub const fn set_sysinstr_128(&mut self, value: u8) {
        let offset = Self::SYSINSTR_128_SHIFT;
        assert!(value & (Self::SYSINSTR_128_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SYSINSTR_128_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `SYSINSTR_128` field set to the given value.
    pub const fn with_sysinstr_128(mut self, value: u8) -> Self {
        self.set_sysinstr_128(value);
        self
    }

    /// Returns the value of the `PRFMSLC` field.
    pub const fn prfmslc(self) -> u8 {
        ((self.bits() >> Self::PRFMSLC_SHIFT) & Self::PRFMSLC_MASK) as u8
    }

    /// Sets the value of the `PRFMSLC` field.
    pub const fn set_prfmslc(&mut self, value: u8) {
        let offset = Self::PRFMSLC_SHIFT;
        assert!(value & (Self::PRFMSLC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PRFMSLC_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PRFMSLC` field set to the given value.
    pub const fn with_prfmslc(mut self, value: u8) -> Self {
        self.set_prfmslc(value);
        self
    }

    /// Returns the value of the `PCDPHINT` field.
    pub const fn pcdphint(self) -> u8 {
        ((self.bits() >> Self::PCDPHINT_SHIFT) & Self::PCDPHINT_MASK) as u8
    }

    /// Sets the value of the `PCDPHINT` field.
    pub const fn set_pcdphint(&mut self, value: u8) {
        let offset = Self::PCDPHINT_SHIFT;
        assert!(value & (Self::PCDPHINT_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PCDPHINT_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PCDPHINT` field set to the given value.
    pub const fn with_pcdphint(mut self, value: u8) -> Self {
        self.set_pcdphint(value);
        self
    }

    /// Returns the value of the `RPRFM` field.
    pub const fn rprfm(self) -> u8 {
        ((self.bits() >> Self::RPRFM_SHIFT) & Self::RPRFM_MASK) as u8
    }

    /// Sets the value of the `RPRFM` field.
    pub const fn set_rprfm(&mut self, value: u8) {
        let offset = Self::RPRFM_SHIFT;
        assert!(value & (Self::RPRFM_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::RPRFM_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `RPRFM` field set to the given value.
    pub const fn with_rprfm(mut self, value: u8) -> Self {
        self.set_rprfm(value);
        self
    }

    /// Returns the value of the `CSSC` field.
    pub const fn cssc(self) -> u8 {
        ((self.bits() >> Self::CSSC_SHIFT) & Self::CSSC_MASK) as u8
    }

    /// Sets the value of the `CSSC` field.
    pub const fn set_cssc(&mut self, value: u8) {
        let offset = Self::CSSC_SHIFT;
        assert!(value & (Self::CSSC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CSSC_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `CSSC` field set to the given value.
    pub const fn with_cssc(mut self, value: u8) -> Self {
        self.set_cssc(value);
        self
    }

    /// Returns the value of the `LUT` field.
    pub const fn lut(self) -> u8 {
        ((self.bits() >> Self::LUT_SHIFT) & Self::LUT_MASK) as u8
    }

    /// Sets the value of the `LUT` field.
    pub const fn set_lut(&mut self, value: u8) {
        let offset = Self::LUT_SHIFT;
        assert!(value & (Self::LUT_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::LUT_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `LUT` field set to the given value.
    pub const fn with_lut(mut self, value: u8) -> Self {
        self.set_lut(value);
        self
    }

    /// Returns the value of the `ATS1A` field.
    pub const fn ats1a(self) -> u8 {
        ((self.bits() >> Self::ATS1A_SHIFT) & Self::ATS1A_MASK) as u8
    }

    /// Sets the value of the `ATS1A` field.
    pub const fn set_ats1a(&mut self, value: u8) {
        let offset = Self::ATS1A_SHIFT;
        assert!(value & (Self::ATS1A_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ATS1A_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `ATS1A` field set to the given value.
    pub const fn with_ats1a(mut self, value: u8) -> Self {
        self.set_ats1a(value);
        self
    }
}

bitflags! {
    /// `ID_AA64ISAR3_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdAa64isar3El1: u64 {
    }
}

impl IdAa64isar3El1 {
    /// Offset of the `CPA` field.
    pub const CPA_SHIFT: u32 = 0;
    /// Mask for the `CPA` field.
    pub const CPA_MASK: u64 = 0b1111;
    /// Offset of the `FAMINMAX` field.
    pub const FAMINMAX_SHIFT: u32 = 4;
    /// Mask for the `FAMINMAX` field.
    pub const FAMINMAX_MASK: u64 = 0b1111;
    /// Offset of the `TLBIW` field.
    pub const TLBIW_SHIFT: u32 = 8;
    /// Mask for the `TLBIW` field.
    pub const TLBIW_MASK: u64 = 0b1111;
    /// Offset of the `PACM` field.
    pub const PACM_SHIFT: u32 = 12;
    /// Mask for the `PACM` field.
    pub const PACM_MASK: u64 = 0b1111;
    /// Offset of the `LSFE` field.
    pub const LSFE_SHIFT: u32 = 16;
    /// Mask for the `LSFE` field.
    pub const LSFE_MASK: u64 = 0b1111;
    /// Offset of the `OCCMO` field.
    pub const OCCMO_SHIFT: u32 = 20;
    /// Mask for the `OCCMO` field.
    pub const OCCMO_MASK: u64 = 0b1111;
    /// Offset of the `LSUI` field.
    pub const LSUI_SHIFT: u32 = 24;
    /// Mask for the `LSUI` field.
    pub const LSUI_MASK: u64 = 0b1111;
    /// Offset of the `FPRCVT` field.
    pub const FPRCVT_SHIFT: u32 = 28;
    /// Mask for the `FPRCVT` field.
    pub const FPRCVT_MASK: u64 = 0b1111;
    /// Offset of the `PAC_frac2` field.
    pub const PAC_FRAC2_SHIFT: u32 = 32;
    /// Mask for the `PAC_frac2` field.
    pub const PAC_FRAC2_MASK: u64 = 0b1111;
    /// Offset of the `MTETC` field.
    pub const MTETC_SHIFT: u32 = 36;
    /// Mask for the `MTETC` field.
    pub const MTETC_MASK: u64 = 0b1111;
    /// Offset of the `LSCSHINT` field.
    pub const LSCSHINT_SHIFT: u32 = 40;
    /// Mask for the `LSCSHINT` field.
    pub const LSCSHINT_MASK: u64 = 0b1111;
    /// Offset of the `LSCP` field.
    pub const LSCP_SHIFT: u32 = 44;
    /// Mask for the `LSCP` field.
    pub const LSCP_MASK: u64 = 0b1111;

    /// Returns the value of the `CPA` field.
    pub const fn cpa(self) -> u8 {
        ((self.bits() >> Self::CPA_SHIFT) & Self::CPA_MASK) as u8
    }

    /// Sets the value of the `CPA` field.
    pub const fn set_cpa(&mut self, value: u8) {
        let offset = Self::CPA_SHIFT;
        assert!(value & (Self::CPA_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CPA_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `CPA` field set to the given value.
    pub const fn with_cpa(mut self, value: u8) -> Self {
        self.set_cpa(value);
        self
    }

    /// Returns the value of the `FAMINMAX` field.
    pub const fn faminmax(self) -> u8 {
        ((self.bits() >> Self::FAMINMAX_SHIFT) & Self::FAMINMAX_MASK) as u8
    }

    /// Sets the value of the `FAMINMAX` field.
    pub const fn set_faminmax(&mut self, value: u8) {
        let offset = Self::FAMINMAX_SHIFT;
        assert!(value & (Self::FAMINMAX_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::FAMINMAX_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `FAMINMAX` field set to the given value.
    pub const fn with_faminmax(mut self, value: u8) -> Self {
        self.set_faminmax(value);
        self
    }

    /// Returns the value of the `TLBIW` field.
    pub const fn tlbiw(self) -> u8 {
        ((self.bits() >> Self::TLBIW_SHIFT) & Self::TLBIW_MASK) as u8
    }

    /// Sets the value of the `TLBIW` field.
    pub const fn set_tlbiw(&mut self, value: u8) {
        let offset = Self::TLBIW_SHIFT;
        assert!(value & (Self::TLBIW_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TLBIW_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `TLBIW` field set to the given value.
    pub const fn with_tlbiw(mut self, value: u8) -> Self {
        self.set_tlbiw(value);
        self
    }

    /// Returns the value of the `PACM` field.
    pub const fn pacm(self) -> u8 {
        ((self.bits() >> Self::PACM_SHIFT) & Self::PACM_MASK) as u8
    }

    /// Sets the value of the `PACM` field.
    pub const fn set_pacm(&mut self, value: u8) {
        let offset = Self::PACM_SHIFT;
        assert!(value & (Self::PACM_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PACM_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PACM` field set to the given value.
    pub const fn with_pacm(mut self, value: u8) -> Self {
        self.set_pacm(value);
        self
    }

    /// Returns the value of the `LSFE` field.
    pub const fn lsfe(self) -> u8 {
        ((self.bits() >> Self::LSFE_SHIFT) & Self::LSFE_MASK) as u8
    }

    /// Sets the value of the `LSFE` field.
    pub const fn set_lsfe(&mut self, value: u8) {
        let offset = Self::LSFE_SHIFT;
        assert!(value & (Self::LSFE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::LSFE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `LSFE` field set to the given value.
    pub const fn with_lsfe(mut self, value: u8) -> Self {
        self.set_lsfe(value);
        self
    }

    /// Returns the value of the `OCCMO` field.
    pub const fn occmo(self) -> u8 {
        ((self.bits() >> Self::OCCMO_SHIFT) & Self::OCCMO_MASK) as u8
    }

    /// Sets the value of the `OCCMO` field.
    pub const fn set_occmo(&mut self, value: u8) {
        let offset = Self::OCCMO_SHIFT;
        assert!(value & (Self::OCCMO_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::OCCMO_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `OCCMO` field set to the given value.
    pub const fn with_occmo(mut self, value: u8) -> Self {
        self.set_occmo(value);
        self
    }

    /// Returns the value of the `LSUI` field.
    pub const fn lsui(self) -> u8 {
        ((self.bits() >> Self::LSUI_SHIFT) & Self::LSUI_MASK) as u8
    }

    /// Sets the value of the `LSUI` field.
    pub const fn set_lsui(&mut self, value: u8) {
        let offset = Self::LSUI_SHIFT;
        assert!(value & (Self::LSUI_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::LSUI_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `LSUI` field set to the given value.
    pub const fn with_lsui(mut self, value: u8) -> Self {
        self.set_lsui(value);
        self
    }

    /// Returns the value of the `FPRCVT` field.
    pub const fn fprcvt(self) -> u8 {
        ((self.bits() >> Self::FPRCVT_SHIFT) & Self::FPRCVT_MASK) as u8
    }

    /// Sets the value of the `FPRCVT` field.
    pub const fn set_fprcvt(&mut self, value: u8) {
        let offset = Self::FPRCVT_SHIFT;
        assert!(value & (Self::FPRCVT_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::FPRCVT_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `FPRCVT` field set to the given value.
    pub const fn with_fprcvt(mut self, value: u8) -> Self {
        self.set_fprcvt(value);
        self
    }

    /// Returns the value of the `PAC_frac2` field.
    pub const fn pac_frac2(self) -> u8 {
        ((self.bits() >> Self::PAC_FRAC2_SHIFT) & Self::PAC_FRAC2_MASK) as u8
    }

    /// Sets the value of the `PAC_frac2` field.
    pub const fn set_pac_frac2(&mut self, value: u8) {
        let offset = Self::PAC_FRAC2_SHIFT;
        assert!(value & (Self::PAC_FRAC2_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PAC_FRAC2_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PAC_frac2` field set to the given value.
    pub const fn with_pac_frac2(mut self, value: u8) -> Self {
        self.set_pac_frac2(value);
        self
    }

    /// Returns the value of the `MTETC` field.
    pub const fn mtetc(self) -> u8 {
        ((self.bits() >> Self::MTETC_SHIFT) & Self::MTETC_MASK) as u8
    }

    /// Sets the value of the `MTETC` field.
    pub const fn set_mtetc(&mut self, value: u8) {
        let offset = Self::MTETC_SHIFT;
        assert!(value & (Self::MTETC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::MTETC_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `MTETC` field set to the given value.
    pub const fn with_mtetc(mut self, value: u8) -> Self {
        self.set_mtetc(value);
        self
    }

    /// Returns the value of the `LSCSHINT` field.
    pub const fn lscshint(self) -> u8 {
        ((self.bits() >> Self::LSCSHINT_SHIFT) & Self::LSCSHINT_MASK) as u8
    }

    /// Sets the value of the `LSCSHINT` field.
    pub const fn set_lscshint(&mut self, value: u8) {
        let offset = Self::LSCSHINT_SHIFT;
        assert!(value & (Self::LSCSHINT_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::LSCSHINT_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `LSCSHINT` field set to the given value.
    pub const fn with_lscshint(mut self, value: u8) -> Self {
        self.set_lscshint(value);
        self
    }

    /// Returns the value of the `LSCP` field.
    pub const fn lscp(self) -> u8 {
        ((self.bits() >> Self::LSCP_SHIFT) & Self::LSCP_MASK) as u8
    }

    /// Sets the value of the `LSCP` field.
    pub const fn set_lscp(&mut self, value: u8) {
        let offset = Self::LSCP_SHIFT;
        assert!(value & (Self::LSCP_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::LSCP_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `LSCP` field set to the given value.
    pub const fn with_lscp(mut self, value: u8) -> Self {
        self.set_lscp(value);
        self
    }
}

bitflags! {
    /// `ID_AA64MMFR0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdAa64mmfr0El1: u64 {
    }
}

impl IdAa64mmfr0El1 {
    /// Offset of the `PARange` field.
    pub const PARANGE_SHIFT: u32 = 0;
    /// Mask for the `PARange` field.
    pub const PARANGE_MASK: u64 = 0b1111;
    /// Offset of the `ASIDBits` field.
    pub const ASIDBITS_SHIFT: u32 = 4;
    /// Mask for the `ASIDBits` field.
    pub const ASIDBITS_MASK: u64 = 0b1111;
    /// Offset of the `BigEnd` field.
    pub const BIGEND_SHIFT: u32 = 8;
    /// Mask for the `BigEnd` field.
    pub const BIGEND_MASK: u64 = 0b1111;
    /// Offset of the `SNSMem` field.
    pub const SNSMEM_SHIFT: u32 = 12;
    /// Mask for the `SNSMem` field.
    pub const SNSMEM_MASK: u64 = 0b1111;
    /// Offset of the `BigEndEL0` field.
    pub const BIGENDEL0_SHIFT: u32 = 16;
    /// Mask for the `BigEndEL0` field.
    pub const BIGENDEL0_MASK: u64 = 0b1111;
    /// Offset of the `TGran16` field.
    pub const TGRAN16_SHIFT: u32 = 20;
    /// Mask for the `TGran16` field.
    pub const TGRAN16_MASK: u64 = 0b1111;
    /// Offset of the `TGran64` field.
    pub const TGRAN64_SHIFT: u32 = 24;
    /// Mask for the `TGran64` field.
    pub const TGRAN64_MASK: u64 = 0b1111;
    /// Offset of the `TGran4` field.
    pub const TGRAN4_SHIFT: u32 = 28;
    /// Mask for the `TGran4` field.
    pub const TGRAN4_MASK: u64 = 0b1111;
    /// Offset of the `TGran16_2` field.
    pub const TGRAN16_2_SHIFT: u32 = 32;
    /// Mask for the `TGran16_2` field.
    pub const TGRAN16_2_MASK: u64 = 0b1111;
    /// Offset of the `TGran64_2` field.
    pub const TGRAN64_2_SHIFT: u32 = 36;
    /// Mask for the `TGran64_2` field.
    pub const TGRAN64_2_MASK: u64 = 0b1111;
    /// Offset of the `TGran4_2` field.
    pub const TGRAN4_2_SHIFT: u32 = 40;
    /// Mask for the `TGran4_2` field.
    pub const TGRAN4_2_MASK: u64 = 0b1111;
    /// Offset of the `ExS` field.
    pub const EXS_SHIFT: u32 = 44;
    /// Mask for the `ExS` field.
    pub const EXS_MASK: u64 = 0b1111;
    /// Offset of the `FGT` field.
    pub const FGT_SHIFT: u32 = 56;
    /// Mask for the `FGT` field.
    pub const FGT_MASK: u64 = 0b1111;
    /// Offset of the `ECV` field.
    pub const ECV_SHIFT: u32 = 60;
    /// Mask for the `ECV` field.
    pub const ECV_MASK: u64 = 0b1111;

    /// Returns the value of the `PARange` field.
    pub const fn parange(self) -> u8 {
        ((self.bits() >> Self::PARANGE_SHIFT) & Self::PARANGE_MASK) as u8
    }

    /// Sets the value of the `PARange` field.
    pub const fn set_parange(&mut self, value: u8) {
        let offset = Self::PARANGE_SHIFT;
        assert!(value & (Self::PARANGE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PARANGE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PARange` field set to the given value.
    pub const fn with_parange(mut self, value: u8) -> Self {
        self.set_parange(value);
        self
    }

    /// Returns the value of the `ASIDBits` field.
    pub const fn asidbits(self) -> u8 {
        ((self.bits() >> Self::ASIDBITS_SHIFT) & Self::ASIDBITS_MASK) as u8
    }

    /// Sets the value of the `ASIDBits` field.
    pub const fn set_asidbits(&mut self, value: u8) {
        let offset = Self::ASIDBITS_SHIFT;
        assert!(value & (Self::ASIDBITS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ASIDBITS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `ASIDBits` field set to the given value.
    pub const fn with_asidbits(mut self, value: u8) -> Self {
        self.set_asidbits(value);
        self
    }

    /// Returns the value of the `BigEnd` field.
    pub const fn bigend(self) -> u8 {
        ((self.bits() >> Self::BIGEND_SHIFT) & Self::BIGEND_MASK) as u8
    }

    /// Sets the value of the `BigEnd` field.
    pub const fn set_bigend(&mut self, value: u8) {
        let offset = Self::BIGEND_SHIFT;
        assert!(value & (Self::BIGEND_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BIGEND_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `BigEnd` field set to the given value.
    pub const fn with_bigend(mut self, value: u8) -> Self {
        self.set_bigend(value);
        self
    }

    /// Returns the value of the `SNSMem` field.
    pub const fn snsmem(self) -> u8 {
        ((self.bits() >> Self::SNSMEM_SHIFT) & Self::SNSMEM_MASK) as u8
    }

    /// Sets the value of the `SNSMem` field.
    pub const fn set_snsmem(&mut self, value: u8) {
        let offset = Self::SNSMEM_SHIFT;
        assert!(value & (Self::SNSMEM_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SNSMEM_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `SNSMem` field set to the given value.
    pub const fn with_snsmem(mut self, value: u8) -> Self {
        self.set_snsmem(value);
        self
    }

    /// Returns the value of the `BigEndEL0` field.
    pub const fn bigendel0(self) -> u8 {
        ((self.bits() >> Self::BIGENDEL0_SHIFT) & Self::BIGENDEL0_MASK) as u8
    }

    /// Sets the value of the `BigEndEL0` field.
    pub const fn set_bigendel0(&mut self, value: u8) {
        let offset = Self::BIGENDEL0_SHIFT;
        assert!(value & (Self::BIGENDEL0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BIGENDEL0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `BigEndEL0` field set to the given value.
    pub const fn with_bigendel0(mut self, value: u8) -> Self {
        self.set_bigendel0(value);
        self
    }

    /// Returns the value of the `TGran16` field.
    pub const fn tgran16(self) -> u8 {
        ((self.bits() >> Self::TGRAN16_SHIFT) & Self::TGRAN16_MASK) as u8
    }

    /// Sets the value of the `TGran16` field.
    pub const fn set_tgran16(&mut self, value: u8) {
        let offset = Self::TGRAN16_SHIFT;
        assert!(value & (Self::TGRAN16_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TGRAN16_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `TGran16` field set to the given value.
    pub const fn with_tgran16(mut self, value: u8) -> Self {
        self.set_tgran16(value);
        self
    }

    /// Returns the value of the `TGran64` field.
    pub const fn tgran64(self) -> u8 {
        ((self.bits() >> Self::TGRAN64_SHIFT) & Self::TGRAN64_MASK) as u8
    }

    /// Sets the value of the `TGran64` field.
    pub const fn set_tgran64(&mut self, value: u8) {
        let offset = Self::TGRAN64_SHIFT;
        assert!(value & (Self::TGRAN64_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TGRAN64_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `TGran64` field set to the given value.
    pub const fn with_tgran64(mut self, value: u8) -> Self {
        self.set_tgran64(value);
        self
    }

    /// Returns the value of the `TGran4` field.
    pub const fn tgran4(self) -> u8 {
        ((self.bits() >> Self::TGRAN4_SHIFT) & Self::TGRAN4_MASK) as u8
    }

    /// Sets the value of the `TGran4` field.
    pub const fn set_tgran4(&mut self, value: u8) {
        let offset = Self::TGRAN4_SHIFT;
        assert!(value & (Self::TGRAN4_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TGRAN4_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `TGran4` field set to the given value.
    pub const fn with_tgran4(mut self, value: u8) -> Self {
        self.set_tgran4(value);
        self
    }

    /// Returns the value of the `TGran16_2` field.
    pub const fn tgran16_2(self) -> u8 {
        ((self.bits() >> Self::TGRAN16_2_SHIFT) & Self::TGRAN16_2_MASK) as u8
    }

    /// Sets the value of the `TGran16_2` field.
    pub const fn set_tgran16_2(&mut self, value: u8) {
        let offset = Self::TGRAN16_2_SHIFT;
        assert!(value & (Self::TGRAN16_2_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TGRAN16_2_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `TGran16_2` field set to the given value.
    pub const fn with_tgran16_2(mut self, value: u8) -> Self {
        self.set_tgran16_2(value);
        self
    }

    /// Returns the value of the `TGran64_2` field.
    pub const fn tgran64_2(self) -> u8 {
        ((self.bits() >> Self::TGRAN64_2_SHIFT) & Self::TGRAN64_2_MASK) as u8
    }

    /// Sets the value of the `TGran64_2` field.
    pub const fn set_tgran64_2(&mut self, value: u8) {
        let offset = Self::TGRAN64_2_SHIFT;
        assert!(value & (Self::TGRAN64_2_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TGRAN64_2_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `TGran64_2` field set to the given value.
    pub const fn with_tgran64_2(mut self, value: u8) -> Self {
        self.set_tgran64_2(value);
        self
    }

    /// Returns the value of the `TGran4_2` field.
    pub const fn tgran4_2(self) -> u8 {
        ((self.bits() >> Self::TGRAN4_2_SHIFT) & Self::TGRAN4_2_MASK) as u8
    }

    /// Sets the value of the `TGran4_2` field.
    pub const fn set_tgran4_2(&mut self, value: u8) {
        let offset = Self::TGRAN4_2_SHIFT;
        assert!(value & (Self::TGRAN4_2_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TGRAN4_2_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `TGran4_2` field set to the given value.
    pub const fn with_tgran4_2(mut self, value: u8) -> Self {
        self.set_tgran4_2(value);
        self
    }

    /// Returns the value of the `ExS` field.
    pub const fn exs(self) -> u8 {
        ((self.bits() >> Self::EXS_SHIFT) & Self::EXS_MASK) as u8
    }

    /// Sets the value of the `ExS` field.
    pub const fn set_exs(&mut self, value: u8) {
        let offset = Self::EXS_SHIFT;
        assert!(value & (Self::EXS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EXS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `ExS` field set to the given value.
    pub const fn with_exs(mut self, value: u8) -> Self {
        self.set_exs(value);
        self
    }

    /// Returns the value of the `FGT` field.
    pub const fn fgt(self) -> u8 {
        ((self.bits() >> Self::FGT_SHIFT) & Self::FGT_MASK) as u8
    }

    /// Sets the value of the `FGT` field.
    pub const fn set_fgt(&mut self, value: u8) {
        let offset = Self::FGT_SHIFT;
        assert!(value & (Self::FGT_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::FGT_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `FGT` field set to the given value.
    pub const fn with_fgt(mut self, value: u8) -> Self {
        self.set_fgt(value);
        self
    }

    /// Returns the value of the `ECV` field.
    pub const fn ecv(self) -> u8 {
        ((self.bits() >> Self::ECV_SHIFT) & Self::ECV_MASK) as u8
    }

    /// Sets the value of the `ECV` field.
    pub const fn set_ecv(&mut self, value: u8) {
        let offset = Self::ECV_SHIFT;
        assert!(value & (Self::ECV_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ECV_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `ECV` field set to the given value.
    pub const fn with_ecv(mut self, value: u8) -> Self {
        self.set_ecv(value);
        self
    }
}

bitflags! {
    /// `ID_AA64MMFR1_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdAa64mmfr1El1: u64 {
    }
}

impl IdAa64mmfr1El1 {
    /// Offset of the `HAFDBS` field.
    pub const HAFDBS_SHIFT: u32 = 0;
    /// Mask for the `HAFDBS` field.
    pub const HAFDBS_MASK: u64 = 0b1111;
    /// Offset of the `VMIDBits` field.
    pub const VMIDBITS_SHIFT: u32 = 4;
    /// Mask for the `VMIDBits` field.
    pub const VMIDBITS_MASK: u64 = 0b1111;
    /// Offset of the `VH` field.
    pub const VH_SHIFT: u32 = 8;
    /// Mask for the `VH` field.
    pub const VH_MASK: u64 = 0b1111;
    /// Offset of the `HPDS` field.
    pub const HPDS_SHIFT: u32 = 12;
    /// Mask for the `HPDS` field.
    pub const HPDS_MASK: u64 = 0b1111;
    /// Offset of the `LO` field.
    pub const LO_SHIFT: u32 = 16;
    /// Mask for the `LO` field.
    pub const LO_MASK: u64 = 0b1111;
    /// Offset of the `PAN` field.
    pub const PAN_SHIFT: u32 = 20;
    /// Mask for the `PAN` field.
    pub const PAN_MASK: u64 = 0b1111;
    /// Offset of the `SpecSEI` field.
    pub const SPECSEI_SHIFT: u32 = 24;
    /// Mask for the `SpecSEI` field.
    pub const SPECSEI_MASK: u64 = 0b1111;
    /// Offset of the `XNX` field.
    pub const XNX_SHIFT: u32 = 28;
    /// Mask for the `XNX` field.
    pub const XNX_MASK: u64 = 0b1111;
    /// Offset of the `TWED` field.
    pub const TWED_SHIFT: u32 = 32;
    /// Mask for the `TWED` field.
    pub const TWED_MASK: u64 = 0b1111;
    /// Offset of the `ETS` field.
    pub const ETS_SHIFT: u32 = 36;
    /// Mask for the `ETS` field.
    pub const ETS_MASK: u64 = 0b1111;
    /// Offset of the `HCX` field.
    pub const HCX_SHIFT: u32 = 40;
    /// Mask for the `HCX` field.
    pub const HCX_MASK: u64 = 0b1111;
    /// Offset of the `AFP` field.
    pub const AFP_SHIFT: u32 = 44;
    /// Mask for the `AFP` field.
    pub const AFP_MASK: u64 = 0b1111;
    /// Offset of the `nTLBPA` field.
    pub const NTLBPA_SHIFT: u32 = 48;
    /// Mask for the `nTLBPA` field.
    pub const NTLBPA_MASK: u64 = 0b1111;
    /// Offset of the `TIDCP1` field.
    pub const TIDCP1_SHIFT: u32 = 52;
    /// Mask for the `TIDCP1` field.
    pub const TIDCP1_MASK: u64 = 0b1111;
    /// Offset of the `CMOW` field.
    pub const CMOW_SHIFT: u32 = 56;
    /// Mask for the `CMOW` field.
    pub const CMOW_MASK: u64 = 0b1111;
    /// Offset of the `ECBHB` field.
    pub const ECBHB_SHIFT: u32 = 60;
    /// Mask for the `ECBHB` field.
    pub const ECBHB_MASK: u64 = 0b1111;

    /// Returns the value of the `HAFDBS` field.
    pub const fn hafdbs(self) -> u8 {
        ((self.bits() >> Self::HAFDBS_SHIFT) & Self::HAFDBS_MASK) as u8
    }

    /// Sets the value of the `HAFDBS` field.
    pub const fn set_hafdbs(&mut self, value: u8) {
        let offset = Self::HAFDBS_SHIFT;
        assert!(value & (Self::HAFDBS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::HAFDBS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `HAFDBS` field set to the given value.
    pub const fn with_hafdbs(mut self, value: u8) -> Self {
        self.set_hafdbs(value);
        self
    }

    /// Returns the value of the `VMIDBits` field.
    pub const fn vmidbits(self) -> u8 {
        ((self.bits() >> Self::VMIDBITS_SHIFT) & Self::VMIDBITS_MASK) as u8
    }

    /// Sets the value of the `VMIDBits` field.
    pub const fn set_vmidbits(&mut self, value: u8) {
        let offset = Self::VMIDBITS_SHIFT;
        assert!(value & (Self::VMIDBITS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VMIDBITS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `VMIDBits` field set to the given value.
    pub const fn with_vmidbits(mut self, value: u8) -> Self {
        self.set_vmidbits(value);
        self
    }

    /// Returns the value of the `VH` field.
    pub const fn vh(self) -> u8 {
        ((self.bits() >> Self::VH_SHIFT) & Self::VH_MASK) as u8
    }

    /// Sets the value of the `VH` field.
    pub const fn set_vh(&mut self, value: u8) {
        let offset = Self::VH_SHIFT;
        assert!(value & (Self::VH_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VH_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `VH` field set to the given value.
    pub const fn with_vh(mut self, value: u8) -> Self {
        self.set_vh(value);
        self
    }

    /// Returns the value of the `HPDS` field.
    pub const fn hpds(self) -> u8 {
        ((self.bits() >> Self::HPDS_SHIFT) & Self::HPDS_MASK) as u8
    }

    /// Sets the value of the `HPDS` field.
    pub const fn set_hpds(&mut self, value: u8) {
        let offset = Self::HPDS_SHIFT;
        assert!(value & (Self::HPDS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::HPDS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `HPDS` field set to the given value.
    pub const fn with_hpds(mut self, value: u8) -> Self {
        self.set_hpds(value);
        self
    }

    /// Returns the value of the `LO` field.
    pub const fn lo(self) -> u8 {
        ((self.bits() >> Self::LO_SHIFT) & Self::LO_MASK) as u8
    }

    /// Sets the value of the `LO` field.
    pub const fn set_lo(&mut self, value: u8) {
        let offset = Self::LO_SHIFT;
        assert!(value & (Self::LO_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::LO_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `LO` field set to the given value.
    pub const fn with_lo(mut self, value: u8) -> Self {
        self.set_lo(value);
        self
    }

    /// Returns the value of the `PAN` field.
    pub const fn pan(self) -> u8 {
        ((self.bits() >> Self::PAN_SHIFT) & Self::PAN_MASK) as u8
    }

    /// Sets the value of the `PAN` field.
    pub const fn set_pan(&mut self, value: u8) {
        let offset = Self::PAN_SHIFT;
        assert!(value & (Self::PAN_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PAN_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PAN` field set to the given value.
    pub const fn with_pan(mut self, value: u8) -> Self {
        self.set_pan(value);
        self
    }

    /// Returns the value of the `SpecSEI` field.
    pub const fn specsei(self) -> u8 {
        ((self.bits() >> Self::SPECSEI_SHIFT) & Self::SPECSEI_MASK) as u8
    }

    /// Sets the value of the `SpecSEI` field.
    pub const fn set_specsei(&mut self, value: u8) {
        let offset = Self::SPECSEI_SHIFT;
        assert!(value & (Self::SPECSEI_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SPECSEI_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `SpecSEI` field set to the given value.
    pub const fn with_specsei(mut self, value: u8) -> Self {
        self.set_specsei(value);
        self
    }

    /// Returns the value of the `XNX` field.
    pub const fn xnx(self) -> u8 {
        ((self.bits() >> Self::XNX_SHIFT) & Self::XNX_MASK) as u8
    }

    /// Sets the value of the `XNX` field.
    pub const fn set_xnx(&mut self, value: u8) {
        let offset = Self::XNX_SHIFT;
        assert!(value & (Self::XNX_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::XNX_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `XNX` field set to the given value.
    pub const fn with_xnx(mut self, value: u8) -> Self {
        self.set_xnx(value);
        self
    }

    /// Returns the value of the `TWED` field.
    pub const fn twed(self) -> u8 {
        ((self.bits() >> Self::TWED_SHIFT) & Self::TWED_MASK) as u8
    }

    /// Sets the value of the `TWED` field.
    pub const fn set_twed(&mut self, value: u8) {
        let offset = Self::TWED_SHIFT;
        assert!(value & (Self::TWED_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TWED_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `TWED` field set to the given value.
    pub const fn with_twed(mut self, value: u8) -> Self {
        self.set_twed(value);
        self
    }

    /// Returns the value of the `ETS` field.
    pub const fn ets(self) -> u8 {
        ((self.bits() >> Self::ETS_SHIFT) & Self::ETS_MASK) as u8
    }

    /// Sets the value of the `ETS` field.
    pub const fn set_ets(&mut self, value: u8) {
        let offset = Self::ETS_SHIFT;
        assert!(value & (Self::ETS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ETS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `ETS` field set to the given value.
    pub const fn with_ets(mut self, value: u8) -> Self {
        self.set_ets(value);
        self
    }

    /// Returns the value of the `HCX` field.
    pub const fn hcx(self) -> u8 {
        ((self.bits() >> Self::HCX_SHIFT) & Self::HCX_MASK) as u8
    }

    /// Sets the value of the `HCX` field.
    pub const fn set_hcx(&mut self, value: u8) {
        let offset = Self::HCX_SHIFT;
        assert!(value & (Self::HCX_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::HCX_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `HCX` field set to the given value.
    pub const fn with_hcx(mut self, value: u8) -> Self {
        self.set_hcx(value);
        self
    }

    /// Returns the value of the `AFP` field.
    pub const fn afp(self) -> u8 {
        ((self.bits() >> Self::AFP_SHIFT) & Self::AFP_MASK) as u8
    }

    /// Sets the value of the `AFP` field.
    pub const fn set_afp(&mut self, value: u8) {
        let offset = Self::AFP_SHIFT;
        assert!(value & (Self::AFP_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::AFP_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `AFP` field set to the given value.
    pub const fn with_afp(mut self, value: u8) -> Self {
        self.set_afp(value);
        self
    }

    /// Returns the value of the `nTLBPA` field.
    pub const fn ntlbpa(self) -> u8 {
        ((self.bits() >> Self::NTLBPA_SHIFT) & Self::NTLBPA_MASK) as u8
    }

    /// Sets the value of the `nTLBPA` field.
    pub const fn set_ntlbpa(&mut self, value: u8) {
        let offset = Self::NTLBPA_SHIFT;
        assert!(value & (Self::NTLBPA_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::NTLBPA_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `nTLBPA` field set to the given value.
    pub const fn with_ntlbpa(mut self, value: u8) -> Self {
        self.set_ntlbpa(value);
        self
    }

    /// Returns the value of the `TIDCP1` field.
    pub const fn tidcp1(self) -> u8 {
        ((self.bits() >> Self::TIDCP1_SHIFT) & Self::TIDCP1_MASK) as u8
    }

    /// Sets the value of the `TIDCP1` field.
    pub const fn set_tidcp1(&mut self, value: u8) {
        let offset = Self::TIDCP1_SHIFT;
        assert!(value & (Self::TIDCP1_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TIDCP1_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `TIDCP1` field set to the given value.
    pub const fn with_tidcp1(mut self, value: u8) -> Self {
        self.set_tidcp1(value);
        self
    }

    /// Returns the value of the `CMOW` field.
    pub const fn cmow(self) -> u8 {
        ((self.bits() >> Self::CMOW_SHIFT) & Self::CMOW_MASK) as u8
    }

    /// Sets the value of the `CMOW` field.
    pub const fn set_cmow(&mut self, value: u8) {
        let offset = Self::CMOW_SHIFT;
        assert!(value & (Self::CMOW_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CMOW_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `CMOW` field set to the given value.
    pub const fn with_cmow(mut self, value: u8) -> Self {
        self.set_cmow(value);
        self
    }

    /// Returns the value of the `ECBHB` field.
    pub const fn ecbhb(self) -> u8 {
        ((self.bits() >> Self::ECBHB_SHIFT) & Self::ECBHB_MASK) as u8
    }

    /// Sets the value of the `ECBHB` field.
    pub const fn set_ecbhb(&mut self, value: u8) {
        let offset = Self::ECBHB_SHIFT;
        assert!(value & (Self::ECBHB_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ECBHB_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `ECBHB` field set to the given value.
    pub const fn with_ecbhb(mut self, value: u8) -> Self {
        self.set_ecbhb(value);
        self
    }
}

bitflags! {
    /// `ID_AA64MMFR2_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdAa64mmfr2El1: u64 {
    }
}

impl IdAa64mmfr2El1 {
    /// Offset of the `CnP` field.
    pub const CNP_SHIFT: u32 = 0;
    /// Mask for the `CnP` field.
    pub const CNP_MASK: u64 = 0b1111;
    /// Offset of the `UAO` field.
    pub const UAO_SHIFT: u32 = 4;
    /// Mask for the `UAO` field.
    pub const UAO_MASK: u64 = 0b1111;
    /// Offset of the `LSM` field.
    pub const LSM_SHIFT: u32 = 8;
    /// Mask for the `LSM` field.
    pub const LSM_MASK: u64 = 0b1111;
    /// Offset of the `IESB` field.
    pub const IESB_SHIFT: u32 = 12;
    /// Mask for the `IESB` field.
    pub const IESB_MASK: u64 = 0b1111;
    /// Offset of the `VARange` field.
    pub const VARANGE_SHIFT: u32 = 16;
    /// Mask for the `VARange` field.
    pub const VARANGE_MASK: u64 = 0b1111;
    /// Offset of the `CCIDX` field.
    pub const CCIDX_SHIFT: u32 = 20;
    /// Mask for the `CCIDX` field.
    pub const CCIDX_MASK: u64 = 0b1111;
    /// Offset of the `NV` field.
    pub const NV_SHIFT: u32 = 24;
    /// Mask for the `NV` field.
    pub const NV_MASK: u64 = 0b1111;
    /// Offset of the `ST` field.
    pub const ST_SHIFT: u32 = 28;
    /// Mask for the `ST` field.
    pub const ST_MASK: u64 = 0b1111;
    /// Offset of the `AT` field.
    pub const AT_SHIFT: u32 = 32;
    /// Mask for the `AT` field.
    pub const AT_MASK: u64 = 0b1111;
    /// Offset of the `IDS` field.
    pub const IDS_SHIFT: u32 = 36;
    /// Mask for the `IDS` field.
    pub const IDS_MASK: u64 = 0b1111;
    /// Offset of the `FWB` field.
    pub const FWB_SHIFT: u32 = 40;
    /// Mask for the `FWB` field.
    pub const FWB_MASK: u64 = 0b1111;
    /// Offset of the `TTL` field.
    pub const TTL_SHIFT: u32 = 48;
    /// Mask for the `TTL` field.
    pub const TTL_MASK: u64 = 0b1111;
    /// Offset of the `BBM` field.
    pub const BBM_SHIFT: u32 = 52;
    /// Mask for the `BBM` field.
    pub const BBM_MASK: u64 = 0b1111;
    /// Offset of the `EVT` field.
    pub const EVT_SHIFT: u32 = 56;
    /// Mask for the `EVT` field.
    pub const EVT_MASK: u64 = 0b1111;
    /// Offset of the `E0PD` field.
    pub const E0PD_SHIFT: u32 = 60;
    /// Mask for the `E0PD` field.
    pub const E0PD_MASK: u64 = 0b1111;

    /// Returns the value of the `CnP` field.
    pub const fn cnp(self) -> u8 {
        ((self.bits() >> Self::CNP_SHIFT) & Self::CNP_MASK) as u8
    }

    /// Sets the value of the `CnP` field.
    pub const fn set_cnp(&mut self, value: u8) {
        let offset = Self::CNP_SHIFT;
        assert!(value & (Self::CNP_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CNP_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `CnP` field set to the given value.
    pub const fn with_cnp(mut self, value: u8) -> Self {
        self.set_cnp(value);
        self
    }

    /// Returns the value of the `UAO` field.
    pub const fn uao(self) -> u8 {
        ((self.bits() >> Self::UAO_SHIFT) & Self::UAO_MASK) as u8
    }

    /// Sets the value of the `UAO` field.
    pub const fn set_uao(&mut self, value: u8) {
        let offset = Self::UAO_SHIFT;
        assert!(value & (Self::UAO_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::UAO_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `UAO` field set to the given value.
    pub const fn with_uao(mut self, value: u8) -> Self {
        self.set_uao(value);
        self
    }

    /// Returns the value of the `LSM` field.
    pub const fn lsm(self) -> u8 {
        ((self.bits() >> Self::LSM_SHIFT) & Self::LSM_MASK) as u8
    }

    /// Sets the value of the `LSM` field.
    pub const fn set_lsm(&mut self, value: u8) {
        let offset = Self::LSM_SHIFT;
        assert!(value & (Self::LSM_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::LSM_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `LSM` field set to the given value.
    pub const fn with_lsm(mut self, value: u8) -> Self {
        self.set_lsm(value);
        self
    }

    /// Returns the value of the `IESB` field.
    pub const fn iesb(self) -> u8 {
        ((self.bits() >> Self::IESB_SHIFT) & Self::IESB_MASK) as u8
    }

    /// Sets the value of the `IESB` field.
    pub const fn set_iesb(&mut self, value: u8) {
        let offset = Self::IESB_SHIFT;
        assert!(value & (Self::IESB_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::IESB_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `IESB` field set to the given value.
    pub const fn with_iesb(mut self, value: u8) -> Self {
        self.set_iesb(value);
        self
    }

    /// Returns the value of the `VARange` field.
    pub const fn varange(self) -> u8 {
        ((self.bits() >> Self::VARANGE_SHIFT) & Self::VARANGE_MASK) as u8
    }

    /// Sets the value of the `VARange` field.
    pub const fn set_varange(&mut self, value: u8) {
        let offset = Self::VARANGE_SHIFT;
        assert!(value & (Self::VARANGE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VARANGE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `VARange` field set to the given value.
    pub const fn with_varange(mut self, value: u8) -> Self {
        self.set_varange(value);
        self
    }

    /// Returns the value of the `CCIDX` field.
    pub const fn ccidx(self) -> u8 {
        ((self.bits() >> Self::CCIDX_SHIFT) & Self::CCIDX_MASK) as u8
    }

    /// Sets the value of the `CCIDX` field.
    pub const fn set_ccidx(&mut self, value: u8) {
        let offset = Self::CCIDX_SHIFT;
        assert!(value & (Self::CCIDX_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CCIDX_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `CCIDX` field set to the given value.
    pub const fn with_ccidx(mut self, value: u8) -> Self {
        self.set_ccidx(value);
        self
    }

    /// Returns the value of the `NV` field.
    pub const fn nv(self) -> u8 {
        ((self.bits() >> Self::NV_SHIFT) & Self::NV_MASK) as u8
    }

    /// Sets the value of the `NV` field.
    pub const fn set_nv(&mut self, value: u8) {
        let offset = Self::NV_SHIFT;
        assert!(value & (Self::NV_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::NV_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `NV` field set to the given value.
    pub const fn with_nv(mut self, value: u8) -> Self {
        self.set_nv(value);
        self
    }

    /// Returns the value of the `ST` field.
    pub const fn st(self) -> u8 {
        ((self.bits() >> Self::ST_SHIFT) & Self::ST_MASK) as u8
    }

    /// Sets the value of the `ST` field.
    pub const fn set_st(&mut self, value: u8) {
        let offset = Self::ST_SHIFT;
        assert!(value & (Self::ST_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ST_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `ST` field set to the given value.
    pub const fn with_st(mut self, value: u8) -> Self {
        self.set_st(value);
        self
    }

    /// Returns the value of the `AT` field.
    pub const fn at(self) -> u8 {
        ((self.bits() >> Self::AT_SHIFT) & Self::AT_MASK) as u8
    }

    /// Sets the value of the `AT` field.
    pub const fn set_at(&mut self, value: u8) {
        let offset = Self::AT_SHIFT;
        assert!(value & (Self::AT_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::AT_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `AT` field set to the given value.
    pub const fn with_at(mut self, value: u8) -> Self {
        self.set_at(value);
        self
    }

    /// Returns the value of the `IDS` field.
    pub const fn ids(self) -> u8 {
        ((self.bits() >> Self::IDS_SHIFT) & Self::IDS_MASK) as u8
    }

    /// Sets the value of the `IDS` field.
    pub const fn set_ids(&mut self, value: u8) {
        let offset = Self::IDS_SHIFT;
        assert!(value & (Self::IDS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::IDS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `IDS` field set to the given value.
    pub const fn with_ids(mut self, value: u8) -> Self {
        self.set_ids(value);
        self
    }

    /// Returns the value of the `FWB` field.
    pub const fn fwb(self) -> u8 {
        ((self.bits() >> Self::FWB_SHIFT) & Self::FWB_MASK) as u8
    }

    /// Sets the value of the `FWB` field.
    pub const fn set_fwb(&mut self, value: u8) {
        let offset = Self::FWB_SHIFT;
        assert!(value & (Self::FWB_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::FWB_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `FWB` field set to the given value.
    pub const fn with_fwb(mut self, value: u8) -> Self {
        self.set_fwb(value);
        self
    }

    /// Returns the value of the `TTL` field.
    pub const fn ttl(self) -> u8 {
        ((self.bits() >> Self::TTL_SHIFT) & Self::TTL_MASK) as u8
    }

    /// Sets the value of the `TTL` field.
    pub const fn set_ttl(&mut self, value: u8) {
        let offset = Self::TTL_SHIFT;
        assert!(value & (Self::TTL_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TTL_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `TTL` field set to the given value.
    pub const fn with_ttl(mut self, value: u8) -> Self {
        self.set_ttl(value);
        self
    }

    /// Returns the value of the `BBM` field.
    pub const fn bbm(self) -> u8 {
        ((self.bits() >> Self::BBM_SHIFT) & Self::BBM_MASK) as u8
    }

    /// Sets the value of the `BBM` field.
    pub const fn set_bbm(&mut self, value: u8) {
        let offset = Self::BBM_SHIFT;
        assert!(value & (Self::BBM_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BBM_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `BBM` field set to the given value.
    pub const fn with_bbm(mut self, value: u8) -> Self {
        self.set_bbm(value);
        self
    }

    /// Returns the value of the `EVT` field.
    pub const fn evt(self) -> u8 {
        ((self.bits() >> Self::EVT_SHIFT) & Self::EVT_MASK) as u8
    }

    /// Sets the value of the `EVT` field.
    pub const fn set_evt(&mut self, value: u8) {
        let offset = Self::EVT_SHIFT;
        assert!(value & (Self::EVT_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EVT_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `EVT` field set to the given value.
    pub const fn with_evt(mut self, value: u8) -> Self {
        self.set_evt(value);
        self
    }

    /// Returns the value of the `E0PD` field.
    pub const fn e0pd(self) -> u8 {
        ((self.bits() >> Self::E0PD_SHIFT) & Self::E0PD_MASK) as u8
    }

    /// Sets the value of the `E0PD` field.
    pub const fn set_e0pd(&mut self, value: u8) {
        let offset = Self::E0PD_SHIFT;
        assert!(value & (Self::E0PD_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::E0PD_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `E0PD` field set to the given value.
    pub const fn with_e0pd(mut self, value: u8) -> Self {
        self.set_e0pd(value);
        self
    }
}

bitflags! {
    /// `ID_AA64MMFR3_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdAa64mmfr3El1: u64 {
    }
}

impl IdAa64mmfr3El1 {
    /// Offset of the `TCRX` field.
    pub const TCRX_SHIFT: u32 = 0;
    /// Mask for the `TCRX` field.
    pub const TCRX_MASK: u64 = 0b1111;
    /// Offset of the `SCTLRX` field.
    pub const SCTLRX_SHIFT: u32 = 4;
    /// Mask for the `SCTLRX` field.
    pub const SCTLRX_MASK: u64 = 0b1111;
    /// Offset of the `S1PIE` field.
    pub const S1PIE_SHIFT: u32 = 8;
    /// Mask for the `S1PIE` field.
    pub const S1PIE_MASK: u64 = 0b1111;
    /// Offset of the `S2PIE` field.
    pub const S2PIE_SHIFT: u32 = 12;
    /// Mask for the `S2PIE` field.
    pub const S2PIE_MASK: u64 = 0b1111;
    /// Offset of the `S1POE` field.
    pub const S1POE_SHIFT: u32 = 16;
    /// Mask for the `S1POE` field.
    pub const S1POE_MASK: u64 = 0b1111;
    /// Offset of the `S2POE` field.
    pub const S2POE_SHIFT: u32 = 20;
    /// Mask for the `S2POE` field.
    pub const S2POE_MASK: u64 = 0b1111;
    /// Offset of the `AIE` field.
    pub const AIE_SHIFT: u32 = 24;
    /// Mask for the `AIE` field.
    pub const AIE_MASK: u64 = 0b1111;
    /// Offset of the `MEC` field.
    pub const MEC_SHIFT: u32 = 28;
    /// Mask for the `MEC` field.
    pub const MEC_MASK: u64 = 0b1111;
    /// Offset of the `D128` field.
    pub const D128_SHIFT: u32 = 32;
    /// Mask for the `D128` field.
    pub const D128_MASK: u64 = 0b1111;
    /// Offset of the `D128_2` field.
    pub const D128_2_SHIFT: u32 = 36;
    /// Mask for the `D128_2` field.
    pub const D128_2_MASK: u64 = 0b1111;
    /// Offset of the `SNERR` field.
    pub const SNERR_SHIFT: u32 = 40;
    /// Mask for the `SNERR` field.
    pub const SNERR_MASK: u64 = 0b1111;
    /// Offset of the `ANERR` field.
    pub const ANERR_SHIFT: u32 = 44;
    /// Mask for the `ANERR` field.
    pub const ANERR_MASK: u64 = 0b1111;
    /// Offset of the `SDERR` field.
    pub const SDERR_SHIFT: u32 = 52;
    /// Mask for the `SDERR` field.
    pub const SDERR_MASK: u64 = 0b1111;
    /// Offset of the `ADERR` field.
    pub const ADERR_SHIFT: u32 = 56;
    /// Mask for the `ADERR` field.
    pub const ADERR_MASK: u64 = 0b1111;
    /// Offset of the `Spec_FPACC` field.
    pub const SPEC_FPACC_SHIFT: u32 = 60;
    /// Mask for the `Spec_FPACC` field.
    pub const SPEC_FPACC_MASK: u64 = 0b1111;

    /// Returns the value of the `TCRX` field.
    pub const fn tcrx(self) -> u8 {
        ((self.bits() >> Self::TCRX_SHIFT) & Self::TCRX_MASK) as u8
    }

    /// Sets the value of the `TCRX` field.
    pub const fn set_tcrx(&mut self, value: u8) {
        let offset = Self::TCRX_SHIFT;
        assert!(value & (Self::TCRX_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TCRX_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `TCRX` field set to the given value.
    pub const fn with_tcrx(mut self, value: u8) -> Self {
        self.set_tcrx(value);
        self
    }

    /// Returns the value of the `SCTLRX` field.
    pub const fn sctlrx(self) -> u8 {
        ((self.bits() >> Self::SCTLRX_SHIFT) & Self::SCTLRX_MASK) as u8
    }

    /// Sets the value of the `SCTLRX` field.
    pub const fn set_sctlrx(&mut self, value: u8) {
        let offset = Self::SCTLRX_SHIFT;
        assert!(value & (Self::SCTLRX_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SCTLRX_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `SCTLRX` field set to the given value.
    pub const fn with_sctlrx(mut self, value: u8) -> Self {
        self.set_sctlrx(value);
        self
    }

    /// Returns the value of the `S1PIE` field.
    pub const fn s1pie(self) -> u8 {
        ((self.bits() >> Self::S1PIE_SHIFT) & Self::S1PIE_MASK) as u8
    }

    /// Sets the value of the `S1PIE` field.
    pub const fn set_s1pie(&mut self, value: u8) {
        let offset = Self::S1PIE_SHIFT;
        assert!(value & (Self::S1PIE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::S1PIE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `S1PIE` field set to the given value.
    pub const fn with_s1pie(mut self, value: u8) -> Self {
        self.set_s1pie(value);
        self
    }

    /// Returns the value of the `S2PIE` field.
    pub const fn s2pie(self) -> u8 {
        ((self.bits() >> Self::S2PIE_SHIFT) & Self::S2PIE_MASK) as u8
    }

    /// Sets the value of the `S2PIE` field.
    pub const fn set_s2pie(&mut self, value: u8) {
        let offset = Self::S2PIE_SHIFT;
        assert!(value & (Self::S2PIE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::S2PIE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `S2PIE` field set to the given value.
    pub const fn with_s2pie(mut self, value: u8) -> Self {
        self.set_s2pie(value);
        self
    }

    /// Returns the value of the `S1POE` field.
    pub const fn s1poe(self) -> u8 {
        ((self.bits() >> Self::S1POE_SHIFT) & Self::S1POE_MASK) as u8
    }

    /// Sets the value of the `S1POE` field.
    pub const fn set_s1poe(&mut self, value: u8) {
        let offset = Self::S1POE_SHIFT;
        assert!(value & (Self::S1POE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::S1POE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `S1POE` field set to the given value.
    pub const fn with_s1poe(mut self, value: u8) -> Self {
        self.set_s1poe(value);
        self
    }

    /// Returns the value of the `S2POE` field.
    pub const fn s2poe(self) -> u8 {
        ((self.bits() >> Self::S2POE_SHIFT) & Self::S2POE_MASK) as u8
    }

    /// Sets the value of the `S2POE` field.
    pub const fn set_s2poe(&mut self, value: u8) {
        let offset = Self::S2POE_SHIFT;
        assert!(value & (Self::S2POE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::S2POE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `S2POE` field set to the given value.
    pub const fn with_s2poe(mut self, value: u8) -> Self {
        self.set_s2poe(value);
        self
    }

    /// Returns the value of the `AIE` field.
    pub const fn aie(self) -> u8 {
        ((self.bits() >> Self::AIE_SHIFT) & Self::AIE_MASK) as u8
    }

    /// Sets the value of the `AIE` field.
    pub const fn set_aie(&mut self, value: u8) {
        let offset = Self::AIE_SHIFT;
        assert!(value & (Self::AIE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::AIE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `AIE` field set to the given value.
    pub const fn with_aie(mut self, value: u8) -> Self {
        self.set_aie(value);
        self
    }

    /// Returns the value of the `MEC` field.
    pub const fn mec(self) -> u8 {
        ((self.bits() >> Self::MEC_SHIFT) & Self::MEC_MASK) as u8
    }

    /// Sets the value of the `MEC` field.
    pub const fn set_mec(&mut self, value: u8) {
        let offset = Self::MEC_SHIFT;
        assert!(value & (Self::MEC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::MEC_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `MEC` field set to the given value.
    pub const fn with_mec(mut self, value: u8) -> Self {
        self.set_mec(value);
        self
    }

    /// Returns the value of the `D128` field.
    pub const fn d128(self) -> u8 {
        ((self.bits() >> Self::D128_SHIFT) & Self::D128_MASK) as u8
    }

    /// Sets the value of the `D128` field.
    pub const fn set_d128(&mut self, value: u8) {
        let offset = Self::D128_SHIFT;
        assert!(value & (Self::D128_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::D128_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `D128` field set to the given value.
    pub const fn with_d128(mut self, value: u8) -> Self {
        self.set_d128(value);
        self
    }

    /// Returns the value of the `D128_2` field.
    pub const fn d128_2(self) -> u8 {
        ((self.bits() >> Self::D128_2_SHIFT) & Self::D128_2_MASK) as u8
    }

    /// Sets the value of the `D128_2` field.
    pub const fn set_d128_2(&mut self, value: u8) {
        let offset = Self::D128_2_SHIFT;
        assert!(value & (Self::D128_2_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::D128_2_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `D128_2` field set to the given value.
    pub const fn with_d128_2(mut self, value: u8) -> Self {
        self.set_d128_2(value);
        self
    }

    /// Returns the value of the `SNERR` field.
    pub const fn snerr(self) -> u8 {
        ((self.bits() >> Self::SNERR_SHIFT) & Self::SNERR_MASK) as u8
    }

    /// Sets the value of the `SNERR` field.
    pub const fn set_snerr(&mut self, value: u8) {
        let offset = Self::SNERR_SHIFT;
        assert!(value & (Self::SNERR_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SNERR_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `SNERR` field set to the given value.
    pub const fn with_snerr(mut self, value: u8) -> Self {
        self.set_snerr(value);
        self
    }

    /// Returns the value of the `ANERR` field.
    pub const fn anerr(self) -> u8 {
        ((self.bits() >> Self::ANERR_SHIFT) & Self::ANERR_MASK) as u8
    }

    /// Sets the value of the `ANERR` field.
    pub const fn set_anerr(&mut self, value: u8) {
        let offset = Self::ANERR_SHIFT;
        assert!(value & (Self::ANERR_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ANERR_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `ANERR` field set to the given value.
    pub const fn with_anerr(mut self, value: u8) -> Self {
        self.set_anerr(value);
        self
    }

    /// Returns the value of the `SDERR` field.
    pub const fn sderr(self) -> u8 {
        ((self.bits() >> Self::SDERR_SHIFT) & Self::SDERR_MASK) as u8
    }

    /// Sets the value of the `SDERR` field.
    pub const fn set_sderr(&mut self, value: u8) {
        let offset = Self::SDERR_SHIFT;
        assert!(value & (Self::SDERR_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SDERR_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `SDERR` field set to the given value.
    pub const fn with_sderr(mut self, value: u8) -> Self {
        self.set_sderr(value);
        self
    }

    /// Returns the value of the `ADERR` field.
    pub const fn aderr(self) -> u8 {
        ((self.bits() >> Self::ADERR_SHIFT) & Self::ADERR_MASK) as u8
    }

    /// Sets the value of the `ADERR` field.
    pub const fn set_aderr(&mut self, value: u8) {
        let offset = Self::ADERR_SHIFT;
        assert!(value & (Self::ADERR_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ADERR_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `ADERR` field set to the given value.
    pub const fn with_aderr(mut self, value: u8) -> Self {
        self.set_aderr(value);
        self
    }

    /// Returns the value of the `Spec_FPACC` field.
    pub const fn spec_fpacc(self) -> u8 {
        ((self.bits() >> Self::SPEC_FPACC_SHIFT) & Self::SPEC_FPACC_MASK) as u8
    }

    /// Sets the value of the `Spec_FPACC` field.
    pub const fn set_spec_fpacc(&mut self, value: u8) {
        let offset = Self::SPEC_FPACC_SHIFT;
        assert!(value & (Self::SPEC_FPACC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SPEC_FPACC_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `Spec_FPACC` field set to the given value.
    pub const fn with_spec_fpacc(mut self, value: u8) -> Self {
        self.set_spec_fpacc(value);
        self
    }
}

bitflags! {
    /// `ID_AA64MMFR4_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdAa64mmfr4El1: u64 {
    }
}

impl IdAa64mmfr4El1 {
    /// Offset of the `PoPS` field.
    pub const POPS_SHIFT: u32 = 0;
    /// Mask for the `PoPS` field.
    pub const POPS_MASK: u64 = 0b1111;
    /// Offset of the `EIESB` field.
    pub const EIESB_SHIFT: u32 = 4;
    /// Mask for the `EIESB` field.
    pub const EIESB_MASK: u64 = 0b1111;
    /// Offset of the `ASID2` field.
    pub const ASID2_SHIFT: u32 = 8;
    /// Mask for the `ASID2` field.
    pub const ASID2_MASK: u64 = 0b1111;
    /// Offset of the `HACDBS` field.
    pub const HACDBS_SHIFT: u32 = 12;
    /// Mask for the `HACDBS` field.
    pub const HACDBS_MASK: u64 = 0b1111;
    /// Offset of the `FGWTE3` field.
    pub const FGWTE3_SHIFT: u32 = 16;
    /// Mask for the `FGWTE3` field.
    pub const FGWTE3_MASK: u64 = 0b1111;
    /// Offset of the `NV_frac` field.
    pub const NV_FRAC_SHIFT: u32 = 20;
    /// Mask for the `NV_frac` field.
    pub const NV_FRAC_MASK: u64 = 0b1111;
    /// Offset of the `E2H0` field.
    pub const E2H0_SHIFT: u32 = 24;
    /// Mask for the `E2H0` field.
    pub const E2H0_MASK: u64 = 0b1111;
    /// Offset of the `RMEGDI` field.
    pub const RMEGDI_SHIFT: u32 = 28;
    /// Mask for the `RMEGDI` field.
    pub const RMEGDI_MASK: u64 = 0b1111;
    /// Offset of the `EAESR` field.
    pub const EAESR_SHIFT: u32 = 32;
    /// Mask for the `EAESR` field.
    pub const EAESR_MASK: u64 = 0b1111;
    /// Offset of the `E3DSE` field.
    pub const E3DSE_SHIFT: u32 = 36;
    /// Mask for the `E3DSE` field.
    pub const E3DSE_MASK: u64 = 0b1111;
    /// Offset of the `TLBID` field.
    pub const TLBID_SHIFT: u32 = 40;
    /// Mask for the `TLBID` field.
    pub const TLBID_MASK: u64 = 0b1111;
    /// Offset of the `SRMASK` field.
    pub const SRMASK_SHIFT: u32 = 44;
    /// Mask for the `SRMASK` field.
    pub const SRMASK_MASK: u64 = 0b1111;
    /// Offset of the `TPS` field.
    pub const TPS_SHIFT: u32 = 48;
    /// Mask for the `TPS` field.
    pub const TPS_MASK: u64 = 0b1111;
    /// Offset of the `TEV` field.
    pub const TEV_SHIFT: u32 = 52;
    /// Mask for the `TEV` field.
    pub const TEV_MASK: u64 = 0b1111;
    /// Offset of the `SCRX` field.
    pub const SCRX_SHIFT: u32 = 56;
    /// Mask for the `SCRX` field.
    pub const SCRX_MASK: u64 = 0b1111;
    /// Offset of the `MTEFGT` field.
    pub const MTEFGT_SHIFT: u32 = 60;
    /// Mask for the `MTEFGT` field.
    pub const MTEFGT_MASK: u64 = 0b1111;

    /// Returns the value of the `PoPS` field.
    pub const fn pops(self) -> u8 {
        ((self.bits() >> Self::POPS_SHIFT) & Self::POPS_MASK) as u8
    }

    /// Sets the value of the `PoPS` field.
    pub const fn set_pops(&mut self, value: u8) {
        let offset = Self::POPS_SHIFT;
        assert!(value & (Self::POPS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::POPS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PoPS` field set to the given value.
    pub const fn with_pops(mut self, value: u8) -> Self {
        self.set_pops(value);
        self
    }

    /// Returns the value of the `EIESB` field.
    pub const fn eiesb(self) -> u8 {
        ((self.bits() >> Self::EIESB_SHIFT) & Self::EIESB_MASK) as u8
    }

    /// Sets the value of the `EIESB` field.
    pub const fn set_eiesb(&mut self, value: u8) {
        let offset = Self::EIESB_SHIFT;
        assert!(value & (Self::EIESB_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EIESB_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `EIESB` field set to the given value.
    pub const fn with_eiesb(mut self, value: u8) -> Self {
        self.set_eiesb(value);
        self
    }

    /// Returns the value of the `ASID2` field.
    pub const fn asid2(self) -> u8 {
        ((self.bits() >> Self::ASID2_SHIFT) & Self::ASID2_MASK) as u8
    }

    /// Sets the value of the `ASID2` field.
    pub const fn set_asid2(&mut self, value: u8) {
        let offset = Self::ASID2_SHIFT;
        assert!(value & (Self::ASID2_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ASID2_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `ASID2` field set to the given value.
    pub const fn with_asid2(mut self, value: u8) -> Self {
        self.set_asid2(value);
        self
    }

    /// Returns the value of the `HACDBS` field.
    pub const fn hacdbs(self) -> u8 {
        ((self.bits() >> Self::HACDBS_SHIFT) & Self::HACDBS_MASK) as u8
    }

    /// Sets the value of the `HACDBS` field.
    pub const fn set_hacdbs(&mut self, value: u8) {
        let offset = Self::HACDBS_SHIFT;
        assert!(value & (Self::HACDBS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::HACDBS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `HACDBS` field set to the given value.
    pub const fn with_hacdbs(mut self, value: u8) -> Self {
        self.set_hacdbs(value);
        self
    }

    /// Returns the value of the `FGWTE3` field.
    pub const fn fgwte3(self) -> u8 {
        ((self.bits() >> Self::FGWTE3_SHIFT) & Self::FGWTE3_MASK) as u8
    }

    /// Sets the value of the `FGWTE3` field.
    pub const fn set_fgwte3(&mut self, value: u8) {
        let offset = Self::FGWTE3_SHIFT;
        assert!(value & (Self::FGWTE3_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::FGWTE3_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `FGWTE3` field set to the given value.
    pub const fn with_fgwte3(mut self, value: u8) -> Self {
        self.set_fgwte3(value);
        self
    }

    /// Returns the value of the `NV_frac` field.
    pub const fn nv_frac(self) -> u8 {
        ((self.bits() >> Self::NV_FRAC_SHIFT) & Self::NV_FRAC_MASK) as u8
    }

    /// Sets the value of the `NV_frac` field.
    pub const fn set_nv_frac(&mut self, value: u8) {
        let offset = Self::NV_FRAC_SHIFT;
        assert!(value & (Self::NV_FRAC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::NV_FRAC_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `NV_frac` field set to the given value.
    pub const fn with_nv_frac(mut self, value: u8) -> Self {
        self.set_nv_frac(value);
        self
    }

    /// Returns the value of the `E2H0` field.
    pub const fn e2h0(self) -> u8 {
        ((self.bits() >> Self::E2H0_SHIFT) & Self::E2H0_MASK) as u8
    }

    /// Sets the value of the `E2H0` field.
    pub const fn set_e2h0(&mut self, value: u8) {
        let offset = Self::E2H0_SHIFT;
        assert!(value & (Self::E2H0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::E2H0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `E2H0` field set to the given value.
    pub const fn with_e2h0(mut self, value: u8) -> Self {
        self.set_e2h0(value);
        self
    }

    /// Returns the value of the `RMEGDI` field.
    pub const fn rmegdi(self) -> u8 {
        ((self.bits() >> Self::RMEGDI_SHIFT) & Self::RMEGDI_MASK) as u8
    }

    /// Sets the value of the `RMEGDI` field.
    pub const fn set_rmegdi(&mut self, value: u8) {
        let offset = Self::RMEGDI_SHIFT;
        assert!(value & (Self::RMEGDI_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::RMEGDI_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `RMEGDI` field set to the given value.
    pub const fn with_rmegdi(mut self, value: u8) -> Self {
        self.set_rmegdi(value);
        self
    }

    /// Returns the value of the `EAESR` field.
    pub const fn eaesr(self) -> u8 {
        ((self.bits() >> Self::EAESR_SHIFT) & Self::EAESR_MASK) as u8
    }

    /// Sets the value of the `EAESR` field.
    pub const fn set_eaesr(&mut self, value: u8) {
        let offset = Self::EAESR_SHIFT;
        assert!(value & (Self::EAESR_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EAESR_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `EAESR` field set to the given value.
    pub const fn with_eaesr(mut self, value: u8) -> Self {
        self.set_eaesr(value);
        self
    }

    /// Returns the value of the `E3DSE` field.
    pub const fn e3dse(self) -> u8 {
        ((self.bits() >> Self::E3DSE_SHIFT) & Self::E3DSE_MASK) as u8
    }

    /// Sets the value of the `E3DSE` field.
    pub const fn set_e3dse(&mut self, value: u8) {
        let offset = Self::E3DSE_SHIFT;
        assert!(value & (Self::E3DSE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::E3DSE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `E3DSE` field set to the given value.
    pub const fn with_e3dse(mut self, value: u8) -> Self {
        self.set_e3dse(value);
        self
    }

    /// Returns the value of the `TLBID` field.
    pub const fn tlbid(self) -> u8 {
        ((self.bits() >> Self::TLBID_SHIFT) & Self::TLBID_MASK) as u8
    }

    /// Sets the value of the `TLBID` field.
    pub const fn set_tlbid(&mut self, value: u8) {
        let offset = Self::TLBID_SHIFT;
        assert!(value & (Self::TLBID_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TLBID_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `TLBID` field set to the given value.
    pub const fn with_tlbid(mut self, value: u8) -> Self {
        self.set_tlbid(value);
        self
    }

    /// Returns the value of the `SRMASK` field.
    pub const fn srmask(self) -> u8 {
        ((self.bits() >> Self::SRMASK_SHIFT) & Self::SRMASK_MASK) as u8
    }

    /// Sets the value of the `SRMASK` field.
    pub const fn set_srmask(&mut self, value: u8) {
        let offset = Self::SRMASK_SHIFT;
        assert!(value & (Self::SRMASK_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SRMASK_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `SRMASK` field set to the given value.
    pub const fn with_srmask(mut self, value: u8) -> Self {
        self.set_srmask(value);
        self
    }

    /// Returns the value of the `TPS` field.
    pub const fn tps(self) -> u8 {
        ((self.bits() >> Self::TPS_SHIFT) & Self::TPS_MASK) as u8
    }

    /// Sets the value of the `TPS` field.
    pub const fn set_tps(&mut self, value: u8) {
        let offset = Self::TPS_SHIFT;
        assert!(value & (Self::TPS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TPS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `TPS` field set to the given value.
    pub const fn with_tps(mut self, value: u8) -> Self {
        self.set_tps(value);
        self
    }

    /// Returns the value of the `TEV` field.
    pub const fn tev(self) -> u8 {
        ((self.bits() >> Self::TEV_SHIFT) & Self::TEV_MASK) as u8
    }

    /// Sets the value of the `TEV` field.
    pub const fn set_tev(&mut self, value: u8) {
        let offset = Self::TEV_SHIFT;
        assert!(value & (Self::TEV_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TEV_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `TEV` field set to the given value.
    pub const fn with_tev(mut self, value: u8) -> Self {
        self.set_tev(value);
        self
    }

    /// Returns the value of the `SCRX` field.
    pub const fn scrx(self) -> u8 {
        ((self.bits() >> Self::SCRX_SHIFT) & Self::SCRX_MASK) as u8
    }

    /// Sets the value of the `SCRX` field.
    pub const fn set_scrx(&mut self, value: u8) {
        let offset = Self::SCRX_SHIFT;
        assert!(value & (Self::SCRX_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SCRX_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `SCRX` field set to the given value.
    pub const fn with_scrx(mut self, value: u8) -> Self {
        self.set_scrx(value);
        self
    }

    /// Returns the value of the `MTEFGT` field.
    pub const fn mtefgt(self) -> u8 {
        ((self.bits() >> Self::MTEFGT_SHIFT) & Self::MTEFGT_MASK) as u8
    }

    /// Sets the value of the `MTEFGT` field.
    pub const fn set_mtefgt(&mut self, value: u8) {
        let offset = Self::MTEFGT_SHIFT;
        assert!(value & (Self::MTEFGT_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::MTEFGT_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `MTEFGT` field set to the given value.
    pub const fn with_mtefgt(mut self, value: u8) -> Self {
        self.set_mtefgt(value);
        self
    }
}

bitflags! {
    /// `ID_AA64PFR0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdAa64pfr0El1: u64 {
    }
}

impl IdAa64pfr0El1 {
    /// Offset of the `EL0` field.
    pub const EL0_SHIFT: u32 = 0;
    /// Mask for the `EL0` field.
    pub const EL0_MASK: u64 = 0b1111;
    /// Offset of the `EL1` field.
    pub const EL1_SHIFT: u32 = 4;
    /// Mask for the `EL1` field.
    pub const EL1_MASK: u64 = 0b1111;
    /// Offset of the `EL2` field.
    pub const EL2_SHIFT: u32 = 8;
    /// Mask for the `EL2` field.
    pub const EL2_MASK: u64 = 0b1111;
    /// Offset of the `EL3` field.
    pub const EL3_SHIFT: u32 = 12;
    /// Mask for the `EL3` field.
    pub const EL3_MASK: u64 = 0b1111;
    /// Offset of the `FP` field.
    pub const FP_SHIFT: u32 = 16;
    /// Mask for the `FP` field.
    pub const FP_MASK: u64 = 0b1111;
    /// Offset of the `AdvSIMD` field.
    pub const ADVSIMD_SHIFT: u32 = 20;
    /// Mask for the `AdvSIMD` field.
    pub const ADVSIMD_MASK: u64 = 0b1111;
    /// Offset of the `GIC` field.
    pub const GIC_SHIFT: u32 = 24;
    /// Mask for the `GIC` field.
    pub const GIC_MASK: u64 = 0b1111;
    /// Offset of the `RAS` field.
    pub const RAS_SHIFT: u32 = 28;
    /// Mask for the `RAS` field.
    pub const RAS_MASK: u64 = 0b1111;
    /// Offset of the `SVE` field.
    pub const SVE_SHIFT: u32 = 32;
    /// Mask for the `SVE` field.
    pub const SVE_MASK: u64 = 0b1111;
    /// Offset of the `SEL2` field.
    pub const SEL2_SHIFT: u32 = 36;
    /// Mask for the `SEL2` field.
    pub const SEL2_MASK: u64 = 0b1111;
    /// Offset of the `MPAM` field.
    pub const MPAM_SHIFT: u32 = 40;
    /// Mask for the `MPAM` field.
    pub const MPAM_MASK: u64 = 0b1111;
    /// Offset of the `AMU` field.
    pub const AMU_SHIFT: u32 = 44;
    /// Mask for the `AMU` field.
    pub const AMU_MASK: u64 = 0b1111;
    /// Offset of the `DIT` field.
    pub const DIT_SHIFT: u32 = 48;
    /// Mask for the `DIT` field.
    pub const DIT_MASK: u64 = 0b1111;
    /// Offset of the `RME` field.
    pub const RME_SHIFT: u32 = 52;
    /// Mask for the `RME` field.
    pub const RME_MASK: u64 = 0b1111;
    /// Offset of the `CSV2` field.
    pub const CSV2_SHIFT: u32 = 56;
    /// Mask for the `CSV2` field.
    pub const CSV2_MASK: u64 = 0b1111;
    /// Offset of the `CSV3` field.
    pub const CSV3_SHIFT: u32 = 60;
    /// Mask for the `CSV3` field.
    pub const CSV3_MASK: u64 = 0b1111;

    /// Returns the value of the `EL0` field.
    pub const fn el0(self) -> u8 {
        ((self.bits() >> Self::EL0_SHIFT) & Self::EL0_MASK) as u8
    }

    /// Sets the value of the `EL0` field.
    pub const fn set_el0(&mut self, value: u8) {
        let offset = Self::EL0_SHIFT;
        assert!(value & (Self::EL0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EL0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `EL0` field set to the given value.
    pub const fn with_el0(mut self, value: u8) -> Self {
        self.set_el0(value);
        self
    }

    /// Returns the value of the `EL1` field.
    pub const fn el1(self) -> u8 {
        ((self.bits() >> Self::EL1_SHIFT) & Self::EL1_MASK) as u8
    }

    /// Sets the value of the `EL1` field.
    pub const fn set_el1(&mut self, value: u8) {
        let offset = Self::EL1_SHIFT;
        assert!(value & (Self::EL1_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EL1_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `EL1` field set to the given value.
    pub const fn with_el1(mut self, value: u8) -> Self {
        self.set_el1(value);
        self
    }

    /// Returns the value of the `EL2` field.
    pub const fn el2(self) -> u8 {
        ((self.bits() >> Self::EL2_SHIFT) & Self::EL2_MASK) as u8
    }

    /// Sets the value of the `EL2` field.
    pub const fn set_el2(&mut self, value: u8) {
        let offset = Self::EL2_SHIFT;
        assert!(value & (Self::EL2_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EL2_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `EL2` field set to the given value.
    pub const fn with_el2(mut self, value: u8) -> Self {
        self.set_el2(value);
        self
    }

    /// Returns the value of the `EL3` field.
    pub const fn el3(self) -> u8 {
        ((self.bits() >> Self::EL3_SHIFT) & Self::EL3_MASK) as u8
    }

    /// Sets the value of the `EL3` field.
    pub const fn set_el3(&mut self, value: u8) {
        let offset = Self::EL3_SHIFT;
        assert!(value & (Self::EL3_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EL3_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `EL3` field set to the given value.
    pub const fn with_el3(mut self, value: u8) -> Self {
        self.set_el3(value);
        self
    }

    /// Returns the value of the `FP` field.
    pub const fn fp(self) -> u8 {
        ((self.bits() >> Self::FP_SHIFT) & Self::FP_MASK) as u8
    }

    /// Sets the value of the `FP` field.
    pub const fn set_fp(&mut self, value: u8) {
        let offset = Self::FP_SHIFT;
        assert!(value & (Self::FP_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::FP_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `FP` field set to the given value.
    pub const fn with_fp(mut self, value: u8) -> Self {
        self.set_fp(value);
        self
    }

    /// Returns the value of the `AdvSIMD` field.
    pub const fn advsimd(self) -> u8 {
        ((self.bits() >> Self::ADVSIMD_SHIFT) & Self::ADVSIMD_MASK) as u8
    }

    /// Sets the value of the `AdvSIMD` field.
    pub const fn set_advsimd(&mut self, value: u8) {
        let offset = Self::ADVSIMD_SHIFT;
        assert!(value & (Self::ADVSIMD_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ADVSIMD_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `AdvSIMD` field set to the given value.
    pub const fn with_advsimd(mut self, value: u8) -> Self {
        self.set_advsimd(value);
        self
    }

    /// Returns the value of the `GIC` field.
    pub const fn gic(self) -> u8 {
        ((self.bits() >> Self::GIC_SHIFT) & Self::GIC_MASK) as u8
    }

    /// Sets the value of the `GIC` field.
    pub const fn set_gic(&mut self, value: u8) {
        let offset = Self::GIC_SHIFT;
        assert!(value & (Self::GIC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::GIC_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `GIC` field set to the given value.
    pub const fn with_gic(mut self, value: u8) -> Self {
        self.set_gic(value);
        self
    }

    /// Returns the value of the `RAS` field.
    pub const fn ras(self) -> u8 {
        ((self.bits() >> Self::RAS_SHIFT) & Self::RAS_MASK) as u8
    }

    /// Sets the value of the `RAS` field.
    pub const fn set_ras(&mut self, value: u8) {
        let offset = Self::RAS_SHIFT;
        assert!(value & (Self::RAS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::RAS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `RAS` field set to the given value.
    pub const fn with_ras(mut self, value: u8) -> Self {
        self.set_ras(value);
        self
    }

    /// Returns the value of the `SVE` field.
    pub const fn sve(self) -> u8 {
        ((self.bits() >> Self::SVE_SHIFT) & Self::SVE_MASK) as u8
    }

    /// Sets the value of the `SVE` field.
    pub const fn set_sve(&mut self, value: u8) {
        let offset = Self::SVE_SHIFT;
        assert!(value & (Self::SVE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SVE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `SVE` field set to the given value.
    pub const fn with_sve(mut self, value: u8) -> Self {
        self.set_sve(value);
        self
    }

    /// Returns the value of the `SEL2` field.
    pub const fn sel2(self) -> u8 {
        ((self.bits() >> Self::SEL2_SHIFT) & Self::SEL2_MASK) as u8
    }

    /// Sets the value of the `SEL2` field.
    pub const fn set_sel2(&mut self, value: u8) {
        let offset = Self::SEL2_SHIFT;
        assert!(value & (Self::SEL2_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SEL2_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `SEL2` field set to the given value.
    pub const fn with_sel2(mut self, value: u8) -> Self {
        self.set_sel2(value);
        self
    }

    /// Returns the value of the `MPAM` field.
    pub const fn mpam(self) -> u8 {
        ((self.bits() >> Self::MPAM_SHIFT) & Self::MPAM_MASK) as u8
    }

    /// Sets the value of the `MPAM` field.
    pub const fn set_mpam(&mut self, value: u8) {
        let offset = Self::MPAM_SHIFT;
        assert!(value & (Self::MPAM_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::MPAM_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `MPAM` field set to the given value.
    pub const fn with_mpam(mut self, value: u8) -> Self {
        self.set_mpam(value);
        self
    }

    /// Returns the value of the `AMU` field.
    pub const fn amu(self) -> u8 {
        ((self.bits() >> Self::AMU_SHIFT) & Self::AMU_MASK) as u8
    }

    /// Sets the value of the `AMU` field.
    pub const fn set_amu(&mut self, value: u8) {
        let offset = Self::AMU_SHIFT;
        assert!(value & (Self::AMU_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::AMU_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `AMU` field set to the given value.
    pub const fn with_amu(mut self, value: u8) -> Self {
        self.set_amu(value);
        self
    }

    /// Returns the value of the `DIT` field.
    pub const fn dit(self) -> u8 {
        ((self.bits() >> Self::DIT_SHIFT) & Self::DIT_MASK) as u8
    }

    /// Sets the value of the `DIT` field.
    pub const fn set_dit(&mut self, value: u8) {
        let offset = Self::DIT_SHIFT;
        assert!(value & (Self::DIT_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::DIT_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `DIT` field set to the given value.
    pub const fn with_dit(mut self, value: u8) -> Self {
        self.set_dit(value);
        self
    }

    /// Returns the value of the `RME` field.
    pub const fn rme(self) -> u8 {
        ((self.bits() >> Self::RME_SHIFT) & Self::RME_MASK) as u8
    }

    /// Sets the value of the `RME` field.
    pub const fn set_rme(&mut self, value: u8) {
        let offset = Self::RME_SHIFT;
        assert!(value & (Self::RME_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::RME_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `RME` field set to the given value.
    pub const fn with_rme(mut self, value: u8) -> Self {
        self.set_rme(value);
        self
    }

    /// Returns the value of the `CSV2` field.
    pub const fn csv2(self) -> u8 {
        ((self.bits() >> Self::CSV2_SHIFT) & Self::CSV2_MASK) as u8
    }

    /// Sets the value of the `CSV2` field.
    pub const fn set_csv2(&mut self, value: u8) {
        let offset = Self::CSV2_SHIFT;
        assert!(value & (Self::CSV2_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CSV2_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `CSV2` field set to the given value.
    pub const fn with_csv2(mut self, value: u8) -> Self {
        self.set_csv2(value);
        self
    }

    /// Returns the value of the `CSV3` field.
    pub const fn csv3(self) -> u8 {
        ((self.bits() >> Self::CSV3_SHIFT) & Self::CSV3_MASK) as u8
    }

    /// Sets the value of the `CSV3` field.
    pub const fn set_csv3(&mut self, value: u8) {
        let offset = Self::CSV3_SHIFT;
        assert!(value & (Self::CSV3_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CSV3_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `CSV3` field set to the given value.
    pub const fn with_csv3(mut self, value: u8) -> Self {
        self.set_csv3(value);
        self
    }
}

bitflags! {
    /// `ID_AA64PFR1_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdAa64pfr1El1: u64 {
    }
}

impl IdAa64pfr1El1 {
    /// Offset of the `BT` field.
    pub const BT_SHIFT: u32 = 0;
    /// Mask for the `BT` field.
    pub const BT_MASK: u64 = 0b1111;
    /// Offset of the `SSBS` field.
    pub const SSBS_SHIFT: u32 = 4;
    /// Mask for the `SSBS` field.
    pub const SSBS_MASK: u64 = 0b1111;
    /// Offset of the `MTE` field.
    pub const MTE_SHIFT: u32 = 8;
    /// Mask for the `MTE` field.
    pub const MTE_MASK: u64 = 0b1111;
    /// Offset of the `RAS_frac` field.
    pub const RAS_FRAC_SHIFT: u32 = 12;
    /// Mask for the `RAS_frac` field.
    pub const RAS_FRAC_MASK: u64 = 0b1111;
    /// Offset of the `MPAM_frac` field.
    pub const MPAM_FRAC_SHIFT: u32 = 16;
    /// Mask for the `MPAM_frac` field.
    pub const MPAM_FRAC_MASK: u64 = 0b1111;
    /// Offset of the `SME` field.
    pub const SME_SHIFT: u32 = 24;
    /// Mask for the `SME` field.
    pub const SME_MASK: u64 = 0b1111;
    /// Offset of the `RNDR_trap` field.
    pub const RNDR_TRAP_SHIFT: u32 = 28;
    /// Mask for the `RNDR_trap` field.
    pub const RNDR_TRAP_MASK: u64 = 0b1111;
    /// Offset of the `CSV2_frac` field.
    pub const CSV2_FRAC_SHIFT: u32 = 32;
    /// Mask for the `CSV2_frac` field.
    pub const CSV2_FRAC_MASK: u64 = 0b1111;
    /// Offset of the `NMI` field.
    pub const NMI_SHIFT: u32 = 36;
    /// Mask for the `NMI` field.
    pub const NMI_MASK: u64 = 0b1111;
    /// Offset of the `MTE_frac` field.
    pub const MTE_FRAC_SHIFT: u32 = 40;
    /// Mask for the `MTE_frac` field.
    pub const MTE_FRAC_MASK: u64 = 0b1111;
    /// Offset of the `GCS` field.
    pub const GCS_SHIFT: u32 = 44;
    /// Mask for the `GCS` field.
    pub const GCS_MASK: u64 = 0b1111;
    /// Offset of the `THE` field.
    pub const THE_SHIFT: u32 = 48;
    /// Mask for the `THE` field.
    pub const THE_MASK: u64 = 0b1111;
    /// Offset of the `MTEX` field.
    pub const MTEX_SHIFT: u32 = 52;
    /// Mask for the `MTEX` field.
    pub const MTEX_MASK: u64 = 0b1111;
    /// Offset of the `DF2` field.
    pub const DF2_SHIFT: u32 = 56;
    /// Mask for the `DF2` field.
    pub const DF2_MASK: u64 = 0b1111;
    /// Offset of the `PFAR` field.
    pub const PFAR_SHIFT: u32 = 60;
    /// Mask for the `PFAR` field.
    pub const PFAR_MASK: u64 = 0b1111;

    /// Returns the value of the `BT` field.
    pub const fn bt(self) -> u8 {
        ((self.bits() >> Self::BT_SHIFT) & Self::BT_MASK) as u8
    }

    /// Sets the value of the `BT` field.
    pub const fn set_bt(&mut self, value: u8) {
        let offset = Self::BT_SHIFT;
        assert!(value & (Self::BT_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BT_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `BT` field set to the given value.
    pub const fn with_bt(mut self, value: u8) -> Self {
        self.set_bt(value);
        self
    }

    /// Returns the value of the `SSBS` field.
    pub const fn ssbs(self) -> u8 {
        ((self.bits() >> Self::SSBS_SHIFT) & Self::SSBS_MASK) as u8
    }

    /// Sets the value of the `SSBS` field.
    pub const fn set_ssbs(&mut self, value: u8) {
        let offset = Self::SSBS_SHIFT;
        assert!(value & (Self::SSBS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SSBS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `SSBS` field set to the given value.
    pub const fn with_ssbs(mut self, value: u8) -> Self {
        self.set_ssbs(value);
        self
    }

    /// Returns the value of the `MTE` field.
    pub const fn mte(self) -> u8 {
        ((self.bits() >> Self::MTE_SHIFT) & Self::MTE_MASK) as u8
    }

    /// Sets the value of the `MTE` field.
    pub const fn set_mte(&mut self, value: u8) {
        let offset = Self::MTE_SHIFT;
        assert!(value & (Self::MTE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::MTE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `MTE` field set to the given value.
    pub const fn with_mte(mut self, value: u8) -> Self {
        self.set_mte(value);
        self
    }

    /// Returns the value of the `RAS_frac` field.
    pub const fn ras_frac(self) -> u8 {
        ((self.bits() >> Self::RAS_FRAC_SHIFT) & Self::RAS_FRAC_MASK) as u8
    }

    /// Sets the value of the `RAS_frac` field.
    pub const fn set_ras_frac(&mut self, value: u8) {
        let offset = Self::RAS_FRAC_SHIFT;
        assert!(value & (Self::RAS_FRAC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::RAS_FRAC_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `RAS_frac` field set to the given value.
    pub const fn with_ras_frac(mut self, value: u8) -> Self {
        self.set_ras_frac(value);
        self
    }

    /// Returns the value of the `MPAM_frac` field.
    pub const fn mpam_frac(self) -> u8 {
        ((self.bits() >> Self::MPAM_FRAC_SHIFT) & Self::MPAM_FRAC_MASK) as u8
    }

    /// Sets the value of the `MPAM_frac` field.
    pub const fn set_mpam_frac(&mut self, value: u8) {
        let offset = Self::MPAM_FRAC_SHIFT;
        assert!(value & (Self::MPAM_FRAC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::MPAM_FRAC_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `MPAM_frac` field set to the given value.
    pub const fn with_mpam_frac(mut self, value: u8) -> Self {
        self.set_mpam_frac(value);
        self
    }

    /// Returns the value of the `SME` field.
    pub const fn sme(self) -> u8 {
        ((self.bits() >> Self::SME_SHIFT) & Self::SME_MASK) as u8
    }

    /// Sets the value of the `SME` field.
    pub const fn set_sme(&mut self, value: u8) {
        let offset = Self::SME_SHIFT;
        assert!(value & (Self::SME_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SME_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `SME` field set to the given value.
    pub const fn with_sme(mut self, value: u8) -> Self {
        self.set_sme(value);
        self
    }

    /// Returns the value of the `RNDR_trap` field.
    pub const fn rndr_trap(self) -> u8 {
        ((self.bits() >> Self::RNDR_TRAP_SHIFT) & Self::RNDR_TRAP_MASK) as u8
    }

    /// Sets the value of the `RNDR_trap` field.
    pub const fn set_rndr_trap(&mut self, value: u8) {
        let offset = Self::RNDR_TRAP_SHIFT;
        assert!(value & (Self::RNDR_TRAP_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::RNDR_TRAP_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `RNDR_trap` field set to the given value.
    pub const fn with_rndr_trap(mut self, value: u8) -> Self {
        self.set_rndr_trap(value);
        self
    }

    /// Returns the value of the `CSV2_frac` field.
    pub const fn csv2_frac(self) -> u8 {
        ((self.bits() >> Self::CSV2_FRAC_SHIFT) & Self::CSV2_FRAC_MASK) as u8
    }

    /// Sets the value of the `CSV2_frac` field.
    pub const fn set_csv2_frac(&mut self, value: u8) {
        let offset = Self::CSV2_FRAC_SHIFT;
        assert!(value & (Self::CSV2_FRAC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CSV2_FRAC_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `CSV2_frac` field set to the given value.
    pub const fn with_csv2_frac(mut self, value: u8) -> Self {
        self.set_csv2_frac(value);
        self
    }

    /// Returns the value of the `NMI` field.
    pub const fn nmi(self) -> u8 {
        ((self.bits() >> Self::NMI_SHIFT) & Self::NMI_MASK) as u8
    }

    /// Sets the value of the `NMI` field.
    pub const fn set_nmi(&mut self, value: u8) {
        let offset = Self::NMI_SHIFT;
        assert!(value & (Self::NMI_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::NMI_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `NMI` field set to the given value.
    pub const fn with_nmi(mut self, value: u8) -> Self {
        self.set_nmi(value);
        self
    }

    /// Returns the value of the `MTE_frac` field.
    pub const fn mte_frac(self) -> u8 {
        ((self.bits() >> Self::MTE_FRAC_SHIFT) & Self::MTE_FRAC_MASK) as u8
    }

    /// Sets the value of the `MTE_frac` field.
    pub const fn set_mte_frac(&mut self, value: u8) {
        let offset = Self::MTE_FRAC_SHIFT;
        assert!(value & (Self::MTE_FRAC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::MTE_FRAC_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `MTE_frac` field set to the given value.
    pub const fn with_mte_frac(mut self, value: u8) -> Self {
        self.set_mte_frac(value);
        self
    }

    /// Returns the value of the `GCS` field.
    pub const fn gcs(self) -> u8 {
        ((self.bits() >> Self::GCS_SHIFT) & Self::GCS_MASK) as u8
    }

    /// Sets the value of the `GCS` field.
    pub const fn set_gcs(&mut self, value: u8) {
        let offset = Self::GCS_SHIFT;
        assert!(value & (Self::GCS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::GCS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `GCS` field set to the given value.
    pub const fn with_gcs(mut self, value: u8) -> Self {
        self.set_gcs(value);
        self
    }

    /// Returns the value of the `THE` field.
    pub const fn the(self) -> u8 {
        ((self.bits() >> Self::THE_SHIFT) & Self::THE_MASK) as u8
    }

    /// Sets the value of the `THE` field.
    pub const fn set_the(&mut self, value: u8) {
        let offset = Self::THE_SHIFT;
        assert!(value & (Self::THE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::THE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `THE` field set to the given value.
    pub const fn with_the(mut self, value: u8) -> Self {
        self.set_the(value);
        self
    }

    /// Returns the value of the `MTEX` field.
    pub const fn mtex(self) -> u8 {
        ((self.bits() >> Self::MTEX_SHIFT) & Self::MTEX_MASK) as u8
    }

    /// Sets the value of the `MTEX` field.
    pub const fn set_mtex(&mut self, value: u8) {
        let offset = Self::MTEX_SHIFT;
        assert!(value & (Self::MTEX_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::MTEX_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `MTEX` field set to the given value.
    pub const fn with_mtex(mut self, value: u8) -> Self {
        self.set_mtex(value);
        self
    }

    /// Returns the value of the `DF2` field.
    pub const fn df2(self) -> u8 {
        ((self.bits() >> Self::DF2_SHIFT) & Self::DF2_MASK) as u8
    }

    /// Sets the value of the `DF2` field.
    pub const fn set_df2(&mut self, value: u8) {
        let offset = Self::DF2_SHIFT;
        assert!(value & (Self::DF2_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::DF2_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `DF2` field set to the given value.
    pub const fn with_df2(mut self, value: u8) -> Self {
        self.set_df2(value);
        self
    }

    /// Returns the value of the `PFAR` field.
    pub const fn pfar(self) -> u8 {
        ((self.bits() >> Self::PFAR_SHIFT) & Self::PFAR_MASK) as u8
    }

    /// Sets the value of the `PFAR` field.
    pub const fn set_pfar(&mut self, value: u8) {
        let offset = Self::PFAR_SHIFT;
        assert!(value & (Self::PFAR_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PFAR_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PFAR` field set to the given value.
    pub const fn with_pfar(mut self, value: u8) -> Self {
        self.set_pfar(value);
        self
    }
}

bitflags! {
    /// `ID_AA64PFR2_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdAa64pfr2El1: u64 {
    }
}

impl IdAa64pfr2El1 {
    /// Offset of the `MTEPERM` field.
    pub const MTEPERM_SHIFT: u32 = 0;
    /// Mask for the `MTEPERM` field.
    pub const MTEPERM_MASK: u64 = 0b1111;
    /// Offset of the `MTESTOREONLY` field.
    pub const MTESTOREONLY_SHIFT: u32 = 4;
    /// Mask for the `MTESTOREONLY` field.
    pub const MTESTOREONLY_MASK: u64 = 0b1111;
    /// Offset of the `MTEFAR` field.
    pub const MTEFAR_SHIFT: u32 = 8;
    /// Mask for the `MTEFAR` field.
    pub const MTEFAR_MASK: u64 = 0b1111;
    /// Offset of the `GCIE` field.
    pub const GCIE_SHIFT: u32 = 12;
    /// Mask for the `GCIE` field.
    pub const GCIE_MASK: u64 = 0b1111;
    /// Offset of the `UINJ` field.
    pub const UINJ_SHIFT: u32 = 16;
    /// Mask for the `UINJ` field.
    pub const UINJ_MASK: u64 = 0b1111;
    /// Offset of the `MTEEIRG` field.
    pub const MTEEIRG_SHIFT: u32 = 20;
    /// Mask for the `MTEEIRG` field.
    pub const MTEEIRG_MASK: u64 = 0b1111;
    /// Offset of the `FGDT` field.
    pub const FGDT_SHIFT: u32 = 24;
    /// Mask for the `FGDT` field.
    pub const FGDT_MASK: u64 = 0b1111;
    /// Offset of the `MPAM2` field.
    pub const MPAM2_SHIFT: u32 = 28;
    /// Mask for the `MPAM2` field.
    pub const MPAM2_MASK: u64 = 0b1111;
    /// Offset of the `FPMR` field.
    pub const FPMR_SHIFT: u32 = 32;
    /// Mask for the `FPMR` field.
    pub const FPMR_MASK: u64 = 0b1111;
    /// Offset of the `VMTE` field.
    pub const VMTE_SHIFT: u32 = 36;
    /// Mask for the `VMTE` field.
    pub const VMTE_MASK: u64 = 0b1111;
    /// Offset of the `VMTETC` field.
    pub const VMTETC_SHIFT: u32 = 40;
    /// Mask for the `VMTETC` field.
    pub const VMTETC_MASK: u64 = 0b1111;
    /// Offset of the `VMTETCL` field.
    pub const VMTETCL_SHIFT: u32 = 44;
    /// Mask for the `VMTETCL` field.
    pub const VMTETCL_MASK: u64 = 0b1111;

    /// Returns the value of the `MTEPERM` field.
    pub const fn mteperm(self) -> u8 {
        ((self.bits() >> Self::MTEPERM_SHIFT) & Self::MTEPERM_MASK) as u8
    }

    /// Sets the value of the `MTEPERM` field.
    pub const fn set_mteperm(&mut self, value: u8) {
        let offset = Self::MTEPERM_SHIFT;
        assert!(value & (Self::MTEPERM_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::MTEPERM_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `MTEPERM` field set to the given value.
    pub const fn with_mteperm(mut self, value: u8) -> Self {
        self.set_mteperm(value);
        self
    }

    /// Returns the value of the `MTESTOREONLY` field.
    pub const fn mtestoreonly(self) -> u8 {
        ((self.bits() >> Self::MTESTOREONLY_SHIFT) & Self::MTESTOREONLY_MASK) as u8
    }

    /// Sets the value of the `MTESTOREONLY` field.
    pub const fn set_mtestoreonly(&mut self, value: u8) {
        let offset = Self::MTESTOREONLY_SHIFT;
        assert!(value & (Self::MTESTOREONLY_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::MTESTOREONLY_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `MTESTOREONLY` field set to the given value.
    pub const fn with_mtestoreonly(mut self, value: u8) -> Self {
        self.set_mtestoreonly(value);
        self
    }

    /// Returns the value of the `MTEFAR` field.
    pub const fn mtefar(self) -> u8 {
        ((self.bits() >> Self::MTEFAR_SHIFT) & Self::MTEFAR_MASK) as u8
    }

    /// Sets the value of the `MTEFAR` field.
    pub const fn set_mtefar(&mut self, value: u8) {
        let offset = Self::MTEFAR_SHIFT;
        assert!(value & (Self::MTEFAR_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::MTEFAR_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `MTEFAR` field set to the given value.
    pub const fn with_mtefar(mut self, value: u8) -> Self {
        self.set_mtefar(value);
        self
    }

    /// Returns the value of the `GCIE` field.
    pub const fn gcie(self) -> u8 {
        ((self.bits() >> Self::GCIE_SHIFT) & Self::GCIE_MASK) as u8
    }

    /// Sets the value of the `GCIE` field.
    pub const fn set_gcie(&mut self, value: u8) {
        let offset = Self::GCIE_SHIFT;
        assert!(value & (Self::GCIE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::GCIE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `GCIE` field set to the given value.
    pub const fn with_gcie(mut self, value: u8) -> Self {
        self.set_gcie(value);
        self
    }

    /// Returns the value of the `UINJ` field.
    pub const fn uinj(self) -> u8 {
        ((self.bits() >> Self::UINJ_SHIFT) & Self::UINJ_MASK) as u8
    }

    /// Sets the value of the `UINJ` field.
    pub const fn set_uinj(&mut self, value: u8) {
        let offset = Self::UINJ_SHIFT;
        assert!(value & (Self::UINJ_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::UINJ_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `UINJ` field set to the given value.
    pub const fn with_uinj(mut self, value: u8) -> Self {
        self.set_uinj(value);
        self
    }

    /// Returns the value of the `MTEEIRG` field.
    pub const fn mteeirg(self) -> u8 {
        ((self.bits() >> Self::MTEEIRG_SHIFT) & Self::MTEEIRG_MASK) as u8
    }

    /// Sets the value of the `MTEEIRG` field.
    pub const fn set_mteeirg(&mut self, value: u8) {
        let offset = Self::MTEEIRG_SHIFT;
        assert!(value & (Self::MTEEIRG_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::MTEEIRG_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `MTEEIRG` field set to the given value.
    pub const fn with_mteeirg(mut self, value: u8) -> Self {
        self.set_mteeirg(value);
        self
    }

    /// Returns the value of the `FGDT` field.
    pub const fn fgdt(self) -> u8 {
        ((self.bits() >> Self::FGDT_SHIFT) & Self::FGDT_MASK) as u8
    }

    /// Sets the value of the `FGDT` field.
    pub const fn set_fgdt(&mut self, value: u8) {
        let offset = Self::FGDT_SHIFT;
        assert!(value & (Self::FGDT_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::FGDT_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `FGDT` field set to the given value.
    pub const fn with_fgdt(mut self, value: u8) -> Self {
        self.set_fgdt(value);
        self
    }

    /// Returns the value of the `MPAM2` field.
    pub const fn mpam2(self) -> u8 {
        ((self.bits() >> Self::MPAM2_SHIFT) & Self::MPAM2_MASK) as u8
    }

    /// Sets the value of the `MPAM2` field.
    pub const fn set_mpam2(&mut self, value: u8) {
        let offset = Self::MPAM2_SHIFT;
        assert!(value & (Self::MPAM2_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::MPAM2_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `MPAM2` field set to the given value.
    pub const fn with_mpam2(mut self, value: u8) -> Self {
        self.set_mpam2(value);
        self
    }

    /// Returns the value of the `FPMR` field.
    pub const fn fpmr(self) -> u8 {
        ((self.bits() >> Self::FPMR_SHIFT) & Self::FPMR_MASK) as u8
    }

    /// Sets the value of the `FPMR` field.
    pub const fn set_fpmr(&mut self, value: u8) {
        let offset = Self::FPMR_SHIFT;
        assert!(value & (Self::FPMR_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::FPMR_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `FPMR` field set to the given value.
    pub const fn with_fpmr(mut self, value: u8) -> Self {
        self.set_fpmr(value);
        self
    }

    /// Returns the value of the `VMTE` field.
    pub const fn vmte(self) -> u8 {
        ((self.bits() >> Self::VMTE_SHIFT) & Self::VMTE_MASK) as u8
    }

    /// Sets the value of the `VMTE` field.
    pub const fn set_vmte(&mut self, value: u8) {
        let offset = Self::VMTE_SHIFT;
        assert!(value & (Self::VMTE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VMTE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `VMTE` field set to the given value.
    pub const fn with_vmte(mut self, value: u8) -> Self {
        self.set_vmte(value);
        self
    }

    /// Returns the value of the `VMTETC` field.
    pub const fn vmtetc(self) -> u8 {
        ((self.bits() >> Self::VMTETC_SHIFT) & Self::VMTETC_MASK) as u8
    }

    /// Sets the value of the `VMTETC` field.
    pub const fn set_vmtetc(&mut self, value: u8) {
        let offset = Self::VMTETC_SHIFT;
        assert!(value & (Self::VMTETC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VMTETC_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `VMTETC` field set to the given value.
    pub const fn with_vmtetc(mut self, value: u8) -> Self {
        self.set_vmtetc(value);
        self
    }

    /// Returns the value of the `VMTETCL` field.
    pub const fn vmtetcl(self) -> u8 {
        ((self.bits() >> Self::VMTETCL_SHIFT) & Self::VMTETCL_MASK) as u8
    }

    /// Sets the value of the `VMTETCL` field.
    pub const fn set_vmtetcl(&mut self, value: u8) {
        let offset = Self::VMTETCL_SHIFT;
        assert!(value & (Self::VMTETCL_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VMTETCL_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `VMTETCL` field set to the given value.
    pub const fn with_vmtetcl(mut self, value: u8) -> Self {
        self.set_vmtetcl(value);
        self
    }
}

bitflags! {
    /// `ID_AA64SMFR0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdAa64smfr0El1: u64 {
        /// `SMOP4` bit.
        const SMOP4 = 1 << 0;
        /// `STMOP` bit.
        const STMOP = 1 << 16;
        /// `SFEXPA` bit.
        const SFEXPA = 1 << 23;
        /// `AES` bit.
        const AES = 1 << 24;
        /// `SBitPerm` bit.
        const SBITPERM = 1 << 25;
        /// `SF8DP2` bit.
        const SF8DP2 = 1 << 28;
        /// `SF8DP4` bit.
        const SF8DP4 = 1 << 29;
        /// `SF8FMA` bit.
        const SF8FMA = 1 << 30;
        /// `F32F32` bit.
        const F32F32 = 1 << 32;
        /// `BI32I32` bit.
        const BI32I32 = 1 << 33;
        /// `B16F32` bit.
        const B16F32 = 1 << 34;
        /// `F16F32` bit.
        const F16F32 = 1 << 35;
        /// `F8F32` bit.
        const F8F32 = 1 << 40;
        /// `F8F16` bit.
        const F8F16 = 1 << 41;
        /// `F16F16` bit.
        const F16F16 = 1 << 42;
        /// `B16B16` bit.
        const B16B16 = 1 << 43;
        /// `F64F64` bit.
        const F64F64 = 1 << 48;
        /// `LUTv2` bit.
        const LUTV2 = 1 << 60;
        /// `LUT6` bit.
        const LUT6 = 1 << 61;
        /// `FA64` bit.
        const FA64 = 1 << 63;
    }
}

impl IdAa64smfr0El1 {
    /// Offset of the `SMOP4` field.
    pub const SMOP4_SHIFT: u32 = 0;
    /// Offset of the `STMOP` field.
    pub const STMOP_SHIFT: u32 = 16;
    /// Offset of the `SFEXPA` field.
    pub const SFEXPA_SHIFT: u32 = 23;
    /// Offset of the `AES` field.
    pub const AES_SHIFT: u32 = 24;
    /// Offset of the `SBitPerm` field.
    pub const SBITPERM_SHIFT: u32 = 25;
    /// Offset of the `SF8DP2` field.
    pub const SF8DP2_SHIFT: u32 = 28;
    /// Offset of the `SF8DP4` field.
    pub const SF8DP4_SHIFT: u32 = 29;
    /// Offset of the `SF8FMA` field.
    pub const SF8FMA_SHIFT: u32 = 30;
    /// Offset of the `F32F32` field.
    pub const F32F32_SHIFT: u32 = 32;
    /// Offset of the `BI32I32` field.
    pub const BI32I32_SHIFT: u32 = 33;
    /// Offset of the `B16F32` field.
    pub const B16F32_SHIFT: u32 = 34;
    /// Offset of the `F16F32` field.
    pub const F16F32_SHIFT: u32 = 35;
    /// Offset of the `I8I32` field.
    pub const I8I32_SHIFT: u32 = 36;
    /// Mask for the `I8I32` field.
    pub const I8I32_MASK: u64 = 0b1111;
    /// Offset of the `F8F32` field.
    pub const F8F32_SHIFT: u32 = 40;
    /// Offset of the `F8F16` field.
    pub const F8F16_SHIFT: u32 = 41;
    /// Offset of the `F16F16` field.
    pub const F16F16_SHIFT: u32 = 42;
    /// Offset of the `B16B16` field.
    pub const B16B16_SHIFT: u32 = 43;
    /// Offset of the `I16I32` field.
    pub const I16I32_SHIFT: u32 = 44;
    /// Mask for the `I16I32` field.
    pub const I16I32_MASK: u64 = 0b1111;
    /// Offset of the `F64F64` field.
    pub const F64F64_SHIFT: u32 = 48;
    /// Offset of the `I16I64` field.
    pub const I16I64_SHIFT: u32 = 52;
    /// Mask for the `I16I64` field.
    pub const I16I64_MASK: u64 = 0b1111;
    /// Offset of the `SMEver` field.
    pub const SMEVER_SHIFT: u32 = 56;
    /// Mask for the `SMEver` field.
    pub const SMEVER_MASK: u64 = 0b1111;
    /// Offset of the `LUTv2` field.
    pub const LUTV2_SHIFT: u32 = 60;
    /// Offset of the `LUT6` field.
    pub const LUT6_SHIFT: u32 = 61;
    /// Offset of the `FA64` field.
    pub const FA64_SHIFT: u32 = 63;

    /// Returns the value of the `I8I32` field.
    pub const fn i8i32(self) -> u8 {
        ((self.bits() >> Self::I8I32_SHIFT) & Self::I8I32_MASK) as u8
    }

    /// Sets the value of the `I8I32` field.
    pub const fn set_i8i32(&mut self, value: u8) {
        let offset = Self::I8I32_SHIFT;
        assert!(value & (Self::I8I32_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::I8I32_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `I8I32` field set to the given value.
    pub const fn with_i8i32(mut self, value: u8) -> Self {
        self.set_i8i32(value);
        self
    }

    /// Returns the value of the `I16I32` field.
    pub const fn i16i32(self) -> u8 {
        ((self.bits() >> Self::I16I32_SHIFT) & Self::I16I32_MASK) as u8
    }

    /// Sets the value of the `I16I32` field.
    pub const fn set_i16i32(&mut self, value: u8) {
        let offset = Self::I16I32_SHIFT;
        assert!(value & (Self::I16I32_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::I16I32_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `I16I32` field set to the given value.
    pub const fn with_i16i32(mut self, value: u8) -> Self {
        self.set_i16i32(value);
        self
    }

    /// Returns the value of the `I16I64` field.
    pub const fn i16i64(self) -> u8 {
        ((self.bits() >> Self::I16I64_SHIFT) & Self::I16I64_MASK) as u8
    }

    /// Sets the value of the `I16I64` field.
    pub const fn set_i16i64(&mut self, value: u8) {
        let offset = Self::I16I64_SHIFT;
        assert!(value & (Self::I16I64_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::I16I64_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `I16I64` field set to the given value.
    pub const fn with_i16i64(mut self, value: u8) -> Self {
        self.set_i16i64(value);
        self
    }

    /// Returns the value of the `SMEver` field.
    pub const fn smever(self) -> u8 {
        ((self.bits() >> Self::SMEVER_SHIFT) & Self::SMEVER_MASK) as u8
    }

    /// Sets the value of the `SMEver` field.
    pub const fn set_smever(&mut self, value: u8) {
        let offset = Self::SMEVER_SHIFT;
        assert!(value & (Self::SMEVER_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SMEVER_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `SMEver` field set to the given value.
    pub const fn with_smever(mut self, value: u8) -> Self {
        self.set_smever(value);
        self
    }
}

bitflags! {
    /// `ISR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IsrEl1: u64 {
        /// `F` bit.
        const F = 1 << 6;
        /// `I` bit.
        const I = 1 << 7;
        /// `A` bit.
        const A = 1 << 8;
        /// `FS` bit.
        const FS = 1 << 9;
        /// `IS` bit.
        const IS = 1 << 10;
    }
}

impl IsrEl1 {
    /// Offset of the `F` field.
    pub const F_SHIFT: u32 = 6;
    /// Offset of the `I` field.
    pub const I_SHIFT: u32 = 7;
    /// Offset of the `A` field.
    pub const A_SHIFT: u32 = 8;
    /// Offset of the `FS` field.
    pub const FS_SHIFT: u32 = 9;
    /// Offset of the `IS` field.
    pub const IS_SHIFT: u32 = 10;
}

bitflags! {
    /// `MAIR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct MairEl1: u64 {
    }
}

impl MairEl1 {
    /// Offset of the `Attr<n>` field.
    pub const ATTR_SHIFT: u32 = 0;
    /// Mask for the `Attr<n>` field.
    pub const ATTR_MASK: u64 = 0b1111_1111;

    /// Returns the value of the given `Attr<n>` field.
    pub const fn attr(self, n: u32) -> u8 {
        assert!(n < 8);
        ((self.bits() >> (Self::ATTR_SHIFT + n * 8)) & Self::ATTR_MASK) as u8
    }

    /// Sets the value of the `Attr<n>` field.
    pub const fn set_attr(&mut self, n: u32, value: u8) {
        assert!(n < 8);
        let offset = Self::ATTR_SHIFT + n * 8;
        assert!(value & (Self::ATTR_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ATTR_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `Attr<n>` field set to the given value.
    pub const fn with_attr(mut self, n: u32, value: u8) -> Self {
        self.set_attr(n, value);
        self
    }
}

bitflags! {
    /// `MDCCINT_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct MdccintEl1: u64 {
        /// `TX` bit.
        const TX = 1 << 29;
        /// `RX` bit.
        const RX = 1 << 30;
    }
}

impl MdccintEl1 {
    /// Offset of the `TX` field.
    pub const TX_SHIFT: u32 = 29;
    /// Offset of the `RX` field.
    pub const RX_SHIFT: u32 = 30;
}

bitflags! {
    /// `MDSCR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct MdscrEl1: u64 {
        /// `SS` bit.
        const SS = 1 << 0;
        /// `ERR` bit.
        const ERR = 1 << 6;
        /// `TDCC` bit.
        const TDCC = 1 << 12;
        /// `KDE` bit.
        const KDE = 1 << 13;
        /// `HDE` bit.
        const HDE = 1 << 14;
        /// `MDE` bit.
        const MDE = 1 << 15;
        /// `SC2` bit.
        const SC2 = 1 << 19;
        /// `TDA` bit.
        const TDA = 1 << 21;
        /// `TXU` bit.
        const TXU = 1 << 26;
        /// `RXO` bit.
        const RXO = 1 << 27;
        /// `TXfull` bit.
        const TXFULL = 1 << 29;
        /// `RXfull` bit.
        const RXFULL = 1 << 30;
        /// `TFO` bit.
        const TFO = 1 << 31;
        /// `EMBWE` bit.
        const EMBWE = 1 << 32;
        /// `TTA` bit.
        const TTA = 1 << 33;
        /// `EnSPM` bit.
        const ENSPM = 1 << 34;
        /// `EHBWE` bit.
        const EHBWE = 1 << 35;
        /// `EnSTEPOP` bit.
        const ENSTEPOP = 1 << 50;
    }
}

impl MdscrEl1 {
    /// Offset of the `SS` field.
    pub const SS_SHIFT: u32 = 0;
    /// Offset of the `ERR` field.
    pub const ERR_SHIFT: u32 = 6;
    /// Offset of the `TDCC` field.
    pub const TDCC_SHIFT: u32 = 12;
    /// Offset of the `KDE` field.
    pub const KDE_SHIFT: u32 = 13;
    /// Offset of the `HDE` field.
    pub const HDE_SHIFT: u32 = 14;
    /// Offset of the `MDE` field.
    pub const MDE_SHIFT: u32 = 15;
    /// Offset of the `SC2` field.
    pub const SC2_SHIFT: u32 = 19;
    /// Offset of the `TDA` field.
    pub const TDA_SHIFT: u32 = 21;
    /// Offset of the `INTdis` field.
    pub const INTDIS_SHIFT: u32 = 22;
    /// Mask for the `INTdis` field.
    pub const INTDIS_MASK: u64 = 0b11;
    /// Offset of the `TXU` field.
    pub const TXU_SHIFT: u32 = 26;
    /// Offset of the `RXO` field.
    pub const RXO_SHIFT: u32 = 27;
    /// Offset of the `TXfull` field.
    pub const TXFULL_SHIFT: u32 = 29;
    /// Offset of the `RXfull` field.
    pub const RXFULL_SHIFT: u32 = 30;
    /// Offset of the `TFO` field.
    pub const TFO_SHIFT: u32 = 31;
    /// Offset of the `EMBWE` field.
    pub const EMBWE_SHIFT: u32 = 32;
    /// Offset of the `TTA` field.
    pub const TTA_SHIFT: u32 = 33;
    /// Offset of the `EnSPM` field.
    pub const ENSPM_SHIFT: u32 = 34;
    /// Offset of the `EHBWE` field.
    pub const EHBWE_SHIFT: u32 = 35;
    /// Offset of the `EnSTEPOP` field.
    pub const ENSTEPOP_SHIFT: u32 = 50;

    /// Returns the value of the `INTdis` field.
    pub const fn intdis(self) -> u8 {
        ((self.bits() >> Self::INTDIS_SHIFT) & Self::INTDIS_MASK) as u8
    }

    /// Sets the value of the `INTdis` field.
    pub const fn set_intdis(&mut self, value: u8) {
        let offset = Self::INTDIS_SHIFT;
        assert!(value & (Self::INTDIS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::INTDIS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `INTdis` field set to the given value.
    pub const fn with_intdis(mut self, value: u8) -> Self {
        self.set_intdis(value);
        self
    }
}

bitflags! {
    /// `MIDR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct MidrEl1: u64 {
    }
}

impl MidrEl1 {
    /// Offset of the `Revision` field.
    pub const REVISION_SHIFT: u32 = 0;
    /// Mask for the `Revision` field.
    pub const REVISION_MASK: u64 = 0b1111;
    /// Offset of the `PartNum` field.
    pub const PARTNUM_SHIFT: u32 = 4;
    /// Mask for the `PartNum` field.
    pub const PARTNUM_MASK: u64 = 0b1111_1111_1111;
    /// Offset of the `Architecture` field.
    pub const ARCHITECTURE_SHIFT: u32 = 16;
    /// Mask for the `Architecture` field.
    pub const ARCHITECTURE_MASK: u64 = 0b1111;
    /// Offset of the `Variant` field.
    pub const VARIANT_SHIFT: u32 = 20;
    /// Mask for the `Variant` field.
    pub const VARIANT_MASK: u64 = 0b1111;
    /// Offset of the `Implementer` field.
    pub const IMPLEMENTER_SHIFT: u32 = 24;
    /// Mask for the `Implementer` field.
    pub const IMPLEMENTER_MASK: u64 = 0b1111_1111;

    /// Returns the value of the `Revision` field.
    pub const fn revision(self) -> u8 {
        ((self.bits() >> Self::REVISION_SHIFT) & Self::REVISION_MASK) as u8
    }

    /// Sets the value of the `Revision` field.
    pub const fn set_revision(&mut self, value: u8) {
        let offset = Self::REVISION_SHIFT;
        assert!(value & (Self::REVISION_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::REVISION_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `Revision` field set to the given value.
    pub const fn with_revision(mut self, value: u8) -> Self {
        self.set_revision(value);
        self
    }

    /// Returns the value of the `PartNum` field.
    pub const fn partnum(self) -> u16 {
        ((self.bits() >> Self::PARTNUM_SHIFT) & Self::PARTNUM_MASK) as u16
    }

    /// Sets the value of the `PartNum` field.
    pub const fn set_partnum(&mut self, value: u16) {
        let offset = Self::PARTNUM_SHIFT;
        assert!(value & (Self::PARTNUM_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PARTNUM_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PartNum` field set to the given value.
    pub const fn with_partnum(mut self, value: u16) -> Self {
        self.set_partnum(value);
        self
    }

    /// Returns the value of the `Architecture` field.
    pub const fn architecture(self) -> u8 {
        ((self.bits() >> Self::ARCHITECTURE_SHIFT) & Self::ARCHITECTURE_MASK) as u8
    }

    /// Sets the value of the `Architecture` field.
    pub const fn set_architecture(&mut self, value: u8) {
        let offset = Self::ARCHITECTURE_SHIFT;
        assert!(value & (Self::ARCHITECTURE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ARCHITECTURE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `Architecture` field set to the given value.
    pub const fn with_architecture(mut self, value: u8) -> Self {
        self.set_architecture(value);
        self
    }

    /// Returns the value of the `Variant` field.
    pub const fn variant(self) -> u8 {
        ((self.bits() >> Self::VARIANT_SHIFT) & Self::VARIANT_MASK) as u8
    }

    /// Sets the value of the `Variant` field.
    pub const fn set_variant(&mut self, value: u8) {
        let offset = Self::VARIANT_SHIFT;
        assert!(value & (Self::VARIANT_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VARIANT_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `Variant` field set to the given value.
    pub const fn with_variant(mut self, value: u8) -> Self {
        self.set_variant(value);
        self
    }

    /// Returns the value of the `Implementer` field.
    pub const fn implementer(self) -> u8 {
        ((self.bits() >> Self::IMPLEMENTER_SHIFT) & Self::IMPLEMENTER_MASK) as u8
    }

    /// Sets the value of the `Implementer` field.
    pub const fn set_implementer(&mut self, value: u8) {
        let offset = Self::IMPLEMENTER_SHIFT;
        assert!(value & (Self::IMPLEMENTER_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::IMPLEMENTER_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `Implementer` field set to the given value.
    pub const fn with_implementer(mut self, value: u8) -> Self {
        self.set_implementer(value);
        self
    }
}

bitflags! {
    /// `MPAMIDR_EL1` system register value.
    ///
    /// Indicates the maximum PARTID and PMG values supported in the implementation and the support for other optional features.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct MpamidrEl1: u64 {
        /// Indicates support for MPAM virtualization.
        const HAS_HCR = 1 << 17;
        /// `HAS_ALT_ID` bit.
        const HAS_ALT_ID = 1 << 21;
        /// `HAS_INSTR_ALT_ID` bit.
        const HAS_INSTR_ALT_ID = 1 << 22;
        /// `HAS_BW_CTRL` bit.
        const HAS_BW_CTRL = 1 << 56;
        /// `HAS_ALTSP` bit.
        const HAS_ALTSP = 1 << 57;
        /// `HAS_TIDR` bit.
        const HAS_TIDR = 1 << 58;
        /// `SP4` bit.
        const SP4 = 1 << 59;
        /// `HAS_FORCE_NS` bit.
        const HAS_FORCE_NS = 1 << 60;
        /// `HAS_SDEFLT` bit.
        const HAS_SDEFLT = 1 << 61;
    }
}

impl MpamidrEl1 {
    /// Offset of the `PARTID_MAX` field.
    pub const PARTID_MAX_SHIFT: u32 = 0;
    /// Mask for the `PARTID_MAX` field.
    pub const PARTID_MAX_MASK: u64 = 0b1111_1111_1111_1111;
    /// Offset of the `HAS_HCR` field.
    pub const HAS_HCR_SHIFT: u32 = 17;
    /// Offset of the `VPMR_MAX` field.
    pub const VPMR_MAX_SHIFT: u32 = 18;
    /// Mask for the `VPMR_MAX` field.
    pub const VPMR_MAX_MASK: u64 = 0b111;
    /// Offset of the `HAS_ALT_ID` field.
    pub const HAS_ALT_ID_SHIFT: u32 = 21;
    /// Offset of the `HAS_INSTR_ALT_ID` field.
    pub const HAS_INSTR_ALT_ID_SHIFT: u32 = 22;
    /// Offset of the `HAS_BW_CTRL` field.
    pub const HAS_BW_CTRL_SHIFT: u32 = 56;
    /// Offset of the `HAS_ALTSP` field.
    pub const HAS_ALTSP_SHIFT: u32 = 57;
    /// Offset of the `HAS_TIDR` field.
    pub const HAS_TIDR_SHIFT: u32 = 58;
    /// Offset of the `SP4` field.
    pub const SP4_SHIFT: u32 = 59;
    /// Offset of the `HAS_FORCE_NS` field.
    pub const HAS_FORCE_NS_SHIFT: u32 = 60;
    /// Offset of the `HAS_SDEFLT` field.
    pub const HAS_SDEFLT_SHIFT: u32 = 61;

    /// Returns the value of the `PARTID_MAX` field.
    pub const fn partid_max(self) -> u16 {
        ((self.bits() >> Self::PARTID_MAX_SHIFT) & Self::PARTID_MAX_MASK) as u16
    }

    /// Sets the value of the `PARTID_MAX` field.
    pub const fn set_partid_max(&mut self, value: u16) {
        let offset = Self::PARTID_MAX_SHIFT;
        assert!(value & (Self::PARTID_MAX_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PARTID_MAX_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PARTID_MAX` field set to the given value.
    pub const fn with_partid_max(mut self, value: u16) -> Self {
        self.set_partid_max(value);
        self
    }

    /// Returns the value of the `VPMR_MAX` field.
    ///
    /// Indicates the maximum register index n for the `MPAMVPM<n>_EL2` registers.
    pub const fn vpmr_max(self) -> u8 {
        ((self.bits() >> Self::VPMR_MAX_SHIFT) & Self::VPMR_MAX_MASK) as u8
    }

    /// Sets the value of the `VPMR_MAX` field.
    ///
    /// Indicates the maximum register index n for the `MPAMVPM<n>_EL2` registers.
    pub const fn set_vpmr_max(&mut self, value: u8) {
        let offset = Self::VPMR_MAX_SHIFT;
        assert!(value & (Self::VPMR_MAX_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VPMR_MAX_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `VPMR_MAX` field set to the given value.
    ///
    /// Indicates the maximum register index n for the `MPAMVPM<n>_EL2` registers.
    pub const fn with_vpmr_max(mut self, value: u8) -> Self {
        self.set_vpmr_max(value);
        self
    }
}

bitflags! {
    /// `MPIDR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct MpidrEl1: u64 {
        /// RES1 bits in the `MPIDR_EL1` register.
        const RES1 = 0b1000_0000_0000_0000_0000_0000_0000_0000;
        /// `MT` bit.
        const MT = 1 << 24;
        /// `U` bit.
        const U = 1 << 30;
    }
}

impl MpidrEl1 {
    /// Offset of the `Aff0` field.
    pub const AFF0_SHIFT: u32 = 0;
    /// Mask for the `Aff0` field.
    pub const AFF0_MASK: u64 = 0b1111_1111;
    /// Offset of the `Aff1` field.
    pub const AFF1_SHIFT: u32 = 8;
    /// Mask for the `Aff1` field.
    pub const AFF1_MASK: u64 = 0b1111_1111;
    /// Offset of the `Aff2` field.
    pub const AFF2_SHIFT: u32 = 16;
    /// Mask for the `Aff2` field.
    pub const AFF2_MASK: u64 = 0b1111_1111;
    /// Offset of the `MT` field.
    pub const MT_SHIFT: u32 = 24;
    /// Offset of the `U` field.
    pub const U_SHIFT: u32 = 30;
    /// Offset of the `Aff3` field.
    pub const AFF3_SHIFT: u32 = 32;
    /// Mask for the `Aff3` field.
    pub const AFF3_MASK: u64 = 0b1111_1111;

    /// Returns the value of the `Aff0` field.
    pub const fn aff0(self) -> u8 {
        ((self.bits() >> Self::AFF0_SHIFT) & Self::AFF0_MASK) as u8
    }

    /// Sets the value of the `Aff0` field.
    pub const fn set_aff0(&mut self, value: u8) {
        let offset = Self::AFF0_SHIFT;
        assert!(value & (Self::AFF0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::AFF0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `Aff0` field set to the given value.
    pub const fn with_aff0(mut self, value: u8) -> Self {
        self.set_aff0(value);
        self
    }

    /// Returns the value of the `Aff1` field.
    pub const fn aff1(self) -> u8 {
        ((self.bits() >> Self::AFF1_SHIFT) & Self::AFF1_MASK) as u8
    }

    /// Sets the value of the `Aff1` field.
    pub const fn set_aff1(&mut self, value: u8) {
        let offset = Self::AFF1_SHIFT;
        assert!(value & (Self::AFF1_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::AFF1_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `Aff1` field set to the given value.
    pub const fn with_aff1(mut self, value: u8) -> Self {
        self.set_aff1(value);
        self
    }

    /// Returns the value of the `Aff2` field.
    pub const fn aff2(self) -> u8 {
        ((self.bits() >> Self::AFF2_SHIFT) & Self::AFF2_MASK) as u8
    }

    /// Sets the value of the `Aff2` field.
    pub const fn set_aff2(&mut self, value: u8) {
        let offset = Self::AFF2_SHIFT;
        assert!(value & (Self::AFF2_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::AFF2_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `Aff2` field set to the given value.
    pub const fn with_aff2(mut self, value: u8) -> Self {
        self.set_aff2(value);
        self
    }

    /// Returns the value of the `Aff3` field.
    pub const fn aff3(self) -> u8 {
        ((self.bits() >> Self::AFF3_SHIFT) & Self::AFF3_MASK) as u8
    }

    /// Sets the value of the `Aff3` field.
    pub const fn set_aff3(&mut self, value: u8) {
        let offset = Self::AFF3_SHIFT;
        assert!(value & (Self::AFF3_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::AFF3_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `Aff3` field set to the given value.
    pub const fn with_aff3(mut self, value: u8) -> Self {
        self.set_aff3(value);
        self
    }
}

bitflags! {
    /// `PAR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ParEl1: u64 {
        /// RES1 bits in the `PAR_EL1` register.
        const RES1 = 0b1000_0000_0000;
        /// `F` bit.
        const F = 1 << 0;
        /// `PTW` bit.
        const PTW = 1 << 8;
        /// `NS` bit.
        const NS = 1 << 9;
        /// `S` bit.
        const S = 1 << 9;
        /// `NSE` bit.
        const NSE = 1 << 11;
        /// `AssuredOnly` bit.
        const ASSUREDONLY = 1 << 12;
        /// `TopLevel` bit.
        const TOPLEVEL = 1 << 13;
        /// `Overlay` bit.
        const OVERLAY = 1 << 14;
        /// `DirtyBit` bit.
        const DIRTYBIT = 1 << 15;
    }
}

impl ParEl1 {
    /// Offset of the `F` field.
    pub const F_SHIFT: u32 = 0;
    /// Offset of the `FST` field.
    pub const FST_SHIFT: u32 = 1;
    /// Mask for the `FST` field.
    pub const FST_MASK: u64 = 0b11_1111;
    /// Offset of the `SH` field.
    pub const SH_SHIFT: u32 = 7;
    /// Mask for the `SH` field.
    pub const SH_MASK: u64 = 0b11;
    /// Offset of the `PTW` field.
    pub const PTW_SHIFT: u32 = 8;
    /// Offset of the `NS` field.
    pub const NS_SHIFT: u32 = 9;
    /// Offset of the `S` field.
    pub const S_SHIFT: u32 = 9;
    /// Offset of the `NSE` field.
    pub const NSE_SHIFT: u32 = 11;
    /// Offset of the `AssuredOnly` field.
    pub const ASSUREDONLY_SHIFT: u32 = 12;
    /// Offset of the `PA[47:12]` field.
    pub const PA_47_12_SHIFT: u32 = 12;
    /// Mask for the `PA[47:12]` field.
    pub const PA_47_12_MASK: u64 = 0b1111_1111_1111_1111_1111_1111_1111_1111_1111;
    /// Offset of the `TopLevel` field.
    pub const TOPLEVEL_SHIFT: u32 = 13;
    /// Offset of the `Overlay` field.
    pub const OVERLAY_SHIFT: u32 = 14;
    /// Offset of the `DirtyBit` field.
    pub const DIRTYBIT_SHIFT: u32 = 15;
    /// Offset of the `PA[51:48]` field.
    pub const PA_51_48_SHIFT: u32 = 48;
    /// Mask for the `PA[51:48]` field.
    pub const PA_51_48_MASK: u64 = 0b1111;
    /// Offset of the `ATTR` field.
    pub const ATTR_SHIFT: u32 = 56;
    /// Mask for the `ATTR` field.
    pub const ATTR_MASK: u64 = 0b1111_1111;

    /// Returns the value of the `FST` field.
    pub const fn fst(self) -> u8 {
        ((self.bits() >> Self::FST_SHIFT) & Self::FST_MASK) as u8
    }

    /// Sets the value of the `FST` field.
    pub const fn set_fst(&mut self, value: u8) {
        let offset = Self::FST_SHIFT;
        assert!(value & (Self::FST_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::FST_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `FST` field set to the given value.
    pub const fn with_fst(mut self, value: u8) -> Self {
        self.set_fst(value);
        self
    }

    /// Returns the value of the `SH` field.
    pub const fn sh(self) -> u8 {
        ((self.bits() >> Self::SH_SHIFT) & Self::SH_MASK) as u8
    }

    /// Sets the value of the `SH` field.
    pub const fn set_sh(&mut self, value: u8) {
        let offset = Self::SH_SHIFT;
        assert!(value & (Self::SH_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SH_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `SH` field set to the given value.
    pub const fn with_sh(mut self, value: u8) -> Self {
        self.set_sh(value);
        self
    }

    /// Returns the value of the `PA[47:12]` field.
    pub const fn pa_47_12(self) -> u64 {
        (self.bits() >> Self::PA_47_12_SHIFT) & Self::PA_47_12_MASK
    }

    /// Sets the value of the `PA[47:12]` field.
    pub const fn set_pa_47_12(&mut self, value: u64) {
        let offset = Self::PA_47_12_SHIFT;
        assert!(value & Self::PA_47_12_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PA_47_12_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `PA[47:12]` field set to the given value.
    pub const fn with_pa_47_12(mut self, value: u64) -> Self {
        self.set_pa_47_12(value);
        self
    }

    /// Returns the value of the `PA[51:48]` field.
    pub const fn pa_51_48(self) -> u8 {
        ((self.bits() >> Self::PA_51_48_SHIFT) & Self::PA_51_48_MASK) as u8
    }

    /// Sets the value of the `PA[51:48]` field.
    pub const fn set_pa_51_48(&mut self, value: u8) {
        let offset = Self::PA_51_48_SHIFT;
        assert!(value & (Self::PA_51_48_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PA_51_48_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PA[51:48]` field set to the given value.
    pub const fn with_pa_51_48(mut self, value: u8) -> Self {
        self.set_pa_51_48(value);
        self
    }

    /// Returns the value of the `ATTR` field.
    pub const fn attr(self) -> u8 {
        ((self.bits() >> Self::ATTR_SHIFT) & Self::ATTR_MASK) as u8
    }

    /// Sets the value of the `ATTR` field.
    pub const fn set_attr(&mut self, value: u8) {
        let offset = Self::ATTR_SHIFT;
        assert!(value & (Self::ATTR_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ATTR_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `ATTR` field set to the given value.
    pub const fn with_attr(mut self, value: u8) -> Self {
        self.set_attr(value);
        self
    }
}

bitflags! {
    /// `PFAR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct PfarEl1: u64 {
        /// `NSE` bit.
        const NSE = 1 << 62;
        /// `NS` bit.
        const NS = 1 << 63;
    }
}

impl PfarEl1 {
    /// Offset of the `PA` field.
    pub const PA_SHIFT: u32 = 0;
    /// Mask for the `PA` field.
    pub const PA_MASK: u64 = 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;
    /// Offset of the `PA[51:48]` field.
    pub const PA_51_48_SHIFT: u32 = 48;
    /// Mask for the `PA[51:48]` field.
    pub const PA_51_48_MASK: u64 = 0b1111;
    /// Offset of the `PA[55:52]` field.
    pub const PA_55_52_SHIFT: u32 = 52;
    /// Mask for the `PA[55:52]` field.
    pub const PA_55_52_MASK: u64 = 0b1111;
    /// Offset of the `NSE` field.
    pub const NSE_SHIFT: u32 = 62;
    /// Offset of the `NS` field.
    pub const NS_SHIFT: u32 = 63;

    /// Returns the value of the `PA` field.
    pub const fn pa(self) -> u64 {
        (self.bits() >> Self::PA_SHIFT) & Self::PA_MASK
    }

    /// Sets the value of the `PA` field.
    pub const fn set_pa(&mut self, value: u64) {
        let offset = Self::PA_SHIFT;
        assert!(value & Self::PA_MASK == value);
        *self =
            Self::from_bits_retain((self.bits() & !(Self::PA_MASK << offset)) | (value << offset));
    }

    /// Returns a copy with the `PA` field set to the given value.
    pub const fn with_pa(mut self, value: u64) -> Self {
        self.set_pa(value);
        self
    }

    /// Returns the value of the `PA[51:48]` field.
    pub const fn pa_51_48(self) -> u8 {
        ((self.bits() >> Self::PA_51_48_SHIFT) & Self::PA_51_48_MASK) as u8
    }

    /// Sets the value of the `PA[51:48]` field.
    pub const fn set_pa_51_48(&mut self, value: u8) {
        let offset = Self::PA_51_48_SHIFT;
        assert!(value & (Self::PA_51_48_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PA_51_48_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PA[51:48]` field set to the given value.
    pub const fn with_pa_51_48(mut self, value: u8) -> Self {
        self.set_pa_51_48(value);
        self
    }

    /// Returns the value of the `PA[55:52]` field.
    pub const fn pa_55_52(self) -> u8 {
        ((self.bits() >> Self::PA_55_52_SHIFT) & Self::PA_55_52_MASK) as u8
    }

    /// Sets the value of the `PA[55:52]` field.
    pub const fn set_pa_55_52(&mut self, value: u8) {
        let offset = Self::PA_55_52_SHIFT;
        assert!(value & (Self::PA_55_52_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PA_55_52_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PA[55:52]` field set to the given value.
    pub const fn with_pa_55_52(mut self, value: u8) -> Self {
        self.set_pa_55_52(value);
        self
    }
}

bitflags! {
    /// `PIRE0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Pire0El1: u64 {
    }
}

impl Pire0El1 {
    /// Offset of the `Perm<m>` field.
    pub const PERM_SHIFT: u32 = 0;
    /// Mask for the `Perm<m>` field.
    pub const PERM_MASK: u64 = 0b1111;

    /// Returns the value of the given `Perm<m>` field.
    pub const fn perm(self, m: u32) -> u8 {
        assert!(m < 16);
        ((self.bits() >> (Self::PERM_SHIFT + m * 4)) & Self::PERM_MASK) as u8
    }

    /// Sets the value of the `Perm<m>` field.
    pub const fn set_perm(&mut self, m: u32, value: u8) {
        assert!(m < 16);
        let offset = Self::PERM_SHIFT + m * 4;
        assert!(value & (Self::PERM_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PERM_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `Perm<m>` field set to the given value.
    pub const fn with_perm(mut self, m: u32, value: u8) -> Self {
        self.set_perm(m, value);
        self
    }
}

/// `PIR_EL1` system register value.
pub type PirEl1 = Pire0El1;

/// `POR_EL1` system register value.
pub type PorEl1 = Pire0El1;

bitflags! {
    /// `RGSR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct RgsrEl1: u64 {
    }
}

impl RgsrEl1 {
    /// Offset of the `TAG` field.
    pub const TAG_SHIFT: u32 = 0;
    /// Mask for the `TAG` field.
    pub const TAG_MASK: u64 = 0b1111;
    /// Offset of the `SEED` field.
    pub const SEED_SHIFT: u32 = 8;
    /// Mask for the `SEED` field.
    pub const SEED_MASK: u64 = 0b1111_1111_1111_1111;

    /// Returns the value of the `TAG` field.
    pub const fn tag(self) -> u8 {
        ((self.bits() >> Self::TAG_SHIFT) & Self::TAG_MASK) as u8
    }

    /// Sets the value of the `TAG` field.
    pub const fn set_tag(&mut self, value: u8) {
        let offset = Self::TAG_SHIFT;
        assert!(value & (Self::TAG_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TAG_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `TAG` field set to the given value.
    pub const fn with_tag(mut self, value: u8) -> Self {
        self.set_tag(value);
        self
    }

    /// Returns the value of the `SEED` field.
    pub const fn seed(self) -> u16 {
        ((self.bits() >> Self::SEED_SHIFT) & Self::SEED_MASK) as u16
    }

    /// Sets the value of the `SEED` field.
    pub const fn set_seed(&mut self, value: u16) {
        let offset = Self::SEED_SHIFT;
        assert!(value & (Self::SEED_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SEED_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `SEED` field set to the given value.
    pub const fn with_seed(mut self, value: u16) -> Self {
        self.set_seed(value);
        self
    }
}

/// `S2POR_EL1` system register value.
pub type S2porEl1 = Pire0El1;

bitflags! {
    /// `SCTLR2_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Sctlr2El1: u64 {
        /// `NMEA` bit.
        const NMEA = 1 << 2;
        /// `EnADERR` bit.
        const ENADERR = 1 << 3;
        /// `EnANERR` bit.
        const ENANERR = 1 << 4;
        /// `EASE` bit.
        const EASE = 1 << 5;
        /// `EnIDCP128` bit.
        const ENIDCP128 = 1 << 6;
        /// `EnPACM` bit.
        const ENPACM = 1 << 7;
        /// `EnPACM0` bit.
        const ENPACM0 = 1 << 8;
        /// `CPTA` bit.
        const CPTA = 1 << 9;
        /// `CPTA0` bit.
        const CPTA0 = 1 << 10;
        /// `CPTM` bit.
        const CPTM = 1 << 11;
        /// `CPTM0` bit.
        const CPTM0 = 1 << 12;
        /// `DTZ` bit.
        const DTZ = 1 << 14;
        /// `TEIS` bit.
        const TEIS = 1 << 15;
        /// `TEOS` bit.
        const TEOS = 1 << 16;
        /// `VT` bit.
        const VT = 1 << 17;
        /// `EnDB2` bit.
        const ENDB2 = 1 << 19;
        /// `EnDA2` bit.
        const ENDA2 = 1 << 20;
        /// `EnIB2` bit.
        const ENIB2 = 1 << 21;
        /// `EnIA2` bit.
        const ENIA2 = 1 << 22;
        /// `BTD0` bit.
        const BTD0 = 1 << 23;
        /// `BTD` bit.
        const BTD = 1 << 24;
        /// `FDIT` bit.
        const FDIT = 1 << 25;
        /// `TLBOSNIS` bit.
        const TLBOSNIS = 1 << 26;
        /// `EnTP3` bit.
        const ENTP3 = 1 << 28;
    }
}

impl Sctlr2El1 {
    /// Offset of the `NMEA` field.
    pub const NMEA_SHIFT: u32 = 2;
    /// Offset of the `EnADERR` field.
    pub const ENADERR_SHIFT: u32 = 3;
    /// Offset of the `EnANERR` field.
    pub const ENANERR_SHIFT: u32 = 4;
    /// Offset of the `EASE` field.
    pub const EASE_SHIFT: u32 = 5;
    /// Offset of the `EnIDCP128` field.
    pub const ENIDCP128_SHIFT: u32 = 6;
    /// Offset of the `EnPACM` field.
    pub const ENPACM_SHIFT: u32 = 7;
    /// Offset of the `EnPACM0` field.
    pub const ENPACM0_SHIFT: u32 = 8;
    /// Offset of the `CPTA` field.
    pub const CPTA_SHIFT: u32 = 9;
    /// Offset of the `CPTA0` field.
    pub const CPTA0_SHIFT: u32 = 10;
    /// Offset of the `CPTM` field.
    pub const CPTM_SHIFT: u32 = 11;
    /// Offset of the `CPTM0` field.
    pub const CPTM0_SHIFT: u32 = 12;
    /// Offset of the `DTZ` field.
    pub const DTZ_SHIFT: u32 = 14;
    /// Offset of the `TEIS` field.
    pub const TEIS_SHIFT: u32 = 15;
    /// Offset of the `TEOS` field.
    pub const TEOS_SHIFT: u32 = 16;
    /// Offset of the `VT` field.
    pub const VT_SHIFT: u32 = 17;
    /// Offset of the `EnDB2` field.
    pub const ENDB2_SHIFT: u32 = 19;
    /// Offset of the `EnDA2` field.
    pub const ENDA2_SHIFT: u32 = 20;
    /// Offset of the `EnIB2` field.
    pub const ENIB2_SHIFT: u32 = 21;
    /// Offset of the `EnIA2` field.
    pub const ENIA2_SHIFT: u32 = 22;
    /// Offset of the `BTD0` field.
    pub const BTD0_SHIFT: u32 = 23;
    /// Offset of the `BTD` field.
    pub const BTD_SHIFT: u32 = 24;
    /// Offset of the `FDIT` field.
    pub const FDIT_SHIFT: u32 = 25;
    /// Offset of the `TLBOSNIS` field.
    pub const TLBOSNIS_SHIFT: u32 = 26;
    /// Offset of the `EnTP3` field.
    pub const ENTP3_SHIFT: u32 = 28;
}

bitflags! {
    /// `SCTLR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct SctlrEl1: u64 {
        /// `M` bit.
        const M = 1 << 0;
        /// `A` bit.
        const A = 1 << 1;
        /// `C` bit.
        const C = 1 << 2;
        /// `SA` bit.
        const SA = 1 << 3;
        /// `SA0` bit.
        const SA0 = 1 << 4;
        /// `CP15BEN` bit.
        const CP15BEN = 1 << 5;
        /// `nAA` bit.
        const NAA = 1 << 6;
        /// `ITD` bit.
        const ITD = 1 << 7;
        /// `SED` bit.
        const SED = 1 << 8;
        /// `UMA` bit.
        const UMA = 1 << 9;
        /// `EnRCTX` bit.
        const ENRCTX = 1 << 10;
        /// `EOS` bit.
        const EOS = 1 << 11;
        /// `I` bit.
        const I = 1 << 12;
        /// `EnDB` bit.
        const ENDB = 1 << 13;
        /// `DZE` bit.
        const DZE = 1 << 14;
        /// `UCT` bit.
        const UCT = 1 << 15;
        /// `nTWI` bit.
        const NTWI = 1 << 16;
        /// `nTWE` bit.
        const NTWE = 1 << 18;
        /// `WXN` bit.
        const WXN = 1 << 19;
        /// `TSCXT` bit.
        const TSCXT = 1 << 20;
        /// `IESB` bit.
        const IESB = 1 << 21;
        /// `EIS` bit.
        const EIS = 1 << 22;
        /// Do not set Privileged Access Never, on taking an exception to EL1.
        const SPAN = 1 << 23;
        /// `UCI` bit.
        const UCI = 1 << 26;
        /// `EnDA` bit.
        const ENDA = 1 << 27;
        /// `nTLSMD` bit.
        const NTLSMD = 1 << 28;
        /// `LSMAOE` bit.
        const LSMAOE = 1 << 29;
        /// Enable pointer authentication using APIBKey_EL1.
        const ENIB = 1 << 30;
        /// Enable pointer authentication using APIAKey_EL1.
        const ENIA = 1 << 31;
        /// `CMOW` bit.
        const CMOW = 1 << 32;
        /// `MSCEn` bit.
        const MSCEN = 1 << 33;
        /// `EnFPM` bit.
        const ENFPM = 1 << 34;
        /// `BT0` bit.
        const BT0 = 1 << 35;
        /// `BT1` bit.
        const BT1 = 1 << 36;
        /// `ITFSB` bit.
        const ITFSB = 1 << 37;
        /// `ATA0` bit.
        const ATA0 = 1 << 42;
        /// `ATA` bit.
        const ATA = 1 << 43;
        /// Default PSTATE.SSBS value on Exception Entry.
        const DSSBS = 1 << 44;
        /// `TWEDEn` bit.
        const TWEDEN = 1 << 45;
        /// `EnASR` bit.
        const ENASR = 1 << 54;
        /// `EnAS0` bit.
        const ENAS0 = 1 << 55;
        /// `EnALS` bit.
        const ENALS = 1 << 56;
        /// `EPAN` bit.
        const EPAN = 1 << 57;
        /// `TCSO0` bit.
        const TCSO0 = 1 << 58;
        /// `TCSO` bit.
        const TCSO = 1 << 59;
        /// `EnTP2` bit.
        const ENTP2 = 1 << 60;
        /// `NMI` bit.
        const NMI = 1 << 61;
        /// SP Interrupt Mask enable.
        const SPINTMASK = 1 << 62;
        /// `TIDCP` bit.
        const TIDCP = 1 << 63;
    }
}

impl SctlrEl1 {
    /// Offset of the `M` field.
    pub const M_SHIFT: u32 = 0;
    /// Offset of the `A` field.
    pub const A_SHIFT: u32 = 1;
    /// Offset of the `C` field.
    pub const C_SHIFT: u32 = 2;
    /// Offset of the `SA` field.
    pub const SA_SHIFT: u32 = 3;
    /// Offset of the `SA0` field.
    pub const SA0_SHIFT: u32 = 4;
    /// Offset of the `CP15BEN` field.
    pub const CP15BEN_SHIFT: u32 = 5;
    /// Offset of the `nAA` field.
    pub const NAA_SHIFT: u32 = 6;
    /// Offset of the `ITD` field.
    pub const ITD_SHIFT: u32 = 7;
    /// Offset of the `SED` field.
    pub const SED_SHIFT: u32 = 8;
    /// Offset of the `UMA` field.
    pub const UMA_SHIFT: u32 = 9;
    /// Offset of the `EnRCTX` field.
    pub const ENRCTX_SHIFT: u32 = 10;
    /// Offset of the `EOS` field.
    pub const EOS_SHIFT: u32 = 11;
    /// Offset of the `I` field.
    pub const I_SHIFT: u32 = 12;
    /// Offset of the `EnDB` field.
    pub const ENDB_SHIFT: u32 = 13;
    /// Offset of the `DZE` field.
    pub const DZE_SHIFT: u32 = 14;
    /// Offset of the `UCT` field.
    pub const UCT_SHIFT: u32 = 15;
    /// Offset of the `nTWI` field.
    pub const NTWI_SHIFT: u32 = 16;
    /// Offset of the `nTWE` field.
    pub const NTWE_SHIFT: u32 = 18;
    /// Offset of the `WXN` field.
    pub const WXN_SHIFT: u32 = 19;
    /// Offset of the `TSCXT` field.
    pub const TSCXT_SHIFT: u32 = 20;
    /// Offset of the `IESB` field.
    pub const IESB_SHIFT: u32 = 21;
    /// Offset of the `EIS` field.
    pub const EIS_SHIFT: u32 = 22;
    /// Offset of the `SPAN` field.
    pub const SPAN_SHIFT: u32 = 23;
    /// Offset of the `UCI` field.
    pub const UCI_SHIFT: u32 = 26;
    /// Offset of the `EnDA` field.
    pub const ENDA_SHIFT: u32 = 27;
    /// Offset of the `nTLSMD` field.
    pub const NTLSMD_SHIFT: u32 = 28;
    /// Offset of the `LSMAOE` field.
    pub const LSMAOE_SHIFT: u32 = 29;
    /// Offset of the `EnIB` field.
    pub const ENIB_SHIFT: u32 = 30;
    /// Offset of the `EnIA` field.
    pub const ENIA_SHIFT: u32 = 31;
    /// Offset of the `CMOW` field.
    pub const CMOW_SHIFT: u32 = 32;
    /// Offset of the `MSCEn` field.
    pub const MSCEN_SHIFT: u32 = 33;
    /// Offset of the `EnFPM` field.
    pub const ENFPM_SHIFT: u32 = 34;
    /// Offset of the `BT0` field.
    pub const BT0_SHIFT: u32 = 35;
    /// Offset of the `BT1` field.
    pub const BT1_SHIFT: u32 = 36;
    /// Offset of the `ITFSB` field.
    pub const ITFSB_SHIFT: u32 = 37;
    /// Offset of the `TCF0` field.
    pub const TCF0_SHIFT: u32 = 38;
    /// Mask for the `TCF0` field.
    pub const TCF0_MASK: u64 = 0b11;
    /// Offset of the `TCF` field.
    pub const TCF_SHIFT: u32 = 40;
    /// Mask for the `TCF` field.
    pub const TCF_MASK: u64 = 0b11;
    /// Offset of the `ATA0` field.
    pub const ATA0_SHIFT: u32 = 42;
    /// Offset of the `ATA` field.
    pub const ATA_SHIFT: u32 = 43;
    /// Offset of the `DSSBS` field.
    pub const DSSBS_SHIFT: u32 = 44;
    /// Offset of the `TWEDEn` field.
    pub const TWEDEN_SHIFT: u32 = 45;
    /// Offset of the `TWEDEL` field.
    pub const TWEDEL_SHIFT: u32 = 46;
    /// Mask for the `TWEDEL` field.
    pub const TWEDEL_MASK: u64 = 0b1111;
    /// Offset of the `EnASR` field.
    pub const ENASR_SHIFT: u32 = 54;
    /// Offset of the `EnAS0` field.
    pub const ENAS0_SHIFT: u32 = 55;
    /// Offset of the `EnALS` field.
    pub const ENALS_SHIFT: u32 = 56;
    /// Offset of the `EPAN` field.
    pub const EPAN_SHIFT: u32 = 57;
    /// Offset of the `TCSO0` field.
    pub const TCSO0_SHIFT: u32 = 58;
    /// Offset of the `TCSO` field.
    pub const TCSO_SHIFT: u32 = 59;
    /// Offset of the `EnTP2` field.
    pub const ENTP2_SHIFT: u32 = 60;
    /// Offset of the `NMI` field.
    pub const NMI_SHIFT: u32 = 61;
    /// Offset of the `SPINTMASK` field.
    pub const SPINTMASK_SHIFT: u32 = 62;
    /// Offset of the `TIDCP` field.
    pub const TIDCP_SHIFT: u32 = 63;

    /// Returns the value of the `TCF0` field.
    pub const fn tcf0(self) -> u8 {
        ((self.bits() >> Self::TCF0_SHIFT) & Self::TCF0_MASK) as u8
    }

    /// Sets the value of the `TCF0` field.
    pub const fn set_tcf0(&mut self, value: u8) {
        let offset = Self::TCF0_SHIFT;
        assert!(value & (Self::TCF0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TCF0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `TCF0` field set to the given value.
    pub const fn with_tcf0(mut self, value: u8) -> Self {
        self.set_tcf0(value);
        self
    }

    /// Returns the value of the `TCF` field.
    pub const fn tcf(self) -> u8 {
        ((self.bits() >> Self::TCF_SHIFT) & Self::TCF_MASK) as u8
    }

    /// Sets the value of the `TCF` field.
    pub const fn set_tcf(&mut self, value: u8) {
        let offset = Self::TCF_SHIFT;
        assert!(value & (Self::TCF_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TCF_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `TCF` field set to the given value.
    pub const fn with_tcf(mut self, value: u8) -> Self {
        self.set_tcf(value);
        self
    }

    /// Returns the value of the `TWEDEL` field.
    pub const fn twedel(self) -> u8 {
        ((self.bits() >> Self::TWEDEL_SHIFT) & Self::TWEDEL_MASK) as u8
    }

    /// Sets the value of the `TWEDEL` field.
    pub const fn set_twedel(&mut self, value: u8) {
        let offset = Self::TWEDEL_SHIFT;
        assert!(value & (Self::TWEDEL_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TWEDEL_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `TWEDEL` field set to the given value.
    pub const fn with_twedel(mut self, value: u8) -> Self {
        self.set_twedel(value);
        self
    }
}

bitflags! {
    /// `SPSR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct SpsrEl1: u64 {
        /// `M[4]` bit.
        const M_4 = 1 << 4;
        /// `T` bit.
        const T = 1 << 5;
        /// `F` bit.
        const F = 1 << 6;
        /// `I` bit.
        const I = 1 << 7;
        /// `A` bit.
        const A = 1 << 8;
        /// `D` bit.
        const D = 1 << 9;
        /// `E` bit.
        const E = 1 << 9;
        /// `ALLINT` bit.
        const ALLINT = 1 << 13;
        /// `BTYPE2` bit.
        const BTYPE2 = 1 << 14;
        /// `IL` bit.
        const IL = 1 << 20;
        /// `SS` bit.
        const SS = 1 << 21;
        /// `PAN` bit.
        const PAN = 1 << 22;
        /// `UAO` bit.
        const UAO = 1 << 23;
        /// `DIT` bit.
        const DIT = 1 << 24;
        /// `TCO` bit.
        const TCO = 1 << 25;
        /// `Q` bit.
        const Q = 1 << 27;
        /// `V` bit.
        const V = 1 << 28;
        /// `C` bit.
        const C = 1 << 29;
        /// `Z` bit.
        const Z = 1 << 30;
        /// `N` bit.
        const N = 1 << 31;
        /// `PM` bit.
        const PM = 1 << 32;
        /// `EXLOCK` bit.
        const EXLOCK = 1 << 34;
        /// `PACM` bit.
        const PACM = 1 << 35;
        /// `UINJ` bit.
        const UINJ = 1 << 36;
    }
}

impl SpsrEl1 {
    /// Offset of the `M[3:0]` field.
    pub const M_3_0_SHIFT: u32 = 0;
    /// Mask for the `M[3:0]` field.
    pub const M_3_0_MASK: u64 = 0b1111;
    /// Offset of the `M[4]` field.
    pub const M_4_SHIFT: u32 = 4;
    /// Offset of the `T` field.
    pub const T_SHIFT: u32 = 5;
    /// Offset of the `F` field.
    pub const F_SHIFT: u32 = 6;
    /// Offset of the `I` field.
    pub const I_SHIFT: u32 = 7;
    /// Offset of the `A` field.
    pub const A_SHIFT: u32 = 8;
    /// Offset of the `D` field.
    pub const D_SHIFT: u32 = 9;
    /// Offset of the `E` field.
    pub const E_SHIFT: u32 = 9;
    /// Offset of the `BTYPE` field.
    pub const BTYPE_SHIFT: u32 = 10;
    /// Mask for the `BTYPE` field.
    pub const BTYPE_MASK: u64 = 0b11;
    /// Offset of the `ALLINT` field.
    pub const ALLINT_SHIFT: u32 = 13;
    /// Offset of the `BTYPE2` field.
    pub const BTYPE2_SHIFT: u32 = 14;
    /// Offset of the `GE` field.
    pub const GE_SHIFT: u32 = 16;
    /// Mask for the `GE` field.
    pub const GE_MASK: u64 = 0b1111;
    /// Offset of the `IL` field.
    pub const IL_SHIFT: u32 = 20;
    /// Offset of the `SS` field.
    pub const SS_SHIFT: u32 = 21;
    /// Offset of the `PAN` field.
    pub const PAN_SHIFT: u32 = 22;
    /// Offset of the `UAO` field.
    pub const UAO_SHIFT: u32 = 23;
    /// Offset of the `DIT` field.
    pub const DIT_SHIFT: u32 = 24;
    /// Offset of the `TCO` field.
    pub const TCO_SHIFT: u32 = 25;
    /// Offset of the `Q` field.
    pub const Q_SHIFT: u32 = 27;
    /// Offset of the `V` field.
    pub const V_SHIFT: u32 = 28;
    /// Offset of the `C` field.
    pub const C_SHIFT: u32 = 29;
    /// Offset of the `Z` field.
    pub const Z_SHIFT: u32 = 30;
    /// Offset of the `N` field.
    pub const N_SHIFT: u32 = 31;
    /// Offset of the `PM` field.
    pub const PM_SHIFT: u32 = 32;
    /// Offset of the `EXLOCK` field.
    pub const EXLOCK_SHIFT: u32 = 34;
    /// Offset of the `PACM` field.
    pub const PACM_SHIFT: u32 = 35;
    /// Offset of the `UINJ` field.
    pub const UINJ_SHIFT: u32 = 36;

    /// Returns the value of the `M[3:0]` field.
    pub const fn m_3_0(self) -> u8 {
        ((self.bits() >> Self::M_3_0_SHIFT) & Self::M_3_0_MASK) as u8
    }

    /// Sets the value of the `M[3:0]` field.
    pub const fn set_m_3_0(&mut self, value: u8) {
        let offset = Self::M_3_0_SHIFT;
        assert!(value & (Self::M_3_0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::M_3_0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `M[3:0]` field set to the given value.
    pub const fn with_m_3_0(mut self, value: u8) -> Self {
        self.set_m_3_0(value);
        self
    }

    /// Returns the value of the `BTYPE` field.
    pub const fn btype(self) -> u8 {
        ((self.bits() >> Self::BTYPE_SHIFT) & Self::BTYPE_MASK) as u8
    }

    /// Sets the value of the `BTYPE` field.
    pub const fn set_btype(&mut self, value: u8) {
        let offset = Self::BTYPE_SHIFT;
        assert!(value & (Self::BTYPE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BTYPE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `BTYPE` field set to the given value.
    pub const fn with_btype(mut self, value: u8) -> Self {
        self.set_btype(value);
        self
    }

    /// Returns the value of the `GE` field.
    pub const fn ge(self) -> u8 {
        ((self.bits() >> Self::GE_SHIFT) & Self::GE_MASK) as u8
    }

    /// Sets the value of the `GE` field.
    pub const fn set_ge(&mut self, value: u8) {
        let offset = Self::GE_SHIFT;
        assert!(value & (Self::GE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::GE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `GE` field set to the given value.
    pub const fn with_ge(mut self, value: u8) -> Self {
        self.set_ge(value);
        self
    }
}

bitflags! {
    /// `SP_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct SpEl1: u64 {
    }
}

impl SpEl1 {
    /// Offset of the `StackPointer` field.
    pub const STACKPOINTER_SHIFT: u32 = 0;
    /// Mask for the `StackPointer` field.
    pub const STACKPOINTER_MASK: u64 =
        0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `StackPointer` field.
    pub const fn stackpointer(self) -> u64 {
        (self.bits() >> Self::STACKPOINTER_SHIFT) & Self::STACKPOINTER_MASK
    }

    /// Sets the value of the `StackPointer` field.
    pub const fn set_stackpointer(&mut self, value: u64) {
        let offset = Self::STACKPOINTER_SHIFT;
        assert!(value & Self::STACKPOINTER_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::STACKPOINTER_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `StackPointer` field set to the given value.
    pub const fn with_stackpointer(mut self, value: u64) -> Self {
        self.set_stackpointer(value);
        self
    }
}

bitflags! {
    /// `TCR2_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Tcr2El1: u64 {
        /// `PnCH` bit.
        const PNCH = 1 << 0;
        /// `PIE` bit.
        const PIE = 1 << 1;
        /// `E0POE` bit.
        const E0POE = 1 << 2;
        /// `POE` bit.
        const POE = 1 << 3;
        /// `AIE` bit.
        const AIE = 1 << 4;
        /// `D128` bit.
        const D128 = 1 << 5;
        /// `PTTWI` bit.
        const PTTWI = 1 << 10;
        /// `HAFT` bit.
        const HAFT = 1 << 11;
        /// `DisCH0` bit.
        const DISCH0 = 1 << 14;
        /// `DisCH1` bit.
        const DISCH1 = 1 << 15;
        /// `A2` bit.
        const A2 = 1 << 16;
        /// `FNG0` bit.
        const FNG0 = 1 << 17;
        /// `FNG1` bit.
        const FNG1 = 1 << 18;
        /// `POE2F` bit.
        const POE2F = 1 << 19;
        /// `FNGNA0` bit.
        const FNGNA0 = 1 << 20;
        /// `FNGNA1` bit.
        const FNGNA1 = 1 << 21;
        /// `TVAD0` bit.
        const TVAD0 = 1 << 35;
        /// `TVAD1` bit.
        const TVAD1 = 1 << 36;
    }
}

impl Tcr2El1 {
    /// Offset of the `PnCH` field.
    pub const PNCH_SHIFT: u32 = 0;
    /// Offset of the `PIE` field.
    pub const PIE_SHIFT: u32 = 1;
    /// Offset of the `E0POE` field.
    pub const E0POE_SHIFT: u32 = 2;
    /// Offset of the `POE` field.
    pub const POE_SHIFT: u32 = 3;
    /// Offset of the `AIE` field.
    pub const AIE_SHIFT: u32 = 4;
    /// Offset of the `D128` field.
    pub const D128_SHIFT: u32 = 5;
    /// Offset of the `PTTWI` field.
    pub const PTTWI_SHIFT: u32 = 10;
    /// Offset of the `HAFT` field.
    pub const HAFT_SHIFT: u32 = 11;
    /// Offset of the `DisCH0` field.
    pub const DISCH0_SHIFT: u32 = 14;
    /// Offset of the `DisCH1` field.
    pub const DISCH1_SHIFT: u32 = 15;
    /// Offset of the `A2` field.
    pub const A2_SHIFT: u32 = 16;
    /// Offset of the `FNG0` field.
    pub const FNG0_SHIFT: u32 = 17;
    /// Offset of the `FNG1` field.
    pub const FNG1_SHIFT: u32 = 18;
    /// Offset of the `POE2F` field.
    pub const POE2F_SHIFT: u32 = 19;
    /// Offset of the `FNGNA0` field.
    pub const FNGNA0_SHIFT: u32 = 20;
    /// Offset of the `FNGNA1` field.
    pub const FNGNA1_SHIFT: u32 = 21;
    /// Offset of the `POIW` field.
    pub const POIW_SHIFT: u32 = 22;
    /// Mask for the `POIW` field.
    pub const POIW_MASK: u64 = 0b111;
    /// Offset of the `VTB0` field.
    pub const VTB0_SHIFT: u32 = 25;
    /// Mask for the `VTB0` field.
    pub const VTB0_MASK: u64 = 0b1_1111;
    /// Offset of the `VTB1` field.
    pub const VTB1_SHIFT: u32 = 30;
    /// Mask for the `VTB1` field.
    pub const VTB1_MASK: u64 = 0b1_1111;
    /// Offset of the `TVAD0` field.
    pub const TVAD0_SHIFT: u32 = 35;
    /// Offset of the `TVAD1` field.
    pub const TVAD1_SHIFT: u32 = 36;

    /// Returns the value of the `POIW` field.
    pub const fn poiw(self) -> u8 {
        ((self.bits() >> Self::POIW_SHIFT) & Self::POIW_MASK) as u8
    }

    /// Sets the value of the `POIW` field.
    pub const fn set_poiw(&mut self, value: u8) {
        let offset = Self::POIW_SHIFT;
        assert!(value & (Self::POIW_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::POIW_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `POIW` field set to the given value.
    pub const fn with_poiw(mut self, value: u8) -> Self {
        self.set_poiw(value);
        self
    }

    /// Returns the value of the `VTB0` field.
    pub const fn vtb0(self) -> u8 {
        ((self.bits() >> Self::VTB0_SHIFT) & Self::VTB0_MASK) as u8
    }

    /// Sets the value of the `VTB0` field.
    pub const fn set_vtb0(&mut self, value: u8) {
        let offset = Self::VTB0_SHIFT;
        assert!(value & (Self::VTB0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VTB0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `VTB0` field set to the given value.
    pub const fn with_vtb0(mut self, value: u8) -> Self {
        self.set_vtb0(value);
        self
    }

    /// Returns the value of the `VTB1` field.
    pub const fn vtb1(self) -> u8 {
        ((self.bits() >> Self::VTB1_SHIFT) & Self::VTB1_MASK) as u8
    }

    /// Sets the value of the `VTB1` field.
    pub const fn set_vtb1(&mut self, value: u8) {
        let offset = Self::VTB1_SHIFT;
        assert!(value & (Self::VTB1_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VTB1_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `VTB1` field set to the given value.
    pub const fn with_vtb1(mut self, value: u8) -> Self {
        self.set_vtb1(value);
        self
    }
}

bitflags! {
    /// `TCR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct TcrEl1: u64 {
        /// `EPD0` bit.
        const EPD0 = 1 << 7;
        /// `A1` bit.
        const A1 = 1 << 22;
        /// `EPD1` bit.
        const EPD1 = 1 << 23;
        /// `AS` bit.
        const AS = 1 << 36;
        /// `TBI0` bit.
        const TBI0 = 1 << 37;
        /// `TBI1` bit.
        const TBI1 = 1 << 38;
        /// `HA` bit.
        const HA = 1 << 39;
        /// `HD` bit.
        const HD = 1 << 40;
        /// `HPD0` bit.
        const HPD0 = 1 << 41;
        /// `HPD1` bit.
        const HPD1 = 1 << 42;
        /// `HWU059` bit.
        const HWU059 = 1 << 43;
        /// `HWU060` bit.
        const HWU060 = 1 << 44;
        /// `HWU061` bit.
        const HWU061 = 1 << 45;
        /// `HWU062` bit.
        const HWU062 = 1 << 46;
        /// `HWU159` bit.
        const HWU159 = 1 << 47;
        /// `HWU160` bit.
        const HWU160 = 1 << 48;
        /// `HWU161` bit.
        const HWU161 = 1 << 49;
        /// `HWU162` bit.
        const HWU162 = 1 << 50;
        /// `TBID0` bit.
        const TBID0 = 1 << 51;
        /// `TBID1` bit.
        const TBID1 = 1 << 52;
        /// `NFD0` bit.
        const NFD0 = 1 << 53;
        /// `NFD1` bit.
        const NFD1 = 1 << 54;
        /// `E0PD0` bit.
        const E0PD0 = 1 << 55;
        /// `E0PD1` bit.
        const E0PD1 = 1 << 56;
        /// `TCMA0` bit.
        const TCMA0 = 1 << 57;
        /// `TCMA1` bit.
        const TCMA1 = 1 << 58;
        /// `DS` bit.
        const DS = 1 << 59;
        /// `MTX0` bit.
        const MTX0 = 1 << 60;
        /// `MTX1` bit.
        const MTX1 = 1 << 61;
    }
}

impl TcrEl1 {
    /// Offset of the `T0SZ` field.
    pub const T0SZ_SHIFT: u32 = 0;
    /// Mask for the `T0SZ` field.
    pub const T0SZ_MASK: u64 = 0b11_1111;
    /// Offset of the `EPD0` field.
    pub const EPD0_SHIFT: u32 = 7;
    /// Offset of the `IRGN0` field.
    pub const IRGN0_SHIFT: u32 = 8;
    /// Mask for the `IRGN0` field.
    pub const IRGN0_MASK: u64 = 0b11;
    /// Offset of the `ORGN0` field.
    pub const ORGN0_SHIFT: u32 = 10;
    /// Mask for the `ORGN0` field.
    pub const ORGN0_MASK: u64 = 0b11;
    /// Offset of the `SH0` field.
    pub const SH0_SHIFT: u32 = 12;
    /// Mask for the `SH0` field.
    pub const SH0_MASK: u64 = 0b11;
    /// Offset of the `TG0` field.
    pub const TG0_SHIFT: u32 = 14;
    /// Mask for the `TG0` field.
    pub const TG0_MASK: u64 = 0b11;
    /// Offset of the `T1SZ` field.
    pub const T1SZ_SHIFT: u32 = 16;
    /// Mask for the `T1SZ` field.
    pub const T1SZ_MASK: u64 = 0b11_1111;
    /// Offset of the `A1` field.
    pub const A1_SHIFT: u32 = 22;
    /// Offset of the `EPD1` field.
    pub const EPD1_SHIFT: u32 = 23;
    /// Offset of the `IRGN1` field.
    pub const IRGN1_SHIFT: u32 = 24;
    /// Mask for the `IRGN1` field.
    pub const IRGN1_MASK: u64 = 0b11;
    /// Offset of the `ORGN1` field.
    pub const ORGN1_SHIFT: u32 = 26;
    /// Mask for the `ORGN1` field.
    pub const ORGN1_MASK: u64 = 0b11;
    /// Offset of the `SH1` field.
    pub const SH1_SHIFT: u32 = 28;
    /// Mask for the `SH1` field.
    pub const SH1_MASK: u64 = 0b11;
    /// Offset of the `TG1` field.
    pub const TG1_SHIFT: u32 = 30;
    /// Mask for the `TG1` field.
    pub const TG1_MASK: u64 = 0b11;
    /// Offset of the `IPS` field.
    pub const IPS_SHIFT: u32 = 32;
    /// Mask for the `IPS` field.
    pub const IPS_MASK: u64 = 0b111;
    /// Offset of the `AS` field.
    pub const AS_SHIFT: u32 = 36;
    /// Offset of the `TBI0` field.
    pub const TBI0_SHIFT: u32 = 37;
    /// Offset of the `TBI1` field.
    pub const TBI1_SHIFT: u32 = 38;
    /// Offset of the `HA` field.
    pub const HA_SHIFT: u32 = 39;
    /// Offset of the `HD` field.
    pub const HD_SHIFT: u32 = 40;
    /// Offset of the `HPD0` field.
    pub const HPD0_SHIFT: u32 = 41;
    /// Offset of the `HPD1` field.
    pub const HPD1_SHIFT: u32 = 42;
    /// Offset of the `HWU059` field.
    pub const HWU059_SHIFT: u32 = 43;
    /// Offset of the `HWU060` field.
    pub const HWU060_SHIFT: u32 = 44;
    /// Offset of the `HWU061` field.
    pub const HWU061_SHIFT: u32 = 45;
    /// Offset of the `HWU062` field.
    pub const HWU062_SHIFT: u32 = 46;
    /// Offset of the `HWU159` field.
    pub const HWU159_SHIFT: u32 = 47;
    /// Offset of the `HWU160` field.
    pub const HWU160_SHIFT: u32 = 48;
    /// Offset of the `HWU161` field.
    pub const HWU161_SHIFT: u32 = 49;
    /// Offset of the `HWU162` field.
    pub const HWU162_SHIFT: u32 = 50;
    /// Offset of the `TBID0` field.
    pub const TBID0_SHIFT: u32 = 51;
    /// Offset of the `TBID1` field.
    pub const TBID1_SHIFT: u32 = 52;
    /// Offset of the `NFD0` field.
    pub const NFD0_SHIFT: u32 = 53;
    /// Offset of the `NFD1` field.
    pub const NFD1_SHIFT: u32 = 54;
    /// Offset of the `E0PD0` field.
    pub const E0PD0_SHIFT: u32 = 55;
    /// Offset of the `E0PD1` field.
    pub const E0PD1_SHIFT: u32 = 56;
    /// Offset of the `TCMA0` field.
    pub const TCMA0_SHIFT: u32 = 57;
    /// Offset of the `TCMA1` field.
    pub const TCMA1_SHIFT: u32 = 58;
    /// Offset of the `DS` field.
    pub const DS_SHIFT: u32 = 59;
    /// Offset of the `MTX0` field.
    pub const MTX0_SHIFT: u32 = 60;
    /// Offset of the `MTX1` field.
    pub const MTX1_SHIFT: u32 = 61;

    /// Returns the value of the `T0SZ` field.
    pub const fn t0sz(self) -> u8 {
        ((self.bits() >> Self::T0SZ_SHIFT) & Self::T0SZ_MASK) as u8
    }

    /// Sets the value of the `T0SZ` field.
    pub const fn set_t0sz(&mut self, value: u8) {
        let offset = Self::T0SZ_SHIFT;
        assert!(value & (Self::T0SZ_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::T0SZ_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `T0SZ` field set to the given value.
    pub const fn with_t0sz(mut self, value: u8) -> Self {
        self.set_t0sz(value);
        self
    }

    /// Returns the value of the `IRGN0` field.
    pub const fn irgn0(self) -> u8 {
        ((self.bits() >> Self::IRGN0_SHIFT) & Self::IRGN0_MASK) as u8
    }

    /// Sets the value of the `IRGN0` field.
    pub const fn set_irgn0(&mut self, value: u8) {
        let offset = Self::IRGN0_SHIFT;
        assert!(value & (Self::IRGN0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::IRGN0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `IRGN0` field set to the given value.
    pub const fn with_irgn0(mut self, value: u8) -> Self {
        self.set_irgn0(value);
        self
    }

    /// Returns the value of the `ORGN0` field.
    pub const fn orgn0(self) -> u8 {
        ((self.bits() >> Self::ORGN0_SHIFT) & Self::ORGN0_MASK) as u8
    }

    /// Sets the value of the `ORGN0` field.
    pub const fn set_orgn0(&mut self, value: u8) {
        let offset = Self::ORGN0_SHIFT;
        assert!(value & (Self::ORGN0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ORGN0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `ORGN0` field set to the given value.
    pub const fn with_orgn0(mut self, value: u8) -> Self {
        self.set_orgn0(value);
        self
    }

    /// Returns the value of the `SH0` field.
    pub const fn sh0(self) -> u8 {
        ((self.bits() >> Self::SH0_SHIFT) & Self::SH0_MASK) as u8
    }

    /// Sets the value of the `SH0` field.
    pub const fn set_sh0(&mut self, value: u8) {
        let offset = Self::SH0_SHIFT;
        assert!(value & (Self::SH0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SH0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `SH0` field set to the given value.
    pub const fn with_sh0(mut self, value: u8) -> Self {
        self.set_sh0(value);
        self
    }

    /// Returns the value of the `TG0` field.
    pub const fn tg0(self) -> u8 {
        ((self.bits() >> Self::TG0_SHIFT) & Self::TG0_MASK) as u8
    }

    /// Sets the value of the `TG0` field.
    pub const fn set_tg0(&mut self, value: u8) {
        let offset = Self::TG0_SHIFT;
        assert!(value & (Self::TG0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TG0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `TG0` field set to the given value.
    pub const fn with_tg0(mut self, value: u8) -> Self {
        self.set_tg0(value);
        self
    }

    /// Returns the value of the `T1SZ` field.
    pub const fn t1sz(self) -> u8 {
        ((self.bits() >> Self::T1SZ_SHIFT) & Self::T1SZ_MASK) as u8
    }

    /// Sets the value of the `T1SZ` field.
    pub const fn set_t1sz(&mut self, value: u8) {
        let offset = Self::T1SZ_SHIFT;
        assert!(value & (Self::T1SZ_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::T1SZ_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `T1SZ` field set to the given value.
    pub const fn with_t1sz(mut self, value: u8) -> Self {
        self.set_t1sz(value);
        self
    }

    /// Returns the value of the `IRGN1` field.
    pub const fn irgn1(self) -> u8 {
        ((self.bits() >> Self::IRGN1_SHIFT) & Self::IRGN1_MASK) as u8
    }

    /// Sets the value of the `IRGN1` field.
    pub const fn set_irgn1(&mut self, value: u8) {
        let offset = Self::IRGN1_SHIFT;
        assert!(value & (Self::IRGN1_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::IRGN1_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `IRGN1` field set to the given value.
    pub const fn with_irgn1(mut self, value: u8) -> Self {
        self.set_irgn1(value);
        self
    }

    /// Returns the value of the `ORGN1` field.
    pub const fn orgn1(self) -> u8 {
        ((self.bits() >> Self::ORGN1_SHIFT) & Self::ORGN1_MASK) as u8
    }

    /// Sets the value of the `ORGN1` field.
    pub const fn set_orgn1(&mut self, value: u8) {
        let offset = Self::ORGN1_SHIFT;
        assert!(value & (Self::ORGN1_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ORGN1_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `ORGN1` field set to the given value.
    pub const fn with_orgn1(mut self, value: u8) -> Self {
        self.set_orgn1(value);
        self
    }

    /// Returns the value of the `SH1` field.
    pub const fn sh1(self) -> u8 {
        ((self.bits() >> Self::SH1_SHIFT) & Self::SH1_MASK) as u8
    }

    /// Sets the value of the `SH1` field.
    pub const fn set_sh1(&mut self, value: u8) {
        let offset = Self::SH1_SHIFT;
        assert!(value & (Self::SH1_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SH1_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `SH1` field set to the given value.
    pub const fn with_sh1(mut self, value: u8) -> Self {
        self.set_sh1(value);
        self
    }

    /// Returns the value of the `TG1` field.
    pub const fn tg1(self) -> u8 {
        ((self.bits() >> Self::TG1_SHIFT) & Self::TG1_MASK) as u8
    }

    /// Sets the value of the `TG1` field.
    pub const fn set_tg1(&mut self, value: u8) {
        let offset = Self::TG1_SHIFT;
        assert!(value & (Self::TG1_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TG1_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `TG1` field set to the given value.
    pub const fn with_tg1(mut self, value: u8) -> Self {
        self.set_tg1(value);
        self
    }

    /// Returns the value of the `IPS` field.
    pub const fn ips(self) -> u8 {
        ((self.bits() >> Self::IPS_SHIFT) & Self::IPS_MASK) as u8
    }

    /// Sets the value of the `IPS` field.
    pub const fn set_ips(&mut self, value: u8) {
        let offset = Self::IPS_SHIFT;
        assert!(value & (Self::IPS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::IPS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `IPS` field set to the given value.
    pub const fn with_ips(mut self, value: u8) -> Self {
        self.set_ips(value);
        self
    }
}

bitflags! {
    /// `TFSRE0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Tfsre0El1: u64 {
        /// `TF0` bit.
        const TF0 = 1 << 0;
        /// `TF1` bit.
        const TF1 = 1 << 1;
    }
}

impl Tfsre0El1 {
    /// Offset of the `TF0` field.
    pub const TF0_SHIFT: u32 = 0;
    /// Offset of the `TF1` field.
    pub const TF1_SHIFT: u32 = 1;
}

/// `TFSR_EL1` system register value.
pub type TfsrEl1 = Tfsre0El1;

bitflags! {
    /// `TPIDR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct TpidrEl1: u64 {
    }
}

impl TpidrEl1 {
    /// Offset of the `ThreadID` field.
    pub const THREADID_SHIFT: u32 = 0;
    /// Mask for the `ThreadID` field.
    pub const THREADID_MASK: u64 =
        0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `ThreadID` field.
    pub const fn threadid(self) -> u64 {
        (self.bits() >> Self::THREADID_SHIFT) & Self::THREADID_MASK
    }

    /// Sets the value of the `ThreadID` field.
    pub const fn set_threadid(&mut self, value: u64) {
        let offset = Self::THREADID_SHIFT;
        assert!(value & Self::THREADID_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::THREADID_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `ThreadID` field set to the given value.
    pub const fn with_threadid(mut self, value: u64) -> Self {
        self.set_threadid(value);
        self
    }
}

bitflags! {
    /// `TTBR0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Ttbr0El1: u64 {
        /// `CnP` bit.
        const CNP = 1 << 0;
    }
}

impl Ttbr0El1 {
    /// Offset of the `CnP` field.
    pub const CNP_SHIFT: u32 = 0;
    /// Offset of the `BADDR[47:1]` field.
    pub const BADDR_47_1_SHIFT: u32 = 1;
    /// Mask for the `BADDR[47:1]` field.
    pub const BADDR_47_1_MASK: u64 = 0b111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;
    /// Offset of the `SKL` field.
    pub const SKL_SHIFT: u32 = 1;
    /// Mask for the `SKL` field.
    pub const SKL_MASK: u64 = 0b11;
    /// Offset of the `ASID` field.
    pub const ASID_SHIFT: u32 = 48;
    /// Mask for the `ASID` field.
    pub const ASID_MASK: u64 = 0b1111_1111_1111_1111;

    /// Returns the value of the `BADDR[47:1]` field.
    pub const fn baddr_47_1(self) -> u64 {
        (self.bits() >> Self::BADDR_47_1_SHIFT) & Self::BADDR_47_1_MASK
    }

    /// Sets the value of the `BADDR[47:1]` field.
    pub const fn set_baddr_47_1(&mut self, value: u64) {
        let offset = Self::BADDR_47_1_SHIFT;
        assert!(value & Self::BADDR_47_1_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BADDR_47_1_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `BADDR[47:1]` field set to the given value.
    pub const fn with_baddr_47_1(mut self, value: u64) -> Self {
        self.set_baddr_47_1(value);
        self
    }

    /// Returns the value of the `SKL` field.
    pub const fn skl(self) -> u8 {
        ((self.bits() >> Self::SKL_SHIFT) & Self::SKL_MASK) as u8
    }

    /// Sets the value of the `SKL` field.
    pub const fn set_skl(&mut self, value: u8) {
        let offset = Self::SKL_SHIFT;
        assert!(value & (Self::SKL_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SKL_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `SKL` field set to the given value.
    pub const fn with_skl(mut self, value: u8) -> Self {
        self.set_skl(value);
        self
    }

    /// Returns the value of the `ASID` field.
    pub const fn asid(self) -> u16 {
        ((self.bits() >> Self::ASID_SHIFT) & Self::ASID_MASK) as u16
    }

    /// Sets the value of the `ASID` field.
    pub const fn set_asid(&mut self, value: u16) {
        let offset = Self::ASID_SHIFT;
        assert!(value & (Self::ASID_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ASID_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `ASID` field set to the given value.
    pub const fn with_asid(mut self, value: u16) -> Self {
        self.set_asid(value);
        self
    }
}

/// `TTBR1_EL1` system register value.
pub type Ttbr1El1 = Ttbr0El1;

bitflags! {
    /// `VBAR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct VbarEl1: u64 {
        /// `UT` bit.
        const UT = 1 << 0;
    }
}

impl VbarEl1 {
    /// Offset of the `UT` field.
    pub const UT_SHIFT: u32 = 0;
    /// Offset of the `VBA` field.
    pub const VBA_SHIFT: u32 = 11;
    /// Mask for the `VBA` field.
    pub const VBA_MASK: u64 = 0b1_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `VBA` field.
    pub const fn vba(self) -> u64 {
        (self.bits() >> Self::VBA_SHIFT) & Self::VBA_MASK
    }

    /// Sets the value of the `VBA` field.
    pub const fn set_vba(&mut self, value: u64) {
        let offset = Self::VBA_SHIFT;
        assert!(value & Self::VBA_MASK == value);
        *self =
            Self::from_bits_retain((self.bits() & !(Self::VBA_MASK << offset)) | (value << offset));
    }

    /// Returns a copy with the `VBA` field set to the given value.
    pub const fn with_vba(mut self, value: u64) -> Self {
        self.set_vba(value);
        self
    }
}
