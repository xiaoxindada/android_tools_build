// SPDX-FileCopyrightText: Copyright The arm-sysregs Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Arm CPU system registers.

// This file is generated, do not edit manually.

use bitflags::bitflags;

bitflags! {
    /// `AMCFGR_EL0` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct AmcfgrEl0: u64 {
        /// `HDBG` bit.
        const HDBG = 1 << 24;
    }
}

impl AmcfgrEl0 {
    /// Offset of the `N` field.
    pub const N_SHIFT: u32 = 0;
    /// Mask for the `N` field.
    pub const N_MASK: u64 = 0b1111_1111;
    /// Offset of the `SIZE` field.
    pub const SIZE_SHIFT: u32 = 8;
    /// Mask for the `SIZE` field.
    pub const SIZE_MASK: u64 = 0b11_1111;
    /// Offset of the `HDBG` field.
    pub const HDBG_SHIFT: u32 = 24;
    /// Offset of the `NCG` field.
    pub const NCG_SHIFT: u32 = 28;
    /// Mask for the `NCG` field.
    pub const NCG_MASK: u64 = 0b1111;

    /// Returns the value of the `N` field.
    pub const fn n(self) -> u8 {
        ((self.bits() >> Self::N_SHIFT) & Self::N_MASK) as u8
    }

    /// Sets the value of the `N` field.
    pub const fn set_n(&mut self, value: u8) {
        let offset = Self::N_SHIFT;
        assert!(value & (Self::N_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::N_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `N` field set to the given value.
    pub const fn with_n(mut self, value: u8) -> Self {
        self.set_n(value);
        self
    }

    /// Returns the value of the `SIZE` field.
    pub const fn size(self) -> u8 {
        ((self.bits() >> Self::SIZE_SHIFT) & Self::SIZE_MASK) as u8
    }

    /// Sets the value of the `SIZE` field.
    pub const fn set_size(&mut self, value: u8) {
        let offset = Self::SIZE_SHIFT;
        assert!(value & (Self::SIZE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SIZE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `SIZE` field set to the given value.
    pub const fn with_size(mut self, value: u8) -> Self {
        self.set_size(value);
        self
    }

    /// Returns the value of the `NCG` field.
    pub const fn ncg(self) -> u8 {
        ((self.bits() >> Self::NCG_SHIFT) & Self::NCG_MASK) as u8
    }

    /// Sets the value of the `NCG` field.
    pub const fn set_ncg(&mut self, value: u8) {
        let offset = Self::NCG_SHIFT;
        assert!(value & (Self::NCG_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::NCG_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `NCG` field set to the given value.
    pub const fn with_ncg(mut self, value: u8) -> Self {
        self.set_ncg(value);
        self
    }
}

bitflags! {
    /// `AMCG1IDR_EL0` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Amcg1idrEl0: u64 {
        /// `AMEVCNTR1<n>_EL0` bit 0.
        const AMEVCNTR10_EL0 = 1 << 0;
        /// `AMEVCNTR1<n>_EL0` bit 1.
        const AMEVCNTR11_EL0 = 1 << 1;
        /// `AMEVCNTR1<n>_EL0` bit 2.
        const AMEVCNTR12_EL0 = 1 << 2;
        /// `AMEVCNTR1<n>_EL0` bit 3.
        const AMEVCNTR13_EL0 = 1 << 3;
        /// `AMEVCNTR1<n>_EL0` bit 4.
        const AMEVCNTR14_EL0 = 1 << 4;
        /// `AMEVCNTR1<n>_EL0` bit 5.
        const AMEVCNTR15_EL0 = 1 << 5;
        /// `AMEVCNTR1<n>_EL0` bit 6.
        const AMEVCNTR16_EL0 = 1 << 6;
        /// `AMEVCNTR1<n>_EL0` bit 7.
        const AMEVCNTR17_EL0 = 1 << 7;
        /// `AMEVCNTR1<n>_EL0` bit 8.
        const AMEVCNTR18_EL0 = 1 << 8;
        /// `AMEVCNTR1<n>_EL0` bit 9.
        const AMEVCNTR19_EL0 = 1 << 9;
        /// `AMEVCNTR1<n>_EL0` bit 10.
        const AMEVCNTR110_EL0 = 1 << 10;
        /// `AMEVCNTR1<n>_EL0` bit 11.
        const AMEVCNTR111_EL0 = 1 << 11;
        /// `AMEVCNTR1<n>_EL0` bit 12.
        const AMEVCNTR112_EL0 = 1 << 12;
        /// `AMEVCNTR1<n>_EL0` bit 13.
        const AMEVCNTR113_EL0 = 1 << 13;
        /// `AMEVCNTR1<n>_EL0` bit 14.
        const AMEVCNTR114_EL0 = 1 << 14;
        /// `AMEVCNTR1<n>_EL0` bit 15.
        const AMEVCNTR115_EL0 = 1 << 15;
        /// `AMEVCNTOFF1<n>_EL2` bit 0.
        const AMEVCNTOFF10_EL2 = 1 << 16;
        /// `AMEVCNTOFF1<n>_EL2` bit 1.
        const AMEVCNTOFF11_EL2 = 1 << 17;
        /// `AMEVCNTOFF1<n>_EL2` bit 2.
        const AMEVCNTOFF12_EL2 = 1 << 18;
        /// `AMEVCNTOFF1<n>_EL2` bit 3.
        const AMEVCNTOFF13_EL2 = 1 << 19;
        /// `AMEVCNTOFF1<n>_EL2` bit 4.
        const AMEVCNTOFF14_EL2 = 1 << 20;
        /// `AMEVCNTOFF1<n>_EL2` bit 5.
        const AMEVCNTOFF15_EL2 = 1 << 21;
        /// `AMEVCNTOFF1<n>_EL2` bit 6.
        const AMEVCNTOFF16_EL2 = 1 << 22;
        /// `AMEVCNTOFF1<n>_EL2` bit 7.
        const AMEVCNTOFF17_EL2 = 1 << 23;
        /// `AMEVCNTOFF1<n>_EL2` bit 8.
        const AMEVCNTOFF18_EL2 = 1 << 24;
        /// `AMEVCNTOFF1<n>_EL2` bit 9.
        const AMEVCNTOFF19_EL2 = 1 << 25;
        /// `AMEVCNTOFF1<n>_EL2` bit 10.
        const AMEVCNTOFF110_EL2 = 1 << 26;
        /// `AMEVCNTOFF1<n>_EL2` bit 11.
        const AMEVCNTOFF111_EL2 = 1 << 27;
        /// `AMEVCNTOFF1<n>_EL2` bit 12.
        const AMEVCNTOFF112_EL2 = 1 << 28;
        /// `AMEVCNTOFF1<n>_EL2` bit 13.
        const AMEVCNTOFF113_EL2 = 1 << 29;
        /// `AMEVCNTOFF1<n>_EL2` bit 14.
        const AMEVCNTOFF114_EL2 = 1 << 30;
        /// `AMEVCNTOFF1<n>_EL2` bit 15.
        const AMEVCNTOFF115_EL2 = 1 << 31;
    }
}

impl Amcg1idrEl0 {
    /// Offset of the `AMEVCNTR1<n>_EL0` field.
    pub const AMEVCNTR1_EL0_SHIFT: u32 = 0;
    /// Offset of the `AMEVCNTOFF1<n>_EL2` field.
    pub const AMEVCNTOFF1_EL2_SHIFT: u32 = 16;
}

bitflags! {
    /// `AMCGCR_EL0` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct AmcgcrEl0: u64 {
    }
}

impl AmcgcrEl0 {
    /// Offset of the `CG0NC` field.
    pub const CG0NC_SHIFT: u32 = 0;
    /// Mask for the `CG0NC` field.
    pub const CG0NC_MASK: u64 = 0b1111_1111;
    /// Offset of the `CG1NC` field.
    pub const CG1NC_SHIFT: u32 = 8;
    /// Mask for the `CG1NC` field.
    pub const CG1NC_MASK: u64 = 0b1111_1111;

    /// Returns the value of the `CG0NC` field.
    pub const fn cg0nc(self) -> u8 {
        ((self.bits() >> Self::CG0NC_SHIFT) & Self::CG0NC_MASK) as u8
    }

    /// Sets the value of the `CG0NC` field.
    pub const fn set_cg0nc(&mut self, value: u8) {
        let offset = Self::CG0NC_SHIFT;
        assert!(value & (Self::CG0NC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CG0NC_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `CG0NC` field set to the given value.
    pub const fn with_cg0nc(mut self, value: u8) -> Self {
        self.set_cg0nc(value);
        self
    }

    /// Returns the value of the `CG1NC` field.
    pub const fn cg1nc(self) -> u8 {
        ((self.bits() >> Self::CG1NC_SHIFT) & Self::CG1NC_MASK) as u8
    }

    /// Sets the value of the `CG1NC` field.
    pub const fn set_cg1nc(&mut self, value: u8) {
        let offset = Self::CG1NC_SHIFT;
        assert!(value & (Self::CG1NC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CG1NC_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `CG1NC` field set to the given value.
    pub const fn with_cg1nc(mut self, value: u8) -> Self {
        self.set_cg1nc(value);
        self
    }
}

bitflags! {
    /// `AMCNTENCLR0_EL0` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Amcntenclr0El0: u64 {
        /// `P<n>` bit 0.
        const P0 = 1 << 0;
        /// `P<n>` bit 1.
        const P1 = 1 << 1;
        /// `P<n>` bit 2.
        const P2 = 1 << 2;
        /// `P<n>` bit 3.
        const P3 = 1 << 3;
    }
}

impl Amcntenclr0El0 {
    /// Offset of the `P<n>` field.
    pub const P_SHIFT: u32 = 0;
}

bitflags! {
    /// `AMCNTENCLR1_EL0` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Amcntenclr1El0: u64 {
        /// `P<n>` bit 0.
        const P0 = 1 << 0;
        /// `P<n>` bit 1.
        const P1 = 1 << 1;
        /// `P<n>` bit 2.
        const P2 = 1 << 2;
        /// `P<n>` bit 3.
        const P3 = 1 << 3;
        /// `P<n>` bit 4.
        const P4 = 1 << 4;
        /// `P<n>` bit 5.
        const P5 = 1 << 5;
        /// `P<n>` bit 6.
        const P6 = 1 << 6;
        /// `P<n>` bit 7.
        const P7 = 1 << 7;
        /// `P<n>` bit 8.
        const P8 = 1 << 8;
        /// `P<n>` bit 9.
        const P9 = 1 << 9;
        /// `P<n>` bit 10.
        const P10 = 1 << 10;
        /// `P<n>` bit 11.
        const P11 = 1 << 11;
        /// `P<n>` bit 12.
        const P12 = 1 << 12;
        /// `P<n>` bit 13.
        const P13 = 1 << 13;
        /// `P<n>` bit 14.
        const P14 = 1 << 14;
        /// `P<n>` bit 15.
        const P15 = 1 << 15;
    }
}

impl Amcntenclr1El0 {
    /// Offset of the `P<n>` field.
    pub const P_SHIFT: u32 = 0;
}

/// `AMCNTENSET0_EL0` system register value.
pub type Amcntenset0El0 = Amcntenclr0El0;

/// `AMCNTENSET1_EL0` system register value.
pub type Amcntenset1El0 = Amcntenclr1El0;

bitflags! {
    /// `AMCR_EL0` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct AmcrEl0: u64 {
        /// `HDBG` bit.
        const HDBG = 1 << 10;
        /// `CG1RZ` bit.
        const CG1RZ = 1 << 17;
    }
}

impl AmcrEl0 {
    /// Offset of the `HDBG` field.
    pub const HDBG_SHIFT: u32 = 10;
    /// Offset of the `CG1RZ` field.
    pub const CG1RZ_SHIFT: u32 = 17;
}

bitflags! {
    /// `AMEVCNTR00_EL0` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Amevcntr00El0: u64 {
    }
}

impl Amevcntr00El0 {
    /// Offset of the `ACNT` field.
    pub const ACNT_SHIFT: u32 = 0;
    /// Mask for the `ACNT` field.
    pub const ACNT_MASK: u64 =
        0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `ACNT` field.
    pub const fn acnt(self) -> u64 {
        (self.bits() >> Self::ACNT_SHIFT) & Self::ACNT_MASK
    }

    /// Sets the value of the `ACNT` field.
    pub const fn set_acnt(&mut self, value: u64) {
        let offset = Self::ACNT_SHIFT;
        assert!(value & Self::ACNT_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ACNT_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `ACNT` field set to the given value.
    pub const fn with_acnt(mut self, value: u64) -> Self {
        self.set_acnt(value);
        self
    }
}

/// `AMEVCNTR01_EL0` system register value.
pub type Amevcntr01El0 = Amevcntr00El0;

/// `AMEVCNTR02_EL0` system register value.
pub type Amevcntr02El0 = Amevcntr00El0;

/// `AMEVCNTR03_EL0` system register value.
pub type Amevcntr03El0 = Amevcntr00El0;

/// `AMEVCNTR10_EL0` system register value.
pub type Amevcntr10El0 = Amevcntr00El0;

/// `AMEVCNTR110_EL0` system register value.
pub type Amevcntr110El0 = Amevcntr00El0;

/// `AMEVCNTR111_EL0` system register value.
pub type Amevcntr111El0 = Amevcntr00El0;

/// `AMEVCNTR112_EL0` system register value.
pub type Amevcntr112El0 = Amevcntr00El0;

/// `AMEVCNTR113_EL0` system register value.
pub type Amevcntr113El0 = Amevcntr00El0;

/// `AMEVCNTR114_EL0` system register value.
pub type Amevcntr114El0 = Amevcntr00El0;

/// `AMEVCNTR115_EL0` system register value.
pub type Amevcntr115El0 = Amevcntr00El0;

/// `AMEVCNTR11_EL0` system register value.
pub type Amevcntr11El0 = Amevcntr00El0;

/// `AMEVCNTR12_EL0` system register value.
pub type Amevcntr12El0 = Amevcntr00El0;

/// `AMEVCNTR13_EL0` system register value.
pub type Amevcntr13El0 = Amevcntr00El0;

/// `AMEVCNTR14_EL0` system register value.
pub type Amevcntr14El0 = Amevcntr00El0;

/// `AMEVCNTR15_EL0` system register value.
pub type Amevcntr15El0 = Amevcntr00El0;

/// `AMEVCNTR16_EL0` system register value.
pub type Amevcntr16El0 = Amevcntr00El0;

/// `AMEVCNTR17_EL0` system register value.
pub type Amevcntr17El0 = Amevcntr00El0;

/// `AMEVCNTR18_EL0` system register value.
pub type Amevcntr18El0 = Amevcntr00El0;

/// `AMEVCNTR19_EL0` system register value.
pub type Amevcntr19El0 = Amevcntr00El0;

bitflags! {
    /// `AMEVTYPER00_EL0` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Amevtyper00El0: u64 {
    }
}

impl Amevtyper00El0 {
    /// Offset of the `evtCount` field.
    pub const EVTCOUNT_SHIFT: u32 = 0;
    /// Mask for the `evtCount` field.
    pub const EVTCOUNT_MASK: u64 = 0b1111_1111_1111_1111;

    /// Returns the value of the `evtCount` field.
    pub const fn evtcount(self) -> u16 {
        ((self.bits() >> Self::EVTCOUNT_SHIFT) & Self::EVTCOUNT_MASK) as u16
    }

    /// Sets the value of the `evtCount` field.
    pub const fn set_evtcount(&mut self, value: u16) {
        let offset = Self::EVTCOUNT_SHIFT;
        assert!(value & (Self::EVTCOUNT_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EVTCOUNT_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `evtCount` field set to the given value.
    pub const fn with_evtcount(mut self, value: u16) -> Self {
        self.set_evtcount(value);
        self
    }
}

/// `AMEVTYPER01_EL0` system register value.
pub type Amevtyper01El0 = Amevtyper00El0;

/// `AMEVTYPER02_EL0` system register value.
pub type Amevtyper02El0 = Amevtyper00El0;

/// `AMEVTYPER03_EL0` system register value.
pub type Amevtyper03El0 = Amevtyper00El0;

bitflags! {
    /// `AMUSERENR_EL0` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct AmuserenrEl0: u64 {
        /// `EN` bit.
        const EN = 1 << 0;
    }
}

impl AmuserenrEl0 {
    /// Offset of the `EN` field.
    pub const EN_SHIFT: u32 = 0;
}

bitflags! {
    /// `CNTFRQ_EL0` system register value.
    ///
    /// Counter-timer Frequency Register
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CntfrqEl0: u64 {
    }
}

impl CntfrqEl0 {
    /// Offset of the `ClockFreq` field.
    pub const CLOCKFREQ_SHIFT: u32 = 0;
    /// Mask for the `ClockFreq` field.
    pub const CLOCKFREQ_MASK: u64 = 0b1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `ClockFreq` field.
    pub const fn clockfreq(self) -> u32 {
        ((self.bits() >> Self::CLOCKFREQ_SHIFT) & Self::CLOCKFREQ_MASK) as u32
    }

    /// Sets the value of the `ClockFreq` field.
    pub const fn set_clockfreq(&mut self, value: u32) {
        let offset = Self::CLOCKFREQ_SHIFT;
        assert!(value & (Self::CLOCKFREQ_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CLOCKFREQ_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `ClockFreq` field set to the given value.
    pub const fn with_clockfreq(mut self, value: u32) -> Self {
        self.set_clockfreq(value);
        self
    }
}

bitflags! {
    /// `CNTPCTSS_EL0` system register value.
    ///
    /// Counter-timer Self-Synchronized Physical Count Register
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CntpctssEl0: u64 {
    }
}

impl CntpctssEl0 {
    /// Offset of the `SSPhysicalCount` field.
    pub const SSPHYSICALCOUNT_SHIFT: u32 = 0;
    /// Mask for the `SSPhysicalCount` field.
    pub const SSPHYSICALCOUNT_MASK: u64 =
        0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `SSPhysicalCount` field.
    pub const fn ssphysicalcount(self) -> u64 {
        (self.bits() >> Self::SSPHYSICALCOUNT_SHIFT) & Self::SSPHYSICALCOUNT_MASK
    }

    /// Sets the value of the `SSPhysicalCount` field.
    pub const fn set_ssphysicalcount(&mut self, value: u64) {
        let offset = Self::SSPHYSICALCOUNT_SHIFT;
        assert!(value & Self::SSPHYSICALCOUNT_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SSPHYSICALCOUNT_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `SSPhysicalCount` field set to the given value.
    pub const fn with_ssphysicalcount(mut self, value: u64) -> Self {
        self.set_ssphysicalcount(value);
        self
    }
}

bitflags! {
    /// `CNTPCT_EL0` system register value.
    ///
    /// Counter-timer Physical Count Register
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CntpctEl0: u64 {
    }
}

impl CntpctEl0 {
    /// Offset of the `PhysicalCount` field.
    pub const PHYSICALCOUNT_SHIFT: u32 = 0;
    /// Mask for the `PhysicalCount` field.
    pub const PHYSICALCOUNT_MASK: u64 =
        0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `PhysicalCount` field.
    pub const fn physicalcount(self) -> u64 {
        (self.bits() >> Self::PHYSICALCOUNT_SHIFT) & Self::PHYSICALCOUNT_MASK
    }

    /// Sets the value of the `PhysicalCount` field.
    pub const fn set_physicalcount(&mut self, value: u64) {
        let offset = Self::PHYSICALCOUNT_SHIFT;
        assert!(value & Self::PHYSICALCOUNT_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYSICALCOUNT_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `PhysicalCount` field set to the given value.
    pub const fn with_physicalcount(mut self, value: u64) -> Self {
        self.set_physicalcount(value);
        self
    }
}

bitflags! {
    /// `CNTP_CTL_EL0` system register value.
    ///
    /// Counter-timer Physical Timer Control Register
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CntpCtlEl0: u64 {
        /// `ENABLE` bit.
        const ENABLE = 1 << 0;
        /// `IMASK` bit.
        const IMASK = 1 << 1;
        /// `ISTATUS` bit.
        const ISTATUS = 1 << 2;
    }
}

impl CntpCtlEl0 {
    /// Offset of the `ENABLE` field.
    pub const ENABLE_SHIFT: u32 = 0;
    /// Offset of the `IMASK` field.
    pub const IMASK_SHIFT: u32 = 1;
    /// Offset of the `ISTATUS` field.
    pub const ISTATUS_SHIFT: u32 = 2;
}

bitflags! {
    /// `CNTP_CVAL_EL0` system register value.
    ///
    /// Counter-timer Physical Timer CompareValue Register
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CntpCvalEl0: u64 {
    }
}

impl CntpCvalEl0 {
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
    /// `CNTP_TVAL_EL0` system register value.
    ///
    /// Counter-timer Physical Timer TimerValue Register
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CntpTvalEl0: u64 {
    }
}

impl CntpTvalEl0 {
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
    /// `CNTVCTSS_EL0` system register value.
    ///
    /// Counter-timer Self-Synchronized Virtual Count Register
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CntvctssEl0: u64 {
    }
}

impl CntvctssEl0 {
    /// Offset of the `SSVirtualCount` field.
    pub const SSVIRTUALCOUNT_SHIFT: u32 = 0;
    /// Mask for the `SSVirtualCount` field.
    pub const SSVIRTUALCOUNT_MASK: u64 =
        0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `SSVirtualCount` field.
    pub const fn ssvirtualcount(self) -> u64 {
        (self.bits() >> Self::SSVIRTUALCOUNT_SHIFT) & Self::SSVIRTUALCOUNT_MASK
    }

    /// Sets the value of the `SSVirtualCount` field.
    pub const fn set_ssvirtualcount(&mut self, value: u64) {
        let offset = Self::SSVIRTUALCOUNT_SHIFT;
        assert!(value & Self::SSVIRTUALCOUNT_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SSVIRTUALCOUNT_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `SSVirtualCount` field set to the given value.
    pub const fn with_ssvirtualcount(mut self, value: u64) -> Self {
        self.set_ssvirtualcount(value);
        self
    }
}

bitflags! {
    /// `CNTVCT_EL0` system register value.
    ///
    /// Counter-timer Virtual Count Register
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CntvctEl0: u64 {
    }
}

impl CntvctEl0 {
    /// Offset of the `VirtualCount` field.
    pub const VIRTUALCOUNT_SHIFT: u32 = 0;
    /// Mask for the `VirtualCount` field.
    pub const VIRTUALCOUNT_MASK: u64 =
        0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `VirtualCount` field.
    pub const fn virtualcount(self) -> u64 {
        (self.bits() >> Self::VIRTUALCOUNT_SHIFT) & Self::VIRTUALCOUNT_MASK
    }

    /// Sets the value of the `VirtualCount` field.
    pub const fn set_virtualcount(&mut self, value: u64) {
        let offset = Self::VIRTUALCOUNT_SHIFT;
        assert!(value & Self::VIRTUALCOUNT_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VIRTUALCOUNT_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `VirtualCount` field set to the given value.
    pub const fn with_virtualcount(mut self, value: u64) -> Self {
        self.set_virtualcount(value);
        self
    }
}

/// `CNTV_CTL_EL0` system register value.
///
/// Counter-timer Virtual Timer Control Register
pub type CntvCtlEl0 = CntpCtlEl0;

/// `CNTV_CVAL_EL0` system register value.
///
/// Counter-timer Virtual Timer CompareValue Register
pub type CntvCvalEl0 = CntpCvalEl0;

/// `CNTV_TVAL_EL0` system register value.
///
/// Counter-timer Virtual Timer TimerValue Register
pub type CntvTvalEl0 = CntpTvalEl0;

bitflags! {
    /// `CTR_EL0` system register value.
    ///
    /// Cache Type Register.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CtrEl0: u64 {
        /// RES1 bits in the `CTR_EL0` register.
        const RES1 = 0b1000_0000_0000_0000_0000_0000_0000_0000;
        /// `IDC` bit.
        const IDC = 1 << 28;
        /// `DIC` bit.
        const DIC = 1 << 29;
    }
}

impl CtrEl0 {
    /// Offset of the `IminLine` field.
    pub const IMINLINE_SHIFT: u32 = 0;
    /// Mask for the `IminLine` field.
    pub const IMINLINE_MASK: u64 = 0b1111;
    /// Offset of the `L1Ip` field.
    pub const L1IP_SHIFT: u32 = 14;
    /// Mask for the `L1Ip` field.
    pub const L1IP_MASK: u64 = 0b11;
    /// Offset of the `DminLine` field.
    pub const DMINLINE_SHIFT: u32 = 16;
    /// Mask for the `DminLine` field.
    pub const DMINLINE_MASK: u64 = 0b1111;
    /// Offset of the `ERG` field.
    pub const ERG_SHIFT: u32 = 20;
    /// Mask for the `ERG` field.
    pub const ERG_MASK: u64 = 0b1111;
    /// Offset of the `CWG` field.
    pub const CWG_SHIFT: u32 = 24;
    /// Mask for the `CWG` field.
    pub const CWG_MASK: u64 = 0b1111;
    /// Offset of the `IDC` field.
    pub const IDC_SHIFT: u32 = 28;
    /// Offset of the `DIC` field.
    pub const DIC_SHIFT: u32 = 29;
    /// Offset of the `TminLine` field.
    pub const TMINLINE_SHIFT: u32 = 32;
    /// Mask for the `TminLine` field.
    pub const TMINLINE_MASK: u64 = 0b11_1111;

    /// Returns the value of the `IminLine` field.
    pub const fn iminline(self) -> u8 {
        ((self.bits() >> Self::IMINLINE_SHIFT) & Self::IMINLINE_MASK) as u8
    }

    /// Sets the value of the `IminLine` field.
    pub const fn set_iminline(&mut self, value: u8) {
        let offset = Self::IMINLINE_SHIFT;
        assert!(value & (Self::IMINLINE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::IMINLINE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `IminLine` field set to the given value.
    pub const fn with_iminline(mut self, value: u8) -> Self {
        self.set_iminline(value);
        self
    }

    /// Returns the value of the `L1Ip` field.
    pub const fn l1ip(self) -> u8 {
        ((self.bits() >> Self::L1IP_SHIFT) & Self::L1IP_MASK) as u8
    }

    /// Sets the value of the `L1Ip` field.
    pub const fn set_l1ip(&mut self, value: u8) {
        let offset = Self::L1IP_SHIFT;
        assert!(value & (Self::L1IP_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::L1IP_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `L1Ip` field set to the given value.
    pub const fn with_l1ip(mut self, value: u8) -> Self {
        self.set_l1ip(value);
        self
    }

    /// Returns the value of the `DminLine` field.
    ///
    /// Log2 of the number of words in the smallest cache line of all the data caches and unified caches that are controlled by the PE.
    pub const fn dminline(self) -> u8 {
        ((self.bits() >> Self::DMINLINE_SHIFT) & Self::DMINLINE_MASK) as u8
    }

    /// Sets the value of the `DminLine` field.
    ///
    /// Log2 of the number of words in the smallest cache line of all the data caches and unified caches that are controlled by the PE.
    pub const fn set_dminline(&mut self, value: u8) {
        let offset = Self::DMINLINE_SHIFT;
        assert!(value & (Self::DMINLINE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::DMINLINE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `DminLine` field set to the given value.
    ///
    /// Log2 of the number of words in the smallest cache line of all the data caches and unified caches that are controlled by the PE.
    pub const fn with_dminline(mut self, value: u8) -> Self {
        self.set_dminline(value);
        self
    }

    /// Returns the value of the `ERG` field.
    pub const fn erg(self) -> u8 {
        ((self.bits() >> Self::ERG_SHIFT) & Self::ERG_MASK) as u8
    }

    /// Sets the value of the `ERG` field.
    pub const fn set_erg(&mut self, value: u8) {
        let offset = Self::ERG_SHIFT;
        assert!(value & (Self::ERG_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ERG_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `ERG` field set to the given value.
    pub const fn with_erg(mut self, value: u8) -> Self {
        self.set_erg(value);
        self
    }

    /// Returns the value of the `CWG` field.
    pub const fn cwg(self) -> u8 {
        ((self.bits() >> Self::CWG_SHIFT) & Self::CWG_MASK) as u8
    }

    /// Sets the value of the `CWG` field.
    pub const fn set_cwg(&mut self, value: u8) {
        let offset = Self::CWG_SHIFT;
        assert!(value & (Self::CWG_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CWG_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `CWG` field set to the given value.
    pub const fn with_cwg(mut self, value: u8) -> Self {
        self.set_cwg(value);
        self
    }

    /// Returns the value of the `TminLine` field.
    pub const fn tminline(self) -> u8 {
        ((self.bits() >> Self::TMINLINE_SHIFT) & Self::TMINLINE_MASK) as u8
    }

    /// Sets the value of the `TminLine` field.
    pub const fn set_tminline(&mut self, value: u8) {
        let offset = Self::TMINLINE_SHIFT;
        assert!(value & (Self::TMINLINE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TMINLINE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `TminLine` field set to the given value.
    pub const fn with_tminline(mut self, value: u8) -> Self {
        self.set_tminline(value);
        self
    }
}

bitflags! {
    /// `CurrentEL` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Currentel: u64 {
    }
}

impl Currentel {
    /// Offset of the `EL` field.
    pub const EL_SHIFT: u32 = 2;
    /// Mask for the `EL` field.
    pub const EL_MASK: u64 = 0b11;

    /// Returns the value of the `EL` field.
    pub const fn el(self) -> u8 {
        ((self.bits() >> Self::EL_SHIFT) & Self::EL_MASK) as u8
    }

    /// Sets the value of the `EL` field.
    pub const fn set_el(&mut self, value: u8) {
        let offset = Self::EL_SHIFT;
        assert!(value & (Self::EL_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EL_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `EL` field set to the given value.
    pub const fn with_el(mut self, value: u8) -> Self {
        self.set_el(value);
        self
    }
}

bitflags! {
    /// `DAIF` system register value.
    ///
    /// Interrupt Mask Bits
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Daif: u64 {
        /// `F` bit.
        const F = 1 << 6;
        /// `I` bit.
        const I = 1 << 7;
        /// `A` bit.
        const A = 1 << 8;
        /// `D` bit.
        const D = 1 << 9;
    }
}

impl Daif {
    /// Offset of the `F` field.
    pub const F_SHIFT: u32 = 6;
    /// Offset of the `I` field.
    pub const I_SHIFT: u32 = 7;
    /// Offset of the `A` field.
    pub const A_SHIFT: u32 = 8;
    /// Offset of the `D` field.
    pub const D_SHIFT: u32 = 9;
}

bitflags! {
    /// `DIT` system register value.
    ///
    /// Data Independent Timing.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dit: u64 {
        /// Enable data independent timing.
        const DIT = 1 << 24;
    }
}

impl Dit {
    /// Offset of the `DIT` field.
    pub const DIT_SHIFT: u32 = 24;
}

bitflags! {
    /// `FPCR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Fpcr: u64 {
        /// `FIZ` bit.
        const FIZ = 1 << 0;
        /// `AH` bit.
        const AH = 1 << 1;
        /// `NEP` bit.
        const NEP = 1 << 2;
        /// `IOE` bit.
        const IOE = 1 << 8;
        /// `DZE` bit.
        const DZE = 1 << 9;
        /// `OFE` bit.
        const OFE = 1 << 10;
        /// `UFE` bit.
        const UFE = 1 << 11;
        /// `IXE` bit.
        const IXE = 1 << 12;
        /// `EBF` bit.
        const EBF = 1 << 13;
        /// `IDE` bit.
        const IDE = 1 << 15;
        /// `FZ16` bit.
        const FZ16 = 1 << 19;
        /// `FZ` bit.
        const FZ = 1 << 24;
        /// `DN` bit.
        const DN = 1 << 25;
        /// `AHP` bit.
        const AHP = 1 << 26;
    }
}

impl Fpcr {
    /// Offset of the `FIZ` field.
    pub const FIZ_SHIFT: u32 = 0;
    /// Offset of the `AH` field.
    pub const AH_SHIFT: u32 = 1;
    /// Offset of the `NEP` field.
    pub const NEP_SHIFT: u32 = 2;
    /// Offset of the `IOE` field.
    pub const IOE_SHIFT: u32 = 8;
    /// Offset of the `DZE` field.
    pub const DZE_SHIFT: u32 = 9;
    /// Offset of the `OFE` field.
    pub const OFE_SHIFT: u32 = 10;
    /// Offset of the `UFE` field.
    pub const UFE_SHIFT: u32 = 11;
    /// Offset of the `IXE` field.
    pub const IXE_SHIFT: u32 = 12;
    /// Offset of the `EBF` field.
    pub const EBF_SHIFT: u32 = 13;
    /// Offset of the `IDE` field.
    pub const IDE_SHIFT: u32 = 15;
    /// Offset of the `Len` field.
    pub const LEN_SHIFT: u32 = 16;
    /// Mask for the `Len` field.
    pub const LEN_MASK: u64 = 0b111;
    /// Offset of the `FZ16` field.
    pub const FZ16_SHIFT: u32 = 19;
    /// Offset of the `Stride` field.
    pub const STRIDE_SHIFT: u32 = 20;
    /// Mask for the `Stride` field.
    pub const STRIDE_MASK: u64 = 0b11;
    /// Offset of the `RMode` field.
    pub const RMODE_SHIFT: u32 = 22;
    /// Mask for the `RMode` field.
    pub const RMODE_MASK: u64 = 0b11;
    /// Offset of the `FZ` field.
    pub const FZ_SHIFT: u32 = 24;
    /// Offset of the `DN` field.
    pub const DN_SHIFT: u32 = 25;
    /// Offset of the `AHP` field.
    pub const AHP_SHIFT: u32 = 26;

    /// Returns the value of the `Len` field.
    pub const fn len(self) -> u8 {
        ((self.bits() >> Self::LEN_SHIFT) & Self::LEN_MASK) as u8
    }

    /// Sets the value of the `Len` field.
    pub const fn set_len(&mut self, value: u8) {
        let offset = Self::LEN_SHIFT;
        assert!(value & (Self::LEN_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::LEN_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `Len` field set to the given value.
    pub const fn with_len(mut self, value: u8) -> Self {
        self.set_len(value);
        self
    }

    /// Returns the value of the `Stride` field.
    pub const fn stride(self) -> u8 {
        ((self.bits() >> Self::STRIDE_SHIFT) & Self::STRIDE_MASK) as u8
    }

    /// Sets the value of the `Stride` field.
    pub const fn set_stride(&mut self, value: u8) {
        let offset = Self::STRIDE_SHIFT;
        assert!(value & (Self::STRIDE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::STRIDE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `Stride` field set to the given value.
    pub const fn with_stride(mut self, value: u8) -> Self {
        self.set_stride(value);
        self
    }

    /// Returns the value of the `RMode` field.
    pub const fn rmode(self) -> u8 {
        ((self.bits() >> Self::RMODE_SHIFT) & Self::RMODE_MASK) as u8
    }

    /// Sets the value of the `RMode` field.
    pub const fn set_rmode(&mut self, value: u8) {
        let offset = Self::RMODE_SHIFT;
        assert!(value & (Self::RMODE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::RMODE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `RMode` field set to the given value.
    pub const fn with_rmode(mut self, value: u8) -> Self {
        self.set_rmode(value);
        self
    }
}

bitflags! {
    /// `FPMR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Fpmr: u64 {
        /// `OSM` bit.
        const OSM = 1 << 14;
        /// `OSC` bit.
        const OSC = 1 << 15;
    }
}

impl Fpmr {
    /// Offset of the `F8S1` field.
    pub const F8S1_SHIFT: u32 = 0;
    /// Mask for the `F8S1` field.
    pub const F8S1_MASK: u64 = 0b111;
    /// Offset of the `F8S2` field.
    pub const F8S2_SHIFT: u32 = 3;
    /// Mask for the `F8S2` field.
    pub const F8S2_MASK: u64 = 0b111;
    /// Offset of the `F8D` field.
    pub const F8D_SHIFT: u32 = 6;
    /// Mask for the `F8D` field.
    pub const F8D_MASK: u64 = 0b111;
    /// Offset of the `OSM` field.
    pub const OSM_SHIFT: u32 = 14;
    /// Offset of the `OSC` field.
    pub const OSC_SHIFT: u32 = 15;
    /// Offset of the `LSCALE` field.
    pub const LSCALE_SHIFT: u32 = 16;
    /// Mask for the `LSCALE` field.
    pub const LSCALE_MASK: u64 = 0b111_1111;
    /// Offset of the `NSCALE` field.
    pub const NSCALE_SHIFT: u32 = 24;
    /// Mask for the `NSCALE` field.
    pub const NSCALE_MASK: u64 = 0b1111_1111;
    /// Offset of the `LSCALE2` field.
    pub const LSCALE2_SHIFT: u32 = 32;
    /// Mask for the `LSCALE2` field.
    pub const LSCALE2_MASK: u64 = 0b11_1111;

    /// Returns the value of the `F8S1` field.
    pub const fn f8s1(self) -> u8 {
        ((self.bits() >> Self::F8S1_SHIFT) & Self::F8S1_MASK) as u8
    }

    /// Sets the value of the `F8S1` field.
    pub const fn set_f8s1(&mut self, value: u8) {
        let offset = Self::F8S1_SHIFT;
        assert!(value & (Self::F8S1_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::F8S1_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `F8S1` field set to the given value.
    pub const fn with_f8s1(mut self, value: u8) -> Self {
        self.set_f8s1(value);
        self
    }

    /// Returns the value of the `F8S2` field.
    pub const fn f8s2(self) -> u8 {
        ((self.bits() >> Self::F8S2_SHIFT) & Self::F8S2_MASK) as u8
    }

    /// Sets the value of the `F8S2` field.
    pub const fn set_f8s2(&mut self, value: u8) {
        let offset = Self::F8S2_SHIFT;
        assert!(value & (Self::F8S2_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::F8S2_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `F8S2` field set to the given value.
    pub const fn with_f8s2(mut self, value: u8) -> Self {
        self.set_f8s2(value);
        self
    }

    /// Returns the value of the `F8D` field.
    pub const fn f8d(self) -> u8 {
        ((self.bits() >> Self::F8D_SHIFT) & Self::F8D_MASK) as u8
    }

    /// Sets the value of the `F8D` field.
    pub const fn set_f8d(&mut self, value: u8) {
        let offset = Self::F8D_SHIFT;
        assert!(value & (Self::F8D_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::F8D_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `F8D` field set to the given value.
    pub const fn with_f8d(mut self, value: u8) -> Self {
        self.set_f8d(value);
        self
    }

    /// Returns the value of the `LSCALE` field.
    pub const fn lscale(self) -> u8 {
        ((self.bits() >> Self::LSCALE_SHIFT) & Self::LSCALE_MASK) as u8
    }

    /// Sets the value of the `LSCALE` field.
    pub const fn set_lscale(&mut self, value: u8) {
        let offset = Self::LSCALE_SHIFT;
        assert!(value & (Self::LSCALE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::LSCALE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `LSCALE` field set to the given value.
    pub const fn with_lscale(mut self, value: u8) -> Self {
        self.set_lscale(value);
        self
    }

    /// Returns the value of the `NSCALE` field.
    pub const fn nscale(self) -> u8 {
        ((self.bits() >> Self::NSCALE_SHIFT) & Self::NSCALE_MASK) as u8
    }

    /// Sets the value of the `NSCALE` field.
    pub const fn set_nscale(&mut self, value: u8) {
        let offset = Self::NSCALE_SHIFT;
        assert!(value & (Self::NSCALE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::NSCALE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `NSCALE` field set to the given value.
    pub const fn with_nscale(mut self, value: u8) -> Self {
        self.set_nscale(value);
        self
    }

    /// Returns the value of the `LSCALE2` field.
    pub const fn lscale2(self) -> u8 {
        ((self.bits() >> Self::LSCALE2_SHIFT) & Self::LSCALE2_MASK) as u8
    }

    /// Sets the value of the `LSCALE2` field.
    pub const fn set_lscale2(&mut self, value: u8) {
        let offset = Self::LSCALE2_SHIFT;
        assert!(value & (Self::LSCALE2_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::LSCALE2_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `LSCALE2` field set to the given value.
    pub const fn with_lscale2(mut self, value: u8) -> Self {
        self.set_lscale2(value);
        self
    }
}

bitflags! {
    /// `FPSR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Fpsr: u64 {
        /// `IOC` bit.
        const IOC = 1 << 0;
        /// `DZC` bit.
        const DZC = 1 << 1;
        /// `OFC` bit.
        const OFC = 1 << 2;
        /// `UFC` bit.
        const UFC = 1 << 3;
        /// `IXC` bit.
        const IXC = 1 << 4;
        /// `IDC` bit.
        const IDC = 1 << 7;
        /// `QC` bit.
        const QC = 1 << 27;
        /// `V` bit.
        const V = 1 << 28;
        /// `C` bit.
        const C = 1 << 29;
        /// `Z` bit.
        const Z = 1 << 30;
        /// `N` bit.
        const N = 1 << 31;
    }
}

impl Fpsr {
    /// Offset of the `IOC` field.
    pub const IOC_SHIFT: u32 = 0;
    /// Offset of the `DZC` field.
    pub const DZC_SHIFT: u32 = 1;
    /// Offset of the `OFC` field.
    pub const OFC_SHIFT: u32 = 2;
    /// Offset of the `UFC` field.
    pub const UFC_SHIFT: u32 = 3;
    /// Offset of the `IXC` field.
    pub const IXC_SHIFT: u32 = 4;
    /// Offset of the `IDC` field.
    pub const IDC_SHIFT: u32 = 7;
    /// Offset of the `QC` field.
    pub const QC_SHIFT: u32 = 27;
    /// Offset of the `V` field.
    pub const V_SHIFT: u32 = 28;
    /// Offset of the `C` field.
    pub const C_SHIFT: u32 = 29;
    /// Offset of the `Z` field.
    pub const Z_SHIFT: u32 = 30;
    /// Offset of the `N` field.
    pub const N_SHIFT: u32 = 31;
}

bitflags! {
    /// `GCSPR_EL0` system register value.
    ///
    /// Guarded Control Stack Pointer register.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct GcsprEl0: u64 {
    }
}

impl GcsprEl0 {
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
    /// `PMCR_EL0` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct PmcrEl0: u64 {
        /// Enable. Affected counters are enabled by PMCNTENSET_EL0.
        const E = 1 << 0;
        /// Event counter reset. Reset all affected event counters PMEVCNTR<n>_EL0 to zero.
        const P = 1 << 1;
        /// Cycle counter reset. Reset PMCCNTR_EL0 to zero.
        const C = 1 << 2;
        /// Clock divider. If set PMCCNTR_EL0 counts once every 64 clock cycles.
        const D = 1 << 3;
        /// Enable export of events in an IMPLEMENTATION DEFINED PMU event export bus. If set, export events where not prohibited.
        const X = 1 << 4;
        /// If set, cycle counting by PMCCNTR_EL0 is disabled in prohibited regions.
        const DP = 1 << 5;
        /// `LC` bit.
        const LC = 1 << 6;
        /// `LP` bit.
        const LP = 1 << 7;
        /// `FZO` bit.
        const FZO = 1 << 9;
        /// `FZS` bit.
        const FZS = 1 << 32;
    }
}

impl PmcrEl0 {
    /// Offset of the `E` field.
    pub const E_SHIFT: u32 = 0;
    /// Offset of the `P` field.
    pub const P_SHIFT: u32 = 1;
    /// Offset of the `C` field.
    pub const C_SHIFT: u32 = 2;
    /// Offset of the `D` field.
    pub const D_SHIFT: u32 = 3;
    /// Offset of the `X` field.
    pub const X_SHIFT: u32 = 4;
    /// Offset of the `DP` field.
    pub const DP_SHIFT: u32 = 5;
    /// Offset of the `LC` field.
    pub const LC_SHIFT: u32 = 6;
    /// Offset of the `LP` field.
    pub const LP_SHIFT: u32 = 7;
    /// Offset of the `FZO` field.
    pub const FZO_SHIFT: u32 = 9;
    /// Offset of the `N` field.
    pub const N_SHIFT: u32 = 11;
    /// Mask for the `N` field.
    pub const N_MASK: u64 = 0b1_1111;
    /// Offset of the `IDCODE` field.
    pub const IDCODE_SHIFT: u32 = 16;
    /// Mask for the `IDCODE` field.
    pub const IDCODE_MASK: u64 = 0b1111_1111;
    /// Offset of the `IMP` field.
    pub const IMP_SHIFT: u32 = 24;
    /// Mask for the `IMP` field.
    pub const IMP_MASK: u64 = 0b1111_1111;
    /// Offset of the `FZS` field.
    pub const FZS_SHIFT: u32 = 32;

    /// Returns the value of the `N` field.
    pub const fn n(self) -> u8 {
        ((self.bits() >> Self::N_SHIFT) & Self::N_MASK) as u8
    }

    /// Sets the value of the `N` field.
    pub const fn set_n(&mut self, value: u8) {
        let offset = Self::N_SHIFT;
        assert!(value & (Self::N_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::N_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `N` field set to the given value.
    pub const fn with_n(mut self, value: u8) -> Self {
        self.set_n(value);
        self
    }

    /// Returns the value of the `IDCODE` field.
    pub const fn idcode(self) -> u8 {
        ((self.bits() >> Self::IDCODE_SHIFT) & Self::IDCODE_MASK) as u8
    }

    /// Sets the value of the `IDCODE` field.
    pub const fn set_idcode(&mut self, value: u8) {
        let offset = Self::IDCODE_SHIFT;
        assert!(value & (Self::IDCODE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::IDCODE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `IDCODE` field set to the given value.
    pub const fn with_idcode(mut self, value: u8) -> Self {
        self.set_idcode(value);
        self
    }

    /// Returns the value of the `IMP` field.
    pub const fn imp(self) -> u8 {
        ((self.bits() >> Self::IMP_SHIFT) & Self::IMP_MASK) as u8
    }

    /// Sets the value of the `IMP` field.
    pub const fn set_imp(&mut self, value: u8) {
        let offset = Self::IMP_SHIFT;
        assert!(value & (Self::IMP_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::IMP_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `IMP` field set to the given value.
    pub const fn with_imp(mut self, value: u8) -> Self {
        self.set_imp(value);
        self
    }
}

bitflags! {
    /// `POR_EL0` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct PorEl0: u64 {
    }
}

impl PorEl0 {
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

bitflags! {
    /// `SVCR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Svcr: u64 {
        /// `SM` bit.
        const SM = 1 << 0;
        /// `ZA` bit.
        const ZA = 1 << 1;
    }
}

impl Svcr {
    /// Offset of the `SM` field.
    pub const SM_SHIFT: u32 = 0;
    /// Offset of the `ZA` field.
    pub const ZA_SHIFT: u32 = 1;
}

bitflags! {
    /// `TPIDRRO_EL0` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct TpidrroEl0: u64 {
    }
}

impl TpidrroEl0 {
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

/// `TPIDR_EL0` system register value.
pub type TpidrEl0 = TpidrroEl0;
