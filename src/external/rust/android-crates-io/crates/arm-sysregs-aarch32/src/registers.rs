// SPDX-FileCopyrightText: Copyright The arm-sysregs Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Arm CPU system registers.

// This file is generated, do not edit manually.

use bitflags::bitflags;

bitflags! {
    /// `AMCFGR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Amcfgr: u32 {
        /// `HDBG` bit.
        const HDBG = 1 << 24;
    }
}

impl Amcfgr {
    /// Offset of the `N` field.
    pub const N_SHIFT: u32 = 0;
    /// Mask for the `N` field.
    pub const N_MASK: u32 = 0b1111_1111;
    /// Offset of the `SIZE` field.
    pub const SIZE_SHIFT: u32 = 8;
    /// Mask for the `SIZE` field.
    pub const SIZE_MASK: u32 = 0b11_1111;
    /// Offset of the `HDBG` field.
    pub const HDBG_SHIFT: u32 = 24;
    /// Offset of the `NCG` field.
    pub const NCG_SHIFT: u32 = 28;
    /// Mask for the `NCG` field.
    pub const NCG_MASK: u32 = 0b1111;

    /// Returns the value of the `N` field.
    pub const fn n(self) -> u8 {
        ((self.bits() >> Self::N_SHIFT) & Self::N_MASK) as u8
    }

    /// Sets the value of the `N` field.
    pub const fn set_n(&mut self, value: u8) {
        let offset = Self::N_SHIFT;
        assert!(value & (Self::N_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::N_MASK << offset)) | ((value as u32) << offset),
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
            (self.bits() & !(Self::SIZE_MASK << offset)) | ((value as u32) << offset),
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
            (self.bits() & !(Self::NCG_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `NCG` field set to the given value.
    pub const fn with_ncg(mut self, value: u8) -> Self {
        self.set_ncg(value);
        self
    }
}

bitflags! {
    /// `AMCGCR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Amcgcr: u32 {
    }
}

impl Amcgcr {
    /// Offset of the `CG0NC` field.
    pub const CG0NC_SHIFT: u32 = 0;
    /// Mask for the `CG0NC` field.
    pub const CG0NC_MASK: u32 = 0b1111_1111;
    /// Offset of the `CG1NC` field.
    pub const CG1NC_SHIFT: u32 = 8;
    /// Mask for the `CG1NC` field.
    pub const CG1NC_MASK: u32 = 0b1111_1111;

    /// Returns the value of the `CG0NC` field.
    pub const fn cg0nc(self) -> u8 {
        ((self.bits() >> Self::CG0NC_SHIFT) & Self::CG0NC_MASK) as u8
    }

    /// Sets the value of the `CG0NC` field.
    pub const fn set_cg0nc(&mut self, value: u8) {
        let offset = Self::CG0NC_SHIFT;
        assert!(value & (Self::CG0NC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CG0NC_MASK << offset)) | ((value as u32) << offset),
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
            (self.bits() & !(Self::CG1NC_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `CG1NC` field set to the given value.
    pub const fn with_cg1nc(mut self, value: u8) -> Self {
        self.set_cg1nc(value);
        self
    }
}

bitflags! {
    /// `AMCNTENCLR0` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Amcntenclr0: u32 {
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

impl Amcntenclr0 {
    /// Offset of the `P<n>` field.
    pub const P_SHIFT: u32 = 0;
}

bitflags! {
    /// `AMCNTENCLR1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Amcntenclr1: u32 {
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

impl Amcntenclr1 {
    /// Offset of the `P<n>` field.
    pub const P_SHIFT: u32 = 0;
}

bitflags! {
    /// `AMCNTENSET0` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Amcntenset0: u32 {
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

impl Amcntenset0 {
    /// Offset of the `P<n>` field.
    pub const P_SHIFT: u32 = 0;
}

bitflags! {
    /// `AMCNTENSET1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Amcntenset1: u32 {
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

impl Amcntenset1 {
    /// Offset of the `P<n>` field.
    pub const P_SHIFT: u32 = 0;
}

bitflags! {
    /// `AMCR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Amcr: u32 {
        /// `HDBG` bit.
        const HDBG = 1 << 10;
        /// `CG1RZ` bit.
        const CG1RZ = 1 << 17;
    }
}

impl Amcr {
    /// Offset of the `HDBG` field.
    pub const HDBG_SHIFT: u32 = 10;
    /// Offset of the `CG1RZ` field.
    pub const CG1RZ_SHIFT: u32 = 17;
}

bitflags! {
    /// `AMEVCNTR00` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Amevcntr00: u64 {
    }
}

impl Amevcntr00 {
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

bitflags! {
    /// `AMEVCNTR01` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Amevcntr01: u64 {
    }
}

impl Amevcntr01 {
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

bitflags! {
    /// `AMEVCNTR02` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Amevcntr02: u64 {
    }
}

impl Amevcntr02 {
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

bitflags! {
    /// `AMEVCNTR03` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Amevcntr03: u64 {
    }
}

impl Amevcntr03 {
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

bitflags! {
    /// `AMEVTYPER00` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Amevtyper00: u32 {
    }
}

impl Amevtyper00 {
    /// Offset of the `evtCount` field.
    pub const EVTCOUNT_SHIFT: u32 = 0;
    /// Mask for the `evtCount` field.
    pub const EVTCOUNT_MASK: u32 = 0b1111_1111_1111_1111;

    /// Returns the value of the `evtCount` field.
    pub const fn evtcount(self) -> u16 {
        ((self.bits() >> Self::EVTCOUNT_SHIFT) & Self::EVTCOUNT_MASK) as u16
    }

    /// Sets the value of the `evtCount` field.
    pub const fn set_evtcount(&mut self, value: u16) {
        let offset = Self::EVTCOUNT_SHIFT;
        assert!(value & (Self::EVTCOUNT_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EVTCOUNT_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `evtCount` field set to the given value.
    pub const fn with_evtcount(mut self, value: u16) -> Self {
        self.set_evtcount(value);
        self
    }
}

bitflags! {
    /// `AMEVTYPER01` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Amevtyper01: u32 {
    }
}

impl Amevtyper01 {
    /// Offset of the `evtCount` field.
    pub const EVTCOUNT_SHIFT: u32 = 0;
    /// Mask for the `evtCount` field.
    pub const EVTCOUNT_MASK: u32 = 0b1111_1111_1111_1111;

    /// Returns the value of the `evtCount` field.
    pub const fn evtcount(self) -> u16 {
        ((self.bits() >> Self::EVTCOUNT_SHIFT) & Self::EVTCOUNT_MASK) as u16
    }

    /// Sets the value of the `evtCount` field.
    pub const fn set_evtcount(&mut self, value: u16) {
        let offset = Self::EVTCOUNT_SHIFT;
        assert!(value & (Self::EVTCOUNT_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EVTCOUNT_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `evtCount` field set to the given value.
    pub const fn with_evtcount(mut self, value: u16) -> Self {
        self.set_evtcount(value);
        self
    }
}

bitflags! {
    /// `AMEVTYPER02` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Amevtyper02: u32 {
    }
}

impl Amevtyper02 {
    /// Offset of the `evtCount` field.
    pub const EVTCOUNT_SHIFT: u32 = 0;
    /// Mask for the `evtCount` field.
    pub const EVTCOUNT_MASK: u32 = 0b1111_1111_1111_1111;

    /// Returns the value of the `evtCount` field.
    pub const fn evtcount(self) -> u16 {
        ((self.bits() >> Self::EVTCOUNT_SHIFT) & Self::EVTCOUNT_MASK) as u16
    }

    /// Sets the value of the `evtCount` field.
    pub const fn set_evtcount(&mut self, value: u16) {
        let offset = Self::EVTCOUNT_SHIFT;
        assert!(value & (Self::EVTCOUNT_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EVTCOUNT_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `evtCount` field set to the given value.
    pub const fn with_evtcount(mut self, value: u16) -> Self {
        self.set_evtcount(value);
        self
    }
}

bitflags! {
    /// `AMEVTYPER03` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Amevtyper03: u32 {
    }
}

impl Amevtyper03 {
    /// Offset of the `evtCount` field.
    pub const EVTCOUNT_SHIFT: u32 = 0;
    /// Mask for the `evtCount` field.
    pub const EVTCOUNT_MASK: u32 = 0b1111_1111_1111_1111;

    /// Returns the value of the `evtCount` field.
    pub const fn evtcount(self) -> u16 {
        ((self.bits() >> Self::EVTCOUNT_SHIFT) & Self::EVTCOUNT_MASK) as u16
    }

    /// Sets the value of the `evtCount` field.
    pub const fn set_evtcount(&mut self, value: u16) {
        let offset = Self::EVTCOUNT_SHIFT;
        assert!(value & (Self::EVTCOUNT_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EVTCOUNT_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `evtCount` field set to the given value.
    pub const fn with_evtcount(mut self, value: u16) -> Self {
        self.set_evtcount(value);
        self
    }
}

bitflags! {
    /// `AMEVTYPER10` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Amevtyper10: u32 {
    }
}

impl Amevtyper10 {
    /// Offset of the `evtCount` field.
    pub const EVTCOUNT_SHIFT: u32 = 0;
    /// Mask for the `evtCount` field.
    pub const EVTCOUNT_MASK: u32 = 0b1111_1111_1111_1111;

    /// Returns the value of the `evtCount` field.
    pub const fn evtcount(self) -> u16 {
        ((self.bits() >> Self::EVTCOUNT_SHIFT) & Self::EVTCOUNT_MASK) as u16
    }

    /// Sets the value of the `evtCount` field.
    pub const fn set_evtcount(&mut self, value: u16) {
        let offset = Self::EVTCOUNT_SHIFT;
        assert!(value & (Self::EVTCOUNT_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EVTCOUNT_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `evtCount` field set to the given value.
    pub const fn with_evtcount(mut self, value: u16) -> Self {
        self.set_evtcount(value);
        self
    }
}

bitflags! {
    /// `AMEVTYPER11` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Amevtyper11: u32 {
    }
}

impl Amevtyper11 {
    /// Offset of the `evtCount` field.
    pub const EVTCOUNT_SHIFT: u32 = 0;
    /// Mask for the `evtCount` field.
    pub const EVTCOUNT_MASK: u32 = 0b1111_1111_1111_1111;

    /// Returns the value of the `evtCount` field.
    pub const fn evtcount(self) -> u16 {
        ((self.bits() >> Self::EVTCOUNT_SHIFT) & Self::EVTCOUNT_MASK) as u16
    }

    /// Sets the value of the `evtCount` field.
    pub const fn set_evtcount(&mut self, value: u16) {
        let offset = Self::EVTCOUNT_SHIFT;
        assert!(value & (Self::EVTCOUNT_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EVTCOUNT_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `evtCount` field set to the given value.
    pub const fn with_evtcount(mut self, value: u16) -> Self {
        self.set_evtcount(value);
        self
    }
}

bitflags! {
    /// `AMEVTYPER110` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Amevtyper110: u32 {
    }
}

impl Amevtyper110 {
    /// Offset of the `evtCount` field.
    pub const EVTCOUNT_SHIFT: u32 = 0;
    /// Mask for the `evtCount` field.
    pub const EVTCOUNT_MASK: u32 = 0b1111_1111_1111_1111;

    /// Returns the value of the `evtCount` field.
    pub const fn evtcount(self) -> u16 {
        ((self.bits() >> Self::EVTCOUNT_SHIFT) & Self::EVTCOUNT_MASK) as u16
    }

    /// Sets the value of the `evtCount` field.
    pub const fn set_evtcount(&mut self, value: u16) {
        let offset = Self::EVTCOUNT_SHIFT;
        assert!(value & (Self::EVTCOUNT_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EVTCOUNT_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `evtCount` field set to the given value.
    pub const fn with_evtcount(mut self, value: u16) -> Self {
        self.set_evtcount(value);
        self
    }
}

bitflags! {
    /// `AMEVTYPER111` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Amevtyper111: u32 {
    }
}

impl Amevtyper111 {
    /// Offset of the `evtCount` field.
    pub const EVTCOUNT_SHIFT: u32 = 0;
    /// Mask for the `evtCount` field.
    pub const EVTCOUNT_MASK: u32 = 0b1111_1111_1111_1111;

    /// Returns the value of the `evtCount` field.
    pub const fn evtcount(self) -> u16 {
        ((self.bits() >> Self::EVTCOUNT_SHIFT) & Self::EVTCOUNT_MASK) as u16
    }

    /// Sets the value of the `evtCount` field.
    pub const fn set_evtcount(&mut self, value: u16) {
        let offset = Self::EVTCOUNT_SHIFT;
        assert!(value & (Self::EVTCOUNT_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EVTCOUNT_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `evtCount` field set to the given value.
    pub const fn with_evtcount(mut self, value: u16) -> Self {
        self.set_evtcount(value);
        self
    }
}

bitflags! {
    /// `AMEVTYPER112` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Amevtyper112: u32 {
    }
}

impl Amevtyper112 {
    /// Offset of the `evtCount` field.
    pub const EVTCOUNT_SHIFT: u32 = 0;
    /// Mask for the `evtCount` field.
    pub const EVTCOUNT_MASK: u32 = 0b1111_1111_1111_1111;

    /// Returns the value of the `evtCount` field.
    pub const fn evtcount(self) -> u16 {
        ((self.bits() >> Self::EVTCOUNT_SHIFT) & Self::EVTCOUNT_MASK) as u16
    }

    /// Sets the value of the `evtCount` field.
    pub const fn set_evtcount(&mut self, value: u16) {
        let offset = Self::EVTCOUNT_SHIFT;
        assert!(value & (Self::EVTCOUNT_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EVTCOUNT_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `evtCount` field set to the given value.
    pub const fn with_evtcount(mut self, value: u16) -> Self {
        self.set_evtcount(value);
        self
    }
}

bitflags! {
    /// `AMEVTYPER113` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Amevtyper113: u32 {
    }
}

impl Amevtyper113 {
    /// Offset of the `evtCount` field.
    pub const EVTCOUNT_SHIFT: u32 = 0;
    /// Mask for the `evtCount` field.
    pub const EVTCOUNT_MASK: u32 = 0b1111_1111_1111_1111;

    /// Returns the value of the `evtCount` field.
    pub const fn evtcount(self) -> u16 {
        ((self.bits() >> Self::EVTCOUNT_SHIFT) & Self::EVTCOUNT_MASK) as u16
    }

    /// Sets the value of the `evtCount` field.
    pub const fn set_evtcount(&mut self, value: u16) {
        let offset = Self::EVTCOUNT_SHIFT;
        assert!(value & (Self::EVTCOUNT_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EVTCOUNT_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `evtCount` field set to the given value.
    pub const fn with_evtcount(mut self, value: u16) -> Self {
        self.set_evtcount(value);
        self
    }
}

bitflags! {
    /// `AMEVTYPER114` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Amevtyper114: u32 {
    }
}

impl Amevtyper114 {
    /// Offset of the `evtCount` field.
    pub const EVTCOUNT_SHIFT: u32 = 0;
    /// Mask for the `evtCount` field.
    pub const EVTCOUNT_MASK: u32 = 0b1111_1111_1111_1111;

    /// Returns the value of the `evtCount` field.
    pub const fn evtcount(self) -> u16 {
        ((self.bits() >> Self::EVTCOUNT_SHIFT) & Self::EVTCOUNT_MASK) as u16
    }

    /// Sets the value of the `evtCount` field.
    pub const fn set_evtcount(&mut self, value: u16) {
        let offset = Self::EVTCOUNT_SHIFT;
        assert!(value & (Self::EVTCOUNT_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EVTCOUNT_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `evtCount` field set to the given value.
    pub const fn with_evtcount(mut self, value: u16) -> Self {
        self.set_evtcount(value);
        self
    }
}

bitflags! {
    /// `AMEVTYPER115` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Amevtyper115: u32 {
    }
}

impl Amevtyper115 {
    /// Offset of the `evtCount` field.
    pub const EVTCOUNT_SHIFT: u32 = 0;
    /// Mask for the `evtCount` field.
    pub const EVTCOUNT_MASK: u32 = 0b1111_1111_1111_1111;

    /// Returns the value of the `evtCount` field.
    pub const fn evtcount(self) -> u16 {
        ((self.bits() >> Self::EVTCOUNT_SHIFT) & Self::EVTCOUNT_MASK) as u16
    }

    /// Sets the value of the `evtCount` field.
    pub const fn set_evtcount(&mut self, value: u16) {
        let offset = Self::EVTCOUNT_SHIFT;
        assert!(value & (Self::EVTCOUNT_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EVTCOUNT_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `evtCount` field set to the given value.
    pub const fn with_evtcount(mut self, value: u16) -> Self {
        self.set_evtcount(value);
        self
    }
}

bitflags! {
    /// `AMEVTYPER12` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Amevtyper12: u32 {
    }
}

impl Amevtyper12 {
    /// Offset of the `evtCount` field.
    pub const EVTCOUNT_SHIFT: u32 = 0;
    /// Mask for the `evtCount` field.
    pub const EVTCOUNT_MASK: u32 = 0b1111_1111_1111_1111;

    /// Returns the value of the `evtCount` field.
    pub const fn evtcount(self) -> u16 {
        ((self.bits() >> Self::EVTCOUNT_SHIFT) & Self::EVTCOUNT_MASK) as u16
    }

    /// Sets the value of the `evtCount` field.
    pub const fn set_evtcount(&mut self, value: u16) {
        let offset = Self::EVTCOUNT_SHIFT;
        assert!(value & (Self::EVTCOUNT_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EVTCOUNT_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `evtCount` field set to the given value.
    pub const fn with_evtcount(mut self, value: u16) -> Self {
        self.set_evtcount(value);
        self
    }
}

bitflags! {
    /// `AMEVTYPER13` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Amevtyper13: u32 {
    }
}

impl Amevtyper13 {
    /// Offset of the `evtCount` field.
    pub const EVTCOUNT_SHIFT: u32 = 0;
    /// Mask for the `evtCount` field.
    pub const EVTCOUNT_MASK: u32 = 0b1111_1111_1111_1111;

    /// Returns the value of the `evtCount` field.
    pub const fn evtcount(self) -> u16 {
        ((self.bits() >> Self::EVTCOUNT_SHIFT) & Self::EVTCOUNT_MASK) as u16
    }

    /// Sets the value of the `evtCount` field.
    pub const fn set_evtcount(&mut self, value: u16) {
        let offset = Self::EVTCOUNT_SHIFT;
        assert!(value & (Self::EVTCOUNT_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EVTCOUNT_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `evtCount` field set to the given value.
    pub const fn with_evtcount(mut self, value: u16) -> Self {
        self.set_evtcount(value);
        self
    }
}

bitflags! {
    /// `AMEVTYPER14` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Amevtyper14: u32 {
    }
}

impl Amevtyper14 {
    /// Offset of the `evtCount` field.
    pub const EVTCOUNT_SHIFT: u32 = 0;
    /// Mask for the `evtCount` field.
    pub const EVTCOUNT_MASK: u32 = 0b1111_1111_1111_1111;

    /// Returns the value of the `evtCount` field.
    pub const fn evtcount(self) -> u16 {
        ((self.bits() >> Self::EVTCOUNT_SHIFT) & Self::EVTCOUNT_MASK) as u16
    }

    /// Sets the value of the `evtCount` field.
    pub const fn set_evtcount(&mut self, value: u16) {
        let offset = Self::EVTCOUNT_SHIFT;
        assert!(value & (Self::EVTCOUNT_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EVTCOUNT_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `evtCount` field set to the given value.
    pub const fn with_evtcount(mut self, value: u16) -> Self {
        self.set_evtcount(value);
        self
    }
}

bitflags! {
    /// `AMEVTYPER15` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Amevtyper15: u32 {
    }
}

impl Amevtyper15 {
    /// Offset of the `evtCount` field.
    pub const EVTCOUNT_SHIFT: u32 = 0;
    /// Mask for the `evtCount` field.
    pub const EVTCOUNT_MASK: u32 = 0b1111_1111_1111_1111;

    /// Returns the value of the `evtCount` field.
    pub const fn evtcount(self) -> u16 {
        ((self.bits() >> Self::EVTCOUNT_SHIFT) & Self::EVTCOUNT_MASK) as u16
    }

    /// Sets the value of the `evtCount` field.
    pub const fn set_evtcount(&mut self, value: u16) {
        let offset = Self::EVTCOUNT_SHIFT;
        assert!(value & (Self::EVTCOUNT_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EVTCOUNT_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `evtCount` field set to the given value.
    pub const fn with_evtcount(mut self, value: u16) -> Self {
        self.set_evtcount(value);
        self
    }
}

bitflags! {
    /// `AMEVTYPER16` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Amevtyper16: u32 {
    }
}

impl Amevtyper16 {
    /// Offset of the `evtCount` field.
    pub const EVTCOUNT_SHIFT: u32 = 0;
    /// Mask for the `evtCount` field.
    pub const EVTCOUNT_MASK: u32 = 0b1111_1111_1111_1111;

    /// Returns the value of the `evtCount` field.
    pub const fn evtcount(self) -> u16 {
        ((self.bits() >> Self::EVTCOUNT_SHIFT) & Self::EVTCOUNT_MASK) as u16
    }

    /// Sets the value of the `evtCount` field.
    pub const fn set_evtcount(&mut self, value: u16) {
        let offset = Self::EVTCOUNT_SHIFT;
        assert!(value & (Self::EVTCOUNT_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EVTCOUNT_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `evtCount` field set to the given value.
    pub const fn with_evtcount(mut self, value: u16) -> Self {
        self.set_evtcount(value);
        self
    }
}

bitflags! {
    /// `AMEVTYPER17` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Amevtyper17: u32 {
    }
}

impl Amevtyper17 {
    /// Offset of the `evtCount` field.
    pub const EVTCOUNT_SHIFT: u32 = 0;
    /// Mask for the `evtCount` field.
    pub const EVTCOUNT_MASK: u32 = 0b1111_1111_1111_1111;

    /// Returns the value of the `evtCount` field.
    pub const fn evtcount(self) -> u16 {
        ((self.bits() >> Self::EVTCOUNT_SHIFT) & Self::EVTCOUNT_MASK) as u16
    }

    /// Sets the value of the `evtCount` field.
    pub const fn set_evtcount(&mut self, value: u16) {
        let offset = Self::EVTCOUNT_SHIFT;
        assert!(value & (Self::EVTCOUNT_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EVTCOUNT_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `evtCount` field set to the given value.
    pub const fn with_evtcount(mut self, value: u16) -> Self {
        self.set_evtcount(value);
        self
    }
}

bitflags! {
    /// `AMEVTYPER18` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Amevtyper18: u32 {
    }
}

impl Amevtyper18 {
    /// Offset of the `evtCount` field.
    pub const EVTCOUNT_SHIFT: u32 = 0;
    /// Mask for the `evtCount` field.
    pub const EVTCOUNT_MASK: u32 = 0b1111_1111_1111_1111;

    /// Returns the value of the `evtCount` field.
    pub const fn evtcount(self) -> u16 {
        ((self.bits() >> Self::EVTCOUNT_SHIFT) & Self::EVTCOUNT_MASK) as u16
    }

    /// Sets the value of the `evtCount` field.
    pub const fn set_evtcount(&mut self, value: u16) {
        let offset = Self::EVTCOUNT_SHIFT;
        assert!(value & (Self::EVTCOUNT_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EVTCOUNT_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `evtCount` field set to the given value.
    pub const fn with_evtcount(mut self, value: u16) -> Self {
        self.set_evtcount(value);
        self
    }
}

bitflags! {
    /// `AMEVTYPER19` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Amevtyper19: u32 {
    }
}

impl Amevtyper19 {
    /// Offset of the `evtCount` field.
    pub const EVTCOUNT_SHIFT: u32 = 0;
    /// Mask for the `evtCount` field.
    pub const EVTCOUNT_MASK: u32 = 0b1111_1111_1111_1111;

    /// Returns the value of the `evtCount` field.
    pub const fn evtcount(self) -> u16 {
        ((self.bits() >> Self::EVTCOUNT_SHIFT) & Self::EVTCOUNT_MASK) as u16
    }

    /// Sets the value of the `evtCount` field.
    pub const fn set_evtcount(&mut self, value: u16) {
        let offset = Self::EVTCOUNT_SHIFT;
        assert!(value & (Self::EVTCOUNT_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EVTCOUNT_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `evtCount` field set to the given value.
    pub const fn with_evtcount(mut self, value: u16) -> Self {
        self.set_evtcount(value);
        self
    }
}

bitflags! {
    /// `AMUSERENR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Amuserenr: u32 {
        /// `EN` bit.
        const EN = 1 << 0;
    }
}

impl Amuserenr {
    /// Offset of the `EN` field.
    pub const EN_SHIFT: u32 = 0;
}

bitflags! {
    /// `CCSIDR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Ccsidr: u32 {
    }
}

impl Ccsidr {
    /// Offset of the `LineSize` field.
    pub const LINESIZE_SHIFT: u32 = 0;
    /// Mask for the `LineSize` field.
    pub const LINESIZE_MASK: u32 = 0b111;
    /// Offset of the `NumSets` field.
    pub const NUMSETS_SHIFT: u32 = 13;
    /// Mask for the `NumSets` field.
    pub const NUMSETS_MASK: u32 = 0b111_1111_1111_1111;

    /// Returns the value of the `LineSize` field.
    pub const fn linesize(self) -> u8 {
        ((self.bits() >> Self::LINESIZE_SHIFT) & Self::LINESIZE_MASK) as u8
    }

    /// Sets the value of the `LineSize` field.
    pub const fn set_linesize(&mut self, value: u8) {
        let offset = Self::LINESIZE_SHIFT;
        assert!(value & (Self::LINESIZE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::LINESIZE_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `LineSize` field set to the given value.
    pub const fn with_linesize(mut self, value: u8) -> Self {
        self.set_linesize(value);
        self
    }

    /// Returns the value of the `NumSets` field.
    pub const fn numsets(self) -> u16 {
        ((self.bits() >> Self::NUMSETS_SHIFT) & Self::NUMSETS_MASK) as u16
    }

    /// Sets the value of the `NumSets` field.
    pub const fn set_numsets(&mut self, value: u16) {
        let offset = Self::NUMSETS_SHIFT;
        assert!(value & (Self::NUMSETS_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::NUMSETS_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `NumSets` field set to the given value.
    pub const fn with_numsets(mut self, value: u16) -> Self {
        self.set_numsets(value);
        self
    }
}

bitflags! {
    /// `CCSIDR2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Ccsidr2: u32 {
    }
}

impl Ccsidr2 {
    /// Offset of the `NumSets` field.
    pub const NUMSETS_SHIFT: u32 = 0;
    /// Mask for the `NumSets` field.
    pub const NUMSETS_MASK: u32 = 0b1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `NumSets` field.
    pub const fn numsets(self) -> u32 {
        (self.bits() >> Self::NUMSETS_SHIFT) & Self::NUMSETS_MASK
    }

    /// Sets the value of the `NumSets` field.
    pub const fn set_numsets(&mut self, value: u32) {
        let offset = Self::NUMSETS_SHIFT;
        assert!(value & Self::NUMSETS_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::NUMSETS_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `NumSets` field set to the given value.
    pub const fn with_numsets(mut self, value: u32) -> Self {
        self.set_numsets(value);
        self
    }
}

bitflags! {
    /// `CLIDR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Clidr: u32 {
    }
}

impl Clidr {
    /// Offset of the `Ctype<n>` field.
    pub const CTYPE_SHIFT: u32 = 0;
    /// Mask for the `Ctype<n>` field.
    pub const CTYPE_MASK: u32 = 0b111;
    /// Offset of the `LoUIS` field.
    pub const LOUIS_SHIFT: u32 = 21;
    /// Mask for the `LoUIS` field.
    pub const LOUIS_MASK: u32 = 0b111;
    /// Offset of the `LoC` field.
    pub const LOC_SHIFT: u32 = 24;
    /// Mask for the `LoC` field.
    pub const LOC_MASK: u32 = 0b111;
    /// Offset of the `LoUU` field.
    pub const LOUU_SHIFT: u32 = 27;
    /// Mask for the `LoUU` field.
    pub const LOUU_MASK: u32 = 0b111;
    /// Offset of the `ICB` field.
    pub const ICB_SHIFT: u32 = 30;
    /// Mask for the `ICB` field.
    pub const ICB_MASK: u32 = 0b11;

    /// Returns the value of the given `Ctype<n>` field.
    pub const fn ctype(self, n: u32) -> u8 {
        assert!(n >= 1 && n < 8);
        ((self.bits() >> (Self::CTYPE_SHIFT + (n - 1) * 3)) & Self::CTYPE_MASK) as u8
    }

    /// Sets the value of the `Ctype<n>` field.
    pub const fn set_ctype(&mut self, n: u32, value: u8) {
        assert!(n >= 1 && n < 8);
        let offset = Self::CTYPE_SHIFT + (n - 1) * 3;
        assert!(value & (Self::CTYPE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CTYPE_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `Ctype<n>` field set to the given value.
    pub const fn with_ctype(mut self, n: u32, value: u8) -> Self {
        self.set_ctype(n, value);
        self
    }

    /// Returns the value of the `LoUIS` field.
    pub const fn louis(self) -> u8 {
        ((self.bits() >> Self::LOUIS_SHIFT) & Self::LOUIS_MASK) as u8
    }

    /// Sets the value of the `LoUIS` field.
    pub const fn set_louis(&mut self, value: u8) {
        let offset = Self::LOUIS_SHIFT;
        assert!(value & (Self::LOUIS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::LOUIS_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `LoUIS` field set to the given value.
    pub const fn with_louis(mut self, value: u8) -> Self {
        self.set_louis(value);
        self
    }

    /// Returns the value of the `LoC` field.
    pub const fn loc(self) -> u8 {
        ((self.bits() >> Self::LOC_SHIFT) & Self::LOC_MASK) as u8
    }

    /// Sets the value of the `LoC` field.
    pub const fn set_loc(&mut self, value: u8) {
        let offset = Self::LOC_SHIFT;
        assert!(value & (Self::LOC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::LOC_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `LoC` field set to the given value.
    pub const fn with_loc(mut self, value: u8) -> Self {
        self.set_loc(value);
        self
    }

    /// Returns the value of the `LoUU` field.
    pub const fn louu(self) -> u8 {
        ((self.bits() >> Self::LOUU_SHIFT) & Self::LOUU_MASK) as u8
    }

    /// Sets the value of the `LoUU` field.
    pub const fn set_louu(&mut self, value: u8) {
        let offset = Self::LOUU_SHIFT;
        assert!(value & (Self::LOUU_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::LOUU_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `LoUU` field set to the given value.
    pub const fn with_louu(mut self, value: u8) -> Self {
        self.set_louu(value);
        self
    }

    /// Returns the value of the `ICB` field.
    pub const fn icb(self) -> u8 {
        ((self.bits() >> Self::ICB_SHIFT) & Self::ICB_MASK) as u8
    }

    /// Sets the value of the `ICB` field.
    pub const fn set_icb(&mut self, value: u8) {
        let offset = Self::ICB_SHIFT;
        assert!(value & (Self::ICB_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ICB_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `ICB` field set to the given value.
    pub const fn with_icb(mut self, value: u8) -> Self {
        self.set_icb(value);
        self
    }
}

bitflags! {
    /// `CNTFRQ` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Cntfrq: u32 {
    }
}

impl Cntfrq {
    /// Offset of the `ClockFreq` field.
    pub const CLOCKFREQ_SHIFT: u32 = 0;
    /// Mask for the `ClockFreq` field.
    pub const CLOCKFREQ_MASK: u32 = 0b1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `ClockFreq` field.
    pub const fn clockfreq(self) -> u32 {
        (self.bits() >> Self::CLOCKFREQ_SHIFT) & Self::CLOCKFREQ_MASK
    }

    /// Sets the value of the `ClockFreq` field.
    pub const fn set_clockfreq(&mut self, value: u32) {
        let offset = Self::CLOCKFREQ_SHIFT;
        assert!(value & Self::CLOCKFREQ_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CLOCKFREQ_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `ClockFreq` field set to the given value.
    pub const fn with_clockfreq(mut self, value: u32) -> Self {
        self.set_clockfreq(value);
        self
    }
}

bitflags! {
    /// `CNTHCTL` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Cnthctl: u32 {
        /// `PL1PCTEN` bit.
        const PL1PCTEN = 1 << 0;
        /// `PL1PCEN` bit.
        const PL1PCEN = 1 << 1;
        /// `EVNTEN` bit.
        const EVNTEN = 1 << 2;
        /// `EVNTDIR` bit.
        const EVNTDIR = 1 << 3;
        /// `EVNTIS` bit.
        const EVNTIS = 1 << 17;
    }
}

impl Cnthctl {
    /// Offset of the `PL1PCTEN` field.
    pub const PL1PCTEN_SHIFT: u32 = 0;
    /// Offset of the `PL1PCEN` field.
    pub const PL1PCEN_SHIFT: u32 = 1;
    /// Offset of the `EVNTEN` field.
    pub const EVNTEN_SHIFT: u32 = 2;
    /// Offset of the `EVNTDIR` field.
    pub const EVNTDIR_SHIFT: u32 = 3;
    /// Offset of the `EVNTI` field.
    pub const EVNTI_SHIFT: u32 = 4;
    /// Mask for the `EVNTI` field.
    pub const EVNTI_MASK: u32 = 0b1111;
    /// Offset of the `EVNTIS` field.
    pub const EVNTIS_SHIFT: u32 = 17;

    /// Returns the value of the `EVNTI` field.
    pub const fn evnti(self) -> u8 {
        ((self.bits() >> Self::EVNTI_SHIFT) & Self::EVNTI_MASK) as u8
    }

    /// Sets the value of the `EVNTI` field.
    pub const fn set_evnti(&mut self, value: u8) {
        let offset = Self::EVNTI_SHIFT;
        assert!(value & (Self::EVNTI_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EVNTI_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `EVNTI` field set to the given value.
    pub const fn with_evnti(mut self, value: u8) -> Self {
        self.set_evnti(value);
        self
    }
}

bitflags! {
    /// `CNTHPS_CTL` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CnthpsCtl: u32 {
        /// `ENABLE` bit.
        const ENABLE = 1 << 0;
        /// `IMASK` bit.
        const IMASK = 1 << 1;
        /// `ISTATUS` bit.
        const ISTATUS = 1 << 2;
    }
}

impl CnthpsCtl {
    /// Offset of the `ENABLE` field.
    pub const ENABLE_SHIFT: u32 = 0;
    /// Offset of the `IMASK` field.
    pub const IMASK_SHIFT: u32 = 1;
    /// Offset of the `ISTATUS` field.
    pub const ISTATUS_SHIFT: u32 = 2;
}

bitflags! {
    /// `CNTHPS_CVAL` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CnthpsCval: u64 {
    }
}

impl CnthpsCval {
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
    /// `CNTHPS_TVAL` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CnthpsTval: u32 {
    }
}

impl CnthpsTval {
    /// Offset of the `TimerValue` field.
    pub const TIMERVALUE_SHIFT: u32 = 0;
    /// Mask for the `TimerValue` field.
    pub const TIMERVALUE_MASK: u32 = 0b1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `TimerValue` field.
    pub const fn timervalue(self) -> u32 {
        (self.bits() >> Self::TIMERVALUE_SHIFT) & Self::TIMERVALUE_MASK
    }

    /// Sets the value of the `TimerValue` field.
    pub const fn set_timervalue(&mut self, value: u32) {
        let offset = Self::TIMERVALUE_SHIFT;
        assert!(value & Self::TIMERVALUE_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TIMERVALUE_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `TimerValue` field set to the given value.
    pub const fn with_timervalue(mut self, value: u32) -> Self {
        self.set_timervalue(value);
        self
    }
}

/// `CNTHP_CTL` system register value.
pub type CnthpCtl = CnthpsCtl;

/// `CNTHP_CVAL` system register value.
pub type CnthpCval = CnthpsCval;

/// `CNTHP_TVAL` system register value.
pub type CnthpTval = CnthpsTval;

bitflags! {
    /// `CNTHVS_CTL` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CnthvsCtl: u32 {
        /// `ENABLE` bit.
        const ENABLE = 1 << 0;
        /// `IMASK` bit.
        const IMASK = 1 << 1;
        /// `ISTATUS` bit.
        const ISTATUS = 1 << 2;
    }
}

impl CnthvsCtl {
    /// Offset of the `ENABLE` field.
    pub const ENABLE_SHIFT: u32 = 0;
    /// Offset of the `IMASK` field.
    pub const IMASK_SHIFT: u32 = 1;
    /// Offset of the `ISTATUS` field.
    pub const ISTATUS_SHIFT: u32 = 2;
}

bitflags! {
    /// `CNTHVS_CVAL` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CnthvsCval: u64 {
    }
}

impl CnthvsCval {
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
    /// `CNTHVS_TVAL` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CnthvsTval: u32 {
    }
}

impl CnthvsTval {
    /// Offset of the `TimerValue` field.
    pub const TIMERVALUE_SHIFT: u32 = 0;
    /// Mask for the `TimerValue` field.
    pub const TIMERVALUE_MASK: u32 = 0b1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `TimerValue` field.
    pub const fn timervalue(self) -> u32 {
        (self.bits() >> Self::TIMERVALUE_SHIFT) & Self::TIMERVALUE_MASK
    }

    /// Sets the value of the `TimerValue` field.
    pub const fn set_timervalue(&mut self, value: u32) {
        let offset = Self::TIMERVALUE_SHIFT;
        assert!(value & Self::TIMERVALUE_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TIMERVALUE_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `TimerValue` field set to the given value.
    pub const fn with_timervalue(mut self, value: u32) -> Self {
        self.set_timervalue(value);
        self
    }
}

/// `CNTHV_CTL` system register value.
pub type CnthvCtl = CnthvsCtl;

/// `CNTHV_CVAL` system register value.
pub type CnthvCval = CnthvsCval;

/// `CNTHV_TVAL` system register value.
pub type CnthvTval = CnthvsTval;

bitflags! {
    /// `CNTKCTL` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Cntkctl: u32 {
        /// `PL0PCTEN` bit.
        const PL0PCTEN = 1 << 0;
        /// `PL0VCTEN` bit.
        const PL0VCTEN = 1 << 1;
        /// `EVNTEN` bit.
        const EVNTEN = 1 << 2;
        /// `EVNTDIR` bit.
        const EVNTDIR = 1 << 3;
        /// `PL0VTEN` bit.
        const PL0VTEN = 1 << 8;
        /// `PL0PTEN` bit.
        const PL0PTEN = 1 << 9;
        /// `EVNTIS` bit.
        const EVNTIS = 1 << 17;
    }
}

impl Cntkctl {
    /// Offset of the `PL0PCTEN` field.
    pub const PL0PCTEN_SHIFT: u32 = 0;
    /// Offset of the `PL0VCTEN` field.
    pub const PL0VCTEN_SHIFT: u32 = 1;
    /// Offset of the `EVNTEN` field.
    pub const EVNTEN_SHIFT: u32 = 2;
    /// Offset of the `EVNTDIR` field.
    pub const EVNTDIR_SHIFT: u32 = 3;
    /// Offset of the `EVNTI` field.
    pub const EVNTI_SHIFT: u32 = 4;
    /// Mask for the `EVNTI` field.
    pub const EVNTI_MASK: u32 = 0b1111;
    /// Offset of the `PL0VTEN` field.
    pub const PL0VTEN_SHIFT: u32 = 8;
    /// Offset of the `PL0PTEN` field.
    pub const PL0PTEN_SHIFT: u32 = 9;
    /// Offset of the `EVNTIS` field.
    pub const EVNTIS_SHIFT: u32 = 17;

    /// Returns the value of the `EVNTI` field.
    pub const fn evnti(self) -> u8 {
        ((self.bits() >> Self::EVNTI_SHIFT) & Self::EVNTI_MASK) as u8
    }

    /// Sets the value of the `EVNTI` field.
    pub const fn set_evnti(&mut self, value: u8) {
        let offset = Self::EVNTI_SHIFT;
        assert!(value & (Self::EVNTI_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EVNTI_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `EVNTI` field set to the given value.
    pub const fn with_evnti(mut self, value: u8) -> Self {
        self.set_evnti(value);
        self
    }
}

bitflags! {
    /// `CNTPCT` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Cntpct: u64 {
    }
}

impl Cntpct {
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
    /// `CNTPCTSS` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Cntpctss: u64 {
    }
}

impl Cntpctss {
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

/// `CNTP_CTL` system register value.
pub type CntpCtl = CnthpsCtl;

/// `CNTP_CVAL` system register value.
pub type CntpCval = CnthpsCval;

/// `CNTP_TVAL` system register value.
pub type CntpTval = CnthpsTval;

bitflags! {
    /// `CNTVCT` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Cntvct: u64 {
    }
}

impl Cntvct {
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

bitflags! {
    /// `CNTVCTSS` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Cntvctss: u64 {
    }
}

impl Cntvctss {
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
    /// `CNTVOFF` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Cntvoff: u64 {
    }
}

impl Cntvoff {
    /// Offset of the `VOffset` field.
    pub const VOFFSET_SHIFT: u32 = 0;
    /// Mask for the `VOffset` field.
    pub const VOFFSET_MASK: u64 =
        0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `VOffset` field.
    pub const fn voffset(self) -> u64 {
        (self.bits() >> Self::VOFFSET_SHIFT) & Self::VOFFSET_MASK
    }

    /// Sets the value of the `VOffset` field.
    pub const fn set_voffset(&mut self, value: u64) {
        let offset = Self::VOFFSET_SHIFT;
        assert!(value & Self::VOFFSET_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VOFFSET_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `VOffset` field set to the given value.
    pub const fn with_voffset(mut self, value: u64) -> Self {
        self.set_voffset(value);
        self
    }
}

/// `CNTV_CTL` system register value.
pub type CntvCtl = CnthvsCtl;

/// `CNTV_CVAL` system register value.
pub type CntvCval = CnthvsCval;

/// `CNTV_TVAL` system register value.
pub type CntvTval = CnthvsTval;

bitflags! {
    /// `CONTEXTIDR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Contextidr: u32 {
    }
}

impl Contextidr {
    /// Offset of the `ASID` field.
    pub const ASID_SHIFT: u32 = 0;
    /// Mask for the `ASID` field.
    pub const ASID_MASK: u32 = 0b1111_1111;

    /// Returns the value of the `ASID` field.
    pub const fn asid(self) -> u8 {
        ((self.bits() >> Self::ASID_SHIFT) & Self::ASID_MASK) as u8
    }

    /// Sets the value of the `ASID` field.
    pub const fn set_asid(&mut self, value: u8) {
        let offset = Self::ASID_SHIFT;
        assert!(value & (Self::ASID_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ASID_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `ASID` field set to the given value.
    pub const fn with_asid(mut self, value: u8) -> Self {
        self.set_asid(value);
        self
    }
}

bitflags! {
    /// `CPACR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Cpacr: u32 {
        /// `TRCDIS` bit.
        const TRCDIS = 1 << 28;
        /// `ASEDIS` bit.
        const ASEDIS = 1 << 31;
    }
}

impl Cpacr {
    /// Offset of the `cp10` field.
    pub const CP10_SHIFT: u32 = 20;
    /// Mask for the `cp10` field.
    pub const CP10_MASK: u32 = 0b11;
    /// Offset of the `cp11` field.
    pub const CP11_SHIFT: u32 = 22;
    /// Mask for the `cp11` field.
    pub const CP11_MASK: u32 = 0b11;
    /// Offset of the `TRCDIS` field.
    pub const TRCDIS_SHIFT: u32 = 28;
    /// Offset of the `ASEDIS` field.
    pub const ASEDIS_SHIFT: u32 = 31;

    /// Returns the value of the `cp10` field.
    pub const fn cp10(self) -> u8 {
        ((self.bits() >> Self::CP10_SHIFT) & Self::CP10_MASK) as u8
    }

    /// Sets the value of the `cp10` field.
    pub const fn set_cp10(&mut self, value: u8) {
        let offset = Self::CP10_SHIFT;
        assert!(value & (Self::CP10_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CP10_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `cp10` field set to the given value.
    pub const fn with_cp10(mut self, value: u8) -> Self {
        self.set_cp10(value);
        self
    }

    /// Returns the value of the `cp11` field.
    pub const fn cp11(self) -> u8 {
        ((self.bits() >> Self::CP11_SHIFT) & Self::CP11_MASK) as u8
    }

    /// Sets the value of the `cp11` field.
    pub const fn set_cp11(&mut self, value: u8) {
        let offset = Self::CP11_SHIFT;
        assert!(value & (Self::CP11_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CP11_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `cp11` field set to the given value.
    pub const fn with_cp11(mut self, value: u8) -> Self {
        self.set_cp11(value);
        self
    }
}

bitflags! {
    /// `CSSELR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Csselr: u32 {
        /// `InD` bit.
        const IND = 1 << 0;
    }
}

impl Csselr {
    /// Offset of the `InD` field.
    pub const IND_SHIFT: u32 = 0;
    /// Offset of the `Level` field.
    pub const LEVEL_SHIFT: u32 = 1;
    /// Mask for the `Level` field.
    pub const LEVEL_MASK: u32 = 0b111;

    /// Returns the value of the `Level` field.
    pub const fn level(self) -> u8 {
        ((self.bits() >> Self::LEVEL_SHIFT) & Self::LEVEL_MASK) as u8
    }

    /// Sets the value of the `Level` field.
    pub const fn set_level(&mut self, value: u8) {
        let offset = Self::LEVEL_SHIFT;
        assert!(value & (Self::LEVEL_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::LEVEL_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `Level` field set to the given value.
    pub const fn with_level(mut self, value: u8) -> Self {
        self.set_level(value);
        self
    }
}

bitflags! {
    /// `CTR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Ctr: u32 {
        /// RES1 bits in the `CTR` register.
        const RES1 = 0b1000_0000_0000_0000_0000_0000_0000_0000;
        /// `IDC` bit.
        const IDC = 1 << 28;
        /// `DIC` bit.
        const DIC = 1 << 29;
    }
}

impl Ctr {
    /// Offset of the `IminLine` field.
    pub const IMINLINE_SHIFT: u32 = 0;
    /// Mask for the `IminLine` field.
    pub const IMINLINE_MASK: u32 = 0b1111;
    /// Offset of the `L1Ip` field.
    pub const L1IP_SHIFT: u32 = 14;
    /// Mask for the `L1Ip` field.
    pub const L1IP_MASK: u32 = 0b11;
    /// Offset of the `DminLine` field.
    pub const DMINLINE_SHIFT: u32 = 16;
    /// Mask for the `DminLine` field.
    pub const DMINLINE_MASK: u32 = 0b1111;
    /// Offset of the `ERG` field.
    pub const ERG_SHIFT: u32 = 20;
    /// Mask for the `ERG` field.
    pub const ERG_MASK: u32 = 0b1111;
    /// Offset of the `CWG` field.
    pub const CWG_SHIFT: u32 = 24;
    /// Mask for the `CWG` field.
    pub const CWG_MASK: u32 = 0b1111;
    /// Offset of the `IDC` field.
    pub const IDC_SHIFT: u32 = 28;
    /// Offset of the `DIC` field.
    pub const DIC_SHIFT: u32 = 29;

    /// Returns the value of the `IminLine` field.
    pub const fn iminline(self) -> u8 {
        ((self.bits() >> Self::IMINLINE_SHIFT) & Self::IMINLINE_MASK) as u8
    }

    /// Sets the value of the `IminLine` field.
    pub const fn set_iminline(&mut self, value: u8) {
        let offset = Self::IMINLINE_SHIFT;
        assert!(value & (Self::IMINLINE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::IMINLINE_MASK << offset)) | ((value as u32) << offset),
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
            (self.bits() & !(Self::L1IP_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `L1Ip` field set to the given value.
    pub const fn with_l1ip(mut self, value: u8) -> Self {
        self.set_l1ip(value);
        self
    }

    /// Returns the value of the `DminLine` field.
    pub const fn dminline(self) -> u8 {
        ((self.bits() >> Self::DMINLINE_SHIFT) & Self::DMINLINE_MASK) as u8
    }

    /// Sets the value of the `DminLine` field.
    pub const fn set_dminline(&mut self, value: u8) {
        let offset = Self::DMINLINE_SHIFT;
        assert!(value & (Self::DMINLINE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::DMINLINE_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `DminLine` field set to the given value.
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
            (self.bits() & !(Self::ERG_MASK << offset)) | ((value as u32) << offset),
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
            (self.bits() & !(Self::CWG_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `CWG` field set to the given value.
    pub const fn with_cwg(mut self, value: u8) -> Self {
        self.set_cwg(value);
        self
    }
}

bitflags! {
    /// `DACR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dacr: u32 {
    }
}

impl Dacr {
    /// Offset of the `D<n>` field.
    pub const D_SHIFT: u32 = 0;
    /// Mask for the `D<n>` field.
    pub const D_MASK: u32 = 0b11;

    /// Returns the value of the given `D<n>` field.
    pub const fn d(self, n: u32) -> u8 {
        assert!(n < 16);
        ((self.bits() >> (Self::D_SHIFT + n * 2)) & Self::D_MASK) as u8
    }

    /// Sets the value of the `D<n>` field.
    pub const fn set_d(&mut self, n: u32, value: u8) {
        assert!(n < 16);
        let offset = Self::D_SHIFT + n * 2;
        assert!(value & (Self::D_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::D_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `D<n>` field set to the given value.
    pub const fn with_d(mut self, n: u32, value: u8) -> Self {
        self.set_d(n, value);
        self
    }
}

bitflags! {
    /// `DBGAUTHSTATUS` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dbgauthstatus: u32 {
    }
}

impl Dbgauthstatus {
    /// Offset of the `NSID` field.
    pub const NSID_SHIFT: u32 = 0;
    /// Mask for the `NSID` field.
    pub const NSID_MASK: u32 = 0b11;
    /// Offset of the `NSNID` field.
    pub const NSNID_SHIFT: u32 = 2;
    /// Mask for the `NSNID` field.
    pub const NSNID_MASK: u32 = 0b11;
    /// Offset of the `SID` field.
    pub const SID_SHIFT: u32 = 4;
    /// Mask for the `SID` field.
    pub const SID_MASK: u32 = 0b11;
    /// Offset of the `SNID` field.
    pub const SNID_SHIFT: u32 = 6;
    /// Mask for the `SNID` field.
    pub const SNID_MASK: u32 = 0b11;

    /// Returns the value of the `NSID` field.
    pub const fn nsid(self) -> u8 {
        ((self.bits() >> Self::NSID_SHIFT) & Self::NSID_MASK) as u8
    }

    /// Sets the value of the `NSID` field.
    pub const fn set_nsid(&mut self, value: u8) {
        let offset = Self::NSID_SHIFT;
        assert!(value & (Self::NSID_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::NSID_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `NSID` field set to the given value.
    pub const fn with_nsid(mut self, value: u8) -> Self {
        self.set_nsid(value);
        self
    }

    /// Returns the value of the `NSNID` field.
    pub const fn nsnid(self) -> u8 {
        ((self.bits() >> Self::NSNID_SHIFT) & Self::NSNID_MASK) as u8
    }

    /// Sets the value of the `NSNID` field.
    pub const fn set_nsnid(&mut self, value: u8) {
        let offset = Self::NSNID_SHIFT;
        assert!(value & (Self::NSNID_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::NSNID_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `NSNID` field set to the given value.
    pub const fn with_nsnid(mut self, value: u8) -> Self {
        self.set_nsnid(value);
        self
    }

    /// Returns the value of the `SID` field.
    pub const fn sid(self) -> u8 {
        ((self.bits() >> Self::SID_SHIFT) & Self::SID_MASK) as u8
    }

    /// Sets the value of the `SID` field.
    pub const fn set_sid(&mut self, value: u8) {
        let offset = Self::SID_SHIFT;
        assert!(value & (Self::SID_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SID_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `SID` field set to the given value.
    pub const fn with_sid(mut self, value: u8) -> Self {
        self.set_sid(value);
        self
    }

    /// Returns the value of the `SNID` field.
    pub const fn snid(self) -> u8 {
        ((self.bits() >> Self::SNID_SHIFT) & Self::SNID_MASK) as u8
    }

    /// Sets the value of the `SNID` field.
    pub const fn set_snid(&mut self, value: u8) {
        let offset = Self::SNID_SHIFT;
        assert!(value & (Self::SNID_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SNID_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `SNID` field set to the given value.
    pub const fn with_snid(mut self, value: u8) -> Self {
        self.set_snid(value);
        self
    }
}

bitflags! {
    /// `DBGCLAIMCLR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dbgclaimclr: u32 {
        /// `CLAIM<m>` bit 0.
        const CLAIM0 = 1 << 0;
        /// `CLAIM<m>` bit 1.
        const CLAIM1 = 1 << 1;
        /// `CLAIM<m>` bit 2.
        const CLAIM2 = 1 << 2;
        /// `CLAIM<m>` bit 3.
        const CLAIM3 = 1 << 3;
        /// `CLAIM<m>` bit 4.
        const CLAIM4 = 1 << 4;
        /// `CLAIM<m>` bit 5.
        const CLAIM5 = 1 << 5;
        /// `CLAIM<m>` bit 6.
        const CLAIM6 = 1 << 6;
        /// `CLAIM<m>` bit 7.
        const CLAIM7 = 1 << 7;
    }
}

impl Dbgclaimclr {
    /// Offset of the `CLAIM<m>` field.
    pub const CLAIM_SHIFT: u32 = 0;
}

bitflags! {
    /// `DBGCLAIMSET` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dbgclaimset: u32 {
        /// `CLAIM<m>` bit 0.
        const CLAIM0 = 1 << 0;
        /// `CLAIM<m>` bit 1.
        const CLAIM1 = 1 << 1;
        /// `CLAIM<m>` bit 2.
        const CLAIM2 = 1 << 2;
        /// `CLAIM<m>` bit 3.
        const CLAIM3 = 1 << 3;
        /// `CLAIM<m>` bit 4.
        const CLAIM4 = 1 << 4;
        /// `CLAIM<m>` bit 5.
        const CLAIM5 = 1 << 5;
        /// `CLAIM<m>` bit 6.
        const CLAIM6 = 1 << 6;
        /// `CLAIM<m>` bit 7.
        const CLAIM7 = 1 << 7;
    }
}

impl Dbgclaimset {
    /// Offset of the `CLAIM<m>` field.
    pub const CLAIM_SHIFT: u32 = 0;
}

bitflags! {
    /// `DBGDCCINT` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dbgdccint: u32 {
        /// `TX` bit.
        const TX = 1 << 29;
        /// `RX` bit.
        const RX = 1 << 30;
    }
}

impl Dbgdccint {
    /// Offset of the `TX` field.
    pub const TX_SHIFT: u32 = 29;
    /// Offset of the `RX` field.
    pub const RX_SHIFT: u32 = 30;
}

bitflags! {
    /// `DBGDEVID` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dbgdevid: u32 {
    }
}

impl Dbgdevid {
    /// Offset of the `PCSample` field.
    pub const PCSAMPLE_SHIFT: u32 = 0;
    /// Mask for the `PCSample` field.
    pub const PCSAMPLE_MASK: u32 = 0b1111;
    /// Offset of the `WPAddrMask` field.
    pub const WPADDRMASK_SHIFT: u32 = 4;
    /// Mask for the `WPAddrMask` field.
    pub const WPADDRMASK_MASK: u32 = 0b1111;
    /// Offset of the `BPAddrMask` field.
    pub const BPADDRMASK_SHIFT: u32 = 8;
    /// Mask for the `BPAddrMask` field.
    pub const BPADDRMASK_MASK: u32 = 0b1111;
    /// Offset of the `VectorCatch` field.
    pub const VECTORCATCH_SHIFT: u32 = 12;
    /// Mask for the `VectorCatch` field.
    pub const VECTORCATCH_MASK: u32 = 0b1111;
    /// Offset of the `VirtExtns` field.
    pub const VIRTEXTNS_SHIFT: u32 = 16;
    /// Mask for the `VirtExtns` field.
    pub const VIRTEXTNS_MASK: u32 = 0b1111;
    /// Offset of the `DoubleLock` field.
    pub const DOUBLELOCK_SHIFT: u32 = 20;
    /// Mask for the `DoubleLock` field.
    pub const DOUBLELOCK_MASK: u32 = 0b1111;
    /// Offset of the `AuxRegs` field.
    pub const AUXREGS_SHIFT: u32 = 24;
    /// Mask for the `AuxRegs` field.
    pub const AUXREGS_MASK: u32 = 0b1111;
    /// Offset of the `CIDMask` field.
    pub const CIDMASK_SHIFT: u32 = 28;
    /// Mask for the `CIDMask` field.
    pub const CIDMASK_MASK: u32 = 0b1111;

    /// Returns the value of the `PCSample` field.
    pub const fn pcsample(self) -> u8 {
        ((self.bits() >> Self::PCSAMPLE_SHIFT) & Self::PCSAMPLE_MASK) as u8
    }

    /// Sets the value of the `PCSample` field.
    pub const fn set_pcsample(&mut self, value: u8) {
        let offset = Self::PCSAMPLE_SHIFT;
        assert!(value & (Self::PCSAMPLE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PCSAMPLE_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `PCSample` field set to the given value.
    pub const fn with_pcsample(mut self, value: u8) -> Self {
        self.set_pcsample(value);
        self
    }

    /// Returns the value of the `WPAddrMask` field.
    pub const fn wpaddrmask(self) -> u8 {
        ((self.bits() >> Self::WPADDRMASK_SHIFT) & Self::WPADDRMASK_MASK) as u8
    }

    /// Sets the value of the `WPAddrMask` field.
    pub const fn set_wpaddrmask(&mut self, value: u8) {
        let offset = Self::WPADDRMASK_SHIFT;
        assert!(value & (Self::WPADDRMASK_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::WPADDRMASK_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `WPAddrMask` field set to the given value.
    pub const fn with_wpaddrmask(mut self, value: u8) -> Self {
        self.set_wpaddrmask(value);
        self
    }

    /// Returns the value of the `BPAddrMask` field.
    pub const fn bpaddrmask(self) -> u8 {
        ((self.bits() >> Self::BPADDRMASK_SHIFT) & Self::BPADDRMASK_MASK) as u8
    }

    /// Sets the value of the `BPAddrMask` field.
    pub const fn set_bpaddrmask(&mut self, value: u8) {
        let offset = Self::BPADDRMASK_SHIFT;
        assert!(value & (Self::BPADDRMASK_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BPADDRMASK_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `BPAddrMask` field set to the given value.
    pub const fn with_bpaddrmask(mut self, value: u8) -> Self {
        self.set_bpaddrmask(value);
        self
    }

    /// Returns the value of the `VectorCatch` field.
    pub const fn vectorcatch(self) -> u8 {
        ((self.bits() >> Self::VECTORCATCH_SHIFT) & Self::VECTORCATCH_MASK) as u8
    }

    /// Sets the value of the `VectorCatch` field.
    pub const fn set_vectorcatch(&mut self, value: u8) {
        let offset = Self::VECTORCATCH_SHIFT;
        assert!(value & (Self::VECTORCATCH_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VECTORCATCH_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `VectorCatch` field set to the given value.
    pub const fn with_vectorcatch(mut self, value: u8) -> Self {
        self.set_vectorcatch(value);
        self
    }

    /// Returns the value of the `VirtExtns` field.
    pub const fn virtextns(self) -> u8 {
        ((self.bits() >> Self::VIRTEXTNS_SHIFT) & Self::VIRTEXTNS_MASK) as u8
    }

    /// Sets the value of the `VirtExtns` field.
    pub const fn set_virtextns(&mut self, value: u8) {
        let offset = Self::VIRTEXTNS_SHIFT;
        assert!(value & (Self::VIRTEXTNS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VIRTEXTNS_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `VirtExtns` field set to the given value.
    pub const fn with_virtextns(mut self, value: u8) -> Self {
        self.set_virtextns(value);
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
            (self.bits() & !(Self::DOUBLELOCK_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `DoubleLock` field set to the given value.
    pub const fn with_doublelock(mut self, value: u8) -> Self {
        self.set_doublelock(value);
        self
    }

    /// Returns the value of the `AuxRegs` field.
    pub const fn auxregs(self) -> u8 {
        ((self.bits() >> Self::AUXREGS_SHIFT) & Self::AUXREGS_MASK) as u8
    }

    /// Sets the value of the `AuxRegs` field.
    pub const fn set_auxregs(&mut self, value: u8) {
        let offset = Self::AUXREGS_SHIFT;
        assert!(value & (Self::AUXREGS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::AUXREGS_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `AuxRegs` field set to the given value.
    pub const fn with_auxregs(mut self, value: u8) -> Self {
        self.set_auxregs(value);
        self
    }

    /// Returns the value of the `CIDMask` field.
    pub const fn cidmask(self) -> u8 {
        ((self.bits() >> Self::CIDMASK_SHIFT) & Self::CIDMASK_MASK) as u8
    }

    /// Sets the value of the `CIDMask` field.
    pub const fn set_cidmask(&mut self, value: u8) {
        let offset = Self::CIDMASK_SHIFT;
        assert!(value & (Self::CIDMASK_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CIDMASK_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `CIDMask` field set to the given value.
    pub const fn with_cidmask(mut self, value: u8) -> Self {
        self.set_cidmask(value);
        self
    }
}

bitflags! {
    /// `DBGDEVID1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dbgdevid1: u32 {
    }
}

impl Dbgdevid1 {
    /// Offset of the `PCSROffset` field.
    pub const PCSROFFSET_SHIFT: u32 = 0;
    /// Mask for the `PCSROffset` field.
    pub const PCSROFFSET_MASK: u32 = 0b1111;

    /// Returns the value of the `PCSROffset` field.
    pub const fn pcsroffset(self) -> u8 {
        ((self.bits() >> Self::PCSROFFSET_SHIFT) & Self::PCSROFFSET_MASK) as u8
    }

    /// Sets the value of the `PCSROffset` field.
    pub const fn set_pcsroffset(&mut self, value: u8) {
        let offset = Self::PCSROFFSET_SHIFT;
        assert!(value & (Self::PCSROFFSET_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PCSROFFSET_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `PCSROffset` field set to the given value.
    pub const fn with_pcsroffset(mut self, value: u8) -> Self {
        self.set_pcsroffset(value);
        self
    }
}

bitflags! {
    /// `DBGDIDR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dbgdidr: u32 {
        /// RES1 bits in the `DBGDIDR` register.
        const RES1 = 0b1000_0000_0000_0000;
        /// `SE_imp` bit.
        const SE_IMP = 1 << 12;
        /// `nSUHD_imp` bit.
        const NSUHD_IMP = 1 << 14;
    }
}

impl Dbgdidr {
    /// Offset of the `SE_imp` field.
    pub const SE_IMP_SHIFT: u32 = 12;
    /// Offset of the `nSUHD_imp` field.
    pub const NSUHD_IMP_SHIFT: u32 = 14;
    /// Offset of the `Version` field.
    pub const VERSION_SHIFT: u32 = 16;
    /// Mask for the `Version` field.
    pub const VERSION_MASK: u32 = 0b1111;
    /// Offset of the `CTX_CMPs` field.
    pub const CTX_CMPS_SHIFT: u32 = 20;
    /// Mask for the `CTX_CMPs` field.
    pub const CTX_CMPS_MASK: u32 = 0b1111;
    /// Offset of the `BRPs` field.
    pub const BRPS_SHIFT: u32 = 24;
    /// Mask for the `BRPs` field.
    pub const BRPS_MASK: u32 = 0b1111;
    /// Offset of the `WRPs` field.
    pub const WRPS_SHIFT: u32 = 28;
    /// Mask for the `WRPs` field.
    pub const WRPS_MASK: u32 = 0b1111;

    /// Returns the value of the `Version` field.
    pub const fn version(self) -> u8 {
        ((self.bits() >> Self::VERSION_SHIFT) & Self::VERSION_MASK) as u8
    }

    /// Sets the value of the `Version` field.
    pub const fn set_version(&mut self, value: u8) {
        let offset = Self::VERSION_SHIFT;
        assert!(value & (Self::VERSION_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VERSION_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `Version` field set to the given value.
    pub const fn with_version(mut self, value: u8) -> Self {
        self.set_version(value);
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
            (self.bits() & !(Self::CTX_CMPS_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `CTX_CMPs` field set to the given value.
    pub const fn with_ctx_cmps(mut self, value: u8) -> Self {
        self.set_ctx_cmps(value);
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
            (self.bits() & !(Self::BRPS_MASK << offset)) | ((value as u32) << offset),
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
            (self.bits() & !(Self::WRPS_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `WRPs` field set to the given value.
    pub const fn with_wrps(mut self, value: u8) -> Self {
        self.set_wrps(value);
        self
    }
}

bitflags! {
    /// `DBGDRAR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dbgdrar: u64 {
    }
}

impl Dbgdrar {
    /// Offset of the `Valid` field.
    pub const VALID_SHIFT: u32 = 0;
    /// Mask for the `Valid` field.
    pub const VALID_MASK: u64 = 0b11;
    /// Offset of the `ROMADDR[47:12]` field.
    pub const ROMADDR_47_12_SHIFT: u32 = 12;
    /// Mask for the `ROMADDR[47:12]` field.
    pub const ROMADDR_47_12_MASK: u64 = 0b1111_1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `Valid` field.
    pub const fn valid(self) -> u8 {
        ((self.bits() >> Self::VALID_SHIFT) & Self::VALID_MASK) as u8
    }

    /// Sets the value of the `Valid` field.
    pub const fn set_valid(&mut self, value: u8) {
        let offset = Self::VALID_SHIFT;
        assert!(value & (Self::VALID_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VALID_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `Valid` field set to the given value.
    pub const fn with_valid(mut self, value: u8) -> Self {
        self.set_valid(value);
        self
    }

    /// Returns the value of the `ROMADDR[47:12]` field.
    pub const fn romaddr_47_12(self) -> u64 {
        (self.bits() >> Self::ROMADDR_47_12_SHIFT) & Self::ROMADDR_47_12_MASK
    }

    /// Sets the value of the `ROMADDR[47:12]` field.
    pub const fn set_romaddr_47_12(&mut self, value: u64) {
        let offset = Self::ROMADDR_47_12_SHIFT;
        assert!(value & Self::ROMADDR_47_12_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ROMADDR_47_12_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `ROMADDR[47:12]` field set to the given value.
    pub const fn with_romaddr_47_12(mut self, value: u64) -> Self {
        self.set_romaddr_47_12(value);
        self
    }
}

bitflags! {
    /// `DBGDSCRext` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dbgdscrext: u32 {
        /// `ERR` bit.
        const ERR = 1 << 6;
        /// `UDCCdis` bit.
        const UDCCDIS = 1 << 12;
        /// `HDE` bit.
        const HDE = 1 << 14;
        /// `MDBGen` bit.
        const MDBGEN = 1 << 15;
        /// `SPIDdis` bit.
        const SPIDDIS = 1 << 16;
        /// `SPNIDdis` bit.
        const SPNIDDIS = 1 << 17;
        /// `NS` bit.
        const NS = 1 << 18;
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
    }
}

impl Dbgdscrext {
    /// Offset of the `MOE` field.
    pub const MOE_SHIFT: u32 = 2;
    /// Mask for the `MOE` field.
    pub const MOE_MASK: u32 = 0b1111;
    /// Offset of the `ERR` field.
    pub const ERR_SHIFT: u32 = 6;
    /// Offset of the `UDCCdis` field.
    pub const UDCCDIS_SHIFT: u32 = 12;
    /// Offset of the `HDE` field.
    pub const HDE_SHIFT: u32 = 14;
    /// Offset of the `MDBGen` field.
    pub const MDBGEN_SHIFT: u32 = 15;
    /// Offset of the `SPIDdis` field.
    pub const SPIDDIS_SHIFT: u32 = 16;
    /// Offset of the `SPNIDdis` field.
    pub const SPNIDDIS_SHIFT: u32 = 17;
    /// Offset of the `NS` field.
    pub const NS_SHIFT: u32 = 18;
    /// Offset of the `SC2` field.
    pub const SC2_SHIFT: u32 = 19;
    /// Offset of the `TDA` field.
    pub const TDA_SHIFT: u32 = 21;
    /// Offset of the `INTdis` field.
    pub const INTDIS_SHIFT: u32 = 22;
    /// Mask for the `INTdis` field.
    pub const INTDIS_MASK: u32 = 0b11;
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

    /// Returns the value of the `MOE` field.
    pub const fn moe(self) -> u8 {
        ((self.bits() >> Self::MOE_SHIFT) & Self::MOE_MASK) as u8
    }

    /// Sets the value of the `MOE` field.
    pub const fn set_moe(&mut self, value: u8) {
        let offset = Self::MOE_SHIFT;
        assert!(value & (Self::MOE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::MOE_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `MOE` field set to the given value.
    pub const fn with_moe(mut self, value: u8) -> Self {
        self.set_moe(value);
        self
    }

    /// Returns the value of the `INTdis` field.
    pub const fn intdis(self) -> u8 {
        ((self.bits() >> Self::INTDIS_SHIFT) & Self::INTDIS_MASK) as u8
    }

    /// Sets the value of the `INTdis` field.
    pub const fn set_intdis(&mut self, value: u8) {
        let offset = Self::INTDIS_SHIFT;
        assert!(value & (Self::INTDIS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::INTDIS_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `INTdis` field set to the given value.
    pub const fn with_intdis(mut self, value: u8) -> Self {
        self.set_intdis(value);
        self
    }
}

bitflags! {
    /// `DBGDSCRint` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dbgdscrint: u32 {
        /// `UDCCdis` bit.
        const UDCCDIS = 1 << 12;
        /// `MDBGen` bit.
        const MDBGEN = 1 << 15;
        /// `SPIDdis` bit.
        const SPIDDIS = 1 << 16;
        /// `SPNIDdis` bit.
        const SPNIDDIS = 1 << 17;
        /// `NS` bit.
        const NS = 1 << 18;
        /// `TXfull` bit.
        const TXFULL = 1 << 29;
        /// `RXfull` bit.
        const RXFULL = 1 << 30;
    }
}

impl Dbgdscrint {
    /// Offset of the `MOE` field.
    pub const MOE_SHIFT: u32 = 2;
    /// Mask for the `MOE` field.
    pub const MOE_MASK: u32 = 0b1111;
    /// Offset of the `UDCCdis` field.
    pub const UDCCDIS_SHIFT: u32 = 12;
    /// Offset of the `MDBGen` field.
    pub const MDBGEN_SHIFT: u32 = 15;
    /// Offset of the `SPIDdis` field.
    pub const SPIDDIS_SHIFT: u32 = 16;
    /// Offset of the `SPNIDdis` field.
    pub const SPNIDDIS_SHIFT: u32 = 17;
    /// Offset of the `NS` field.
    pub const NS_SHIFT: u32 = 18;
    /// Offset of the `TXfull` field.
    pub const TXFULL_SHIFT: u32 = 29;
    /// Offset of the `RXfull` field.
    pub const RXFULL_SHIFT: u32 = 30;

    /// Returns the value of the `MOE` field.
    pub const fn moe(self) -> u8 {
        ((self.bits() >> Self::MOE_SHIFT) & Self::MOE_MASK) as u8
    }

    /// Sets the value of the `MOE` field.
    pub const fn set_moe(&mut self, value: u8) {
        let offset = Self::MOE_SHIFT;
        assert!(value & (Self::MOE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::MOE_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `MOE` field set to the given value.
    pub const fn with_moe(mut self, value: u8) -> Self {
        self.set_moe(value);
        self
    }
}

bitflags! {
    /// `DBGDTRRXext` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dbgdtrrxext: u32 {
    }
}

impl Dbgdtrrxext {
    /// Offset of the `DTRRX` field.
    pub const DTRRX_SHIFT: u32 = 0;
    /// Mask for the `DTRRX` field.
    pub const DTRRX_MASK: u32 = 0b1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `DTRRX` field.
    pub const fn dtrrx(self) -> u32 {
        (self.bits() >> Self::DTRRX_SHIFT) & Self::DTRRX_MASK
    }

    /// Sets the value of the `DTRRX` field.
    pub const fn set_dtrrx(&mut self, value: u32) {
        let offset = Self::DTRRX_SHIFT;
        assert!(value & Self::DTRRX_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::DTRRX_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `DTRRX` field set to the given value.
    pub const fn with_dtrrx(mut self, value: u32) -> Self {
        self.set_dtrrx(value);
        self
    }
}

bitflags! {
    /// `DBGDTRRXint` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dbgdtrrxint: u32 {
    }
}

impl Dbgdtrrxint {
    /// Offset of the `DTRRX` field.
    pub const DTRRX_SHIFT: u32 = 0;
    /// Mask for the `DTRRX` field.
    pub const DTRRX_MASK: u32 = 0b1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `DTRRX` field.
    pub const fn dtrrx(self) -> u32 {
        (self.bits() >> Self::DTRRX_SHIFT) & Self::DTRRX_MASK
    }

    /// Sets the value of the `DTRRX` field.
    pub const fn set_dtrrx(&mut self, value: u32) {
        let offset = Self::DTRRX_SHIFT;
        assert!(value & Self::DTRRX_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::DTRRX_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `DTRRX` field set to the given value.
    pub const fn with_dtrrx(mut self, value: u32) -> Self {
        self.set_dtrrx(value);
        self
    }
}

bitflags! {
    /// `DBGDTRTXext` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dbgdtrtxext: u32 {
    }
}

impl Dbgdtrtxext {
    /// Offset of the `DTRTX` field.
    pub const DTRTX_SHIFT: u32 = 0;
    /// Mask for the `DTRTX` field.
    pub const DTRTX_MASK: u32 = 0b1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `DTRTX` field.
    pub const fn dtrtx(self) -> u32 {
        (self.bits() >> Self::DTRTX_SHIFT) & Self::DTRTX_MASK
    }

    /// Sets the value of the `DTRTX` field.
    pub const fn set_dtrtx(&mut self, value: u32) {
        let offset = Self::DTRTX_SHIFT;
        assert!(value & Self::DTRTX_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::DTRTX_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `DTRTX` field set to the given value.
    pub const fn with_dtrtx(mut self, value: u32) -> Self {
        self.set_dtrtx(value);
        self
    }
}

bitflags! {
    /// `DBGDTRTXint` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dbgdtrtxint: u32 {
    }
}

impl Dbgdtrtxint {
    /// Offset of the `DTRTX` field.
    pub const DTRTX_SHIFT: u32 = 0;
    /// Mask for the `DTRTX` field.
    pub const DTRTX_MASK: u32 = 0b1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `DTRTX` field.
    pub const fn dtrtx(self) -> u32 {
        (self.bits() >> Self::DTRTX_SHIFT) & Self::DTRTX_MASK
    }

    /// Sets the value of the `DTRTX` field.
    pub const fn set_dtrtx(&mut self, value: u32) {
        let offset = Self::DTRTX_SHIFT;
        assert!(value & Self::DTRTX_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::DTRTX_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `DTRTX` field set to the given value.
    pub const fn with_dtrtx(mut self, value: u32) -> Self {
        self.set_dtrtx(value);
        self
    }
}

bitflags! {
    /// `DBGOSDLR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dbgosdlr: u32 {
        /// `DLK` bit.
        const DLK = 1 << 0;
    }
}

impl Dbgosdlr {
    /// Offset of the `DLK` field.
    pub const DLK_SHIFT: u32 = 0;
}

bitflags! {
    /// `DBGOSECCR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dbgoseccr: u32 {
    }
}

impl Dbgoseccr {
    /// Offset of the `EDECCR` field.
    pub const EDECCR_SHIFT: u32 = 0;
    /// Mask for the `EDECCR` field.
    pub const EDECCR_MASK: u32 = 0b1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `EDECCR` field.
    pub const fn edeccr(self) -> u32 {
        (self.bits() >> Self::EDECCR_SHIFT) & Self::EDECCR_MASK
    }

    /// Sets the value of the `EDECCR` field.
    pub const fn set_edeccr(&mut self, value: u32) {
        let offset = Self::EDECCR_SHIFT;
        assert!(value & Self::EDECCR_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EDECCR_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `EDECCR` field set to the given value.
    pub const fn with_edeccr(mut self, value: u32) -> Self {
        self.set_edeccr(value);
        self
    }
}

bitflags! {
    /// `DBGOSLAR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dbgoslar: u32 {
    }
}

impl Dbgoslar {
    /// Offset of the `OSLA` field.
    pub const OSLA_SHIFT: u32 = 0;
    /// Mask for the `OSLA` field.
    pub const OSLA_MASK: u32 = 0b1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `OSLA` field.
    pub const fn osla(self) -> u32 {
        (self.bits() >> Self::OSLA_SHIFT) & Self::OSLA_MASK
    }

    /// Sets the value of the `OSLA` field.
    pub const fn set_osla(&mut self, value: u32) {
        let offset = Self::OSLA_SHIFT;
        assert!(value & Self::OSLA_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::OSLA_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `OSLA` field set to the given value.
    pub const fn with_osla(mut self, value: u32) -> Self {
        self.set_osla(value);
        self
    }
}

bitflags! {
    /// `DBGOSLSR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dbgoslsr: u32 {
        /// `OSLK` bit.
        const OSLK = 1 << 1;
        /// `nTT` bit.
        const NTT = 1 << 2;
    }
}

impl Dbgoslsr {
    /// Offset of the `OSLK` field.
    pub const OSLK_SHIFT: u32 = 1;
    /// Offset of the `nTT` field.
    pub const NTT_SHIFT: u32 = 2;
}

bitflags! {
    /// `DBGPRCR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dbgprcr: u32 {
        /// `CORENPDRQ` bit.
        const CORENPDRQ = 1 << 0;
    }
}

impl Dbgprcr {
    /// Offset of the `CORENPDRQ` field.
    pub const CORENPDRQ_SHIFT: u32 = 0;
}

bitflags! {
    /// `DBGVCR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dbgvcr: u32 {
        /// `SU` bit.
        const SU = 1 << 1;
        /// `U` bit.
        const U = 1 << 1;
        /// `S` bit.
        const S = 1 << 2;
        /// `SS` bit.
        const SS = 1 << 2;
        /// `P` bit.
        const P = 1 << 3;
        /// `SP` bit.
        const SP = 1 << 3;
        /// `D` bit.
        const D = 1 << 4;
        /// `SD` bit.
        const SD = 1 << 4;
        /// `I` bit.
        const I = 1 << 6;
        /// `SI` bit.
        const SI = 1 << 6;
        /// `F` bit.
        const F = 1 << 7;
        /// `SF` bit.
        const SF = 1 << 7;
        /// `MS` bit.
        const MS = 1 << 10;
        /// `MP` bit.
        const MP = 1 << 11;
        /// `MD` bit.
        const MD = 1 << 12;
        /// `MI` bit.
        const MI = 1 << 14;
        /// `MF` bit.
        const MF = 1 << 15;
        /// `NSU` bit.
        const NSU = 1 << 25;
        /// `NSS` bit.
        const NSS = 1 << 26;
        /// `NSP` bit.
        const NSP = 1 << 27;
        /// `NSD` bit.
        const NSD = 1 << 28;
        /// `NSI` bit.
        const NSI = 1 << 30;
        /// `NSF` bit.
        const NSF = 1 << 31;
    }
}

impl Dbgvcr {
    /// Offset of the `SU` field.
    pub const SU_SHIFT: u32 = 1;
    /// Offset of the `U` field.
    pub const U_SHIFT: u32 = 1;
    /// Offset of the `S` field.
    pub const S_SHIFT: u32 = 2;
    /// Offset of the `SS` field.
    pub const SS_SHIFT: u32 = 2;
    /// Offset of the `P` field.
    pub const P_SHIFT: u32 = 3;
    /// Offset of the `SP` field.
    pub const SP_SHIFT: u32 = 3;
    /// Offset of the `D` field.
    pub const D_SHIFT: u32 = 4;
    /// Offset of the `SD` field.
    pub const SD_SHIFT: u32 = 4;
    /// Offset of the `I` field.
    pub const I_SHIFT: u32 = 6;
    /// Offset of the `SI` field.
    pub const SI_SHIFT: u32 = 6;
    /// Offset of the `F` field.
    pub const F_SHIFT: u32 = 7;
    /// Offset of the `SF` field.
    pub const SF_SHIFT: u32 = 7;
    /// Offset of the `MS` field.
    pub const MS_SHIFT: u32 = 10;
    /// Offset of the `MP` field.
    pub const MP_SHIFT: u32 = 11;
    /// Offset of the `MD` field.
    pub const MD_SHIFT: u32 = 12;
    /// Offset of the `MI` field.
    pub const MI_SHIFT: u32 = 14;
    /// Offset of the `MF` field.
    pub const MF_SHIFT: u32 = 15;
    /// Offset of the `NSU` field.
    pub const NSU_SHIFT: u32 = 25;
    /// Offset of the `NSS` field.
    pub const NSS_SHIFT: u32 = 26;
    /// Offset of the `NSP` field.
    pub const NSP_SHIFT: u32 = 27;
    /// Offset of the `NSD` field.
    pub const NSD_SHIFT: u32 = 28;
    /// Offset of the `NSI` field.
    pub const NSI_SHIFT: u32 = 30;
    /// Offset of the `NSF` field.
    pub const NSF_SHIFT: u32 = 31;
}

bitflags! {
    /// `DFAR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dfar: u32 {
    }
}

impl Dfar {
    /// Offset of the `VA` field.
    pub const VA_SHIFT: u32 = 0;
    /// Mask for the `VA` field.
    pub const VA_MASK: u32 = 0b1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `VA` field.
    pub const fn va(self) -> u32 {
        (self.bits() >> Self::VA_SHIFT) & Self::VA_MASK
    }

    /// Sets the value of the `VA` field.
    pub const fn set_va(&mut self, value: u32) {
        let offset = Self::VA_SHIFT;
        assert!(value & Self::VA_MASK == value);
        *self =
            Self::from_bits_retain((self.bits() & !(Self::VA_MASK << offset)) | (value << offset));
    }

    /// Returns a copy with the `VA` field set to the given value.
    pub const fn with_va(mut self, value: u32) -> Self {
        self.set_va(value);
        self
    }
}

bitflags! {
    /// `DFSR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dfsr: u32 {
        /// `LPAE` bit.
        const LPAE = 1 << 9;
        /// `WnR` bit.
        const WNR = 1 << 11;
        /// `ExT` bit.
        const EXT = 1 << 12;
        /// `CM` bit.
        const CM = 1 << 13;
        /// `FnV` bit.
        const FNV = 1 << 16;
    }
}

impl Dfsr {
    /// Offset of the `STATUS` field.
    pub const STATUS_SHIFT: u32 = 0;
    /// Mask for the `STATUS` field.
    pub const STATUS_MASK: u32 = 0b11_1111;
    /// Offset of the `Domain` field.
    pub const DOMAIN_SHIFT: u32 = 4;
    /// Mask for the `Domain` field.
    pub const DOMAIN_MASK: u32 = 0b1111;
    /// Offset of the `LPAE` field.
    pub const LPAE_SHIFT: u32 = 9;
    /// Offset of the `WnR` field.
    pub const WNR_SHIFT: u32 = 11;
    /// Offset of the `ExT` field.
    pub const EXT_SHIFT: u32 = 12;
    /// Offset of the `CM` field.
    pub const CM_SHIFT: u32 = 13;
    /// Offset of the `AET` field.
    pub const AET_SHIFT: u32 = 14;
    /// Mask for the `AET` field.
    pub const AET_MASK: u32 = 0b11;
    /// Offset of the `FnV` field.
    pub const FNV_SHIFT: u32 = 16;

    /// Returns the value of the `STATUS` field.
    pub const fn status(self) -> u8 {
        ((self.bits() >> Self::STATUS_SHIFT) & Self::STATUS_MASK) as u8
    }

    /// Sets the value of the `STATUS` field.
    pub const fn set_status(&mut self, value: u8) {
        let offset = Self::STATUS_SHIFT;
        assert!(value & (Self::STATUS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::STATUS_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `STATUS` field set to the given value.
    pub const fn with_status(mut self, value: u8) -> Self {
        self.set_status(value);
        self
    }

    /// Returns the value of the `Domain` field.
    pub const fn domain(self) -> u8 {
        ((self.bits() >> Self::DOMAIN_SHIFT) & Self::DOMAIN_MASK) as u8
    }

    /// Sets the value of the `Domain` field.
    pub const fn set_domain(&mut self, value: u8) {
        let offset = Self::DOMAIN_SHIFT;
        assert!(value & (Self::DOMAIN_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::DOMAIN_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `Domain` field set to the given value.
    pub const fn with_domain(mut self, value: u8) -> Self {
        self.set_domain(value);
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
            (self.bits() & !(Self::AET_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `AET` field set to the given value.
    pub const fn with_aet(mut self, value: u8) -> Self {
        self.set_aet(value);
        self
    }
}

bitflags! {
    /// `DISR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Disr: u32 {
        /// `EA` bit.
        const EA = 1 << 9;
        /// `LPAE` bit.
        const LPAE = 1 << 9;
        /// `ExT` bit.
        const EXT = 1 << 12;
        /// `A` bit.
        const A = 1 << 31;
    }
}

impl Disr {
    /// Offset of the `DFSC` field.
    pub const DFSC_SHIFT: u32 = 0;
    /// Mask for the `DFSC` field.
    pub const DFSC_MASK: u32 = 0b11_1111;
    /// Offset of the `STATUS` field.
    pub const STATUS_SHIFT: u32 = 0;
    /// Mask for the `STATUS` field.
    pub const STATUS_MASK: u32 = 0b11_1111;
    /// Offset of the `EA` field.
    pub const EA_SHIFT: u32 = 9;
    /// Offset of the `LPAE` field.
    pub const LPAE_SHIFT: u32 = 9;
    /// Offset of the `ExT` field.
    pub const EXT_SHIFT: u32 = 12;
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
            (self.bits() & !(Self::DFSC_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `DFSC` field set to the given value.
    pub const fn with_dfsc(mut self, value: u8) -> Self {
        self.set_dfsc(value);
        self
    }

    /// Returns the value of the `STATUS` field.
    pub const fn status(self) -> u8 {
        ((self.bits() >> Self::STATUS_SHIFT) & Self::STATUS_MASK) as u8
    }

    /// Sets the value of the `STATUS` field.
    pub const fn set_status(&mut self, value: u8) {
        let offset = Self::STATUS_SHIFT;
        assert!(value & (Self::STATUS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::STATUS_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `STATUS` field set to the given value.
    pub const fn with_status(mut self, value: u8) -> Self {
        self.set_status(value);
        self
    }
}

bitflags! {
    /// `DLR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dlr: u32 {
    }
}

impl Dlr {
    /// Offset of the `ADDR` field.
    pub const ADDR_SHIFT: u32 = 0;
    /// Mask for the `ADDR` field.
    pub const ADDR_MASK: u32 = 0b1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `ADDR` field.
    pub const fn addr(self) -> u32 {
        (self.bits() >> Self::ADDR_SHIFT) & Self::ADDR_MASK
    }

    /// Sets the value of the `ADDR` field.
    pub const fn set_addr(&mut self, value: u32) {
        let offset = Self::ADDR_SHIFT;
        assert!(value & Self::ADDR_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ADDR_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `ADDR` field set to the given value.
    pub const fn with_addr(mut self, value: u32) -> Self {
        self.set_addr(value);
        self
    }
}

bitflags! {
    /// `DSPSR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dspsr: u32 {
        /// `T` bit.
        const T = 1 << 5;
        /// `F` bit.
        const F = 1 << 6;
        /// `I` bit.
        const I = 1 << 7;
        /// `A` bit.
        const A = 1 << 8;
        /// `E` bit.
        const E = 1 << 9;
        /// `IL` bit.
        const IL = 1 << 20;
        /// `SS` bit.
        const SS = 1 << 21;
        /// `PAN` bit.
        const PAN = 1 << 22;
        /// `SSBS` bit.
        const SSBS = 1 << 23;
        /// `DIT` bit.
        const DIT = 1 << 24;
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
    }
}

impl Dspsr {
    /// Offset of the `M[4:0]` field.
    pub const M_4_0_SHIFT: u32 = 0;
    /// Mask for the `M[4:0]` field.
    pub const M_4_0_MASK: u32 = 0b1_1111;
    /// Offset of the `T` field.
    pub const T_SHIFT: u32 = 5;
    /// Offset of the `F` field.
    pub const F_SHIFT: u32 = 6;
    /// Offset of the `I` field.
    pub const I_SHIFT: u32 = 7;
    /// Offset of the `A` field.
    pub const A_SHIFT: u32 = 8;
    /// Offset of the `E` field.
    pub const E_SHIFT: u32 = 9;
    /// Offset of the `GE` field.
    pub const GE_SHIFT: u32 = 16;
    /// Mask for the `GE` field.
    pub const GE_MASK: u32 = 0b1111;
    /// Offset of the `IL` field.
    pub const IL_SHIFT: u32 = 20;
    /// Offset of the `SS` field.
    pub const SS_SHIFT: u32 = 21;
    /// Offset of the `PAN` field.
    pub const PAN_SHIFT: u32 = 22;
    /// Offset of the `SSBS` field.
    pub const SSBS_SHIFT: u32 = 23;
    /// Offset of the `DIT` field.
    pub const DIT_SHIFT: u32 = 24;
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

    /// Returns the value of the `M[4:0]` field.
    pub const fn m_4_0(self) -> u8 {
        ((self.bits() >> Self::M_4_0_SHIFT) & Self::M_4_0_MASK) as u8
    }

    /// Sets the value of the `M[4:0]` field.
    pub const fn set_m_4_0(&mut self, value: u8) {
        let offset = Self::M_4_0_SHIFT;
        assert!(value & (Self::M_4_0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::M_4_0_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `M[4:0]` field set to the given value.
    pub const fn with_m_4_0(mut self, value: u8) -> Self {
        self.set_m_4_0(value);
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
            (self.bits() & !(Self::GE_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `GE` field set to the given value.
    pub const fn with_ge(mut self, value: u8) -> Self {
        self.set_ge(value);
        self
    }
}

bitflags! {
    /// `DSPSR2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dspsr2: u32 {
        /// `UINJ` bit.
        const UINJ = 1 << 4;
    }
}

impl Dspsr2 {
    /// Offset of the `UINJ` field.
    pub const UINJ_SHIFT: u32 = 4;
}

#[cfg(feature = "el2")]
bitflags! {
    /// `ELR_hyp` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ElrHyp: u32 {
    }
}

#[cfg(feature = "el2")]
impl ElrHyp {
    /// Offset of the `ADDR` field.
    pub const ADDR_SHIFT: u32 = 0;
    /// Mask for the `ADDR` field.
    pub const ADDR_MASK: u32 = 0b1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `ADDR` field.
    pub const fn addr(self) -> u32 {
        (self.bits() >> Self::ADDR_SHIFT) & Self::ADDR_MASK
    }

    /// Sets the value of the `ADDR` field.
    pub const fn set_addr(&mut self, value: u32) {
        let offset = Self::ADDR_SHIFT;
        assert!(value & Self::ADDR_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ADDR_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `ADDR` field set to the given value.
    pub const fn with_addr(mut self, value: u32) -> Self {
        self.set_addr(value);
        self
    }
}

bitflags! {
    /// `ERRIDR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Erridr: u32 {
    }
}

impl Erridr {
    /// Offset of the `NUM` field.
    pub const NUM_SHIFT: u32 = 0;
    /// Mask for the `NUM` field.
    pub const NUM_MASK: u32 = 0b1111_1111_1111_1111;

    /// Returns the value of the `NUM` field.
    pub const fn num(self) -> u16 {
        ((self.bits() >> Self::NUM_SHIFT) & Self::NUM_MASK) as u16
    }

    /// Sets the value of the `NUM` field.
    pub const fn set_num(&mut self, value: u16) {
        let offset = Self::NUM_SHIFT;
        assert!(value & (Self::NUM_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::NUM_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `NUM` field set to the given value.
    pub const fn with_num(mut self, value: u16) -> Self {
        self.set_num(value);
        self
    }
}

bitflags! {
    /// `ERRSELR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Errselr: u32 {
    }
}

impl Errselr {
    /// Offset of the `SEL` field.
    pub const SEL_SHIFT: u32 = 0;
    /// Mask for the `SEL` field.
    pub const SEL_MASK: u32 = 0b1111_1111_1111_1111;

    /// Returns the value of the `SEL` field.
    pub const fn sel(self) -> u16 {
        ((self.bits() >> Self::SEL_SHIFT) & Self::SEL_MASK) as u16
    }

    /// Sets the value of the `SEL` field.
    pub const fn set_sel(&mut self, value: u16) {
        let offset = Self::SEL_SHIFT;
        assert!(value & (Self::SEL_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SEL_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `SEL` field set to the given value.
    pub const fn with_sel(mut self, value: u16) -> Self {
        self.set_sel(value);
        self
    }
}

bitflags! {
    /// `ERXADDR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Erxaddr: u32 {
    }
}

impl Erxaddr {
    /// Offset of the `ERRnADDRlo` field.
    pub const ERRNADDRLO_SHIFT: u32 = 0;
    /// Mask for the `ERRnADDRlo` field.
    pub const ERRNADDRLO_MASK: u32 = 0b1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `ERRnADDRlo` field.
    pub const fn errnaddrlo(self) -> u32 {
        (self.bits() >> Self::ERRNADDRLO_SHIFT) & Self::ERRNADDRLO_MASK
    }

    /// Sets the value of the `ERRnADDRlo` field.
    pub const fn set_errnaddrlo(&mut self, value: u32) {
        let offset = Self::ERRNADDRLO_SHIFT;
        assert!(value & Self::ERRNADDRLO_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ERRNADDRLO_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `ERRnADDRlo` field set to the given value.
    pub const fn with_errnaddrlo(mut self, value: u32) -> Self {
        self.set_errnaddrlo(value);
        self
    }
}

bitflags! {
    /// `ERXADDR2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Erxaddr2: u32 {
    }
}

impl Erxaddr2 {
    /// Offset of the `ERRnADDRhi` field.
    pub const ERRNADDRHI_SHIFT: u32 = 0;
    /// Mask for the `ERRnADDRhi` field.
    pub const ERRNADDRHI_MASK: u32 = 0b1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `ERRnADDRhi` field.
    pub const fn errnaddrhi(self) -> u32 {
        (self.bits() >> Self::ERRNADDRHI_SHIFT) & Self::ERRNADDRHI_MASK
    }

    /// Sets the value of the `ERRnADDRhi` field.
    pub const fn set_errnaddrhi(&mut self, value: u32) {
        let offset = Self::ERRNADDRHI_SHIFT;
        assert!(value & Self::ERRNADDRHI_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ERRNADDRHI_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `ERRnADDRhi` field set to the given value.
    pub const fn with_errnaddrhi(mut self, value: u32) -> Self {
        self.set_errnaddrhi(value);
        self
    }
}

bitflags! {
    /// `ERXCTLR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Erxctlr: u32 {
    }
}

impl Erxctlr {
    /// Offset of the `ERRnCTLRlo` field.
    pub const ERRNCTLRLO_SHIFT: u32 = 0;
    /// Mask for the `ERRnCTLRlo` field.
    pub const ERRNCTLRLO_MASK: u32 = 0b1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `ERRnCTLRlo` field.
    pub const fn errnctlrlo(self) -> u32 {
        (self.bits() >> Self::ERRNCTLRLO_SHIFT) & Self::ERRNCTLRLO_MASK
    }

    /// Sets the value of the `ERRnCTLRlo` field.
    pub const fn set_errnctlrlo(&mut self, value: u32) {
        let offset = Self::ERRNCTLRLO_SHIFT;
        assert!(value & Self::ERRNCTLRLO_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ERRNCTLRLO_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `ERRnCTLRlo` field set to the given value.
    pub const fn with_errnctlrlo(mut self, value: u32) -> Self {
        self.set_errnctlrlo(value);
        self
    }
}

bitflags! {
    /// `ERXCTLR2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Erxctlr2: u32 {
    }
}

impl Erxctlr2 {
    /// Offset of the `ERRnCTLRhi` field.
    pub const ERRNCTLRHI_SHIFT: u32 = 0;
    /// Mask for the `ERRnCTLRhi` field.
    pub const ERRNCTLRHI_MASK: u32 = 0b1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `ERRnCTLRhi` field.
    pub const fn errnctlrhi(self) -> u32 {
        (self.bits() >> Self::ERRNCTLRHI_SHIFT) & Self::ERRNCTLRHI_MASK
    }

    /// Sets the value of the `ERRnCTLRhi` field.
    pub const fn set_errnctlrhi(&mut self, value: u32) {
        let offset = Self::ERRNCTLRHI_SHIFT;
        assert!(value & Self::ERRNCTLRHI_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ERRNCTLRHI_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `ERRnCTLRhi` field set to the given value.
    pub const fn with_errnctlrhi(mut self, value: u32) -> Self {
        self.set_errnctlrhi(value);
        self
    }
}

bitflags! {
    /// `ERXFR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Erxfr: u32 {
    }
}

impl Erxfr {
    /// Offset of the `ERRnFRlo` field.
    pub const ERRNFRLO_SHIFT: u32 = 0;
    /// Mask for the `ERRnFRlo` field.
    pub const ERRNFRLO_MASK: u32 = 0b1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `ERRnFRlo` field.
    pub const fn errnfrlo(self) -> u32 {
        (self.bits() >> Self::ERRNFRLO_SHIFT) & Self::ERRNFRLO_MASK
    }

    /// Sets the value of the `ERRnFRlo` field.
    pub const fn set_errnfrlo(&mut self, value: u32) {
        let offset = Self::ERRNFRLO_SHIFT;
        assert!(value & Self::ERRNFRLO_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ERRNFRLO_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `ERRnFRlo` field set to the given value.
    pub const fn with_errnfrlo(mut self, value: u32) -> Self {
        self.set_errnfrlo(value);
        self
    }
}

bitflags! {
    /// `ERXFR2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Erxfr2: u32 {
    }
}

impl Erxfr2 {
    /// Offset of the `ERRnFRhi` field.
    pub const ERRNFRHI_SHIFT: u32 = 0;
    /// Mask for the `ERRnFRhi` field.
    pub const ERRNFRHI_MASK: u32 = 0b1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `ERRnFRhi` field.
    pub const fn errnfrhi(self) -> u32 {
        (self.bits() >> Self::ERRNFRHI_SHIFT) & Self::ERRNFRHI_MASK
    }

    /// Sets the value of the `ERRnFRhi` field.
    pub const fn set_errnfrhi(&mut self, value: u32) {
        let offset = Self::ERRNFRHI_SHIFT;
        assert!(value & Self::ERRNFRHI_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ERRNFRHI_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `ERRnFRhi` field set to the given value.
    pub const fn with_errnfrhi(mut self, value: u32) -> Self {
        self.set_errnfrhi(value);
        self
    }
}

bitflags! {
    /// `ERXMISC0` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Erxmisc0: u32 {
    }
}

impl Erxmisc0 {
    /// Offset of the `ERRnMISC0lo` field.
    pub const ERRNMISC0LO_SHIFT: u32 = 0;
    /// Mask for the `ERRnMISC0lo` field.
    pub const ERRNMISC0LO_MASK: u32 = 0b1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `ERRnMISC0lo` field.
    pub const fn errnmisc0lo(self) -> u32 {
        (self.bits() >> Self::ERRNMISC0LO_SHIFT) & Self::ERRNMISC0LO_MASK
    }

    /// Sets the value of the `ERRnMISC0lo` field.
    pub const fn set_errnmisc0lo(&mut self, value: u32) {
        let offset = Self::ERRNMISC0LO_SHIFT;
        assert!(value & Self::ERRNMISC0LO_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ERRNMISC0LO_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `ERRnMISC0lo` field set to the given value.
    pub const fn with_errnmisc0lo(mut self, value: u32) -> Self {
        self.set_errnmisc0lo(value);
        self
    }
}

bitflags! {
    /// `ERXMISC1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Erxmisc1: u32 {
    }
}

impl Erxmisc1 {
    /// Offset of the `ERRnMISC0hi` field.
    pub const ERRNMISC0HI_SHIFT: u32 = 0;
    /// Mask for the `ERRnMISC0hi` field.
    pub const ERRNMISC0HI_MASK: u32 = 0b1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `ERRnMISC0hi` field.
    pub const fn errnmisc0hi(self) -> u32 {
        (self.bits() >> Self::ERRNMISC0HI_SHIFT) & Self::ERRNMISC0HI_MASK
    }

    /// Sets the value of the `ERRnMISC0hi` field.
    pub const fn set_errnmisc0hi(&mut self, value: u32) {
        let offset = Self::ERRNMISC0HI_SHIFT;
        assert!(value & Self::ERRNMISC0HI_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ERRNMISC0HI_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `ERRnMISC0hi` field set to the given value.
    pub const fn with_errnmisc0hi(mut self, value: u32) -> Self {
        self.set_errnmisc0hi(value);
        self
    }
}

bitflags! {
    /// `ERXMISC2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Erxmisc2: u32 {
    }
}

impl Erxmisc2 {
    /// Offset of the `ERRnMISC1lo` field.
    pub const ERRNMISC1LO_SHIFT: u32 = 0;
    /// Mask for the `ERRnMISC1lo` field.
    pub const ERRNMISC1LO_MASK: u32 = 0b1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `ERRnMISC1lo` field.
    pub const fn errnmisc1lo(self) -> u32 {
        (self.bits() >> Self::ERRNMISC1LO_SHIFT) & Self::ERRNMISC1LO_MASK
    }

    /// Sets the value of the `ERRnMISC1lo` field.
    pub const fn set_errnmisc1lo(&mut self, value: u32) {
        let offset = Self::ERRNMISC1LO_SHIFT;
        assert!(value & Self::ERRNMISC1LO_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ERRNMISC1LO_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `ERRnMISC1lo` field set to the given value.
    pub const fn with_errnmisc1lo(mut self, value: u32) -> Self {
        self.set_errnmisc1lo(value);
        self
    }
}

bitflags! {
    /// `ERXMISC3` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Erxmisc3: u32 {
    }
}

impl Erxmisc3 {
    /// Offset of the `ERRnMISC1hi` field.
    pub const ERRNMISC1HI_SHIFT: u32 = 0;
    /// Mask for the `ERRnMISC1hi` field.
    pub const ERRNMISC1HI_MASK: u32 = 0b1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `ERRnMISC1hi` field.
    pub const fn errnmisc1hi(self) -> u32 {
        (self.bits() >> Self::ERRNMISC1HI_SHIFT) & Self::ERRNMISC1HI_MASK
    }

    /// Sets the value of the `ERRnMISC1hi` field.
    pub const fn set_errnmisc1hi(&mut self, value: u32) {
        let offset = Self::ERRNMISC1HI_SHIFT;
        assert!(value & Self::ERRNMISC1HI_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ERRNMISC1HI_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `ERRnMISC1hi` field set to the given value.
    pub const fn with_errnmisc1hi(mut self, value: u32) -> Self {
        self.set_errnmisc1hi(value);
        self
    }
}

bitflags! {
    /// `ERXMISC4` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Erxmisc4: u32 {
    }
}

impl Erxmisc4 {
    /// Offset of the `ERRnMISC2lo` field.
    pub const ERRNMISC2LO_SHIFT: u32 = 0;
    /// Mask for the `ERRnMISC2lo` field.
    pub const ERRNMISC2LO_MASK: u32 = 0b1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `ERRnMISC2lo` field.
    pub const fn errnmisc2lo(self) -> u32 {
        (self.bits() >> Self::ERRNMISC2LO_SHIFT) & Self::ERRNMISC2LO_MASK
    }

    /// Sets the value of the `ERRnMISC2lo` field.
    pub const fn set_errnmisc2lo(&mut self, value: u32) {
        let offset = Self::ERRNMISC2LO_SHIFT;
        assert!(value & Self::ERRNMISC2LO_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ERRNMISC2LO_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `ERRnMISC2lo` field set to the given value.
    pub const fn with_errnmisc2lo(mut self, value: u32) -> Self {
        self.set_errnmisc2lo(value);
        self
    }
}

bitflags! {
    /// `ERXMISC5` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Erxmisc5: u32 {
    }
}

impl Erxmisc5 {
    /// Offset of the `ERRnMISC2hi` field.
    pub const ERRNMISC2HI_SHIFT: u32 = 0;
    /// Mask for the `ERRnMISC2hi` field.
    pub const ERRNMISC2HI_MASK: u32 = 0b1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `ERRnMISC2hi` field.
    pub const fn errnmisc2hi(self) -> u32 {
        (self.bits() >> Self::ERRNMISC2HI_SHIFT) & Self::ERRNMISC2HI_MASK
    }

    /// Sets the value of the `ERRnMISC2hi` field.
    pub const fn set_errnmisc2hi(&mut self, value: u32) {
        let offset = Self::ERRNMISC2HI_SHIFT;
        assert!(value & Self::ERRNMISC2HI_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ERRNMISC2HI_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `ERRnMISC2hi` field set to the given value.
    pub const fn with_errnmisc2hi(mut self, value: u32) -> Self {
        self.set_errnmisc2hi(value);
        self
    }
}

bitflags! {
    /// `ERXMISC6` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Erxmisc6: u32 {
    }
}

impl Erxmisc6 {
    /// Offset of the `ERRnMISC3lo` field.
    pub const ERRNMISC3LO_SHIFT: u32 = 0;
    /// Mask for the `ERRnMISC3lo` field.
    pub const ERRNMISC3LO_MASK: u32 = 0b1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `ERRnMISC3lo` field.
    pub const fn errnmisc3lo(self) -> u32 {
        (self.bits() >> Self::ERRNMISC3LO_SHIFT) & Self::ERRNMISC3LO_MASK
    }

    /// Sets the value of the `ERRnMISC3lo` field.
    pub const fn set_errnmisc3lo(&mut self, value: u32) {
        let offset = Self::ERRNMISC3LO_SHIFT;
        assert!(value & Self::ERRNMISC3LO_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ERRNMISC3LO_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `ERRnMISC3lo` field set to the given value.
    pub const fn with_errnmisc3lo(mut self, value: u32) -> Self {
        self.set_errnmisc3lo(value);
        self
    }
}

bitflags! {
    /// `ERXMISC7` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Erxmisc7: u32 {
    }
}

impl Erxmisc7 {
    /// Offset of the `ERRnMISC3hi` field.
    pub const ERRNMISC3HI_SHIFT: u32 = 0;
    /// Mask for the `ERRnMISC3hi` field.
    pub const ERRNMISC3HI_MASK: u32 = 0b1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `ERRnMISC3hi` field.
    pub const fn errnmisc3hi(self) -> u32 {
        (self.bits() >> Self::ERRNMISC3HI_SHIFT) & Self::ERRNMISC3HI_MASK
    }

    /// Sets the value of the `ERRnMISC3hi` field.
    pub const fn set_errnmisc3hi(&mut self, value: u32) {
        let offset = Self::ERRNMISC3HI_SHIFT;
        assert!(value & Self::ERRNMISC3HI_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ERRNMISC3HI_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `ERRnMISC3hi` field set to the given value.
    pub const fn with_errnmisc3hi(mut self, value: u32) -> Self {
        self.set_errnmisc3hi(value);
        self
    }
}

bitflags! {
    /// `ERXSTATUS` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Erxstatus: u32 {
    }
}

impl Erxstatus {
    /// Offset of the `ERRnSTATUSlo` field.
    pub const ERRNSTATUSLO_SHIFT: u32 = 0;
    /// Mask for the `ERRnSTATUSlo` field.
    pub const ERRNSTATUSLO_MASK: u32 = 0b1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `ERRnSTATUSlo` field.
    pub const fn errnstatuslo(self) -> u32 {
        (self.bits() >> Self::ERRNSTATUSLO_SHIFT) & Self::ERRNSTATUSLO_MASK
    }

    /// Sets the value of the `ERRnSTATUSlo` field.
    pub const fn set_errnstatuslo(&mut self, value: u32) {
        let offset = Self::ERRNSTATUSLO_SHIFT;
        assert!(value & Self::ERRNSTATUSLO_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ERRNSTATUSLO_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `ERRnSTATUSlo` field set to the given value.
    pub const fn with_errnstatuslo(mut self, value: u32) -> Self {
        self.set_errnstatuslo(value);
        self
    }
}

bitflags! {
    /// `HCPTR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Hcptr: u32 {
        /// RES1 bits in the `HCPTR` register.
        const RES1 = 0b11_0011_1111_1111;
        /// `TCP10` bit.
        const TCP10 = 1 << 10;
        /// `TCP11` bit.
        const TCP11 = 1 << 11;
        /// `TASE` bit.
        const TASE = 1 << 15;
        /// `TTA` bit.
        const TTA = 1 << 20;
        /// `TAM` bit.
        const TAM = 1 << 30;
        /// `TCPAC` bit.
        const TCPAC = 1 << 31;
    }
}

impl Hcptr {
    /// Offset of the `TCP10` field.
    pub const TCP10_SHIFT: u32 = 10;
    /// Offset of the `TCP11` field.
    pub const TCP11_SHIFT: u32 = 11;
    /// Offset of the `TASE` field.
    pub const TASE_SHIFT: u32 = 15;
    /// Offset of the `TTA` field.
    pub const TTA_SHIFT: u32 = 20;
    /// Offset of the `TAM` field.
    pub const TAM_SHIFT: u32 = 30;
    /// Offset of the `TCPAC` field.
    pub const TCPAC_SHIFT: u32 = 31;
}

bitflags! {
    /// `HCR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Hcr: u32 {
        /// `VM` bit.
        const VM = 1 << 0;
        /// `SWIO` bit.
        const SWIO = 1 << 1;
        /// `PTW` bit.
        const PTW = 1 << 2;
        /// `FMO` bit.
        const FMO = 1 << 3;
        /// `IMO` bit.
        const IMO = 1 << 4;
        /// `AMO` bit.
        const AMO = 1 << 5;
        /// `VF` bit.
        const VF = 1 << 6;
        /// `VI` bit.
        const VI = 1 << 7;
        /// `VA` bit.
        const VA = 1 << 8;
        /// `FB` bit.
        const FB = 1 << 9;
        /// `DC` bit.
        const DC = 1 << 12;
        /// `TWI` bit.
        const TWI = 1 << 13;
        /// `TWE` bit.
        const TWE = 1 << 14;
        /// `TID0` bit.
        const TID0 = 1 << 15;
        /// `TID1` bit.
        const TID1 = 1 << 16;
        /// `TID2` bit.
        const TID2 = 1 << 17;
        /// `TID3` bit.
        const TID3 = 1 << 18;
        /// `TSC` bit.
        const TSC = 1 << 19;
        /// `TIDCP` bit.
        const TIDCP = 1 << 20;
        /// `TAC` bit.
        const TAC = 1 << 21;
        /// `TSW` bit.
        const TSW = 1 << 22;
        /// `TPC` bit.
        const TPC = 1 << 23;
        /// `TPU` bit.
        const TPU = 1 << 24;
        /// `TTLB` bit.
        const TTLB = 1 << 25;
        /// `TVM` bit.
        const TVM = 1 << 26;
        /// `TGE` bit.
        const TGE = 1 << 27;
        /// `HCD` bit.
        const HCD = 1 << 29;
        /// `TRVM` bit.
        const TRVM = 1 << 30;
    }
}

impl Hcr {
    /// Offset of the `VM` field.
    pub const VM_SHIFT: u32 = 0;
    /// Offset of the `SWIO` field.
    pub const SWIO_SHIFT: u32 = 1;
    /// Offset of the `PTW` field.
    pub const PTW_SHIFT: u32 = 2;
    /// Offset of the `FMO` field.
    pub const FMO_SHIFT: u32 = 3;
    /// Offset of the `IMO` field.
    pub const IMO_SHIFT: u32 = 4;
    /// Offset of the `AMO` field.
    pub const AMO_SHIFT: u32 = 5;
    /// Offset of the `VF` field.
    pub const VF_SHIFT: u32 = 6;
    /// Offset of the `VI` field.
    pub const VI_SHIFT: u32 = 7;
    /// Offset of the `VA` field.
    pub const VA_SHIFT: u32 = 8;
    /// Offset of the `FB` field.
    pub const FB_SHIFT: u32 = 9;
    /// Offset of the `BSU` field.
    pub const BSU_SHIFT: u32 = 10;
    /// Mask for the `BSU` field.
    pub const BSU_MASK: u32 = 0b11;
    /// Offset of the `DC` field.
    pub const DC_SHIFT: u32 = 12;
    /// Offset of the `TWI` field.
    pub const TWI_SHIFT: u32 = 13;
    /// Offset of the `TWE` field.
    pub const TWE_SHIFT: u32 = 14;
    /// Offset of the `TID0` field.
    pub const TID0_SHIFT: u32 = 15;
    /// Offset of the `TID1` field.
    pub const TID1_SHIFT: u32 = 16;
    /// Offset of the `TID2` field.
    pub const TID2_SHIFT: u32 = 17;
    /// Offset of the `TID3` field.
    pub const TID3_SHIFT: u32 = 18;
    /// Offset of the `TSC` field.
    pub const TSC_SHIFT: u32 = 19;
    /// Offset of the `TIDCP` field.
    pub const TIDCP_SHIFT: u32 = 20;
    /// Offset of the `TAC` field.
    pub const TAC_SHIFT: u32 = 21;
    /// Offset of the `TSW` field.
    pub const TSW_SHIFT: u32 = 22;
    /// Offset of the `TPC` field.
    pub const TPC_SHIFT: u32 = 23;
    /// Offset of the `TPU` field.
    pub const TPU_SHIFT: u32 = 24;
    /// Offset of the `TTLB` field.
    pub const TTLB_SHIFT: u32 = 25;
    /// Offset of the `TVM` field.
    pub const TVM_SHIFT: u32 = 26;
    /// Offset of the `TGE` field.
    pub const TGE_SHIFT: u32 = 27;
    /// Offset of the `HCD` field.
    pub const HCD_SHIFT: u32 = 29;
    /// Offset of the `TRVM` field.
    pub const TRVM_SHIFT: u32 = 30;

    /// Returns the value of the `BSU` field.
    pub const fn bsu(self) -> u8 {
        ((self.bits() >> Self::BSU_SHIFT) & Self::BSU_MASK) as u8
    }

    /// Sets the value of the `BSU` field.
    pub const fn set_bsu(&mut self, value: u8) {
        let offset = Self::BSU_SHIFT;
        assert!(value & (Self::BSU_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BSU_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `BSU` field set to the given value.
    pub const fn with_bsu(mut self, value: u8) -> Self {
        self.set_bsu(value);
        self
    }
}

bitflags! {
    /// `HCR2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Hcr2: u32 {
        /// `CD` bit.
        const CD = 1 << 0;
        /// `ID` bit.
        const ID = 1 << 1;
        /// `TERR` bit.
        const TERR = 1 << 4;
        /// `TEA` bit.
        const TEA = 1 << 5;
        /// `TID4` bit.
        const TID4 = 1 << 17;
        /// `TICAB` bit.
        const TICAB = 1 << 18;
        /// `TOCU` bit.
        const TOCU = 1 << 20;
        /// `TTLBIS` bit.
        const TTLBIS = 1 << 22;
    }
}

impl Hcr2 {
    /// Offset of the `CD` field.
    pub const CD_SHIFT: u32 = 0;
    /// Offset of the `ID` field.
    pub const ID_SHIFT: u32 = 1;
    /// Offset of the `TERR` field.
    pub const TERR_SHIFT: u32 = 4;
    /// Offset of the `TEA` field.
    pub const TEA_SHIFT: u32 = 5;
    /// Offset of the `TID4` field.
    pub const TID4_SHIFT: u32 = 17;
    /// Offset of the `TICAB` field.
    pub const TICAB_SHIFT: u32 = 18;
    /// Offset of the `TOCU` field.
    pub const TOCU_SHIFT: u32 = 20;
    /// Offset of the `TTLBIS` field.
    pub const TTLBIS_SHIFT: u32 = 22;
}

bitflags! {
    /// `HDCR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Hdcr: u32 {
        /// `TPMCR` bit.
        const TPMCR = 1 << 5;
        /// `TPM` bit.
        const TPM = 1 << 6;
        /// `HPME` bit.
        const HPME = 1 << 7;
        /// `TDE` bit.
        const TDE = 1 << 8;
        /// `TDA` bit.
        const TDA = 1 << 9;
        /// `TDOSA` bit.
        const TDOSA = 1 << 10;
        /// `TDRA` bit.
        const TDRA = 1 << 11;
        /// `HPMD` bit.
        const HPMD = 1 << 17;
        /// `TTRF` bit.
        const TTRF = 1 << 19;
        /// `HCCD` bit.
        const HCCD = 1 << 23;
        /// `HLP` bit.
        const HLP = 1 << 26;
        /// `TDCC` bit.
        const TDCC = 1 << 27;
        /// `MTPME` bit.
        const MTPME = 1 << 28;
        /// `HPMFZO` bit.
        const HPMFZO = 1 << 29;
    }
}

impl Hdcr {
    /// Offset of the `HPMN` field.
    pub const HPMN_SHIFT: u32 = 0;
    /// Mask for the `HPMN` field.
    pub const HPMN_MASK: u32 = 0b1_1111;
    /// Offset of the `TPMCR` field.
    pub const TPMCR_SHIFT: u32 = 5;
    /// Offset of the `TPM` field.
    pub const TPM_SHIFT: u32 = 6;
    /// Offset of the `HPME` field.
    pub const HPME_SHIFT: u32 = 7;
    /// Offset of the `TDE` field.
    pub const TDE_SHIFT: u32 = 8;
    /// Offset of the `TDA` field.
    pub const TDA_SHIFT: u32 = 9;
    /// Offset of the `TDOSA` field.
    pub const TDOSA_SHIFT: u32 = 10;
    /// Offset of the `TDRA` field.
    pub const TDRA_SHIFT: u32 = 11;
    /// Offset of the `HPMD` field.
    pub const HPMD_SHIFT: u32 = 17;
    /// Offset of the `TTRF` field.
    pub const TTRF_SHIFT: u32 = 19;
    /// Offset of the `HCCD` field.
    pub const HCCD_SHIFT: u32 = 23;
    /// Offset of the `HLP` field.
    pub const HLP_SHIFT: u32 = 26;
    /// Offset of the `TDCC` field.
    pub const TDCC_SHIFT: u32 = 27;
    /// Offset of the `MTPME` field.
    pub const MTPME_SHIFT: u32 = 28;
    /// Offset of the `HPMFZO` field.
    pub const HPMFZO_SHIFT: u32 = 29;

    /// Returns the value of the `HPMN` field.
    pub const fn hpmn(self) -> u8 {
        ((self.bits() >> Self::HPMN_SHIFT) & Self::HPMN_MASK) as u8
    }

    /// Sets the value of the `HPMN` field.
    pub const fn set_hpmn(&mut self, value: u8) {
        let offset = Self::HPMN_SHIFT;
        assert!(value & (Self::HPMN_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::HPMN_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `HPMN` field set to the given value.
    pub const fn with_hpmn(mut self, value: u8) -> Self {
        self.set_hpmn(value);
        self
    }
}

bitflags! {
    /// `HDFAR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Hdfar: u32 {
    }
}

impl Hdfar {
    /// Offset of the `VA` field.
    pub const VA_SHIFT: u32 = 0;
    /// Mask for the `VA` field.
    pub const VA_MASK: u32 = 0b1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `VA` field.
    pub const fn va(self) -> u32 {
        (self.bits() >> Self::VA_SHIFT) & Self::VA_MASK
    }

    /// Sets the value of the `VA` field.
    pub const fn set_va(&mut self, value: u32) {
        let offset = Self::VA_SHIFT;
        assert!(value & Self::VA_MASK == value);
        *self =
            Self::from_bits_retain((self.bits() & !(Self::VA_MASK << offset)) | (value << offset));
    }

    /// Returns a copy with the `VA` field set to the given value.
    pub const fn with_va(mut self, value: u32) -> Self {
        self.set_va(value);
        self
    }
}

bitflags! {
    /// `HIFAR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Hifar: u32 {
    }
}

impl Hifar {
    /// Offset of the `VA` field.
    pub const VA_SHIFT: u32 = 0;
    /// Mask for the `VA` field.
    pub const VA_MASK: u32 = 0b1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `VA` field.
    pub const fn va(self) -> u32 {
        (self.bits() >> Self::VA_SHIFT) & Self::VA_MASK
    }

    /// Sets the value of the `VA` field.
    pub const fn set_va(&mut self, value: u32) {
        let offset = Self::VA_SHIFT;
        assert!(value & Self::VA_MASK == value);
        *self =
            Self::from_bits_retain((self.bits() & !(Self::VA_MASK << offset)) | (value << offset));
    }

    /// Returns a copy with the `VA` field set to the given value.
    pub const fn with_va(mut self, value: u32) -> Self {
        self.set_va(value);
        self
    }
}

bitflags! {
    /// `HMAIR0` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Hmair0: u32 {
    }
}

impl Hmair0 {
    /// Offset of the `Attr<n>` field.
    pub const ATTR_SHIFT: u32 = 0;
    /// Mask for the `Attr<n>` field.
    pub const ATTR_MASK: u32 = 0b1111_1111;

    /// Returns the value of the given `Attr<n>` field.
    pub const fn attr(self, n: u32) -> u8 {
        assert!(n < 4);
        ((self.bits() >> (Self::ATTR_SHIFT + n * 8)) & Self::ATTR_MASK) as u8
    }

    /// Sets the value of the `Attr<n>` field.
    pub const fn set_attr(&mut self, n: u32, value: u8) {
        assert!(n < 4);
        let offset = Self::ATTR_SHIFT + n * 8;
        assert!(value & (Self::ATTR_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ATTR_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `Attr<n>` field set to the given value.
    pub const fn with_attr(mut self, n: u32, value: u8) -> Self {
        self.set_attr(n, value);
        self
    }
}

bitflags! {
    /// `HMAIR1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Hmair1: u32 {
    }
}

impl Hmair1 {
    /// Offset of the `Attr<n>` field.
    pub const ATTR_SHIFT: u32 = 0;
    /// Mask for the `Attr<n>` field.
    pub const ATTR_MASK: u32 = 0b1111_1111;

    /// Returns the value of the given `Attr<n>` field.
    pub const fn attr(self, n: u32) -> u8 {
        assert!(n >= 4 && n < 8);
        ((self.bits() >> (Self::ATTR_SHIFT + (n - 4) * 8)) & Self::ATTR_MASK) as u8
    }

    /// Sets the value of the `Attr<n>` field.
    pub const fn set_attr(&mut self, n: u32, value: u8) {
        assert!(n >= 4 && n < 8);
        let offset = Self::ATTR_SHIFT + (n - 4) * 8;
        assert!(value & (Self::ATTR_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ATTR_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `Attr<n>` field set to the given value.
    pub const fn with_attr(mut self, n: u32, value: u8) -> Self {
        self.set_attr(n, value);
        self
    }
}

bitflags! {
    /// `HPFAR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Hpfar: u32 {
    }
}

impl Hpfar {
    /// Offset of the `FIPA[39:12]` field.
    pub const FIPA_39_12_SHIFT: u32 = 4;
    /// Mask for the `FIPA[39:12]` field.
    pub const FIPA_39_12_MASK: u32 = 0b1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `FIPA[39:12]` field.
    pub const fn fipa_39_12(self) -> u32 {
        (self.bits() >> Self::FIPA_39_12_SHIFT) & Self::FIPA_39_12_MASK
    }

    /// Sets the value of the `FIPA[39:12]` field.
    pub const fn set_fipa_39_12(&mut self, value: u32) {
        let offset = Self::FIPA_39_12_SHIFT;
        assert!(value & Self::FIPA_39_12_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::FIPA_39_12_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `FIPA[39:12]` field set to the given value.
    pub const fn with_fipa_39_12(mut self, value: u32) -> Self {
        self.set_fipa_39_12(value);
        self
    }
}

bitflags! {
    /// `HRMR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Hrmr: u32 {
        /// `AA64` bit.
        const AA64 = 1 << 0;
        /// `RR` bit.
        const RR = 1 << 1;
    }
}

impl Hrmr {
    /// Offset of the `AA64` field.
    pub const AA64_SHIFT: u32 = 0;
    /// Offset of the `RR` field.
    pub const RR_SHIFT: u32 = 1;
}

bitflags! {
    /// `HSCTLR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Hsctlr: u32 {
        /// RES1 bits in the `HSCTLR` register.
        const RES1 = 0b11_0000_1100_0101_0000_1000_0000_0000;
        /// `M` bit.
        const M = 1 << 0;
        /// `A` bit.
        const A = 1 << 1;
        /// `C` bit.
        const C = 1 << 2;
        /// `nTLSMD` bit.
        const NTLSMD = 1 << 3;
        /// `LSMAOE` bit.
        const LSMAOE = 1 << 4;
        /// `CP15BEN` bit.
        const CP15BEN = 1 << 5;
        /// `ITD` bit.
        const ITD = 1 << 7;
        /// `SED` bit.
        const SED = 1 << 8;
        /// `I` bit.
        const I = 1 << 12;
        /// `WXN` bit.
        const WXN = 1 << 19;
        /// `TE` bit.
        const TE = 1 << 30;
        /// `DSSBS` bit.
        const DSSBS = 1 << 31;
    }
}

impl Hsctlr {
    /// Offset of the `M` field.
    pub const M_SHIFT: u32 = 0;
    /// Offset of the `A` field.
    pub const A_SHIFT: u32 = 1;
    /// Offset of the `C` field.
    pub const C_SHIFT: u32 = 2;
    /// Offset of the `nTLSMD` field.
    pub const NTLSMD_SHIFT: u32 = 3;
    /// Offset of the `LSMAOE` field.
    pub const LSMAOE_SHIFT: u32 = 4;
    /// Offset of the `CP15BEN` field.
    pub const CP15BEN_SHIFT: u32 = 5;
    /// Offset of the `ITD` field.
    pub const ITD_SHIFT: u32 = 7;
    /// Offset of the `SED` field.
    pub const SED_SHIFT: u32 = 8;
    /// Offset of the `I` field.
    pub const I_SHIFT: u32 = 12;
    /// Offset of the `WXN` field.
    pub const WXN_SHIFT: u32 = 19;
    /// Offset of the `TE` field.
    pub const TE_SHIFT: u32 = 30;
    /// Offset of the `DSSBS` field.
    pub const DSSBS_SHIFT: u32 = 31;
}

bitflags! {
    /// `HSR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Hsr: u32 {
        /// `IL` bit.
        const IL = 1 << 25;
    }
}

impl Hsr {
    /// Offset of the `ISS` field.
    pub const ISS_SHIFT: u32 = 0;
    /// Mask for the `ISS` field.
    pub const ISS_MASK: u32 = 0b1_1111_1111_1111_1111_1111_1111;
    /// Offset of the `IL` field.
    pub const IL_SHIFT: u32 = 25;
    /// Offset of the `EC` field.
    pub const EC_SHIFT: u32 = 26;
    /// Mask for the `EC` field.
    pub const EC_MASK: u32 = 0b11_1111;

    /// Returns the value of the `ISS` field.
    pub const fn iss(self) -> u32 {
        (self.bits() >> Self::ISS_SHIFT) & Self::ISS_MASK
    }

    /// Sets the value of the `ISS` field.
    pub const fn set_iss(&mut self, value: u32) {
        let offset = Self::ISS_SHIFT;
        assert!(value & Self::ISS_MASK == value);
        *self =
            Self::from_bits_retain((self.bits() & !(Self::ISS_MASK << offset)) | (value << offset));
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
            (self.bits() & !(Self::EC_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `EC` field set to the given value.
    pub const fn with_ec(mut self, value: u8) -> Self {
        self.set_ec(value);
        self
    }
}

bitflags! {
    /// `HTCR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Htcr: u32 {
        /// RES1 bits in the `HTCR` register.
        const RES1 = 0b1000_0000_1000_0000_0000_0000_0000_0000;
        /// `HPD` bit.
        const HPD = 1 << 24;
        /// `HWU59` bit.
        const HWU59 = 1 << 25;
        /// `HWU60` bit.
        const HWU60 = 1 << 26;
        /// `HWU61` bit.
        const HWU61 = 1 << 27;
        /// `HWU62` bit.
        const HWU62 = 1 << 28;
    }
}

impl Htcr {
    /// Offset of the `T0SZ` field.
    pub const T0SZ_SHIFT: u32 = 0;
    /// Mask for the `T0SZ` field.
    pub const T0SZ_MASK: u32 = 0b111;
    /// Offset of the `IRGN0` field.
    pub const IRGN0_SHIFT: u32 = 8;
    /// Mask for the `IRGN0` field.
    pub const IRGN0_MASK: u32 = 0b11;
    /// Offset of the `ORGN0` field.
    pub const ORGN0_SHIFT: u32 = 10;
    /// Mask for the `ORGN0` field.
    pub const ORGN0_MASK: u32 = 0b11;
    /// Offset of the `SH0` field.
    pub const SH0_SHIFT: u32 = 12;
    /// Mask for the `SH0` field.
    pub const SH0_MASK: u32 = 0b11;
    /// Offset of the `HPD` field.
    pub const HPD_SHIFT: u32 = 24;
    /// Offset of the `HWU59` field.
    pub const HWU59_SHIFT: u32 = 25;
    /// Offset of the `HWU60` field.
    pub const HWU60_SHIFT: u32 = 26;
    /// Offset of the `HWU61` field.
    pub const HWU61_SHIFT: u32 = 27;
    /// Offset of the `HWU62` field.
    pub const HWU62_SHIFT: u32 = 28;

    /// Returns the value of the `T0SZ` field.
    pub const fn t0sz(self) -> u8 {
        ((self.bits() >> Self::T0SZ_SHIFT) & Self::T0SZ_MASK) as u8
    }

    /// Sets the value of the `T0SZ` field.
    pub const fn set_t0sz(&mut self, value: u8) {
        let offset = Self::T0SZ_SHIFT;
        assert!(value & (Self::T0SZ_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::T0SZ_MASK << offset)) | ((value as u32) << offset),
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
            (self.bits() & !(Self::IRGN0_MASK << offset)) | ((value as u32) << offset),
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
            (self.bits() & !(Self::ORGN0_MASK << offset)) | ((value as u32) << offset),
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
            (self.bits() & !(Self::SH0_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `SH0` field set to the given value.
    pub const fn with_sh0(mut self, value: u8) -> Self {
        self.set_sh0(value);
        self
    }
}

bitflags! {
    /// `HTPIDR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Htpidr: u32 {
    }
}

impl Htpidr {
    /// Offset of the `TID` field.
    pub const TID_SHIFT: u32 = 0;
    /// Mask for the `TID` field.
    pub const TID_MASK: u32 = 0b1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `TID` field.
    pub const fn tid(self) -> u32 {
        (self.bits() >> Self::TID_SHIFT) & Self::TID_MASK
    }

    /// Sets the value of the `TID` field.
    pub const fn set_tid(&mut self, value: u32) {
        let offset = Self::TID_SHIFT;
        assert!(value & Self::TID_MASK == value);
        *self =
            Self::from_bits_retain((self.bits() & !(Self::TID_MASK << offset)) | (value << offset));
    }

    /// Returns a copy with the `TID` field set to the given value.
    pub const fn with_tid(mut self, value: u32) -> Self {
        self.set_tid(value);
        self
    }
}

bitflags! {
    /// `HTRFCR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Htrfcr: u32 {
        /// `E0HTRE` bit.
        const E0HTRE = 1 << 0;
        /// `E2TRE` bit.
        const E2TRE = 1 << 1;
        /// `CX` bit.
        const CX = 1 << 3;
    }
}

impl Htrfcr {
    /// Offset of the `E0HTRE` field.
    pub const E0HTRE_SHIFT: u32 = 0;
    /// Offset of the `E2TRE` field.
    pub const E2TRE_SHIFT: u32 = 1;
    /// Offset of the `CX` field.
    pub const CX_SHIFT: u32 = 3;
    /// Offset of the `TS` field.
    pub const TS_SHIFT: u32 = 5;
    /// Mask for the `TS` field.
    pub const TS_MASK: u32 = 0b11;

    /// Returns the value of the `TS` field.
    pub const fn ts(self) -> u8 {
        ((self.bits() >> Self::TS_SHIFT) & Self::TS_MASK) as u8
    }

    /// Sets the value of the `TS` field.
    pub const fn set_ts(&mut self, value: u8) {
        let offset = Self::TS_SHIFT;
        assert!(value & (Self::TS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TS_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `TS` field set to the given value.
    pub const fn with_ts(mut self, value: u8) -> Self {
        self.set_ts(value);
        self
    }
}

bitflags! {
    /// `HTTBR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Httbr: u64 {
        /// `CnP` bit.
        const CNP = 1 << 0;
    }
}

impl Httbr {
    /// Offset of the `CnP` field.
    pub const CNP_SHIFT: u32 = 0;
    /// Offset of the `BADDR` field.
    pub const BADDR_SHIFT: u32 = 1;
    /// Mask for the `BADDR` field.
    pub const BADDR_MASK: u64 = 0b111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `BADDR` field.
    pub const fn baddr(self) -> u64 {
        (self.bits() >> Self::BADDR_SHIFT) & Self::BADDR_MASK
    }

    /// Sets the value of the `BADDR` field.
    pub const fn set_baddr(&mut self, value: u64) {
        let offset = Self::BADDR_SHIFT;
        assert!(value & Self::BADDR_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BADDR_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `BADDR` field set to the given value.
    pub const fn with_baddr(mut self, value: u64) -> Self {
        self.set_baddr(value);
        self
    }
}

bitflags! {
    /// `HVBAR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Hvbar: u32 {
    }
}

impl Hvbar {
    /// Offset of the `VBA` field.
    pub const VBA_SHIFT: u32 = 5;
    /// Mask for the `VBA` field.
    pub const VBA_MASK: u32 = 0b111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `VBA` field.
    pub const fn vba(self) -> u32 {
        (self.bits() >> Self::VBA_SHIFT) & Self::VBA_MASK
    }

    /// Sets the value of the `VBA` field.
    pub const fn set_vba(&mut self, value: u32) {
        let offset = Self::VBA_SHIFT;
        assert!(value & Self::VBA_MASK == value);
        *self =
            Self::from_bits_retain((self.bits() & !(Self::VBA_MASK << offset)) | (value << offset));
    }

    /// Returns a copy with the `VBA` field set to the given value.
    pub const fn with_vba(mut self, value: u32) -> Self {
        self.set_vba(value);
        self
    }
}

bitflags! {
    /// `ICC_ASGI1R` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccAsgi1r: u64 {
        /// `IRM` bit.
        const IRM = 1 << 40;
    }
}

impl IccAsgi1r {
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
    /// `ICC_BPR0` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccBpr0: u32 {
    }
}

impl IccBpr0 {
    /// Offset of the `BinaryPoint` field.
    pub const BINARYPOINT_SHIFT: u32 = 0;
    /// Mask for the `BinaryPoint` field.
    pub const BINARYPOINT_MASK: u32 = 0b111;

    /// Returns the value of the `BinaryPoint` field.
    pub const fn binarypoint(self) -> u8 {
        ((self.bits() >> Self::BINARYPOINT_SHIFT) & Self::BINARYPOINT_MASK) as u8
    }

    /// Sets the value of the `BinaryPoint` field.
    pub const fn set_binarypoint(&mut self, value: u8) {
        let offset = Self::BINARYPOINT_SHIFT;
        assert!(value & (Self::BINARYPOINT_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BINARYPOINT_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `BinaryPoint` field set to the given value.
    pub const fn with_binarypoint(mut self, value: u8) -> Self {
        self.set_binarypoint(value);
        self
    }
}

bitflags! {
    /// `ICC_BPR1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccBpr1: u32 {
    }
}

impl IccBpr1 {
    /// Offset of the `BinaryPoint` field.
    pub const BINARYPOINT_SHIFT: u32 = 0;
    /// Mask for the `BinaryPoint` field.
    pub const BINARYPOINT_MASK: u32 = 0b111;

    /// Returns the value of the `BinaryPoint` field.
    pub const fn binarypoint(self) -> u8 {
        ((self.bits() >> Self::BINARYPOINT_SHIFT) & Self::BINARYPOINT_MASK) as u8
    }

    /// Sets the value of the `BinaryPoint` field.
    pub const fn set_binarypoint(&mut self, value: u8) {
        let offset = Self::BINARYPOINT_SHIFT;
        assert!(value & (Self::BINARYPOINT_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BINARYPOINT_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `BinaryPoint` field set to the given value.
    pub const fn with_binarypoint(mut self, value: u8) -> Self {
        self.set_binarypoint(value);
        self
    }
}

bitflags! {
    /// `ICC_CTLR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccCtlr: u32 {
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

impl IccCtlr {
    /// Offset of the `CBPR` field.
    pub const CBPR_SHIFT: u32 = 0;
    /// Offset of the `EOImode` field.
    pub const EOIMODE_SHIFT: u32 = 1;
    /// Offset of the `PMHE` field.
    pub const PMHE_SHIFT: u32 = 6;
    /// Offset of the `PRIbits` field.
    pub const PRIBITS_SHIFT: u32 = 8;
    /// Mask for the `PRIbits` field.
    pub const PRIBITS_MASK: u32 = 0b111;
    /// Offset of the `IDbits` field.
    pub const IDBITS_SHIFT: u32 = 11;
    /// Mask for the `IDbits` field.
    pub const IDBITS_MASK: u32 = 0b111;
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
            (self.bits() & !(Self::PRIBITS_MASK << offset)) | ((value as u32) << offset),
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
            (self.bits() & !(Self::IDBITS_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `IDbits` field set to the given value.
    pub const fn with_idbits(mut self, value: u8) -> Self {
        self.set_idbits(value);
        self
    }
}

bitflags! {
    /// `ICC_DIR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccDir: u32 {
    }
}

impl IccDir {
    /// Offset of the `INTID` field.
    pub const INTID_SHIFT: u32 = 0;
    /// Mask for the `INTID` field.
    pub const INTID_MASK: u32 = 0b1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `INTID` field.
    pub const fn intid(self) -> u32 {
        (self.bits() >> Self::INTID_SHIFT) & Self::INTID_MASK
    }

    /// Sets the value of the `INTID` field.
    pub const fn set_intid(&mut self, value: u32) {
        let offset = Self::INTID_SHIFT;
        assert!(value & Self::INTID_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::INTID_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `INTID` field set to the given value.
    pub const fn with_intid(mut self, value: u32) -> Self {
        self.set_intid(value);
        self
    }
}

bitflags! {
    /// `ICC_EOIR0` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccEoir0: u32 {
    }
}

impl IccEoir0 {
    /// Offset of the `INTID` field.
    pub const INTID_SHIFT: u32 = 0;
    /// Mask for the `INTID` field.
    pub const INTID_MASK: u32 = 0b1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `INTID` field.
    pub const fn intid(self) -> u32 {
        (self.bits() >> Self::INTID_SHIFT) & Self::INTID_MASK
    }

    /// Sets the value of the `INTID` field.
    pub const fn set_intid(&mut self, value: u32) {
        let offset = Self::INTID_SHIFT;
        assert!(value & Self::INTID_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::INTID_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `INTID` field set to the given value.
    pub const fn with_intid(mut self, value: u32) -> Self {
        self.set_intid(value);
        self
    }
}

bitflags! {
    /// `ICC_EOIR1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccEoir1: u32 {
    }
}

impl IccEoir1 {
    /// Offset of the `INTID` field.
    pub const INTID_SHIFT: u32 = 0;
    /// Mask for the `INTID` field.
    pub const INTID_MASK: u32 = 0b1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `INTID` field.
    pub const fn intid(self) -> u32 {
        (self.bits() >> Self::INTID_SHIFT) & Self::INTID_MASK
    }

    /// Sets the value of the `INTID` field.
    pub const fn set_intid(&mut self, value: u32) {
        let offset = Self::INTID_SHIFT;
        assert!(value & Self::INTID_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::INTID_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `INTID` field set to the given value.
    pub const fn with_intid(mut self, value: u32) -> Self {
        self.set_intid(value);
        self
    }
}

bitflags! {
    /// `ICC_HPPIR0` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccHppir0: u32 {
    }
}

impl IccHppir0 {
    /// Offset of the `INTID` field.
    pub const INTID_SHIFT: u32 = 0;
    /// Mask for the `INTID` field.
    pub const INTID_MASK: u32 = 0b1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `INTID` field.
    pub const fn intid(self) -> u32 {
        (self.bits() >> Self::INTID_SHIFT) & Self::INTID_MASK
    }

    /// Sets the value of the `INTID` field.
    pub const fn set_intid(&mut self, value: u32) {
        let offset = Self::INTID_SHIFT;
        assert!(value & Self::INTID_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::INTID_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `INTID` field set to the given value.
    pub const fn with_intid(mut self, value: u32) -> Self {
        self.set_intid(value);
        self
    }
}

bitflags! {
    /// `ICC_HPPIR1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccHppir1: u32 {
    }
}

impl IccHppir1 {
    /// Offset of the `INTID` field.
    pub const INTID_SHIFT: u32 = 0;
    /// Mask for the `INTID` field.
    pub const INTID_MASK: u32 = 0b1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `INTID` field.
    pub const fn intid(self) -> u32 {
        (self.bits() >> Self::INTID_SHIFT) & Self::INTID_MASK
    }

    /// Sets the value of the `INTID` field.
    pub const fn set_intid(&mut self, value: u32) {
        let offset = Self::INTID_SHIFT;
        assert!(value & Self::INTID_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::INTID_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `INTID` field set to the given value.
    pub const fn with_intid(mut self, value: u32) -> Self {
        self.set_intid(value);
        self
    }
}

bitflags! {
    /// `ICC_HSRE` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccHsre: u32 {
        /// `SRE` bit.
        const SRE = 1 << 0;
        /// `DFB` bit.
        const DFB = 1 << 1;
        /// `DIB` bit.
        const DIB = 1 << 2;
        /// `Enable` bit.
        const ENABLE = 1 << 3;
    }
}

impl IccHsre {
    /// Offset of the `SRE` field.
    pub const SRE_SHIFT: u32 = 0;
    /// Offset of the `DFB` field.
    pub const DFB_SHIFT: u32 = 1;
    /// Offset of the `DIB` field.
    pub const DIB_SHIFT: u32 = 2;
    /// Offset of the `Enable` field.
    pub const ENABLE_SHIFT: u32 = 3;
}

bitflags! {
    /// `ICC_IAR0` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccIar0: u32 {
    }
}

impl IccIar0 {
    /// Offset of the `INTID` field.
    pub const INTID_SHIFT: u32 = 0;
    /// Mask for the `INTID` field.
    pub const INTID_MASK: u32 = 0b1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `INTID` field.
    pub const fn intid(self) -> u32 {
        (self.bits() >> Self::INTID_SHIFT) & Self::INTID_MASK
    }

    /// Sets the value of the `INTID` field.
    pub const fn set_intid(&mut self, value: u32) {
        let offset = Self::INTID_SHIFT;
        assert!(value & Self::INTID_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::INTID_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `INTID` field set to the given value.
    pub const fn with_intid(mut self, value: u32) -> Self {
        self.set_intid(value);
        self
    }
}

bitflags! {
    /// `ICC_IAR1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccIar1: u32 {
    }
}

impl IccIar1 {
    /// Offset of the `INTID` field.
    pub const INTID_SHIFT: u32 = 0;
    /// Mask for the `INTID` field.
    pub const INTID_MASK: u32 = 0b1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `INTID` field.
    pub const fn intid(self) -> u32 {
        (self.bits() >> Self::INTID_SHIFT) & Self::INTID_MASK
    }

    /// Sets the value of the `INTID` field.
    pub const fn set_intid(&mut self, value: u32) {
        let offset = Self::INTID_SHIFT;
        assert!(value & Self::INTID_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::INTID_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `INTID` field set to the given value.
    pub const fn with_intid(mut self, value: u32) -> Self {
        self.set_intid(value);
        self
    }
}

bitflags! {
    /// `ICC_IGRPEN0` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccIgrpen0: u32 {
        /// `Enable` bit.
        const ENABLE = 1 << 0;
    }
}

impl IccIgrpen0 {
    /// Offset of the `Enable` field.
    pub const ENABLE_SHIFT: u32 = 0;
}

bitflags! {
    /// `ICC_IGRPEN1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccIgrpen1: u32 {
        /// `Enable` bit.
        const ENABLE = 1 << 0;
    }
}

impl IccIgrpen1 {
    /// Offset of the `Enable` field.
    pub const ENABLE_SHIFT: u32 = 0;
}

bitflags! {
    /// `ICC_MCTLR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccMctlr: u32 {
        /// `CBPR_EL1S` bit.
        const CBPR_EL1S = 1 << 0;
        /// `CBPR_EL1NS` bit.
        const CBPR_EL1NS = 1 << 1;
        /// `EOImode_EL3` bit.
        const EOIMODE_EL3 = 1 << 2;
        /// `EOImode_EL1S` bit.
        const EOIMODE_EL1S = 1 << 3;
        /// `EOImode_EL1NS` bit.
        const EOIMODE_EL1NS = 1 << 4;
        /// `RM` bit.
        const RM = 1 << 5;
        /// `PMHE` bit.
        const PMHE = 1 << 6;
        /// `SEIS` bit.
        const SEIS = 1 << 14;
        /// `A3V` bit.
        const A3V = 1 << 15;
        /// `nDS` bit.
        const NDS = 1 << 17;
        /// `RSS` bit.
        const RSS = 1 << 18;
        /// `ExtRange` bit.
        const EXTRANGE = 1 << 19;
    }
}

impl IccMctlr {
    /// Offset of the `CBPR_EL1S` field.
    pub const CBPR_EL1S_SHIFT: u32 = 0;
    /// Offset of the `CBPR_EL1NS` field.
    pub const CBPR_EL1NS_SHIFT: u32 = 1;
    /// Offset of the `EOImode_EL3` field.
    pub const EOIMODE_EL3_SHIFT: u32 = 2;
    /// Offset of the `EOImode_EL1S` field.
    pub const EOIMODE_EL1S_SHIFT: u32 = 3;
    /// Offset of the `EOImode_EL1NS` field.
    pub const EOIMODE_EL1NS_SHIFT: u32 = 4;
    /// Offset of the `RM` field.
    pub const RM_SHIFT: u32 = 5;
    /// Offset of the `PMHE` field.
    pub const PMHE_SHIFT: u32 = 6;
    /// Offset of the `PRIbits` field.
    pub const PRIBITS_SHIFT: u32 = 8;
    /// Mask for the `PRIbits` field.
    pub const PRIBITS_MASK: u32 = 0b111;
    /// Offset of the `IDbits` field.
    pub const IDBITS_SHIFT: u32 = 11;
    /// Mask for the `IDbits` field.
    pub const IDBITS_MASK: u32 = 0b111;
    /// Offset of the `SEIS` field.
    pub const SEIS_SHIFT: u32 = 14;
    /// Offset of the `A3V` field.
    pub const A3V_SHIFT: u32 = 15;
    /// Offset of the `nDS` field.
    pub const NDS_SHIFT: u32 = 17;
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
            (self.bits() & !(Self::PRIBITS_MASK << offset)) | ((value as u32) << offset),
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
            (self.bits() & !(Self::IDBITS_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `IDbits` field set to the given value.
    pub const fn with_idbits(mut self, value: u8) -> Self {
        self.set_idbits(value);
        self
    }
}

bitflags! {
    /// `ICC_MGRPEN1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccMgrpen1: u32 {
        /// `EnableGrp1NS` bit.
        const ENABLEGRP1NS = 1 << 0;
        /// `EnableGrp1S` bit.
        const ENABLEGRP1S = 1 << 1;
    }
}

impl IccMgrpen1 {
    /// Offset of the `EnableGrp1NS` field.
    pub const ENABLEGRP1NS_SHIFT: u32 = 0;
    /// Offset of the `EnableGrp1S` field.
    pub const ENABLEGRP1S_SHIFT: u32 = 1;
}

bitflags! {
    /// `ICC_MSRE` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccMsre: u32 {
        /// `SRE` bit.
        const SRE = 1 << 0;
        /// `DFB` bit.
        const DFB = 1 << 1;
        /// `DIB` bit.
        const DIB = 1 << 2;
        /// `Enable` bit.
        const ENABLE = 1 << 3;
    }
}

impl IccMsre {
    /// Offset of the `SRE` field.
    pub const SRE_SHIFT: u32 = 0;
    /// Offset of the `DFB` field.
    pub const DFB_SHIFT: u32 = 1;
    /// Offset of the `DIB` field.
    pub const DIB_SHIFT: u32 = 2;
    /// Offset of the `Enable` field.
    pub const ENABLE_SHIFT: u32 = 3;
}

bitflags! {
    /// `ICC_PMR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccPmr: u32 {
    }
}

impl IccPmr {
    /// Offset of the `Priority` field.
    pub const PRIORITY_SHIFT: u32 = 0;
    /// Mask for the `Priority` field.
    pub const PRIORITY_MASK: u32 = 0b1111_1111;

    /// Returns the value of the `Priority` field.
    pub const fn priority(self) -> u8 {
        ((self.bits() >> Self::PRIORITY_SHIFT) & Self::PRIORITY_MASK) as u8
    }

    /// Sets the value of the `Priority` field.
    pub const fn set_priority(&mut self, value: u8) {
        let offset = Self::PRIORITY_SHIFT;
        assert!(value & (Self::PRIORITY_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PRIORITY_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `Priority` field set to the given value.
    pub const fn with_priority(mut self, value: u8) -> Self {
        self.set_priority(value);
        self
    }
}

bitflags! {
    /// `ICC_RPR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccRpr: u32 {
    }
}

impl IccRpr {
    /// Offset of the `Priority` field.
    pub const PRIORITY_SHIFT: u32 = 0;
    /// Mask for the `Priority` field.
    pub const PRIORITY_MASK: u32 = 0b1111_1111;

    /// Returns the value of the `Priority` field.
    pub const fn priority(self) -> u8 {
        ((self.bits() >> Self::PRIORITY_SHIFT) & Self::PRIORITY_MASK) as u8
    }

    /// Sets the value of the `Priority` field.
    pub const fn set_priority(&mut self, value: u8) {
        let offset = Self::PRIORITY_SHIFT;
        assert!(value & (Self::PRIORITY_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PRIORITY_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `Priority` field set to the given value.
    pub const fn with_priority(mut self, value: u8) -> Self {
        self.set_priority(value);
        self
    }
}

bitflags! {
    /// `ICC_SGI0R` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccSgi0r: u64 {
        /// `IRM` bit.
        const IRM = 1 << 40;
    }
}

impl IccSgi0r {
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
    /// `ICC_SGI1R` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccSgi1r: u64 {
        /// `IRM` bit.
        const IRM = 1 << 40;
    }
}

impl IccSgi1r {
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
    /// `ICC_SRE` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccSre: u32 {
        /// `SRE` bit.
        const SRE = 1 << 0;
        /// `DFB` bit.
        const DFB = 1 << 1;
        /// `DIB` bit.
        const DIB = 1 << 2;
    }
}

impl IccSre {
    /// Offset of the `SRE` field.
    pub const SRE_SHIFT: u32 = 0;
    /// Offset of the `DFB` field.
    pub const DFB_SHIFT: u32 = 1;
    /// Offset of the `DIB` field.
    pub const DIB_SHIFT: u32 = 2;
}

bitflags! {
    /// `ID_DFR0` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdDfr0: u32 {
    }
}

impl IdDfr0 {
    /// Offset of the `CopDbg` field.
    pub const COPDBG_SHIFT: u32 = 0;
    /// Mask for the `CopDbg` field.
    pub const COPDBG_MASK: u32 = 0b1111;
    /// Offset of the `CopSDbg` field.
    pub const COPSDBG_SHIFT: u32 = 4;
    /// Mask for the `CopSDbg` field.
    pub const COPSDBG_MASK: u32 = 0b1111;
    /// Offset of the `MMapDbg` field.
    pub const MMAPDBG_SHIFT: u32 = 8;
    /// Mask for the `MMapDbg` field.
    pub const MMAPDBG_MASK: u32 = 0b1111;
    /// Offset of the `CopTrc` field.
    pub const COPTRC_SHIFT: u32 = 12;
    /// Mask for the `CopTrc` field.
    pub const COPTRC_MASK: u32 = 0b1111;
    /// Offset of the `MMapTrc` field.
    pub const MMAPTRC_SHIFT: u32 = 16;
    /// Mask for the `MMapTrc` field.
    pub const MMAPTRC_MASK: u32 = 0b1111;
    /// Offset of the `MProfDbg` field.
    pub const MPROFDBG_SHIFT: u32 = 20;
    /// Mask for the `MProfDbg` field.
    pub const MPROFDBG_MASK: u32 = 0b1111;
    /// Offset of the `PerfMon` field.
    pub const PERFMON_SHIFT: u32 = 24;
    /// Mask for the `PerfMon` field.
    pub const PERFMON_MASK: u32 = 0b1111;
    /// Offset of the `TraceFilt` field.
    pub const TRACEFILT_SHIFT: u32 = 28;
    /// Mask for the `TraceFilt` field.
    pub const TRACEFILT_MASK: u32 = 0b1111;

    /// Returns the value of the `CopDbg` field.
    pub const fn copdbg(self) -> u8 {
        ((self.bits() >> Self::COPDBG_SHIFT) & Self::COPDBG_MASK) as u8
    }

    /// Sets the value of the `CopDbg` field.
    pub const fn set_copdbg(&mut self, value: u8) {
        let offset = Self::COPDBG_SHIFT;
        assert!(value & (Self::COPDBG_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::COPDBG_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `CopDbg` field set to the given value.
    pub const fn with_copdbg(mut self, value: u8) -> Self {
        self.set_copdbg(value);
        self
    }

    /// Returns the value of the `CopSDbg` field.
    pub const fn copsdbg(self) -> u8 {
        ((self.bits() >> Self::COPSDBG_SHIFT) & Self::COPSDBG_MASK) as u8
    }

    /// Sets the value of the `CopSDbg` field.
    pub const fn set_copsdbg(&mut self, value: u8) {
        let offset = Self::COPSDBG_SHIFT;
        assert!(value & (Self::COPSDBG_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::COPSDBG_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `CopSDbg` field set to the given value.
    pub const fn with_copsdbg(mut self, value: u8) -> Self {
        self.set_copsdbg(value);
        self
    }

    /// Returns the value of the `MMapDbg` field.
    pub const fn mmapdbg(self) -> u8 {
        ((self.bits() >> Self::MMAPDBG_SHIFT) & Self::MMAPDBG_MASK) as u8
    }

    /// Sets the value of the `MMapDbg` field.
    pub const fn set_mmapdbg(&mut self, value: u8) {
        let offset = Self::MMAPDBG_SHIFT;
        assert!(value & (Self::MMAPDBG_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::MMAPDBG_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `MMapDbg` field set to the given value.
    pub const fn with_mmapdbg(mut self, value: u8) -> Self {
        self.set_mmapdbg(value);
        self
    }

    /// Returns the value of the `CopTrc` field.
    pub const fn coptrc(self) -> u8 {
        ((self.bits() >> Self::COPTRC_SHIFT) & Self::COPTRC_MASK) as u8
    }

    /// Sets the value of the `CopTrc` field.
    pub const fn set_coptrc(&mut self, value: u8) {
        let offset = Self::COPTRC_SHIFT;
        assert!(value & (Self::COPTRC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::COPTRC_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `CopTrc` field set to the given value.
    pub const fn with_coptrc(mut self, value: u8) -> Self {
        self.set_coptrc(value);
        self
    }

    /// Returns the value of the `MMapTrc` field.
    pub const fn mmaptrc(self) -> u8 {
        ((self.bits() >> Self::MMAPTRC_SHIFT) & Self::MMAPTRC_MASK) as u8
    }

    /// Sets the value of the `MMapTrc` field.
    pub const fn set_mmaptrc(&mut self, value: u8) {
        let offset = Self::MMAPTRC_SHIFT;
        assert!(value & (Self::MMAPTRC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::MMAPTRC_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `MMapTrc` field set to the given value.
    pub const fn with_mmaptrc(mut self, value: u8) -> Self {
        self.set_mmaptrc(value);
        self
    }

    /// Returns the value of the `MProfDbg` field.
    pub const fn mprofdbg(self) -> u8 {
        ((self.bits() >> Self::MPROFDBG_SHIFT) & Self::MPROFDBG_MASK) as u8
    }

    /// Sets the value of the `MProfDbg` field.
    pub const fn set_mprofdbg(&mut self, value: u8) {
        let offset = Self::MPROFDBG_SHIFT;
        assert!(value & (Self::MPROFDBG_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::MPROFDBG_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `MProfDbg` field set to the given value.
    pub const fn with_mprofdbg(mut self, value: u8) -> Self {
        self.set_mprofdbg(value);
        self
    }

    /// Returns the value of the `PerfMon` field.
    pub const fn perfmon(self) -> u8 {
        ((self.bits() >> Self::PERFMON_SHIFT) & Self::PERFMON_MASK) as u8
    }

    /// Sets the value of the `PerfMon` field.
    pub const fn set_perfmon(&mut self, value: u8) {
        let offset = Self::PERFMON_SHIFT;
        assert!(value & (Self::PERFMON_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PERFMON_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `PerfMon` field set to the given value.
    pub const fn with_perfmon(mut self, value: u8) -> Self {
        self.set_perfmon(value);
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
            (self.bits() & !(Self::TRACEFILT_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `TraceFilt` field set to the given value.
    pub const fn with_tracefilt(mut self, value: u8) -> Self {
        self.set_tracefilt(value);
        self
    }
}

bitflags! {
    /// `ID_DFR1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdDfr1: u32 {
    }
}

impl IdDfr1 {
    /// Offset of the `MTPMU` field.
    pub const MTPMU_SHIFT: u32 = 0;
    /// Mask for the `MTPMU` field.
    pub const MTPMU_MASK: u32 = 0b1111;
    /// Offset of the `HPMN0` field.
    pub const HPMN0_SHIFT: u32 = 4;
    /// Mask for the `HPMN0` field.
    pub const HPMN0_MASK: u32 = 0b1111;

    /// Returns the value of the `MTPMU` field.
    pub const fn mtpmu(self) -> u8 {
        ((self.bits() >> Self::MTPMU_SHIFT) & Self::MTPMU_MASK) as u8
    }

    /// Sets the value of the `MTPMU` field.
    pub const fn set_mtpmu(&mut self, value: u8) {
        let offset = Self::MTPMU_SHIFT;
        assert!(value & (Self::MTPMU_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::MTPMU_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `MTPMU` field set to the given value.
    pub const fn with_mtpmu(mut self, value: u8) -> Self {
        self.set_mtpmu(value);
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
            (self.bits() & !(Self::HPMN0_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `HPMN0` field set to the given value.
    pub const fn with_hpmn0(mut self, value: u8) -> Self {
        self.set_hpmn0(value);
        self
    }
}

bitflags! {
    /// `ID_ISAR0` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdIsar0: u32 {
    }
}

impl IdIsar0 {
    /// Offset of the `Swap` field.
    pub const SWAP_SHIFT: u32 = 0;
    /// Mask for the `Swap` field.
    pub const SWAP_MASK: u32 = 0b1111;
    /// Offset of the `BitCount` field.
    pub const BITCOUNT_SHIFT: u32 = 4;
    /// Mask for the `BitCount` field.
    pub const BITCOUNT_MASK: u32 = 0b1111;
    /// Offset of the `BitField` field.
    pub const BITFIELD_SHIFT: u32 = 8;
    /// Mask for the `BitField` field.
    pub const BITFIELD_MASK: u32 = 0b1111;
    /// Offset of the `CmpBranch` field.
    pub const CMPBRANCH_SHIFT: u32 = 12;
    /// Mask for the `CmpBranch` field.
    pub const CMPBRANCH_MASK: u32 = 0b1111;
    /// Offset of the `Coproc` field.
    pub const COPROC_SHIFT: u32 = 16;
    /// Mask for the `Coproc` field.
    pub const COPROC_MASK: u32 = 0b1111;
    /// Offset of the `Debug` field.
    pub const DEBUG_SHIFT: u32 = 20;
    /// Mask for the `Debug` field.
    pub const DEBUG_MASK: u32 = 0b1111;
    /// Offset of the `Divide` field.
    pub const DIVIDE_SHIFT: u32 = 24;
    /// Mask for the `Divide` field.
    pub const DIVIDE_MASK: u32 = 0b1111;

    /// Returns the value of the `Swap` field.
    pub const fn swap(self) -> u8 {
        ((self.bits() >> Self::SWAP_SHIFT) & Self::SWAP_MASK) as u8
    }

    /// Sets the value of the `Swap` field.
    pub const fn set_swap(&mut self, value: u8) {
        let offset = Self::SWAP_SHIFT;
        assert!(value & (Self::SWAP_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SWAP_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `Swap` field set to the given value.
    pub const fn with_swap(mut self, value: u8) -> Self {
        self.set_swap(value);
        self
    }

    /// Returns the value of the `BitCount` field.
    pub const fn bitcount(self) -> u8 {
        ((self.bits() >> Self::BITCOUNT_SHIFT) & Self::BITCOUNT_MASK) as u8
    }

    /// Sets the value of the `BitCount` field.
    pub const fn set_bitcount(&mut self, value: u8) {
        let offset = Self::BITCOUNT_SHIFT;
        assert!(value & (Self::BITCOUNT_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BITCOUNT_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `BitCount` field set to the given value.
    pub const fn with_bitcount(mut self, value: u8) -> Self {
        self.set_bitcount(value);
        self
    }

    /// Returns the value of the `BitField` field.
    pub const fn bitfield(self) -> u8 {
        ((self.bits() >> Self::BITFIELD_SHIFT) & Self::BITFIELD_MASK) as u8
    }

    /// Sets the value of the `BitField` field.
    pub const fn set_bitfield(&mut self, value: u8) {
        let offset = Self::BITFIELD_SHIFT;
        assert!(value & (Self::BITFIELD_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BITFIELD_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `BitField` field set to the given value.
    pub const fn with_bitfield(mut self, value: u8) -> Self {
        self.set_bitfield(value);
        self
    }

    /// Returns the value of the `CmpBranch` field.
    pub const fn cmpbranch(self) -> u8 {
        ((self.bits() >> Self::CMPBRANCH_SHIFT) & Self::CMPBRANCH_MASK) as u8
    }

    /// Sets the value of the `CmpBranch` field.
    pub const fn set_cmpbranch(&mut self, value: u8) {
        let offset = Self::CMPBRANCH_SHIFT;
        assert!(value & (Self::CMPBRANCH_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CMPBRANCH_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `CmpBranch` field set to the given value.
    pub const fn with_cmpbranch(mut self, value: u8) -> Self {
        self.set_cmpbranch(value);
        self
    }

    /// Returns the value of the `Coproc` field.
    pub const fn coproc(self) -> u8 {
        ((self.bits() >> Self::COPROC_SHIFT) & Self::COPROC_MASK) as u8
    }

    /// Sets the value of the `Coproc` field.
    pub const fn set_coproc(&mut self, value: u8) {
        let offset = Self::COPROC_SHIFT;
        assert!(value & (Self::COPROC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::COPROC_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `Coproc` field set to the given value.
    pub const fn with_coproc(mut self, value: u8) -> Self {
        self.set_coproc(value);
        self
    }

    /// Returns the value of the `Debug` field.
    pub const fn debug(self) -> u8 {
        ((self.bits() >> Self::DEBUG_SHIFT) & Self::DEBUG_MASK) as u8
    }

    /// Sets the value of the `Debug` field.
    pub const fn set_debug(&mut self, value: u8) {
        let offset = Self::DEBUG_SHIFT;
        assert!(value & (Self::DEBUG_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::DEBUG_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `Debug` field set to the given value.
    pub const fn with_debug(mut self, value: u8) -> Self {
        self.set_debug(value);
        self
    }

    /// Returns the value of the `Divide` field.
    pub const fn divide(self) -> u8 {
        ((self.bits() >> Self::DIVIDE_SHIFT) & Self::DIVIDE_MASK) as u8
    }

    /// Sets the value of the `Divide` field.
    pub const fn set_divide(&mut self, value: u8) {
        let offset = Self::DIVIDE_SHIFT;
        assert!(value & (Self::DIVIDE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::DIVIDE_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `Divide` field set to the given value.
    pub const fn with_divide(mut self, value: u8) -> Self {
        self.set_divide(value);
        self
    }
}

bitflags! {
    /// `ID_ISAR1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdIsar1: u32 {
    }
}

impl IdIsar1 {
    /// Offset of the `Endian` field.
    pub const ENDIAN_SHIFT: u32 = 0;
    /// Mask for the `Endian` field.
    pub const ENDIAN_MASK: u32 = 0b1111;
    /// Offset of the `Except` field.
    pub const EXCEPT_SHIFT: u32 = 4;
    /// Mask for the `Except` field.
    pub const EXCEPT_MASK: u32 = 0b1111;
    /// Offset of the `Except_AR` field.
    pub const EXCEPT_AR_SHIFT: u32 = 8;
    /// Mask for the `Except_AR` field.
    pub const EXCEPT_AR_MASK: u32 = 0b1111;
    /// Offset of the `Extend` field.
    pub const EXTEND_SHIFT: u32 = 12;
    /// Mask for the `Extend` field.
    pub const EXTEND_MASK: u32 = 0b1111;
    /// Offset of the `IfThen` field.
    pub const IFTHEN_SHIFT: u32 = 16;
    /// Mask for the `IfThen` field.
    pub const IFTHEN_MASK: u32 = 0b1111;
    /// Offset of the `Immediate` field.
    pub const IMMEDIATE_SHIFT: u32 = 20;
    /// Mask for the `Immediate` field.
    pub const IMMEDIATE_MASK: u32 = 0b1111;
    /// Offset of the `Interwork` field.
    pub const INTERWORK_SHIFT: u32 = 24;
    /// Mask for the `Interwork` field.
    pub const INTERWORK_MASK: u32 = 0b1111;
    /// Offset of the `Jazelle` field.
    pub const JAZELLE_SHIFT: u32 = 28;
    /// Mask for the `Jazelle` field.
    pub const JAZELLE_MASK: u32 = 0b1111;

    /// Returns the value of the `Endian` field.
    pub const fn endian(self) -> u8 {
        ((self.bits() >> Self::ENDIAN_SHIFT) & Self::ENDIAN_MASK) as u8
    }

    /// Sets the value of the `Endian` field.
    pub const fn set_endian(&mut self, value: u8) {
        let offset = Self::ENDIAN_SHIFT;
        assert!(value & (Self::ENDIAN_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ENDIAN_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `Endian` field set to the given value.
    pub const fn with_endian(mut self, value: u8) -> Self {
        self.set_endian(value);
        self
    }

    /// Returns the value of the `Except` field.
    pub const fn except(self) -> u8 {
        ((self.bits() >> Self::EXCEPT_SHIFT) & Self::EXCEPT_MASK) as u8
    }

    /// Sets the value of the `Except` field.
    pub const fn set_except(&mut self, value: u8) {
        let offset = Self::EXCEPT_SHIFT;
        assert!(value & (Self::EXCEPT_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EXCEPT_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `Except` field set to the given value.
    pub const fn with_except(mut self, value: u8) -> Self {
        self.set_except(value);
        self
    }

    /// Returns the value of the `Except_AR` field.
    pub const fn except_ar(self) -> u8 {
        ((self.bits() >> Self::EXCEPT_AR_SHIFT) & Self::EXCEPT_AR_MASK) as u8
    }

    /// Sets the value of the `Except_AR` field.
    pub const fn set_except_ar(&mut self, value: u8) {
        let offset = Self::EXCEPT_AR_SHIFT;
        assert!(value & (Self::EXCEPT_AR_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EXCEPT_AR_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `Except_AR` field set to the given value.
    pub const fn with_except_ar(mut self, value: u8) -> Self {
        self.set_except_ar(value);
        self
    }

    /// Returns the value of the `Extend` field.
    pub const fn extend_(self) -> u8 {
        ((self.bits() >> Self::EXTEND_SHIFT) & Self::EXTEND_MASK) as u8
    }

    /// Sets the value of the `Extend` field.
    pub const fn set_extend_(&mut self, value: u8) {
        let offset = Self::EXTEND_SHIFT;
        assert!(value & (Self::EXTEND_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EXTEND_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `Extend` field set to the given value.
    pub const fn with_extend_(mut self, value: u8) -> Self {
        self.set_extend_(value);
        self
    }

    /// Returns the value of the `IfThen` field.
    pub const fn ifthen(self) -> u8 {
        ((self.bits() >> Self::IFTHEN_SHIFT) & Self::IFTHEN_MASK) as u8
    }

    /// Sets the value of the `IfThen` field.
    pub const fn set_ifthen(&mut self, value: u8) {
        let offset = Self::IFTHEN_SHIFT;
        assert!(value & (Self::IFTHEN_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::IFTHEN_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `IfThen` field set to the given value.
    pub const fn with_ifthen(mut self, value: u8) -> Self {
        self.set_ifthen(value);
        self
    }

    /// Returns the value of the `Immediate` field.
    pub const fn immediate(self) -> u8 {
        ((self.bits() >> Self::IMMEDIATE_SHIFT) & Self::IMMEDIATE_MASK) as u8
    }

    /// Sets the value of the `Immediate` field.
    pub const fn set_immediate(&mut self, value: u8) {
        let offset = Self::IMMEDIATE_SHIFT;
        assert!(value & (Self::IMMEDIATE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::IMMEDIATE_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `Immediate` field set to the given value.
    pub const fn with_immediate(mut self, value: u8) -> Self {
        self.set_immediate(value);
        self
    }

    /// Returns the value of the `Interwork` field.
    pub const fn interwork(self) -> u8 {
        ((self.bits() >> Self::INTERWORK_SHIFT) & Self::INTERWORK_MASK) as u8
    }

    /// Sets the value of the `Interwork` field.
    pub const fn set_interwork(&mut self, value: u8) {
        let offset = Self::INTERWORK_SHIFT;
        assert!(value & (Self::INTERWORK_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::INTERWORK_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `Interwork` field set to the given value.
    pub const fn with_interwork(mut self, value: u8) -> Self {
        self.set_interwork(value);
        self
    }

    /// Returns the value of the `Jazelle` field.
    pub const fn jazelle(self) -> u8 {
        ((self.bits() >> Self::JAZELLE_SHIFT) & Self::JAZELLE_MASK) as u8
    }

    /// Sets the value of the `Jazelle` field.
    pub const fn set_jazelle(&mut self, value: u8) {
        let offset = Self::JAZELLE_SHIFT;
        assert!(value & (Self::JAZELLE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::JAZELLE_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `Jazelle` field set to the given value.
    pub const fn with_jazelle(mut self, value: u8) -> Self {
        self.set_jazelle(value);
        self
    }
}

bitflags! {
    /// `ID_ISAR2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdIsar2: u32 {
    }
}

impl IdIsar2 {
    /// Offset of the `LoadStore` field.
    pub const LOADSTORE_SHIFT: u32 = 0;
    /// Mask for the `LoadStore` field.
    pub const LOADSTORE_MASK: u32 = 0b1111;
    /// Offset of the `MemHint` field.
    pub const MEMHINT_SHIFT: u32 = 4;
    /// Mask for the `MemHint` field.
    pub const MEMHINT_MASK: u32 = 0b1111;
    /// Offset of the `MultiAccessInt` field.
    pub const MULTIACCESSINT_SHIFT: u32 = 8;
    /// Mask for the `MultiAccessInt` field.
    pub const MULTIACCESSINT_MASK: u32 = 0b1111;
    /// Offset of the `Mult` field.
    pub const MULT_SHIFT: u32 = 12;
    /// Mask for the `Mult` field.
    pub const MULT_MASK: u32 = 0b1111;
    /// Offset of the `MultS` field.
    pub const MULTS_SHIFT: u32 = 16;
    /// Mask for the `MultS` field.
    pub const MULTS_MASK: u32 = 0b1111;
    /// Offset of the `MultU` field.
    pub const MULTU_SHIFT: u32 = 20;
    /// Mask for the `MultU` field.
    pub const MULTU_MASK: u32 = 0b1111;
    /// Offset of the `PSR_AR` field.
    pub const PSR_AR_SHIFT: u32 = 24;
    /// Mask for the `PSR_AR` field.
    pub const PSR_AR_MASK: u32 = 0b1111;
    /// Offset of the `Reversal` field.
    pub const REVERSAL_SHIFT: u32 = 28;
    /// Mask for the `Reversal` field.
    pub const REVERSAL_MASK: u32 = 0b1111;

    /// Returns the value of the `LoadStore` field.
    pub const fn loadstore(self) -> u8 {
        ((self.bits() >> Self::LOADSTORE_SHIFT) & Self::LOADSTORE_MASK) as u8
    }

    /// Sets the value of the `LoadStore` field.
    pub const fn set_loadstore(&mut self, value: u8) {
        let offset = Self::LOADSTORE_SHIFT;
        assert!(value & (Self::LOADSTORE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::LOADSTORE_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `LoadStore` field set to the given value.
    pub const fn with_loadstore(mut self, value: u8) -> Self {
        self.set_loadstore(value);
        self
    }

    /// Returns the value of the `MemHint` field.
    pub const fn memhint(self) -> u8 {
        ((self.bits() >> Self::MEMHINT_SHIFT) & Self::MEMHINT_MASK) as u8
    }

    /// Sets the value of the `MemHint` field.
    pub const fn set_memhint(&mut self, value: u8) {
        let offset = Self::MEMHINT_SHIFT;
        assert!(value & (Self::MEMHINT_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::MEMHINT_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `MemHint` field set to the given value.
    pub const fn with_memhint(mut self, value: u8) -> Self {
        self.set_memhint(value);
        self
    }

    /// Returns the value of the `MultiAccessInt` field.
    pub const fn multiaccessint(self) -> u8 {
        ((self.bits() >> Self::MULTIACCESSINT_SHIFT) & Self::MULTIACCESSINT_MASK) as u8
    }

    /// Sets the value of the `MultiAccessInt` field.
    pub const fn set_multiaccessint(&mut self, value: u8) {
        let offset = Self::MULTIACCESSINT_SHIFT;
        assert!(value & (Self::MULTIACCESSINT_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::MULTIACCESSINT_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `MultiAccessInt` field set to the given value.
    pub const fn with_multiaccessint(mut self, value: u8) -> Self {
        self.set_multiaccessint(value);
        self
    }

    /// Returns the value of the `Mult` field.
    pub const fn mult(self) -> u8 {
        ((self.bits() >> Self::MULT_SHIFT) & Self::MULT_MASK) as u8
    }

    /// Sets the value of the `Mult` field.
    pub const fn set_mult(&mut self, value: u8) {
        let offset = Self::MULT_SHIFT;
        assert!(value & (Self::MULT_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::MULT_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `Mult` field set to the given value.
    pub const fn with_mult(mut self, value: u8) -> Self {
        self.set_mult(value);
        self
    }

    /// Returns the value of the `MultS` field.
    pub const fn mults(self) -> u8 {
        ((self.bits() >> Self::MULTS_SHIFT) & Self::MULTS_MASK) as u8
    }

    /// Sets the value of the `MultS` field.
    pub const fn set_mults(&mut self, value: u8) {
        let offset = Self::MULTS_SHIFT;
        assert!(value & (Self::MULTS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::MULTS_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `MultS` field set to the given value.
    pub const fn with_mults(mut self, value: u8) -> Self {
        self.set_mults(value);
        self
    }

    /// Returns the value of the `MultU` field.
    pub const fn multu(self) -> u8 {
        ((self.bits() >> Self::MULTU_SHIFT) & Self::MULTU_MASK) as u8
    }

    /// Sets the value of the `MultU` field.
    pub const fn set_multu(&mut self, value: u8) {
        let offset = Self::MULTU_SHIFT;
        assert!(value & (Self::MULTU_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::MULTU_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `MultU` field set to the given value.
    pub const fn with_multu(mut self, value: u8) -> Self {
        self.set_multu(value);
        self
    }

    /// Returns the value of the `PSR_AR` field.
    pub const fn psr_ar(self) -> u8 {
        ((self.bits() >> Self::PSR_AR_SHIFT) & Self::PSR_AR_MASK) as u8
    }

    /// Sets the value of the `PSR_AR` field.
    pub const fn set_psr_ar(&mut self, value: u8) {
        let offset = Self::PSR_AR_SHIFT;
        assert!(value & (Self::PSR_AR_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PSR_AR_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `PSR_AR` field set to the given value.
    pub const fn with_psr_ar(mut self, value: u8) -> Self {
        self.set_psr_ar(value);
        self
    }

    /// Returns the value of the `Reversal` field.
    pub const fn reversal(self) -> u8 {
        ((self.bits() >> Self::REVERSAL_SHIFT) & Self::REVERSAL_MASK) as u8
    }

    /// Sets the value of the `Reversal` field.
    pub const fn set_reversal(&mut self, value: u8) {
        let offset = Self::REVERSAL_SHIFT;
        assert!(value & (Self::REVERSAL_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::REVERSAL_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `Reversal` field set to the given value.
    pub const fn with_reversal(mut self, value: u8) -> Self {
        self.set_reversal(value);
        self
    }
}

bitflags! {
    /// `ID_ISAR3` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdIsar3: u32 {
    }
}

impl IdIsar3 {
    /// Offset of the `Saturate` field.
    pub const SATURATE_SHIFT: u32 = 0;
    /// Mask for the `Saturate` field.
    pub const SATURATE_MASK: u32 = 0b1111;
    /// Offset of the `SIMD` field.
    pub const SIMD_SHIFT: u32 = 4;
    /// Mask for the `SIMD` field.
    pub const SIMD_MASK: u32 = 0b1111;
    /// Offset of the `SVC` field.
    pub const SVC_SHIFT: u32 = 8;
    /// Mask for the `SVC` field.
    pub const SVC_MASK: u32 = 0b1111;
    /// Offset of the `SynchPrim` field.
    pub const SYNCHPRIM_SHIFT: u32 = 12;
    /// Mask for the `SynchPrim` field.
    pub const SYNCHPRIM_MASK: u32 = 0b1111;
    /// Offset of the `TabBranch` field.
    pub const TABBRANCH_SHIFT: u32 = 16;
    /// Mask for the `TabBranch` field.
    pub const TABBRANCH_MASK: u32 = 0b1111;
    /// Offset of the `T32Copy` field.
    pub const T32COPY_SHIFT: u32 = 20;
    /// Mask for the `T32Copy` field.
    pub const T32COPY_MASK: u32 = 0b1111;
    /// Offset of the `TrueNOP` field.
    pub const TRUENOP_SHIFT: u32 = 24;
    /// Mask for the `TrueNOP` field.
    pub const TRUENOP_MASK: u32 = 0b1111;
    /// Offset of the `T32EE` field.
    pub const T32EE_SHIFT: u32 = 28;
    /// Mask for the `T32EE` field.
    pub const T32EE_MASK: u32 = 0b1111;

    /// Returns the value of the `Saturate` field.
    pub const fn saturate(self) -> u8 {
        ((self.bits() >> Self::SATURATE_SHIFT) & Self::SATURATE_MASK) as u8
    }

    /// Sets the value of the `Saturate` field.
    pub const fn set_saturate(&mut self, value: u8) {
        let offset = Self::SATURATE_SHIFT;
        assert!(value & (Self::SATURATE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SATURATE_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `Saturate` field set to the given value.
    pub const fn with_saturate(mut self, value: u8) -> Self {
        self.set_saturate(value);
        self
    }

    /// Returns the value of the `SIMD` field.
    pub const fn simd(self) -> u8 {
        ((self.bits() >> Self::SIMD_SHIFT) & Self::SIMD_MASK) as u8
    }

    /// Sets the value of the `SIMD` field.
    pub const fn set_simd(&mut self, value: u8) {
        let offset = Self::SIMD_SHIFT;
        assert!(value & (Self::SIMD_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SIMD_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `SIMD` field set to the given value.
    pub const fn with_simd(mut self, value: u8) -> Self {
        self.set_simd(value);
        self
    }

    /// Returns the value of the `SVC` field.
    pub const fn svc(self) -> u8 {
        ((self.bits() >> Self::SVC_SHIFT) & Self::SVC_MASK) as u8
    }

    /// Sets the value of the `SVC` field.
    pub const fn set_svc(&mut self, value: u8) {
        let offset = Self::SVC_SHIFT;
        assert!(value & (Self::SVC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SVC_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `SVC` field set to the given value.
    pub const fn with_svc(mut self, value: u8) -> Self {
        self.set_svc(value);
        self
    }

    /// Returns the value of the `SynchPrim` field.
    pub const fn synchprim(self) -> u8 {
        ((self.bits() >> Self::SYNCHPRIM_SHIFT) & Self::SYNCHPRIM_MASK) as u8
    }

    /// Sets the value of the `SynchPrim` field.
    pub const fn set_synchprim(&mut self, value: u8) {
        let offset = Self::SYNCHPRIM_SHIFT;
        assert!(value & (Self::SYNCHPRIM_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SYNCHPRIM_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `SynchPrim` field set to the given value.
    pub const fn with_synchprim(mut self, value: u8) -> Self {
        self.set_synchprim(value);
        self
    }

    /// Returns the value of the `TabBranch` field.
    pub const fn tabbranch(self) -> u8 {
        ((self.bits() >> Self::TABBRANCH_SHIFT) & Self::TABBRANCH_MASK) as u8
    }

    /// Sets the value of the `TabBranch` field.
    pub const fn set_tabbranch(&mut self, value: u8) {
        let offset = Self::TABBRANCH_SHIFT;
        assert!(value & (Self::TABBRANCH_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TABBRANCH_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `TabBranch` field set to the given value.
    pub const fn with_tabbranch(mut self, value: u8) -> Self {
        self.set_tabbranch(value);
        self
    }

    /// Returns the value of the `T32Copy` field.
    pub const fn t32copy(self) -> u8 {
        ((self.bits() >> Self::T32COPY_SHIFT) & Self::T32COPY_MASK) as u8
    }

    /// Sets the value of the `T32Copy` field.
    pub const fn set_t32copy(&mut self, value: u8) {
        let offset = Self::T32COPY_SHIFT;
        assert!(value & (Self::T32COPY_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::T32COPY_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `T32Copy` field set to the given value.
    pub const fn with_t32copy(mut self, value: u8) -> Self {
        self.set_t32copy(value);
        self
    }

    /// Returns the value of the `TrueNOP` field.
    pub const fn truenop(self) -> u8 {
        ((self.bits() >> Self::TRUENOP_SHIFT) & Self::TRUENOP_MASK) as u8
    }

    /// Sets the value of the `TrueNOP` field.
    pub const fn set_truenop(&mut self, value: u8) {
        let offset = Self::TRUENOP_SHIFT;
        assert!(value & (Self::TRUENOP_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TRUENOP_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `TrueNOP` field set to the given value.
    pub const fn with_truenop(mut self, value: u8) -> Self {
        self.set_truenop(value);
        self
    }

    /// Returns the value of the `T32EE` field.
    pub const fn t32ee(self) -> u8 {
        ((self.bits() >> Self::T32EE_SHIFT) & Self::T32EE_MASK) as u8
    }

    /// Sets the value of the `T32EE` field.
    pub const fn set_t32ee(&mut self, value: u8) {
        let offset = Self::T32EE_SHIFT;
        assert!(value & (Self::T32EE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::T32EE_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `T32EE` field set to the given value.
    pub const fn with_t32ee(mut self, value: u8) -> Self {
        self.set_t32ee(value);
        self
    }
}

bitflags! {
    /// `ID_ISAR4` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdIsar4: u32 {
    }
}

impl IdIsar4 {
    /// Offset of the `Unpriv` field.
    pub const UNPRIV_SHIFT: u32 = 0;
    /// Mask for the `Unpriv` field.
    pub const UNPRIV_MASK: u32 = 0b1111;
    /// Offset of the `WithShifts` field.
    pub const WITHSHIFTS_SHIFT: u32 = 4;
    /// Mask for the `WithShifts` field.
    pub const WITHSHIFTS_MASK: u32 = 0b1111;
    /// Offset of the `Writeback` field.
    pub const WRITEBACK_SHIFT: u32 = 8;
    /// Mask for the `Writeback` field.
    pub const WRITEBACK_MASK: u32 = 0b1111;
    /// Offset of the `SMC` field.
    pub const SMC_SHIFT: u32 = 12;
    /// Mask for the `SMC` field.
    pub const SMC_MASK: u32 = 0b1111;
    /// Offset of the `Barrier` field.
    pub const BARRIER_SHIFT: u32 = 16;
    /// Mask for the `Barrier` field.
    pub const BARRIER_MASK: u32 = 0b1111;
    /// Offset of the `SynchPrim_frac` field.
    pub const SYNCHPRIM_FRAC_SHIFT: u32 = 20;
    /// Mask for the `SynchPrim_frac` field.
    pub const SYNCHPRIM_FRAC_MASK: u32 = 0b1111;
    /// Offset of the `PSR_M` field.
    pub const PSR_M_SHIFT: u32 = 24;
    /// Mask for the `PSR_M` field.
    pub const PSR_M_MASK: u32 = 0b1111;
    /// Offset of the `SWP_frac` field.
    pub const SWP_FRAC_SHIFT: u32 = 28;
    /// Mask for the `SWP_frac` field.
    pub const SWP_FRAC_MASK: u32 = 0b1111;

    /// Returns the value of the `Unpriv` field.
    pub const fn unpriv(self) -> u8 {
        ((self.bits() >> Self::UNPRIV_SHIFT) & Self::UNPRIV_MASK) as u8
    }

    /// Sets the value of the `Unpriv` field.
    pub const fn set_unpriv(&mut self, value: u8) {
        let offset = Self::UNPRIV_SHIFT;
        assert!(value & (Self::UNPRIV_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::UNPRIV_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `Unpriv` field set to the given value.
    pub const fn with_unpriv(mut self, value: u8) -> Self {
        self.set_unpriv(value);
        self
    }

    /// Returns the value of the `WithShifts` field.
    pub const fn withshifts(self) -> u8 {
        ((self.bits() >> Self::WITHSHIFTS_SHIFT) & Self::WITHSHIFTS_MASK) as u8
    }

    /// Sets the value of the `WithShifts` field.
    pub const fn set_withshifts(&mut self, value: u8) {
        let offset = Self::WITHSHIFTS_SHIFT;
        assert!(value & (Self::WITHSHIFTS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::WITHSHIFTS_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `WithShifts` field set to the given value.
    pub const fn with_withshifts(mut self, value: u8) -> Self {
        self.set_withshifts(value);
        self
    }

    /// Returns the value of the `Writeback` field.
    pub const fn writeback(self) -> u8 {
        ((self.bits() >> Self::WRITEBACK_SHIFT) & Self::WRITEBACK_MASK) as u8
    }

    /// Sets the value of the `Writeback` field.
    pub const fn set_writeback(&mut self, value: u8) {
        let offset = Self::WRITEBACK_SHIFT;
        assert!(value & (Self::WRITEBACK_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::WRITEBACK_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `Writeback` field set to the given value.
    pub const fn with_writeback(mut self, value: u8) -> Self {
        self.set_writeback(value);
        self
    }

    /// Returns the value of the `SMC` field.
    pub const fn smc(self) -> u8 {
        ((self.bits() >> Self::SMC_SHIFT) & Self::SMC_MASK) as u8
    }

    /// Sets the value of the `SMC` field.
    pub const fn set_smc(&mut self, value: u8) {
        let offset = Self::SMC_SHIFT;
        assert!(value & (Self::SMC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SMC_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `SMC` field set to the given value.
    pub const fn with_smc(mut self, value: u8) -> Self {
        self.set_smc(value);
        self
    }

    /// Returns the value of the `Barrier` field.
    pub const fn barrier(self) -> u8 {
        ((self.bits() >> Self::BARRIER_SHIFT) & Self::BARRIER_MASK) as u8
    }

    /// Sets the value of the `Barrier` field.
    pub const fn set_barrier(&mut self, value: u8) {
        let offset = Self::BARRIER_SHIFT;
        assert!(value & (Self::BARRIER_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BARRIER_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `Barrier` field set to the given value.
    pub const fn with_barrier(mut self, value: u8) -> Self {
        self.set_barrier(value);
        self
    }

    /// Returns the value of the `SynchPrim_frac` field.
    pub const fn synchprim_frac(self) -> u8 {
        ((self.bits() >> Self::SYNCHPRIM_FRAC_SHIFT) & Self::SYNCHPRIM_FRAC_MASK) as u8
    }

    /// Sets the value of the `SynchPrim_frac` field.
    pub const fn set_synchprim_frac(&mut self, value: u8) {
        let offset = Self::SYNCHPRIM_FRAC_SHIFT;
        assert!(value & (Self::SYNCHPRIM_FRAC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SYNCHPRIM_FRAC_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `SynchPrim_frac` field set to the given value.
    pub const fn with_synchprim_frac(mut self, value: u8) -> Self {
        self.set_synchprim_frac(value);
        self
    }

    /// Returns the value of the `PSR_M` field.
    pub const fn psr_m(self) -> u8 {
        ((self.bits() >> Self::PSR_M_SHIFT) & Self::PSR_M_MASK) as u8
    }

    /// Sets the value of the `PSR_M` field.
    pub const fn set_psr_m(&mut self, value: u8) {
        let offset = Self::PSR_M_SHIFT;
        assert!(value & (Self::PSR_M_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PSR_M_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `PSR_M` field set to the given value.
    pub const fn with_psr_m(mut self, value: u8) -> Self {
        self.set_psr_m(value);
        self
    }

    /// Returns the value of the `SWP_frac` field.
    pub const fn swp_frac(self) -> u8 {
        ((self.bits() >> Self::SWP_FRAC_SHIFT) & Self::SWP_FRAC_MASK) as u8
    }

    /// Sets the value of the `SWP_frac` field.
    pub const fn set_swp_frac(&mut self, value: u8) {
        let offset = Self::SWP_FRAC_SHIFT;
        assert!(value & (Self::SWP_FRAC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SWP_FRAC_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `SWP_frac` field set to the given value.
    pub const fn with_swp_frac(mut self, value: u8) -> Self {
        self.set_swp_frac(value);
        self
    }
}

bitflags! {
    /// `ID_ISAR5` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdIsar5: u32 {
    }
}

impl IdIsar5 {
    /// Offset of the `SEVL` field.
    pub const SEVL_SHIFT: u32 = 0;
    /// Mask for the `SEVL` field.
    pub const SEVL_MASK: u32 = 0b1111;
    /// Offset of the `AES` field.
    pub const AES_SHIFT: u32 = 4;
    /// Mask for the `AES` field.
    pub const AES_MASK: u32 = 0b1111;
    /// Offset of the `SHA1` field.
    pub const SHA1_SHIFT: u32 = 8;
    /// Mask for the `SHA1` field.
    pub const SHA1_MASK: u32 = 0b1111;
    /// Offset of the `SHA2` field.
    pub const SHA2_SHIFT: u32 = 12;
    /// Mask for the `SHA2` field.
    pub const SHA2_MASK: u32 = 0b1111;
    /// Offset of the `CRC32` field.
    pub const CRC32_SHIFT: u32 = 16;
    /// Mask for the `CRC32` field.
    pub const CRC32_MASK: u32 = 0b1111;
    /// Offset of the `RDM` field.
    pub const RDM_SHIFT: u32 = 24;
    /// Mask for the `RDM` field.
    pub const RDM_MASK: u32 = 0b1111;
    /// Offset of the `VCMA` field.
    pub const VCMA_SHIFT: u32 = 28;
    /// Mask for the `VCMA` field.
    pub const VCMA_MASK: u32 = 0b1111;

    /// Returns the value of the `SEVL` field.
    pub const fn sevl(self) -> u8 {
        ((self.bits() >> Self::SEVL_SHIFT) & Self::SEVL_MASK) as u8
    }

    /// Sets the value of the `SEVL` field.
    pub const fn set_sevl(&mut self, value: u8) {
        let offset = Self::SEVL_SHIFT;
        assert!(value & (Self::SEVL_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SEVL_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `SEVL` field set to the given value.
    pub const fn with_sevl(mut self, value: u8) -> Self {
        self.set_sevl(value);
        self
    }

    /// Returns the value of the `AES` field.
    pub const fn aes(self) -> u8 {
        ((self.bits() >> Self::AES_SHIFT) & Self::AES_MASK) as u8
    }

    /// Sets the value of the `AES` field.
    pub const fn set_aes(&mut self, value: u8) {
        let offset = Self::AES_SHIFT;
        assert!(value & (Self::AES_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::AES_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `AES` field set to the given value.
    pub const fn with_aes(mut self, value: u8) -> Self {
        self.set_aes(value);
        self
    }

    /// Returns the value of the `SHA1` field.
    pub const fn sha1(self) -> u8 {
        ((self.bits() >> Self::SHA1_SHIFT) & Self::SHA1_MASK) as u8
    }

    /// Sets the value of the `SHA1` field.
    pub const fn set_sha1(&mut self, value: u8) {
        let offset = Self::SHA1_SHIFT;
        assert!(value & (Self::SHA1_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SHA1_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `SHA1` field set to the given value.
    pub const fn with_sha1(mut self, value: u8) -> Self {
        self.set_sha1(value);
        self
    }

    /// Returns the value of the `SHA2` field.
    pub const fn sha2(self) -> u8 {
        ((self.bits() >> Self::SHA2_SHIFT) & Self::SHA2_MASK) as u8
    }

    /// Sets the value of the `SHA2` field.
    pub const fn set_sha2(&mut self, value: u8) {
        let offset = Self::SHA2_SHIFT;
        assert!(value & (Self::SHA2_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SHA2_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `SHA2` field set to the given value.
    pub const fn with_sha2(mut self, value: u8) -> Self {
        self.set_sha2(value);
        self
    }

    /// Returns the value of the `CRC32` field.
    pub const fn crc32(self) -> u8 {
        ((self.bits() >> Self::CRC32_SHIFT) & Self::CRC32_MASK) as u8
    }

    /// Sets the value of the `CRC32` field.
    pub const fn set_crc32(&mut self, value: u8) {
        let offset = Self::CRC32_SHIFT;
        assert!(value & (Self::CRC32_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CRC32_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `CRC32` field set to the given value.
    pub const fn with_crc32(mut self, value: u8) -> Self {
        self.set_crc32(value);
        self
    }

    /// Returns the value of the `RDM` field.
    pub const fn rdm(self) -> u8 {
        ((self.bits() >> Self::RDM_SHIFT) & Self::RDM_MASK) as u8
    }

    /// Sets the value of the `RDM` field.
    pub const fn set_rdm(&mut self, value: u8) {
        let offset = Self::RDM_SHIFT;
        assert!(value & (Self::RDM_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::RDM_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `RDM` field set to the given value.
    pub const fn with_rdm(mut self, value: u8) -> Self {
        self.set_rdm(value);
        self
    }

    /// Returns the value of the `VCMA` field.
    pub const fn vcma(self) -> u8 {
        ((self.bits() >> Self::VCMA_SHIFT) & Self::VCMA_MASK) as u8
    }

    /// Sets the value of the `VCMA` field.
    pub const fn set_vcma(&mut self, value: u8) {
        let offset = Self::VCMA_SHIFT;
        assert!(value & (Self::VCMA_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VCMA_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `VCMA` field set to the given value.
    pub const fn with_vcma(mut self, value: u8) -> Self {
        self.set_vcma(value);
        self
    }
}

bitflags! {
    /// `ID_ISAR6` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdIsar6: u32 {
    }
}

impl IdIsar6 {
    /// Offset of the `JSCVT` field.
    pub const JSCVT_SHIFT: u32 = 0;
    /// Mask for the `JSCVT` field.
    pub const JSCVT_MASK: u32 = 0b1111;
    /// Offset of the `DP` field.
    pub const DP_SHIFT: u32 = 4;
    /// Mask for the `DP` field.
    pub const DP_MASK: u32 = 0b1111;
    /// Offset of the `FHM` field.
    pub const FHM_SHIFT: u32 = 8;
    /// Mask for the `FHM` field.
    pub const FHM_MASK: u32 = 0b1111;
    /// Offset of the `SB` field.
    pub const SB_SHIFT: u32 = 12;
    /// Mask for the `SB` field.
    pub const SB_MASK: u32 = 0b1111;
    /// Offset of the `SPECRES` field.
    pub const SPECRES_SHIFT: u32 = 16;
    /// Mask for the `SPECRES` field.
    pub const SPECRES_MASK: u32 = 0b1111;
    /// Offset of the `BF16` field.
    pub const BF16_SHIFT: u32 = 20;
    /// Mask for the `BF16` field.
    pub const BF16_MASK: u32 = 0b1111;
    /// Offset of the `I8MM` field.
    pub const I8MM_SHIFT: u32 = 24;
    /// Mask for the `I8MM` field.
    pub const I8MM_MASK: u32 = 0b1111;
    /// Offset of the `CLRBHB` field.
    pub const CLRBHB_SHIFT: u32 = 28;
    /// Mask for the `CLRBHB` field.
    pub const CLRBHB_MASK: u32 = 0b1111;

    /// Returns the value of the `JSCVT` field.
    pub const fn jscvt(self) -> u8 {
        ((self.bits() >> Self::JSCVT_SHIFT) & Self::JSCVT_MASK) as u8
    }

    /// Sets the value of the `JSCVT` field.
    pub const fn set_jscvt(&mut self, value: u8) {
        let offset = Self::JSCVT_SHIFT;
        assert!(value & (Self::JSCVT_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::JSCVT_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `JSCVT` field set to the given value.
    pub const fn with_jscvt(mut self, value: u8) -> Self {
        self.set_jscvt(value);
        self
    }

    /// Returns the value of the `DP` field.
    pub const fn dp(self) -> u8 {
        ((self.bits() >> Self::DP_SHIFT) & Self::DP_MASK) as u8
    }

    /// Sets the value of the `DP` field.
    pub const fn set_dp(&mut self, value: u8) {
        let offset = Self::DP_SHIFT;
        assert!(value & (Self::DP_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::DP_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `DP` field set to the given value.
    pub const fn with_dp(mut self, value: u8) -> Self {
        self.set_dp(value);
        self
    }

    /// Returns the value of the `FHM` field.
    pub const fn fhm(self) -> u8 {
        ((self.bits() >> Self::FHM_SHIFT) & Self::FHM_MASK) as u8
    }

    /// Sets the value of the `FHM` field.
    pub const fn set_fhm(&mut self, value: u8) {
        let offset = Self::FHM_SHIFT;
        assert!(value & (Self::FHM_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::FHM_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `FHM` field set to the given value.
    pub const fn with_fhm(mut self, value: u8) -> Self {
        self.set_fhm(value);
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
            (self.bits() & !(Self::SB_MASK << offset)) | ((value as u32) << offset),
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
            (self.bits() & !(Self::SPECRES_MASK << offset)) | ((value as u32) << offset),
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
            (self.bits() & !(Self::BF16_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `BF16` field set to the given value.
    pub const fn with_bf16(mut self, value: u8) -> Self {
        self.set_bf16(value);
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
            (self.bits() & !(Self::I8MM_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `I8MM` field set to the given value.
    pub const fn with_i8mm(mut self, value: u8) -> Self {
        self.set_i8mm(value);
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
            (self.bits() & !(Self::CLRBHB_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `CLRBHB` field set to the given value.
    pub const fn with_clrbhb(mut self, value: u8) -> Self {
        self.set_clrbhb(value);
        self
    }
}

bitflags! {
    /// `ID_MMFR0` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdMmfr0: u32 {
    }
}

impl IdMmfr0 {
    /// Offset of the `VMSA` field.
    pub const VMSA_SHIFT: u32 = 0;
    /// Mask for the `VMSA` field.
    pub const VMSA_MASK: u32 = 0b1111;
    /// Offset of the `PMSA` field.
    pub const PMSA_SHIFT: u32 = 4;
    /// Mask for the `PMSA` field.
    pub const PMSA_MASK: u32 = 0b1111;
    /// Offset of the `OuterShr` field.
    pub const OUTERSHR_SHIFT: u32 = 8;
    /// Mask for the `OuterShr` field.
    pub const OUTERSHR_MASK: u32 = 0b1111;
    /// Offset of the `ShareLvl` field.
    pub const SHARELVL_SHIFT: u32 = 12;
    /// Mask for the `ShareLvl` field.
    pub const SHARELVL_MASK: u32 = 0b1111;
    /// Offset of the `TCM` field.
    pub const TCM_SHIFT: u32 = 16;
    /// Mask for the `TCM` field.
    pub const TCM_MASK: u32 = 0b1111;
    /// Offset of the `AuxReg` field.
    pub const AUXREG_SHIFT: u32 = 20;
    /// Mask for the `AuxReg` field.
    pub const AUXREG_MASK: u32 = 0b1111;
    /// Offset of the `FCSE` field.
    pub const FCSE_SHIFT: u32 = 24;
    /// Mask for the `FCSE` field.
    pub const FCSE_MASK: u32 = 0b1111;
    /// Offset of the `InnerShr` field.
    pub const INNERSHR_SHIFT: u32 = 28;
    /// Mask for the `InnerShr` field.
    pub const INNERSHR_MASK: u32 = 0b1111;

    /// Returns the value of the `VMSA` field.
    pub const fn vmsa(self) -> u8 {
        ((self.bits() >> Self::VMSA_SHIFT) & Self::VMSA_MASK) as u8
    }

    /// Sets the value of the `VMSA` field.
    pub const fn set_vmsa(&mut self, value: u8) {
        let offset = Self::VMSA_SHIFT;
        assert!(value & (Self::VMSA_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VMSA_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `VMSA` field set to the given value.
    pub const fn with_vmsa(mut self, value: u8) -> Self {
        self.set_vmsa(value);
        self
    }

    /// Returns the value of the `PMSA` field.
    pub const fn pmsa(self) -> u8 {
        ((self.bits() >> Self::PMSA_SHIFT) & Self::PMSA_MASK) as u8
    }

    /// Sets the value of the `PMSA` field.
    pub const fn set_pmsa(&mut self, value: u8) {
        let offset = Self::PMSA_SHIFT;
        assert!(value & (Self::PMSA_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PMSA_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `PMSA` field set to the given value.
    pub const fn with_pmsa(mut self, value: u8) -> Self {
        self.set_pmsa(value);
        self
    }

    /// Returns the value of the `OuterShr` field.
    pub const fn outershr(self) -> u8 {
        ((self.bits() >> Self::OUTERSHR_SHIFT) & Self::OUTERSHR_MASK) as u8
    }

    /// Sets the value of the `OuterShr` field.
    pub const fn set_outershr(&mut self, value: u8) {
        let offset = Self::OUTERSHR_SHIFT;
        assert!(value & (Self::OUTERSHR_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::OUTERSHR_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `OuterShr` field set to the given value.
    pub const fn with_outershr(mut self, value: u8) -> Self {
        self.set_outershr(value);
        self
    }

    /// Returns the value of the `ShareLvl` field.
    pub const fn sharelvl(self) -> u8 {
        ((self.bits() >> Self::SHARELVL_SHIFT) & Self::SHARELVL_MASK) as u8
    }

    /// Sets the value of the `ShareLvl` field.
    pub const fn set_sharelvl(&mut self, value: u8) {
        let offset = Self::SHARELVL_SHIFT;
        assert!(value & (Self::SHARELVL_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SHARELVL_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `ShareLvl` field set to the given value.
    pub const fn with_sharelvl(mut self, value: u8) -> Self {
        self.set_sharelvl(value);
        self
    }

    /// Returns the value of the `TCM` field.
    pub const fn tcm(self) -> u8 {
        ((self.bits() >> Self::TCM_SHIFT) & Self::TCM_MASK) as u8
    }

    /// Sets the value of the `TCM` field.
    pub const fn set_tcm(&mut self, value: u8) {
        let offset = Self::TCM_SHIFT;
        assert!(value & (Self::TCM_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TCM_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `TCM` field set to the given value.
    pub const fn with_tcm(mut self, value: u8) -> Self {
        self.set_tcm(value);
        self
    }

    /// Returns the value of the `AuxReg` field.
    pub const fn auxreg(self) -> u8 {
        ((self.bits() >> Self::AUXREG_SHIFT) & Self::AUXREG_MASK) as u8
    }

    /// Sets the value of the `AuxReg` field.
    pub const fn set_auxreg(&mut self, value: u8) {
        let offset = Self::AUXREG_SHIFT;
        assert!(value & (Self::AUXREG_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::AUXREG_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `AuxReg` field set to the given value.
    pub const fn with_auxreg(mut self, value: u8) -> Self {
        self.set_auxreg(value);
        self
    }

    /// Returns the value of the `FCSE` field.
    pub const fn fcse(self) -> u8 {
        ((self.bits() >> Self::FCSE_SHIFT) & Self::FCSE_MASK) as u8
    }

    /// Sets the value of the `FCSE` field.
    pub const fn set_fcse(&mut self, value: u8) {
        let offset = Self::FCSE_SHIFT;
        assert!(value & (Self::FCSE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::FCSE_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `FCSE` field set to the given value.
    pub const fn with_fcse(mut self, value: u8) -> Self {
        self.set_fcse(value);
        self
    }

    /// Returns the value of the `InnerShr` field.
    pub const fn innershr(self) -> u8 {
        ((self.bits() >> Self::INNERSHR_SHIFT) & Self::INNERSHR_MASK) as u8
    }

    /// Sets the value of the `InnerShr` field.
    pub const fn set_innershr(&mut self, value: u8) {
        let offset = Self::INNERSHR_SHIFT;
        assert!(value & (Self::INNERSHR_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::INNERSHR_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `InnerShr` field set to the given value.
    pub const fn with_innershr(mut self, value: u8) -> Self {
        self.set_innershr(value);
        self
    }
}

bitflags! {
    /// `ID_MMFR1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdMmfr1: u32 {
    }
}

impl IdMmfr1 {
    /// Offset of the `L1HvdVA` field.
    pub const L1HVDVA_SHIFT: u32 = 0;
    /// Mask for the `L1HvdVA` field.
    pub const L1HVDVA_MASK: u32 = 0b1111;
    /// Offset of the `L1UniVA` field.
    pub const L1UNIVA_SHIFT: u32 = 4;
    /// Mask for the `L1UniVA` field.
    pub const L1UNIVA_MASK: u32 = 0b1111;
    /// Offset of the `L1HvdSW` field.
    pub const L1HVDSW_SHIFT: u32 = 8;
    /// Mask for the `L1HvdSW` field.
    pub const L1HVDSW_MASK: u32 = 0b1111;
    /// Offset of the `L1UniSW` field.
    pub const L1UNISW_SHIFT: u32 = 12;
    /// Mask for the `L1UniSW` field.
    pub const L1UNISW_MASK: u32 = 0b1111;
    /// Offset of the `L1Hvd` field.
    pub const L1HVD_SHIFT: u32 = 16;
    /// Mask for the `L1Hvd` field.
    pub const L1HVD_MASK: u32 = 0b1111;
    /// Offset of the `L1Uni` field.
    pub const L1UNI_SHIFT: u32 = 20;
    /// Mask for the `L1Uni` field.
    pub const L1UNI_MASK: u32 = 0b1111;
    /// Offset of the `L1TstCln` field.
    pub const L1TSTCLN_SHIFT: u32 = 24;
    /// Mask for the `L1TstCln` field.
    pub const L1TSTCLN_MASK: u32 = 0b1111;
    /// Offset of the `BPred` field.
    pub const BPRED_SHIFT: u32 = 28;
    /// Mask for the `BPred` field.
    pub const BPRED_MASK: u32 = 0b1111;

    /// Returns the value of the `L1HvdVA` field.
    pub const fn l1hvdva(self) -> u8 {
        ((self.bits() >> Self::L1HVDVA_SHIFT) & Self::L1HVDVA_MASK) as u8
    }

    /// Sets the value of the `L1HvdVA` field.
    pub const fn set_l1hvdva(&mut self, value: u8) {
        let offset = Self::L1HVDVA_SHIFT;
        assert!(value & (Self::L1HVDVA_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::L1HVDVA_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `L1HvdVA` field set to the given value.
    pub const fn with_l1hvdva(mut self, value: u8) -> Self {
        self.set_l1hvdva(value);
        self
    }

    /// Returns the value of the `L1UniVA` field.
    pub const fn l1univa(self) -> u8 {
        ((self.bits() >> Self::L1UNIVA_SHIFT) & Self::L1UNIVA_MASK) as u8
    }

    /// Sets the value of the `L1UniVA` field.
    pub const fn set_l1univa(&mut self, value: u8) {
        let offset = Self::L1UNIVA_SHIFT;
        assert!(value & (Self::L1UNIVA_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::L1UNIVA_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `L1UniVA` field set to the given value.
    pub const fn with_l1univa(mut self, value: u8) -> Self {
        self.set_l1univa(value);
        self
    }

    /// Returns the value of the `L1HvdSW` field.
    pub const fn l1hvdsw(self) -> u8 {
        ((self.bits() >> Self::L1HVDSW_SHIFT) & Self::L1HVDSW_MASK) as u8
    }

    /// Sets the value of the `L1HvdSW` field.
    pub const fn set_l1hvdsw(&mut self, value: u8) {
        let offset = Self::L1HVDSW_SHIFT;
        assert!(value & (Self::L1HVDSW_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::L1HVDSW_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `L1HvdSW` field set to the given value.
    pub const fn with_l1hvdsw(mut self, value: u8) -> Self {
        self.set_l1hvdsw(value);
        self
    }

    /// Returns the value of the `L1UniSW` field.
    pub const fn l1unisw(self) -> u8 {
        ((self.bits() >> Self::L1UNISW_SHIFT) & Self::L1UNISW_MASK) as u8
    }

    /// Sets the value of the `L1UniSW` field.
    pub const fn set_l1unisw(&mut self, value: u8) {
        let offset = Self::L1UNISW_SHIFT;
        assert!(value & (Self::L1UNISW_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::L1UNISW_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `L1UniSW` field set to the given value.
    pub const fn with_l1unisw(mut self, value: u8) -> Self {
        self.set_l1unisw(value);
        self
    }

    /// Returns the value of the `L1Hvd` field.
    pub const fn l1hvd(self) -> u8 {
        ((self.bits() >> Self::L1HVD_SHIFT) & Self::L1HVD_MASK) as u8
    }

    /// Sets the value of the `L1Hvd` field.
    pub const fn set_l1hvd(&mut self, value: u8) {
        let offset = Self::L1HVD_SHIFT;
        assert!(value & (Self::L1HVD_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::L1HVD_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `L1Hvd` field set to the given value.
    pub const fn with_l1hvd(mut self, value: u8) -> Self {
        self.set_l1hvd(value);
        self
    }

    /// Returns the value of the `L1Uni` field.
    pub const fn l1uni(self) -> u8 {
        ((self.bits() >> Self::L1UNI_SHIFT) & Self::L1UNI_MASK) as u8
    }

    /// Sets the value of the `L1Uni` field.
    pub const fn set_l1uni(&mut self, value: u8) {
        let offset = Self::L1UNI_SHIFT;
        assert!(value & (Self::L1UNI_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::L1UNI_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `L1Uni` field set to the given value.
    pub const fn with_l1uni(mut self, value: u8) -> Self {
        self.set_l1uni(value);
        self
    }

    /// Returns the value of the `L1TstCln` field.
    pub const fn l1tstcln(self) -> u8 {
        ((self.bits() >> Self::L1TSTCLN_SHIFT) & Self::L1TSTCLN_MASK) as u8
    }

    /// Sets the value of the `L1TstCln` field.
    pub const fn set_l1tstcln(&mut self, value: u8) {
        let offset = Self::L1TSTCLN_SHIFT;
        assert!(value & (Self::L1TSTCLN_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::L1TSTCLN_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `L1TstCln` field set to the given value.
    pub const fn with_l1tstcln(mut self, value: u8) -> Self {
        self.set_l1tstcln(value);
        self
    }

    /// Returns the value of the `BPred` field.
    pub const fn bpred(self) -> u8 {
        ((self.bits() >> Self::BPRED_SHIFT) & Self::BPRED_MASK) as u8
    }

    /// Sets the value of the `BPred` field.
    pub const fn set_bpred(&mut self, value: u8) {
        let offset = Self::BPRED_SHIFT;
        assert!(value & (Self::BPRED_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BPRED_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `BPred` field set to the given value.
    pub const fn with_bpred(mut self, value: u8) -> Self {
        self.set_bpred(value);
        self
    }
}

bitflags! {
    /// `ID_MMFR2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdMmfr2: u32 {
    }
}

impl IdMmfr2 {
    /// Offset of the `L1HvdFG` field.
    pub const L1HVDFG_SHIFT: u32 = 0;
    /// Mask for the `L1HvdFG` field.
    pub const L1HVDFG_MASK: u32 = 0b1111;
    /// Offset of the `L1HvdBG` field.
    pub const L1HVDBG_SHIFT: u32 = 4;
    /// Mask for the `L1HvdBG` field.
    pub const L1HVDBG_MASK: u32 = 0b1111;
    /// Offset of the `L1HvdRng` field.
    pub const L1HVDRNG_SHIFT: u32 = 8;
    /// Mask for the `L1HvdRng` field.
    pub const L1HVDRNG_MASK: u32 = 0b1111;
    /// Offset of the `HvdTLB` field.
    pub const HVDTLB_SHIFT: u32 = 12;
    /// Mask for the `HvdTLB` field.
    pub const HVDTLB_MASK: u32 = 0b1111;
    /// Offset of the `UniTLB` field.
    pub const UNITLB_SHIFT: u32 = 16;
    /// Mask for the `UniTLB` field.
    pub const UNITLB_MASK: u32 = 0b1111;
    /// Offset of the `MemBarr` field.
    pub const MEMBARR_SHIFT: u32 = 20;
    /// Mask for the `MemBarr` field.
    pub const MEMBARR_MASK: u32 = 0b1111;
    /// Offset of the `WFIStall` field.
    pub const WFISTALL_SHIFT: u32 = 24;
    /// Mask for the `WFIStall` field.
    pub const WFISTALL_MASK: u32 = 0b1111;
    /// Offset of the `HWAccFlg` field.
    pub const HWACCFLG_SHIFT: u32 = 28;
    /// Mask for the `HWAccFlg` field.
    pub const HWACCFLG_MASK: u32 = 0b1111;

    /// Returns the value of the `L1HvdFG` field.
    pub const fn l1hvdfg(self) -> u8 {
        ((self.bits() >> Self::L1HVDFG_SHIFT) & Self::L1HVDFG_MASK) as u8
    }

    /// Sets the value of the `L1HvdFG` field.
    pub const fn set_l1hvdfg(&mut self, value: u8) {
        let offset = Self::L1HVDFG_SHIFT;
        assert!(value & (Self::L1HVDFG_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::L1HVDFG_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `L1HvdFG` field set to the given value.
    pub const fn with_l1hvdfg(mut self, value: u8) -> Self {
        self.set_l1hvdfg(value);
        self
    }

    /// Returns the value of the `L1HvdBG` field.
    pub const fn l1hvdbg(self) -> u8 {
        ((self.bits() >> Self::L1HVDBG_SHIFT) & Self::L1HVDBG_MASK) as u8
    }

    /// Sets the value of the `L1HvdBG` field.
    pub const fn set_l1hvdbg(&mut self, value: u8) {
        let offset = Self::L1HVDBG_SHIFT;
        assert!(value & (Self::L1HVDBG_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::L1HVDBG_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `L1HvdBG` field set to the given value.
    pub const fn with_l1hvdbg(mut self, value: u8) -> Self {
        self.set_l1hvdbg(value);
        self
    }

    /// Returns the value of the `L1HvdRng` field.
    pub const fn l1hvdrng(self) -> u8 {
        ((self.bits() >> Self::L1HVDRNG_SHIFT) & Self::L1HVDRNG_MASK) as u8
    }

    /// Sets the value of the `L1HvdRng` field.
    pub const fn set_l1hvdrng(&mut self, value: u8) {
        let offset = Self::L1HVDRNG_SHIFT;
        assert!(value & (Self::L1HVDRNG_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::L1HVDRNG_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `L1HvdRng` field set to the given value.
    pub const fn with_l1hvdrng(mut self, value: u8) -> Self {
        self.set_l1hvdrng(value);
        self
    }

    /// Returns the value of the `HvdTLB` field.
    pub const fn hvdtlb(self) -> u8 {
        ((self.bits() >> Self::HVDTLB_SHIFT) & Self::HVDTLB_MASK) as u8
    }

    /// Sets the value of the `HvdTLB` field.
    pub const fn set_hvdtlb(&mut self, value: u8) {
        let offset = Self::HVDTLB_SHIFT;
        assert!(value & (Self::HVDTLB_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::HVDTLB_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `HvdTLB` field set to the given value.
    pub const fn with_hvdtlb(mut self, value: u8) -> Self {
        self.set_hvdtlb(value);
        self
    }

    /// Returns the value of the `UniTLB` field.
    pub const fn unitlb(self) -> u8 {
        ((self.bits() >> Self::UNITLB_SHIFT) & Self::UNITLB_MASK) as u8
    }

    /// Sets the value of the `UniTLB` field.
    pub const fn set_unitlb(&mut self, value: u8) {
        let offset = Self::UNITLB_SHIFT;
        assert!(value & (Self::UNITLB_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::UNITLB_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `UniTLB` field set to the given value.
    pub const fn with_unitlb(mut self, value: u8) -> Self {
        self.set_unitlb(value);
        self
    }

    /// Returns the value of the `MemBarr` field.
    pub const fn membarr(self) -> u8 {
        ((self.bits() >> Self::MEMBARR_SHIFT) & Self::MEMBARR_MASK) as u8
    }

    /// Sets the value of the `MemBarr` field.
    pub const fn set_membarr(&mut self, value: u8) {
        let offset = Self::MEMBARR_SHIFT;
        assert!(value & (Self::MEMBARR_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::MEMBARR_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `MemBarr` field set to the given value.
    pub const fn with_membarr(mut self, value: u8) -> Self {
        self.set_membarr(value);
        self
    }

    /// Returns the value of the `WFIStall` field.
    pub const fn wfistall(self) -> u8 {
        ((self.bits() >> Self::WFISTALL_SHIFT) & Self::WFISTALL_MASK) as u8
    }

    /// Sets the value of the `WFIStall` field.
    pub const fn set_wfistall(&mut self, value: u8) {
        let offset = Self::WFISTALL_SHIFT;
        assert!(value & (Self::WFISTALL_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::WFISTALL_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `WFIStall` field set to the given value.
    pub const fn with_wfistall(mut self, value: u8) -> Self {
        self.set_wfistall(value);
        self
    }

    /// Returns the value of the `HWAccFlg` field.
    pub const fn hwaccflg(self) -> u8 {
        ((self.bits() >> Self::HWACCFLG_SHIFT) & Self::HWACCFLG_MASK) as u8
    }

    /// Sets the value of the `HWAccFlg` field.
    pub const fn set_hwaccflg(&mut self, value: u8) {
        let offset = Self::HWACCFLG_SHIFT;
        assert!(value & (Self::HWACCFLG_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::HWACCFLG_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `HWAccFlg` field set to the given value.
    pub const fn with_hwaccflg(mut self, value: u8) -> Self {
        self.set_hwaccflg(value);
        self
    }
}

bitflags! {
    /// `ID_MMFR3` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdMmfr3: u32 {
    }
}

impl IdMmfr3 {
    /// Offset of the `CMaintVA` field.
    pub const CMAINTVA_SHIFT: u32 = 0;
    /// Mask for the `CMaintVA` field.
    pub const CMAINTVA_MASK: u32 = 0b1111;
    /// Offset of the `CMaintSW` field.
    pub const CMAINTSW_SHIFT: u32 = 4;
    /// Mask for the `CMaintSW` field.
    pub const CMAINTSW_MASK: u32 = 0b1111;
    /// Offset of the `BPMaint` field.
    pub const BPMAINT_SHIFT: u32 = 8;
    /// Mask for the `BPMaint` field.
    pub const BPMAINT_MASK: u32 = 0b1111;
    /// Offset of the `MaintBcst` field.
    pub const MAINTBCST_SHIFT: u32 = 12;
    /// Mask for the `MaintBcst` field.
    pub const MAINTBCST_MASK: u32 = 0b1111;
    /// Offset of the `PAN` field.
    pub const PAN_SHIFT: u32 = 16;
    /// Mask for the `PAN` field.
    pub const PAN_MASK: u32 = 0b1111;
    /// Offset of the `CohWalk` field.
    pub const COHWALK_SHIFT: u32 = 20;
    /// Mask for the `CohWalk` field.
    pub const COHWALK_MASK: u32 = 0b1111;
    /// Offset of the `CMemSz` field.
    pub const CMEMSZ_SHIFT: u32 = 24;
    /// Mask for the `CMemSz` field.
    pub const CMEMSZ_MASK: u32 = 0b1111;
    /// Offset of the `Supersec` field.
    pub const SUPERSEC_SHIFT: u32 = 28;
    /// Mask for the `Supersec` field.
    pub const SUPERSEC_MASK: u32 = 0b1111;

    /// Returns the value of the `CMaintVA` field.
    pub const fn cmaintva(self) -> u8 {
        ((self.bits() >> Self::CMAINTVA_SHIFT) & Self::CMAINTVA_MASK) as u8
    }

    /// Sets the value of the `CMaintVA` field.
    pub const fn set_cmaintva(&mut self, value: u8) {
        let offset = Self::CMAINTVA_SHIFT;
        assert!(value & (Self::CMAINTVA_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CMAINTVA_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `CMaintVA` field set to the given value.
    pub const fn with_cmaintva(mut self, value: u8) -> Self {
        self.set_cmaintva(value);
        self
    }

    /// Returns the value of the `CMaintSW` field.
    pub const fn cmaintsw(self) -> u8 {
        ((self.bits() >> Self::CMAINTSW_SHIFT) & Self::CMAINTSW_MASK) as u8
    }

    /// Sets the value of the `CMaintSW` field.
    pub const fn set_cmaintsw(&mut self, value: u8) {
        let offset = Self::CMAINTSW_SHIFT;
        assert!(value & (Self::CMAINTSW_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CMAINTSW_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `CMaintSW` field set to the given value.
    pub const fn with_cmaintsw(mut self, value: u8) -> Self {
        self.set_cmaintsw(value);
        self
    }

    /// Returns the value of the `BPMaint` field.
    pub const fn bpmaint(self) -> u8 {
        ((self.bits() >> Self::BPMAINT_SHIFT) & Self::BPMAINT_MASK) as u8
    }

    /// Sets the value of the `BPMaint` field.
    pub const fn set_bpmaint(&mut self, value: u8) {
        let offset = Self::BPMAINT_SHIFT;
        assert!(value & (Self::BPMAINT_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BPMAINT_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `BPMaint` field set to the given value.
    pub const fn with_bpmaint(mut self, value: u8) -> Self {
        self.set_bpmaint(value);
        self
    }

    /// Returns the value of the `MaintBcst` field.
    pub const fn maintbcst(self) -> u8 {
        ((self.bits() >> Self::MAINTBCST_SHIFT) & Self::MAINTBCST_MASK) as u8
    }

    /// Sets the value of the `MaintBcst` field.
    pub const fn set_maintbcst(&mut self, value: u8) {
        let offset = Self::MAINTBCST_SHIFT;
        assert!(value & (Self::MAINTBCST_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::MAINTBCST_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `MaintBcst` field set to the given value.
    pub const fn with_maintbcst(mut self, value: u8) -> Self {
        self.set_maintbcst(value);
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
            (self.bits() & !(Self::PAN_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `PAN` field set to the given value.
    pub const fn with_pan(mut self, value: u8) -> Self {
        self.set_pan(value);
        self
    }

    /// Returns the value of the `CohWalk` field.
    pub const fn cohwalk(self) -> u8 {
        ((self.bits() >> Self::COHWALK_SHIFT) & Self::COHWALK_MASK) as u8
    }

    /// Sets the value of the `CohWalk` field.
    pub const fn set_cohwalk(&mut self, value: u8) {
        let offset = Self::COHWALK_SHIFT;
        assert!(value & (Self::COHWALK_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::COHWALK_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `CohWalk` field set to the given value.
    pub const fn with_cohwalk(mut self, value: u8) -> Self {
        self.set_cohwalk(value);
        self
    }

    /// Returns the value of the `CMemSz` field.
    pub const fn cmemsz(self) -> u8 {
        ((self.bits() >> Self::CMEMSZ_SHIFT) & Self::CMEMSZ_MASK) as u8
    }

    /// Sets the value of the `CMemSz` field.
    pub const fn set_cmemsz(&mut self, value: u8) {
        let offset = Self::CMEMSZ_SHIFT;
        assert!(value & (Self::CMEMSZ_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CMEMSZ_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `CMemSz` field set to the given value.
    pub const fn with_cmemsz(mut self, value: u8) -> Self {
        self.set_cmemsz(value);
        self
    }

    /// Returns the value of the `Supersec` field.
    pub const fn supersec(self) -> u8 {
        ((self.bits() >> Self::SUPERSEC_SHIFT) & Self::SUPERSEC_MASK) as u8
    }

    /// Sets the value of the `Supersec` field.
    pub const fn set_supersec(&mut self, value: u8) {
        let offset = Self::SUPERSEC_SHIFT;
        assert!(value & (Self::SUPERSEC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SUPERSEC_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `Supersec` field set to the given value.
    pub const fn with_supersec(mut self, value: u8) -> Self {
        self.set_supersec(value);
        self
    }
}

bitflags! {
    /// `ID_MMFR4` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdMmfr4: u32 {
    }
}

impl IdMmfr4 {
    /// Offset of the `SpecSEI` field.
    pub const SPECSEI_SHIFT: u32 = 0;
    /// Mask for the `SpecSEI` field.
    pub const SPECSEI_MASK: u32 = 0b1111;
    /// Offset of the `AC2` field.
    pub const AC2_SHIFT: u32 = 4;
    /// Mask for the `AC2` field.
    pub const AC2_MASK: u32 = 0b1111;
    /// Offset of the `XNX` field.
    pub const XNX_SHIFT: u32 = 8;
    /// Mask for the `XNX` field.
    pub const XNX_MASK: u32 = 0b1111;
    /// Offset of the `CnP` field.
    pub const CNP_SHIFT: u32 = 12;
    /// Mask for the `CnP` field.
    pub const CNP_MASK: u32 = 0b1111;
    /// Offset of the `HPDS` field.
    pub const HPDS_SHIFT: u32 = 16;
    /// Mask for the `HPDS` field.
    pub const HPDS_MASK: u32 = 0b1111;
    /// Offset of the `LSM` field.
    pub const LSM_SHIFT: u32 = 20;
    /// Mask for the `LSM` field.
    pub const LSM_MASK: u32 = 0b1111;
    /// Offset of the `CCIDX` field.
    pub const CCIDX_SHIFT: u32 = 24;
    /// Mask for the `CCIDX` field.
    pub const CCIDX_MASK: u32 = 0b1111;
    /// Offset of the `EVT` field.
    pub const EVT_SHIFT: u32 = 28;
    /// Mask for the `EVT` field.
    pub const EVT_MASK: u32 = 0b1111;

    /// Returns the value of the `SpecSEI` field.
    pub const fn specsei(self) -> u8 {
        ((self.bits() >> Self::SPECSEI_SHIFT) & Self::SPECSEI_MASK) as u8
    }

    /// Sets the value of the `SpecSEI` field.
    pub const fn set_specsei(&mut self, value: u8) {
        let offset = Self::SPECSEI_SHIFT;
        assert!(value & (Self::SPECSEI_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SPECSEI_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `SpecSEI` field set to the given value.
    pub const fn with_specsei(mut self, value: u8) -> Self {
        self.set_specsei(value);
        self
    }

    /// Returns the value of the `AC2` field.
    pub const fn ac2(self) -> u8 {
        ((self.bits() >> Self::AC2_SHIFT) & Self::AC2_MASK) as u8
    }

    /// Sets the value of the `AC2` field.
    pub const fn set_ac2(&mut self, value: u8) {
        let offset = Self::AC2_SHIFT;
        assert!(value & (Self::AC2_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::AC2_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `AC2` field set to the given value.
    pub const fn with_ac2(mut self, value: u8) -> Self {
        self.set_ac2(value);
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
            (self.bits() & !(Self::XNX_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `XNX` field set to the given value.
    pub const fn with_xnx(mut self, value: u8) -> Self {
        self.set_xnx(value);
        self
    }

    /// Returns the value of the `CnP` field.
    pub const fn cnp(self) -> u8 {
        ((self.bits() >> Self::CNP_SHIFT) & Self::CNP_MASK) as u8
    }

    /// Sets the value of the `CnP` field.
    pub const fn set_cnp(&mut self, value: u8) {
        let offset = Self::CNP_SHIFT;
        assert!(value & (Self::CNP_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CNP_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `CnP` field set to the given value.
    pub const fn with_cnp(mut self, value: u8) -> Self {
        self.set_cnp(value);
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
            (self.bits() & !(Self::HPDS_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `HPDS` field set to the given value.
    pub const fn with_hpds(mut self, value: u8) -> Self {
        self.set_hpds(value);
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
            (self.bits() & !(Self::LSM_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `LSM` field set to the given value.
    pub const fn with_lsm(mut self, value: u8) -> Self {
        self.set_lsm(value);
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
            (self.bits() & !(Self::CCIDX_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `CCIDX` field set to the given value.
    pub const fn with_ccidx(mut self, value: u8) -> Self {
        self.set_ccidx(value);
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
            (self.bits() & !(Self::EVT_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `EVT` field set to the given value.
    pub const fn with_evt(mut self, value: u8) -> Self {
        self.set_evt(value);
        self
    }
}

bitflags! {
    /// `ID_MMFR5` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdMmfr5: u32 {
    }
}

impl IdMmfr5 {
    /// Offset of the `ETS` field.
    pub const ETS_SHIFT: u32 = 0;
    /// Mask for the `ETS` field.
    pub const ETS_MASK: u32 = 0b1111;
    /// Offset of the `nTLBPA` field.
    pub const NTLBPA_SHIFT: u32 = 4;
    /// Mask for the `nTLBPA` field.
    pub const NTLBPA_MASK: u32 = 0b1111;

    /// Returns the value of the `ETS` field.
    pub const fn ets(self) -> u8 {
        ((self.bits() >> Self::ETS_SHIFT) & Self::ETS_MASK) as u8
    }

    /// Sets the value of the `ETS` field.
    pub const fn set_ets(&mut self, value: u8) {
        let offset = Self::ETS_SHIFT;
        assert!(value & (Self::ETS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ETS_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `ETS` field set to the given value.
    pub const fn with_ets(mut self, value: u8) -> Self {
        self.set_ets(value);
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
            (self.bits() & !(Self::NTLBPA_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `nTLBPA` field set to the given value.
    pub const fn with_ntlbpa(mut self, value: u8) -> Self {
        self.set_ntlbpa(value);
        self
    }
}

bitflags! {
    /// `ID_PFR0` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdPfr0: u32 {
    }
}

impl IdPfr0 {
    /// Offset of the `State0` field.
    pub const STATE0_SHIFT: u32 = 0;
    /// Mask for the `State0` field.
    pub const STATE0_MASK: u32 = 0b1111;
    /// Offset of the `State1` field.
    pub const STATE1_SHIFT: u32 = 4;
    /// Mask for the `State1` field.
    pub const STATE1_MASK: u32 = 0b1111;
    /// Offset of the `State2` field.
    pub const STATE2_SHIFT: u32 = 8;
    /// Mask for the `State2` field.
    pub const STATE2_MASK: u32 = 0b1111;
    /// Offset of the `State3` field.
    pub const STATE3_SHIFT: u32 = 12;
    /// Mask for the `State3` field.
    pub const STATE3_MASK: u32 = 0b1111;
    /// Offset of the `CSV2` field.
    pub const CSV2_SHIFT: u32 = 16;
    /// Mask for the `CSV2` field.
    pub const CSV2_MASK: u32 = 0b1111;
    /// Offset of the `AMU` field.
    pub const AMU_SHIFT: u32 = 20;
    /// Mask for the `AMU` field.
    pub const AMU_MASK: u32 = 0b1111;
    /// Offset of the `DIT` field.
    pub const DIT_SHIFT: u32 = 24;
    /// Mask for the `DIT` field.
    pub const DIT_MASK: u32 = 0b1111;
    /// Offset of the `RAS` field.
    pub const RAS_SHIFT: u32 = 28;
    /// Mask for the `RAS` field.
    pub const RAS_MASK: u32 = 0b1111;

    /// Returns the value of the `State0` field.
    pub const fn state0(self) -> u8 {
        ((self.bits() >> Self::STATE0_SHIFT) & Self::STATE0_MASK) as u8
    }

    /// Sets the value of the `State0` field.
    pub const fn set_state0(&mut self, value: u8) {
        let offset = Self::STATE0_SHIFT;
        assert!(value & (Self::STATE0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::STATE0_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `State0` field set to the given value.
    pub const fn with_state0(mut self, value: u8) -> Self {
        self.set_state0(value);
        self
    }

    /// Returns the value of the `State1` field.
    pub const fn state1(self) -> u8 {
        ((self.bits() >> Self::STATE1_SHIFT) & Self::STATE1_MASK) as u8
    }

    /// Sets the value of the `State1` field.
    pub const fn set_state1(&mut self, value: u8) {
        let offset = Self::STATE1_SHIFT;
        assert!(value & (Self::STATE1_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::STATE1_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `State1` field set to the given value.
    pub const fn with_state1(mut self, value: u8) -> Self {
        self.set_state1(value);
        self
    }

    /// Returns the value of the `State2` field.
    pub const fn state2(self) -> u8 {
        ((self.bits() >> Self::STATE2_SHIFT) & Self::STATE2_MASK) as u8
    }

    /// Sets the value of the `State2` field.
    pub const fn set_state2(&mut self, value: u8) {
        let offset = Self::STATE2_SHIFT;
        assert!(value & (Self::STATE2_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::STATE2_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `State2` field set to the given value.
    pub const fn with_state2(mut self, value: u8) -> Self {
        self.set_state2(value);
        self
    }

    /// Returns the value of the `State3` field.
    pub const fn state3(self) -> u8 {
        ((self.bits() >> Self::STATE3_SHIFT) & Self::STATE3_MASK) as u8
    }

    /// Sets the value of the `State3` field.
    pub const fn set_state3(&mut self, value: u8) {
        let offset = Self::STATE3_SHIFT;
        assert!(value & (Self::STATE3_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::STATE3_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `State3` field set to the given value.
    pub const fn with_state3(mut self, value: u8) -> Self {
        self.set_state3(value);
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
            (self.bits() & !(Self::CSV2_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `CSV2` field set to the given value.
    pub const fn with_csv2(mut self, value: u8) -> Self {
        self.set_csv2(value);
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
            (self.bits() & !(Self::AMU_MASK << offset)) | ((value as u32) << offset),
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
            (self.bits() & !(Self::DIT_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `DIT` field set to the given value.
    pub const fn with_dit(mut self, value: u8) -> Self {
        self.set_dit(value);
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
            (self.bits() & !(Self::RAS_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `RAS` field set to the given value.
    pub const fn with_ras(mut self, value: u8) -> Self {
        self.set_ras(value);
        self
    }
}

bitflags! {
    /// `ID_PFR1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdPfr1: u32 {
    }
}

impl IdPfr1 {
    /// Offset of the `ProgMod` field.
    pub const PROGMOD_SHIFT: u32 = 0;
    /// Mask for the `ProgMod` field.
    pub const PROGMOD_MASK: u32 = 0b1111;
    /// Offset of the `Security` field.
    pub const SECURITY_SHIFT: u32 = 4;
    /// Mask for the `Security` field.
    pub const SECURITY_MASK: u32 = 0b1111;
    /// Offset of the `MProgMod` field.
    pub const MPROGMOD_SHIFT: u32 = 8;
    /// Mask for the `MProgMod` field.
    pub const MPROGMOD_MASK: u32 = 0b1111;
    /// Offset of the `Virtualization` field.
    pub const VIRTUALIZATION_SHIFT: u32 = 12;
    /// Mask for the `Virtualization` field.
    pub const VIRTUALIZATION_MASK: u32 = 0b1111;
    /// Offset of the `GenTimer` field.
    pub const GENTIMER_SHIFT: u32 = 16;
    /// Mask for the `GenTimer` field.
    pub const GENTIMER_MASK: u32 = 0b1111;
    /// Offset of the `Sec_frac` field.
    pub const SEC_FRAC_SHIFT: u32 = 20;
    /// Mask for the `Sec_frac` field.
    pub const SEC_FRAC_MASK: u32 = 0b1111;
    /// Offset of the `Virt_frac` field.
    pub const VIRT_FRAC_SHIFT: u32 = 24;
    /// Mask for the `Virt_frac` field.
    pub const VIRT_FRAC_MASK: u32 = 0b1111;
    /// Offset of the `GIC` field.
    pub const GIC_SHIFT: u32 = 28;
    /// Mask for the `GIC` field.
    pub const GIC_MASK: u32 = 0b1111;

    /// Returns the value of the `ProgMod` field.
    pub const fn progmod(self) -> u8 {
        ((self.bits() >> Self::PROGMOD_SHIFT) & Self::PROGMOD_MASK) as u8
    }

    /// Sets the value of the `ProgMod` field.
    pub const fn set_progmod(&mut self, value: u8) {
        let offset = Self::PROGMOD_SHIFT;
        assert!(value & (Self::PROGMOD_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PROGMOD_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `ProgMod` field set to the given value.
    pub const fn with_progmod(mut self, value: u8) -> Self {
        self.set_progmod(value);
        self
    }

    /// Returns the value of the `Security` field.
    pub const fn security(self) -> u8 {
        ((self.bits() >> Self::SECURITY_SHIFT) & Self::SECURITY_MASK) as u8
    }

    /// Sets the value of the `Security` field.
    pub const fn set_security(&mut self, value: u8) {
        let offset = Self::SECURITY_SHIFT;
        assert!(value & (Self::SECURITY_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SECURITY_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `Security` field set to the given value.
    pub const fn with_security(mut self, value: u8) -> Self {
        self.set_security(value);
        self
    }

    /// Returns the value of the `MProgMod` field.
    pub const fn mprogmod(self) -> u8 {
        ((self.bits() >> Self::MPROGMOD_SHIFT) & Self::MPROGMOD_MASK) as u8
    }

    /// Sets the value of the `MProgMod` field.
    pub const fn set_mprogmod(&mut self, value: u8) {
        let offset = Self::MPROGMOD_SHIFT;
        assert!(value & (Self::MPROGMOD_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::MPROGMOD_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `MProgMod` field set to the given value.
    pub const fn with_mprogmod(mut self, value: u8) -> Self {
        self.set_mprogmod(value);
        self
    }

    /// Returns the value of the `Virtualization` field.
    pub const fn virtualization(self) -> u8 {
        ((self.bits() >> Self::VIRTUALIZATION_SHIFT) & Self::VIRTUALIZATION_MASK) as u8
    }

    /// Sets the value of the `Virtualization` field.
    pub const fn set_virtualization(&mut self, value: u8) {
        let offset = Self::VIRTUALIZATION_SHIFT;
        assert!(value & (Self::VIRTUALIZATION_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VIRTUALIZATION_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `Virtualization` field set to the given value.
    pub const fn with_virtualization(mut self, value: u8) -> Self {
        self.set_virtualization(value);
        self
    }

    /// Returns the value of the `GenTimer` field.
    pub const fn gentimer(self) -> u8 {
        ((self.bits() >> Self::GENTIMER_SHIFT) & Self::GENTIMER_MASK) as u8
    }

    /// Sets the value of the `GenTimer` field.
    pub const fn set_gentimer(&mut self, value: u8) {
        let offset = Self::GENTIMER_SHIFT;
        assert!(value & (Self::GENTIMER_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::GENTIMER_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `GenTimer` field set to the given value.
    pub const fn with_gentimer(mut self, value: u8) -> Self {
        self.set_gentimer(value);
        self
    }

    /// Returns the value of the `Sec_frac` field.
    pub const fn sec_frac(self) -> u8 {
        ((self.bits() >> Self::SEC_FRAC_SHIFT) & Self::SEC_FRAC_MASK) as u8
    }

    /// Sets the value of the `Sec_frac` field.
    pub const fn set_sec_frac(&mut self, value: u8) {
        let offset = Self::SEC_FRAC_SHIFT;
        assert!(value & (Self::SEC_FRAC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SEC_FRAC_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `Sec_frac` field set to the given value.
    pub const fn with_sec_frac(mut self, value: u8) -> Self {
        self.set_sec_frac(value);
        self
    }

    /// Returns the value of the `Virt_frac` field.
    pub const fn virt_frac(self) -> u8 {
        ((self.bits() >> Self::VIRT_FRAC_SHIFT) & Self::VIRT_FRAC_MASK) as u8
    }

    /// Sets the value of the `Virt_frac` field.
    pub const fn set_virt_frac(&mut self, value: u8) {
        let offset = Self::VIRT_FRAC_SHIFT;
        assert!(value & (Self::VIRT_FRAC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VIRT_FRAC_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `Virt_frac` field set to the given value.
    pub const fn with_virt_frac(mut self, value: u8) -> Self {
        self.set_virt_frac(value);
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
            (self.bits() & !(Self::GIC_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `GIC` field set to the given value.
    pub const fn with_gic(mut self, value: u8) -> Self {
        self.set_gic(value);
        self
    }
}

bitflags! {
    /// `ID_PFR2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdPfr2: u32 {
    }
}

impl IdPfr2 {
    /// Offset of the `CSV3` field.
    pub const CSV3_SHIFT: u32 = 0;
    /// Mask for the `CSV3` field.
    pub const CSV3_MASK: u32 = 0b1111;
    /// Offset of the `SSBS` field.
    pub const SSBS_SHIFT: u32 = 4;
    /// Mask for the `SSBS` field.
    pub const SSBS_MASK: u32 = 0b1111;
    /// Offset of the `RAS_frac` field.
    pub const RAS_FRAC_SHIFT: u32 = 8;
    /// Mask for the `RAS_frac` field.
    pub const RAS_FRAC_MASK: u32 = 0b1111;

    /// Returns the value of the `CSV3` field.
    pub const fn csv3(self) -> u8 {
        ((self.bits() >> Self::CSV3_SHIFT) & Self::CSV3_MASK) as u8
    }

    /// Sets the value of the `CSV3` field.
    pub const fn set_csv3(&mut self, value: u8) {
        let offset = Self::CSV3_SHIFT;
        assert!(value & (Self::CSV3_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CSV3_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `CSV3` field set to the given value.
    pub const fn with_csv3(mut self, value: u8) -> Self {
        self.set_csv3(value);
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
            (self.bits() & !(Self::SSBS_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `SSBS` field set to the given value.
    pub const fn with_ssbs(mut self, value: u8) -> Self {
        self.set_ssbs(value);
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
            (self.bits() & !(Self::RAS_FRAC_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `RAS_frac` field set to the given value.
    pub const fn with_ras_frac(mut self, value: u8) -> Self {
        self.set_ras_frac(value);
        self
    }
}

bitflags! {
    /// `IFAR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Ifar: u32 {
    }
}

impl Ifar {
    /// Offset of the `VA` field.
    pub const VA_SHIFT: u32 = 0;
    /// Mask for the `VA` field.
    pub const VA_MASK: u32 = 0b1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `VA` field.
    pub const fn va(self) -> u32 {
        (self.bits() >> Self::VA_SHIFT) & Self::VA_MASK
    }

    /// Sets the value of the `VA` field.
    pub const fn set_va(&mut self, value: u32) {
        let offset = Self::VA_SHIFT;
        assert!(value & Self::VA_MASK == value);
        *self =
            Self::from_bits_retain((self.bits() & !(Self::VA_MASK << offset)) | (value << offset));
    }

    /// Returns a copy with the `VA` field set to the given value.
    pub const fn with_va(mut self, value: u32) -> Self {
        self.set_va(value);
        self
    }
}

bitflags! {
    /// `IFSR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Ifsr: u32 {
        /// `LPAE` bit.
        const LPAE = 1 << 9;
        /// `ExT` bit.
        const EXT = 1 << 12;
        /// `FnV` bit.
        const FNV = 1 << 16;
    }
}

impl Ifsr {
    /// Offset of the `STATUS` field.
    pub const STATUS_SHIFT: u32 = 0;
    /// Mask for the `STATUS` field.
    pub const STATUS_MASK: u32 = 0b11_1111;
    /// Offset of the `LPAE` field.
    pub const LPAE_SHIFT: u32 = 9;
    /// Offset of the `ExT` field.
    pub const EXT_SHIFT: u32 = 12;
    /// Offset of the `FnV` field.
    pub const FNV_SHIFT: u32 = 16;

    /// Returns the value of the `STATUS` field.
    pub const fn status(self) -> u8 {
        ((self.bits() >> Self::STATUS_SHIFT) & Self::STATUS_MASK) as u8
    }

    /// Sets the value of the `STATUS` field.
    pub const fn set_status(&mut self, value: u8) {
        let offset = Self::STATUS_SHIFT;
        assert!(value & (Self::STATUS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::STATUS_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `STATUS` field set to the given value.
    pub const fn with_status(mut self, value: u8) -> Self {
        self.set_status(value);
        self
    }
}

bitflags! {
    /// `ISR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Isr: u32 {
        /// `F` bit.
        const F = 1 << 6;
        /// `I` bit.
        const I = 1 << 7;
        /// `A` bit.
        const A = 1 << 8;
    }
}

impl Isr {
    /// Offset of the `F` field.
    pub const F_SHIFT: u32 = 6;
    /// Offset of the `I` field.
    pub const I_SHIFT: u32 = 7;
    /// Offset of the `A` field.
    pub const A_SHIFT: u32 = 8;
}

bitflags! {
    /// `MAIR0` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mair0: u32 {
    }
}

impl Mair0 {
    /// Offset of the `Attr<n>` field.
    pub const ATTR_SHIFT: u32 = 0;
    /// Mask for the `Attr<n>` field.
    pub const ATTR_MASK: u32 = 0b1111_1111;

    /// Returns the value of the given `Attr<n>` field.
    pub const fn attr(self, n: u32) -> u8 {
        assert!(n < 4);
        ((self.bits() >> (Self::ATTR_SHIFT + n * 8)) & Self::ATTR_MASK) as u8
    }

    /// Sets the value of the `Attr<n>` field.
    pub const fn set_attr(&mut self, n: u32, value: u8) {
        assert!(n < 4);
        let offset = Self::ATTR_SHIFT + n * 8;
        assert!(value & (Self::ATTR_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ATTR_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `Attr<n>` field set to the given value.
    pub const fn with_attr(mut self, n: u32, value: u8) -> Self {
        self.set_attr(n, value);
        self
    }
}

bitflags! {
    /// `MAIR1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mair1: u32 {
    }
}

impl Mair1 {
    /// Offset of the `Attr<n>` field.
    pub const ATTR_SHIFT: u32 = 0;
    /// Mask for the `Attr<n>` field.
    pub const ATTR_MASK: u32 = 0b1111_1111;

    /// Returns the value of the given `Attr<n>` field.
    pub const fn attr(self, n: u32) -> u8 {
        assert!(n >= 4 && n < 8);
        ((self.bits() >> (Self::ATTR_SHIFT + (n - 4) * 8)) & Self::ATTR_MASK) as u8
    }

    /// Sets the value of the `Attr<n>` field.
    pub const fn set_attr(&mut self, n: u32, value: u8) {
        assert!(n >= 4 && n < 8);
        let offset = Self::ATTR_SHIFT + (n - 4) * 8;
        assert!(value & (Self::ATTR_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ATTR_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `Attr<n>` field set to the given value.
    pub const fn with_attr(mut self, n: u32, value: u8) -> Self {
        self.set_attr(n, value);
        self
    }
}

bitflags! {
    /// `MIDR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Midr: u32 {
    }
}

impl Midr {
    /// Offset of the `Revision` field.
    pub const REVISION_SHIFT: u32 = 0;
    /// Mask for the `Revision` field.
    pub const REVISION_MASK: u32 = 0b1111;
    /// Offset of the `PartNum` field.
    pub const PARTNUM_SHIFT: u32 = 4;
    /// Mask for the `PartNum` field.
    pub const PARTNUM_MASK: u32 = 0b1111_1111_1111;
    /// Offset of the `Architecture` field.
    pub const ARCHITECTURE_SHIFT: u32 = 16;
    /// Mask for the `Architecture` field.
    pub const ARCHITECTURE_MASK: u32 = 0b1111;
    /// Offset of the `Variant` field.
    pub const VARIANT_SHIFT: u32 = 20;
    /// Mask for the `Variant` field.
    pub const VARIANT_MASK: u32 = 0b1111;
    /// Offset of the `Implementer` field.
    pub const IMPLEMENTER_SHIFT: u32 = 24;
    /// Mask for the `Implementer` field.
    pub const IMPLEMENTER_MASK: u32 = 0b1111_1111;

    /// Returns the value of the `Revision` field.
    pub const fn revision(self) -> u8 {
        ((self.bits() >> Self::REVISION_SHIFT) & Self::REVISION_MASK) as u8
    }

    /// Sets the value of the `Revision` field.
    pub const fn set_revision(&mut self, value: u8) {
        let offset = Self::REVISION_SHIFT;
        assert!(value & (Self::REVISION_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::REVISION_MASK << offset)) | ((value as u32) << offset),
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
            (self.bits() & !(Self::PARTNUM_MASK << offset)) | ((value as u32) << offset),
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
            (self.bits() & !(Self::ARCHITECTURE_MASK << offset)) | ((value as u32) << offset),
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
            (self.bits() & !(Self::VARIANT_MASK << offset)) | ((value as u32) << offset),
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
            (self.bits() & !(Self::IMPLEMENTER_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `Implementer` field set to the given value.
    pub const fn with_implementer(mut self, value: u8) -> Self {
        self.set_implementer(value);
        self
    }
}

bitflags! {
    /// `MPIDR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mpidr: u32 {
        /// `MT` bit.
        const MT = 1 << 24;
        /// `U` bit.
        const U = 1 << 30;
        /// `M` bit.
        const M = 1 << 31;
    }
}

impl Mpidr {
    /// Offset of the `Aff0` field.
    pub const AFF0_SHIFT: u32 = 0;
    /// Mask for the `Aff0` field.
    pub const AFF0_MASK: u32 = 0b1111_1111;
    /// Offset of the `Aff1` field.
    pub const AFF1_SHIFT: u32 = 8;
    /// Mask for the `Aff1` field.
    pub const AFF1_MASK: u32 = 0b1111_1111;
    /// Offset of the `Aff2` field.
    pub const AFF2_SHIFT: u32 = 16;
    /// Mask for the `Aff2` field.
    pub const AFF2_MASK: u32 = 0b1111_1111;
    /// Offset of the `MT` field.
    pub const MT_SHIFT: u32 = 24;
    /// Offset of the `U` field.
    pub const U_SHIFT: u32 = 30;
    /// Offset of the `M` field.
    pub const M_SHIFT: u32 = 31;

    /// Returns the value of the `Aff0` field.
    pub const fn aff0(self) -> u8 {
        ((self.bits() >> Self::AFF0_SHIFT) & Self::AFF0_MASK) as u8
    }

    /// Sets the value of the `Aff0` field.
    pub const fn set_aff0(&mut self, value: u8) {
        let offset = Self::AFF0_SHIFT;
        assert!(value & (Self::AFF0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::AFF0_MASK << offset)) | ((value as u32) << offset),
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
            (self.bits() & !(Self::AFF1_MASK << offset)) | ((value as u32) << offset),
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
            (self.bits() & !(Self::AFF2_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `Aff2` field set to the given value.
    pub const fn with_aff2(mut self, value: u8) -> Self {
        self.set_aff2(value);
        self
    }
}

bitflags! {
    /// `MVBAR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mvbar: u32 {
    }
}

impl Mvbar {
    /// Offset of the `Reserved` field.
    pub const RESERVED_SHIFT: u32 = 0;
    /// Mask for the `Reserved` field.
    pub const RESERVED_MASK: u32 = 0b1_1111;
    /// Offset of the `VBA` field.
    pub const VBA_SHIFT: u32 = 5;
    /// Mask for the `VBA` field.
    pub const VBA_MASK: u32 = 0b111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `Reserved` field.
    pub const fn reserved(self) -> u8 {
        ((self.bits() >> Self::RESERVED_SHIFT) & Self::RESERVED_MASK) as u8
    }

    /// Sets the value of the `Reserved` field.
    pub const fn set_reserved(&mut self, value: u8) {
        let offset = Self::RESERVED_SHIFT;
        assert!(value & (Self::RESERVED_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::RESERVED_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `Reserved` field set to the given value.
    pub const fn with_reserved(mut self, value: u8) -> Self {
        self.set_reserved(value);
        self
    }

    /// Returns the value of the `VBA` field.
    pub const fn vba(self) -> u32 {
        (self.bits() >> Self::VBA_SHIFT) & Self::VBA_MASK
    }

    /// Sets the value of the `VBA` field.
    pub const fn set_vba(&mut self, value: u32) {
        let offset = Self::VBA_SHIFT;
        assert!(value & Self::VBA_MASK == value);
        *self =
            Self::from_bits_retain((self.bits() & !(Self::VBA_MASK << offset)) | (value << offset));
    }

    /// Returns a copy with the `VBA` field set to the given value.
    pub const fn with_vba(mut self, value: u32) -> Self {
        self.set_vba(value);
        self
    }
}

bitflags! {
    /// `NMRR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Nmrr: u32 {
    }
}

impl Nmrr {
    /// Offset of the `IR<n>` field.
    pub const IR_SHIFT: u32 = 0;
    /// Mask for the `IR<n>` field.
    pub const IR_MASK: u32 = 0b11;
    /// Offset of the `OR<n>` field.
    pub const OR_SHIFT: u32 = 16;
    /// Mask for the `OR<n>` field.
    pub const OR_MASK: u32 = 0b11;

    /// Returns the value of the given `IR<n>` field.
    pub const fn ir(self, n: u32) -> u8 {
        assert!(n < 8);
        ((self.bits() >> (Self::IR_SHIFT + n * 2)) & Self::IR_MASK) as u8
    }

    /// Sets the value of the `IR<n>` field.
    pub const fn set_ir(&mut self, n: u32, value: u8) {
        assert!(n < 8);
        let offset = Self::IR_SHIFT + n * 2;
        assert!(value & (Self::IR_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::IR_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `IR<n>` field set to the given value.
    pub const fn with_ir(mut self, n: u32, value: u8) -> Self {
        self.set_ir(n, value);
        self
    }

    /// Returns the value of the given `OR<n>` field.
    pub const fn or(self, n: u32) -> u8 {
        assert!(n < 8);
        ((self.bits() >> (Self::OR_SHIFT + n * 2)) & Self::OR_MASK) as u8
    }

    /// Sets the value of the `OR<n>` field.
    pub const fn set_or(&mut self, n: u32, value: u8) {
        assert!(n < 8);
        let offset = Self::OR_SHIFT + n * 2;
        assert!(value & (Self::OR_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::OR_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `OR<n>` field set to the given value.
    pub const fn with_or(mut self, n: u32, value: u8) -> Self {
        self.set_or(n, value);
        self
    }
}

bitflags! {
    /// `NSACR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Nsacr: u32 {
        /// `cp10` bit.
        const CP10 = 1 << 10;
        /// `cp11` bit.
        const CP11 = 1 << 11;
        /// `NSASEDIS` bit.
        const NSASEDIS = 1 << 15;
        /// `NSTRCDIS` bit.
        const NSTRCDIS = 1 << 20;
    }
}

impl Nsacr {
    /// Offset of the `cp10` field.
    pub const CP10_SHIFT: u32 = 10;
    /// Offset of the `cp11` field.
    pub const CP11_SHIFT: u32 = 11;
    /// Offset of the `NSASEDIS` field.
    pub const NSASEDIS_SHIFT: u32 = 15;
    /// Offset of the `NSTRCDIS` field.
    pub const NSTRCDIS_SHIFT: u32 = 20;
}

bitflags! {
    /// `PAR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Par: u64 {
        /// `F` bit.
        const F = 1 << 0;
        /// `SS` bit.
        const SS = 1 << 1;
        /// `FS[5]` bit.
        const FS_5 = 1 << 6;
        /// `S2WLK` bit.
        const S2WLK = 1 << 8;
        /// `FSTAGE` bit.
        const FSTAGE = 1 << 9;
        /// `NS` bit.
        const NS = 1 << 9;
        /// `NOS` bit.
        const NOS = 1 << 10;
        /// `LPAE` bit.
        const LPAE = 1 << 11;
    }
}

impl Par {
    /// Offset of the `F` field.
    pub const F_SHIFT: u32 = 0;
    /// Offset of the `FST` field.
    pub const FST_SHIFT: u32 = 1;
    /// Mask for the `FST` field.
    pub const FST_MASK: u64 = 0b11_1111;
    /// Offset of the `FS[4:0]` field.
    pub const FS_4_0_SHIFT: u32 = 1;
    /// Mask for the `FS[4:0]` field.
    pub const FS_4_0_MASK: u64 = 0b1_1111;
    /// Offset of the `SS` field.
    pub const SS_SHIFT: u32 = 1;
    /// Offset of the `Outer[1:0]` field.
    pub const OUTER_1_0_SHIFT: u32 = 2;
    /// Mask for the `Outer[1:0]` field.
    pub const OUTER_1_0_MASK: u64 = 0b11;
    /// Offset of the `Inner[2:0]` field.
    pub const INNER_2_0_SHIFT: u32 = 4;
    /// Mask for the `Inner[2:0]` field.
    pub const INNER_2_0_MASK: u64 = 0b111;
    /// Offset of the `FS[5]` field.
    pub const FS_5_SHIFT: u32 = 6;
    /// Offset of the `S2WLK` field.
    pub const S2WLK_SHIFT: u32 = 8;
    /// Offset of the `FSTAGE` field.
    pub const FSTAGE_SHIFT: u32 = 9;
    /// Offset of the `NS` field.
    pub const NS_SHIFT: u32 = 9;
    /// Offset of the `NOS` field.
    pub const NOS_SHIFT: u32 = 10;
    /// Offset of the `LPAE` field.
    pub const LPAE_SHIFT: u32 = 11;
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

    /// Returns the value of the `FS[4:0]` field.
    pub const fn fs_4_0(self) -> u8 {
        ((self.bits() >> Self::FS_4_0_SHIFT) & Self::FS_4_0_MASK) as u8
    }

    /// Sets the value of the `FS[4:0]` field.
    pub const fn set_fs_4_0(&mut self, value: u8) {
        let offset = Self::FS_4_0_SHIFT;
        assert!(value & (Self::FS_4_0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::FS_4_0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `FS[4:0]` field set to the given value.
    pub const fn with_fs_4_0(mut self, value: u8) -> Self {
        self.set_fs_4_0(value);
        self
    }

    /// Returns the value of the `Outer[1:0]` field.
    pub const fn outer_1_0(self) -> u8 {
        ((self.bits() >> Self::OUTER_1_0_SHIFT) & Self::OUTER_1_0_MASK) as u8
    }

    /// Sets the value of the `Outer[1:0]` field.
    pub const fn set_outer_1_0(&mut self, value: u8) {
        let offset = Self::OUTER_1_0_SHIFT;
        assert!(value & (Self::OUTER_1_0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::OUTER_1_0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `Outer[1:0]` field set to the given value.
    pub const fn with_outer_1_0(mut self, value: u8) -> Self {
        self.set_outer_1_0(value);
        self
    }

    /// Returns the value of the `Inner[2:0]` field.
    pub const fn inner_2_0(self) -> u8 {
        ((self.bits() >> Self::INNER_2_0_SHIFT) & Self::INNER_2_0_MASK) as u8
    }

    /// Sets the value of the `Inner[2:0]` field.
    pub const fn set_inner_2_0(&mut self, value: u8) {
        let offset = Self::INNER_2_0_SHIFT;
        assert!(value & (Self::INNER_2_0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::INNER_2_0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `Inner[2:0]` field set to the given value.
    pub const fn with_inner_2_0(mut self, value: u8) -> Self {
        self.set_inner_2_0(value);
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
    /// `PMCCFILTR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Pmccfiltr: u32 {
        /// `RLU` bit.
        const RLU = 1 << 21;
        /// `NSH` bit.
        const NSH = 1 << 27;
        /// `NSU` bit.
        const NSU = 1 << 28;
        /// `NSK` bit.
        const NSK = 1 << 29;
        /// `U` bit.
        const U = 1 << 30;
        /// `P` bit.
        const P = 1 << 31;
    }
}

impl Pmccfiltr {
    /// Offset of the `RLU` field.
    pub const RLU_SHIFT: u32 = 21;
    /// Offset of the `NSH` field.
    pub const NSH_SHIFT: u32 = 27;
    /// Offset of the `NSU` field.
    pub const NSU_SHIFT: u32 = 28;
    /// Offset of the `NSK` field.
    pub const NSK_SHIFT: u32 = 29;
    /// Offset of the `U` field.
    pub const U_SHIFT: u32 = 30;
    /// Offset of the `P` field.
    pub const P_SHIFT: u32 = 31;
}

bitflags! {
    /// `PMCCNTR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Pmccntr: u64 {
    }
}

impl Pmccntr {
    /// Offset of the `CCNT` field.
    pub const CCNT_SHIFT: u32 = 0;
    /// Mask for the `CCNT` field.
    pub const CCNT_MASK: u64 =
        0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `CCNT` field.
    pub const fn ccnt(self) -> u64 {
        (self.bits() >> Self::CCNT_SHIFT) & Self::CCNT_MASK
    }

    /// Sets the value of the `CCNT` field.
    pub const fn set_ccnt(&mut self, value: u64) {
        let offset = Self::CCNT_SHIFT;
        assert!(value & Self::CCNT_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CCNT_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `CCNT` field set to the given value.
    pub const fn with_ccnt(mut self, value: u64) -> Self {
        self.set_ccnt(value);
        self
    }
}

bitflags! {
    /// `PMCEID0` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Pmceid0: u32 {
        /// `ID<n>` bit 0.
        const ID0 = 1 << 0;
        /// `ID<n>` bit 1.
        const ID1 = 1 << 1;
        /// `ID<n>` bit 2.
        const ID2 = 1 << 2;
        /// `ID<n>` bit 3.
        const ID3 = 1 << 3;
        /// `ID<n>` bit 4.
        const ID4 = 1 << 4;
        /// `ID<n>` bit 5.
        const ID5 = 1 << 5;
        /// `ID<n>` bit 6.
        const ID6 = 1 << 6;
        /// `ID<n>` bit 7.
        const ID7 = 1 << 7;
        /// `ID<n>` bit 8.
        const ID8 = 1 << 8;
        /// `ID<n>` bit 9.
        const ID9 = 1 << 9;
        /// `ID<n>` bit 10.
        const ID10 = 1 << 10;
        /// `ID<n>` bit 11.
        const ID11 = 1 << 11;
        /// `ID<n>` bit 12.
        const ID12 = 1 << 12;
        /// `ID<n>` bit 13.
        const ID13 = 1 << 13;
        /// `ID<n>` bit 14.
        const ID14 = 1 << 14;
        /// `ID<n>` bit 15.
        const ID15 = 1 << 15;
        /// `ID<n>` bit 16.
        const ID16 = 1 << 16;
        /// `ID<n>` bit 17.
        const ID17 = 1 << 17;
        /// `ID<n>` bit 18.
        const ID18 = 1 << 18;
        /// `ID<n>` bit 19.
        const ID19 = 1 << 19;
        /// `ID<n>` bit 20.
        const ID20 = 1 << 20;
        /// `ID<n>` bit 21.
        const ID21 = 1 << 21;
        /// `ID<n>` bit 22.
        const ID22 = 1 << 22;
        /// `ID<n>` bit 23.
        const ID23 = 1 << 23;
        /// `ID<n>` bit 24.
        const ID24 = 1 << 24;
        /// `ID<n>` bit 25.
        const ID25 = 1 << 25;
        /// `ID<n>` bit 26.
        const ID26 = 1 << 26;
        /// `ID<n>` bit 27.
        const ID27 = 1 << 27;
        /// `ID<n>` bit 28.
        const ID28 = 1 << 28;
        /// `ID<n>` bit 29.
        const ID29 = 1 << 29;
        /// `ID<n>` bit 30.
        const ID30 = 1 << 30;
        /// `ID<n>` bit 31.
        const ID31 = 1 << 31;
    }
}

impl Pmceid0 {
    /// Offset of the `ID<n>` field.
    pub const ID_SHIFT: u32 = 0;
}

bitflags! {
    /// `PMCEID1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Pmceid1: u32 {
        /// `ID<n>` bit 0.
        const ID0 = 1 << 0;
        /// `ID<n>` bit 1.
        const ID1 = 1 << 1;
        /// `ID<n>` bit 2.
        const ID2 = 1 << 2;
        /// `ID<n>` bit 3.
        const ID3 = 1 << 3;
        /// `ID<n>` bit 4.
        const ID4 = 1 << 4;
        /// `ID<n>` bit 5.
        const ID5 = 1 << 5;
        /// `ID<n>` bit 6.
        const ID6 = 1 << 6;
        /// `ID<n>` bit 7.
        const ID7 = 1 << 7;
        /// `ID<n>` bit 8.
        const ID8 = 1 << 8;
        /// `ID<n>` bit 9.
        const ID9 = 1 << 9;
        /// `ID<n>` bit 10.
        const ID10 = 1 << 10;
        /// `ID<n>` bit 11.
        const ID11 = 1 << 11;
        /// `ID<n>` bit 12.
        const ID12 = 1 << 12;
        /// `ID<n>` bit 13.
        const ID13 = 1 << 13;
        /// `ID<n>` bit 14.
        const ID14 = 1 << 14;
        /// `ID<n>` bit 15.
        const ID15 = 1 << 15;
        /// `ID<n>` bit 16.
        const ID16 = 1 << 16;
        /// `ID<n>` bit 17.
        const ID17 = 1 << 17;
        /// `ID<n>` bit 18.
        const ID18 = 1 << 18;
        /// `ID<n>` bit 19.
        const ID19 = 1 << 19;
        /// `ID<n>` bit 20.
        const ID20 = 1 << 20;
        /// `ID<n>` bit 21.
        const ID21 = 1 << 21;
        /// `ID<n>` bit 22.
        const ID22 = 1 << 22;
        /// `ID<n>` bit 23.
        const ID23 = 1 << 23;
        /// `ID<n>` bit 24.
        const ID24 = 1 << 24;
        /// `ID<n>` bit 25.
        const ID25 = 1 << 25;
        /// `ID<n>` bit 26.
        const ID26 = 1 << 26;
        /// `ID<n>` bit 27.
        const ID27 = 1 << 27;
        /// `ID<n>` bit 28.
        const ID28 = 1 << 28;
        /// `ID<n>` bit 29.
        const ID29 = 1 << 29;
        /// `ID<n>` bit 30.
        const ID30 = 1 << 30;
        /// `ID<n>` bit 31.
        const ID31 = 1 << 31;
    }
}

impl Pmceid1 {
    /// Offset of the `ID<n>` field.
    pub const ID_SHIFT: u32 = 0;
}

bitflags! {
    /// `PMCEID2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Pmceid2: u32 {
        /// `IDhi<n>` bit 0.
        const IDHI0 = 1 << 0;
        /// `IDhi<n>` bit 1.
        const IDHI1 = 1 << 1;
        /// `IDhi<n>` bit 2.
        const IDHI2 = 1 << 2;
        /// `IDhi<n>` bit 3.
        const IDHI3 = 1 << 3;
        /// `IDhi<n>` bit 4.
        const IDHI4 = 1 << 4;
        /// `IDhi<n>` bit 5.
        const IDHI5 = 1 << 5;
        /// `IDhi<n>` bit 6.
        const IDHI6 = 1 << 6;
        /// `IDhi<n>` bit 7.
        const IDHI7 = 1 << 7;
        /// `IDhi<n>` bit 8.
        const IDHI8 = 1 << 8;
        /// `IDhi<n>` bit 9.
        const IDHI9 = 1 << 9;
        /// `IDhi<n>` bit 10.
        const IDHI10 = 1 << 10;
        /// `IDhi<n>` bit 11.
        const IDHI11 = 1 << 11;
        /// `IDhi<n>` bit 12.
        const IDHI12 = 1 << 12;
        /// `IDhi<n>` bit 13.
        const IDHI13 = 1 << 13;
        /// `IDhi<n>` bit 14.
        const IDHI14 = 1 << 14;
        /// `IDhi<n>` bit 15.
        const IDHI15 = 1 << 15;
        /// `IDhi<n>` bit 16.
        const IDHI16 = 1 << 16;
        /// `IDhi<n>` bit 17.
        const IDHI17 = 1 << 17;
        /// `IDhi<n>` bit 18.
        const IDHI18 = 1 << 18;
        /// `IDhi<n>` bit 19.
        const IDHI19 = 1 << 19;
        /// `IDhi<n>` bit 20.
        const IDHI20 = 1 << 20;
        /// `IDhi<n>` bit 21.
        const IDHI21 = 1 << 21;
        /// `IDhi<n>` bit 22.
        const IDHI22 = 1 << 22;
        /// `IDhi<n>` bit 23.
        const IDHI23 = 1 << 23;
        /// `IDhi<n>` bit 24.
        const IDHI24 = 1 << 24;
        /// `IDhi<n>` bit 25.
        const IDHI25 = 1 << 25;
        /// `IDhi<n>` bit 26.
        const IDHI26 = 1 << 26;
        /// `IDhi<n>` bit 27.
        const IDHI27 = 1 << 27;
        /// `IDhi<n>` bit 28.
        const IDHI28 = 1 << 28;
        /// `IDhi<n>` bit 29.
        const IDHI29 = 1 << 29;
        /// `IDhi<n>` bit 30.
        const IDHI30 = 1 << 30;
        /// `IDhi<n>` bit 31.
        const IDHI31 = 1 << 31;
    }
}

impl Pmceid2 {
    /// Offset of the `IDhi<n>` field.
    pub const IDHI_SHIFT: u32 = 0;
}

bitflags! {
    /// `PMCEID3` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Pmceid3: u32 {
        /// `IDhi<n>` bit 0.
        const IDHI0 = 1 << 0;
        /// `IDhi<n>` bit 1.
        const IDHI1 = 1 << 1;
        /// `IDhi<n>` bit 2.
        const IDHI2 = 1 << 2;
        /// `IDhi<n>` bit 3.
        const IDHI3 = 1 << 3;
        /// `IDhi<n>` bit 4.
        const IDHI4 = 1 << 4;
        /// `IDhi<n>` bit 5.
        const IDHI5 = 1 << 5;
        /// `IDhi<n>` bit 6.
        const IDHI6 = 1 << 6;
        /// `IDhi<n>` bit 7.
        const IDHI7 = 1 << 7;
        /// `IDhi<n>` bit 8.
        const IDHI8 = 1 << 8;
        /// `IDhi<n>` bit 9.
        const IDHI9 = 1 << 9;
        /// `IDhi<n>` bit 10.
        const IDHI10 = 1 << 10;
        /// `IDhi<n>` bit 11.
        const IDHI11 = 1 << 11;
        /// `IDhi<n>` bit 12.
        const IDHI12 = 1 << 12;
        /// `IDhi<n>` bit 13.
        const IDHI13 = 1 << 13;
        /// `IDhi<n>` bit 14.
        const IDHI14 = 1 << 14;
        /// `IDhi<n>` bit 15.
        const IDHI15 = 1 << 15;
        /// `IDhi<n>` bit 16.
        const IDHI16 = 1 << 16;
        /// `IDhi<n>` bit 17.
        const IDHI17 = 1 << 17;
        /// `IDhi<n>` bit 18.
        const IDHI18 = 1 << 18;
        /// `IDhi<n>` bit 19.
        const IDHI19 = 1 << 19;
        /// `IDhi<n>` bit 20.
        const IDHI20 = 1 << 20;
        /// `IDhi<n>` bit 21.
        const IDHI21 = 1 << 21;
        /// `IDhi<n>` bit 22.
        const IDHI22 = 1 << 22;
        /// `IDhi<n>` bit 23.
        const IDHI23 = 1 << 23;
        /// `IDhi<n>` bit 24.
        const IDHI24 = 1 << 24;
        /// `IDhi<n>` bit 25.
        const IDHI25 = 1 << 25;
        /// `IDhi<n>` bit 26.
        const IDHI26 = 1 << 26;
        /// `IDhi<n>` bit 27.
        const IDHI27 = 1 << 27;
        /// `IDhi<n>` bit 28.
        const IDHI28 = 1 << 28;
        /// `IDhi<n>` bit 29.
        const IDHI29 = 1 << 29;
        /// `IDhi<n>` bit 30.
        const IDHI30 = 1 << 30;
        /// `IDhi<n>` bit 31.
        const IDHI31 = 1 << 31;
    }
}

impl Pmceid3 {
    /// Offset of the `IDhi<n>` field.
    pub const IDHI_SHIFT: u32 = 0;
}

bitflags! {
    /// `PMCNTENCLR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Pmcntenclr: u32 {
        /// `P<m>` bit 0.
        const P0 = 1 << 0;
        /// `P<m>` bit 1.
        const P1 = 1 << 1;
        /// `P<m>` bit 2.
        const P2 = 1 << 2;
        /// `P<m>` bit 3.
        const P3 = 1 << 3;
        /// `P<m>` bit 4.
        const P4 = 1 << 4;
        /// `P<m>` bit 5.
        const P5 = 1 << 5;
        /// `P<m>` bit 6.
        const P6 = 1 << 6;
        /// `P<m>` bit 7.
        const P7 = 1 << 7;
        /// `P<m>` bit 8.
        const P8 = 1 << 8;
        /// `P<m>` bit 9.
        const P9 = 1 << 9;
        /// `P<m>` bit 10.
        const P10 = 1 << 10;
        /// `P<m>` bit 11.
        const P11 = 1 << 11;
        /// `P<m>` bit 12.
        const P12 = 1 << 12;
        /// `P<m>` bit 13.
        const P13 = 1 << 13;
        /// `P<m>` bit 14.
        const P14 = 1 << 14;
        /// `P<m>` bit 15.
        const P15 = 1 << 15;
        /// `P<m>` bit 16.
        const P16 = 1 << 16;
        /// `P<m>` bit 17.
        const P17 = 1 << 17;
        /// `P<m>` bit 18.
        const P18 = 1 << 18;
        /// `P<m>` bit 19.
        const P19 = 1 << 19;
        /// `P<m>` bit 20.
        const P20 = 1 << 20;
        /// `P<m>` bit 21.
        const P21 = 1 << 21;
        /// `P<m>` bit 22.
        const P22 = 1 << 22;
        /// `P<m>` bit 23.
        const P23 = 1 << 23;
        /// `P<m>` bit 24.
        const P24 = 1 << 24;
        /// `P<m>` bit 25.
        const P25 = 1 << 25;
        /// `P<m>` bit 26.
        const P26 = 1 << 26;
        /// `P<m>` bit 27.
        const P27 = 1 << 27;
        /// `P<m>` bit 28.
        const P28 = 1 << 28;
        /// `P<m>` bit 29.
        const P29 = 1 << 29;
        /// `P<m>` bit 30.
        const P30 = 1 << 30;
        /// `C` bit.
        const C = 1 << 31;
    }
}

impl Pmcntenclr {
    /// Offset of the `P<m>` field.
    pub const P_SHIFT: u32 = 0;
    /// Offset of the `C` field.
    pub const C_SHIFT: u32 = 31;
}

bitflags! {
    /// `PMCNTENSET` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Pmcntenset: u32 {
        /// `P<m>` bit 0.
        const P0 = 1 << 0;
        /// `P<m>` bit 1.
        const P1 = 1 << 1;
        /// `P<m>` bit 2.
        const P2 = 1 << 2;
        /// `P<m>` bit 3.
        const P3 = 1 << 3;
        /// `P<m>` bit 4.
        const P4 = 1 << 4;
        /// `P<m>` bit 5.
        const P5 = 1 << 5;
        /// `P<m>` bit 6.
        const P6 = 1 << 6;
        /// `P<m>` bit 7.
        const P7 = 1 << 7;
        /// `P<m>` bit 8.
        const P8 = 1 << 8;
        /// `P<m>` bit 9.
        const P9 = 1 << 9;
        /// `P<m>` bit 10.
        const P10 = 1 << 10;
        /// `P<m>` bit 11.
        const P11 = 1 << 11;
        /// `P<m>` bit 12.
        const P12 = 1 << 12;
        /// `P<m>` bit 13.
        const P13 = 1 << 13;
        /// `P<m>` bit 14.
        const P14 = 1 << 14;
        /// `P<m>` bit 15.
        const P15 = 1 << 15;
        /// `P<m>` bit 16.
        const P16 = 1 << 16;
        /// `P<m>` bit 17.
        const P17 = 1 << 17;
        /// `P<m>` bit 18.
        const P18 = 1 << 18;
        /// `P<m>` bit 19.
        const P19 = 1 << 19;
        /// `P<m>` bit 20.
        const P20 = 1 << 20;
        /// `P<m>` bit 21.
        const P21 = 1 << 21;
        /// `P<m>` bit 22.
        const P22 = 1 << 22;
        /// `P<m>` bit 23.
        const P23 = 1 << 23;
        /// `P<m>` bit 24.
        const P24 = 1 << 24;
        /// `P<m>` bit 25.
        const P25 = 1 << 25;
        /// `P<m>` bit 26.
        const P26 = 1 << 26;
        /// `P<m>` bit 27.
        const P27 = 1 << 27;
        /// `P<m>` bit 28.
        const P28 = 1 << 28;
        /// `P<m>` bit 29.
        const P29 = 1 << 29;
        /// `P<m>` bit 30.
        const P30 = 1 << 30;
        /// `C` bit.
        const C = 1 << 31;
    }
}

impl Pmcntenset {
    /// Offset of the `P<m>` field.
    pub const P_SHIFT: u32 = 0;
    /// Offset of the `C` field.
    pub const C_SHIFT: u32 = 31;
}

bitflags! {
    /// `PMCR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Pmcr: u32 {
        /// `E` bit.
        const E = 1 << 0;
        /// `P` bit.
        const P = 1 << 1;
        /// `C` bit.
        const C = 1 << 2;
        /// `D` bit.
        const D = 1 << 3;
        /// `X` bit.
        const X = 1 << 4;
        /// `DP` bit.
        const DP = 1 << 5;
        /// `LC` bit.
        const LC = 1 << 6;
        /// `LP` bit.
        const LP = 1 << 7;
        /// `FZO` bit.
        const FZO = 1 << 9;
    }
}

impl Pmcr {
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
    pub const N_MASK: u32 = 0b1_1111;
    /// Offset of the `IDCODE` field.
    pub const IDCODE_SHIFT: u32 = 16;
    /// Mask for the `IDCODE` field.
    pub const IDCODE_MASK: u32 = 0b1111_1111;
    /// Offset of the `IMP` field.
    pub const IMP_SHIFT: u32 = 24;
    /// Mask for the `IMP` field.
    pub const IMP_MASK: u32 = 0b1111_1111;

    /// Returns the value of the `N` field.
    pub const fn n(self) -> u8 {
        ((self.bits() >> Self::N_SHIFT) & Self::N_MASK) as u8
    }

    /// Sets the value of the `N` field.
    pub const fn set_n(&mut self, value: u8) {
        let offset = Self::N_SHIFT;
        assert!(value & (Self::N_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::N_MASK << offset)) | ((value as u32) << offset),
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
            (self.bits() & !(Self::IDCODE_MASK << offset)) | ((value as u32) << offset),
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
            (self.bits() & !(Self::IMP_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `IMP` field set to the given value.
    pub const fn with_imp(mut self, value: u8) -> Self {
        self.set_imp(value);
        self
    }
}

bitflags! {
    /// `PMINTENCLR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Pmintenclr: u32 {
        /// `P<m>` bit 0.
        const P0 = 1 << 0;
        /// `P<m>` bit 1.
        const P1 = 1 << 1;
        /// `P<m>` bit 2.
        const P2 = 1 << 2;
        /// `P<m>` bit 3.
        const P3 = 1 << 3;
        /// `P<m>` bit 4.
        const P4 = 1 << 4;
        /// `P<m>` bit 5.
        const P5 = 1 << 5;
        /// `P<m>` bit 6.
        const P6 = 1 << 6;
        /// `P<m>` bit 7.
        const P7 = 1 << 7;
        /// `P<m>` bit 8.
        const P8 = 1 << 8;
        /// `P<m>` bit 9.
        const P9 = 1 << 9;
        /// `P<m>` bit 10.
        const P10 = 1 << 10;
        /// `P<m>` bit 11.
        const P11 = 1 << 11;
        /// `P<m>` bit 12.
        const P12 = 1 << 12;
        /// `P<m>` bit 13.
        const P13 = 1 << 13;
        /// `P<m>` bit 14.
        const P14 = 1 << 14;
        /// `P<m>` bit 15.
        const P15 = 1 << 15;
        /// `P<m>` bit 16.
        const P16 = 1 << 16;
        /// `P<m>` bit 17.
        const P17 = 1 << 17;
        /// `P<m>` bit 18.
        const P18 = 1 << 18;
        /// `P<m>` bit 19.
        const P19 = 1 << 19;
        /// `P<m>` bit 20.
        const P20 = 1 << 20;
        /// `P<m>` bit 21.
        const P21 = 1 << 21;
        /// `P<m>` bit 22.
        const P22 = 1 << 22;
        /// `P<m>` bit 23.
        const P23 = 1 << 23;
        /// `P<m>` bit 24.
        const P24 = 1 << 24;
        /// `P<m>` bit 25.
        const P25 = 1 << 25;
        /// `P<m>` bit 26.
        const P26 = 1 << 26;
        /// `P<m>` bit 27.
        const P27 = 1 << 27;
        /// `P<m>` bit 28.
        const P28 = 1 << 28;
        /// `P<m>` bit 29.
        const P29 = 1 << 29;
        /// `P<m>` bit 30.
        const P30 = 1 << 30;
        /// `C` bit.
        const C = 1 << 31;
    }
}

impl Pmintenclr {
    /// Offset of the `P<m>` field.
    pub const P_SHIFT: u32 = 0;
    /// Offset of the `C` field.
    pub const C_SHIFT: u32 = 31;
}

bitflags! {
    /// `PMINTENSET` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Pmintenset: u32 {
        /// `P<m>` bit 0.
        const P0 = 1 << 0;
        /// `P<m>` bit 1.
        const P1 = 1 << 1;
        /// `P<m>` bit 2.
        const P2 = 1 << 2;
        /// `P<m>` bit 3.
        const P3 = 1 << 3;
        /// `P<m>` bit 4.
        const P4 = 1 << 4;
        /// `P<m>` bit 5.
        const P5 = 1 << 5;
        /// `P<m>` bit 6.
        const P6 = 1 << 6;
        /// `P<m>` bit 7.
        const P7 = 1 << 7;
        /// `P<m>` bit 8.
        const P8 = 1 << 8;
        /// `P<m>` bit 9.
        const P9 = 1 << 9;
        /// `P<m>` bit 10.
        const P10 = 1 << 10;
        /// `P<m>` bit 11.
        const P11 = 1 << 11;
        /// `P<m>` bit 12.
        const P12 = 1 << 12;
        /// `P<m>` bit 13.
        const P13 = 1 << 13;
        /// `P<m>` bit 14.
        const P14 = 1 << 14;
        /// `P<m>` bit 15.
        const P15 = 1 << 15;
        /// `P<m>` bit 16.
        const P16 = 1 << 16;
        /// `P<m>` bit 17.
        const P17 = 1 << 17;
        /// `P<m>` bit 18.
        const P18 = 1 << 18;
        /// `P<m>` bit 19.
        const P19 = 1 << 19;
        /// `P<m>` bit 20.
        const P20 = 1 << 20;
        /// `P<m>` bit 21.
        const P21 = 1 << 21;
        /// `P<m>` bit 22.
        const P22 = 1 << 22;
        /// `P<m>` bit 23.
        const P23 = 1 << 23;
        /// `P<m>` bit 24.
        const P24 = 1 << 24;
        /// `P<m>` bit 25.
        const P25 = 1 << 25;
        /// `P<m>` bit 26.
        const P26 = 1 << 26;
        /// `P<m>` bit 27.
        const P27 = 1 << 27;
        /// `P<m>` bit 28.
        const P28 = 1 << 28;
        /// `P<m>` bit 29.
        const P29 = 1 << 29;
        /// `P<m>` bit 30.
        const P30 = 1 << 30;
        /// `C` bit.
        const C = 1 << 31;
    }
}

impl Pmintenset {
    /// Offset of the `P<m>` field.
    pub const P_SHIFT: u32 = 0;
    /// Offset of the `C` field.
    pub const C_SHIFT: u32 = 31;
}

bitflags! {
    /// `PMMIR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Pmmir: u32 {
    }
}

impl Pmmir {
    /// Offset of the `SLOTS` field.
    pub const SLOTS_SHIFT: u32 = 0;
    /// Mask for the `SLOTS` field.
    pub const SLOTS_MASK: u32 = 0b1111_1111;
    /// Offset of the `BUS_SLOTS` field.
    pub const BUS_SLOTS_SHIFT: u32 = 8;
    /// Mask for the `BUS_SLOTS` field.
    pub const BUS_SLOTS_MASK: u32 = 0b1111_1111;
    /// Offset of the `BUS_WIDTH` field.
    pub const BUS_WIDTH_SHIFT: u32 = 16;
    /// Mask for the `BUS_WIDTH` field.
    pub const BUS_WIDTH_MASK: u32 = 0b1111;
    /// Offset of the `THWIDTH` field.
    pub const THWIDTH_SHIFT: u32 = 20;
    /// Mask for the `THWIDTH` field.
    pub const THWIDTH_MASK: u32 = 0b1111;
    /// Offset of the `EDGE` field.
    pub const EDGE_SHIFT: u32 = 24;
    /// Mask for the `EDGE` field.
    pub const EDGE_MASK: u32 = 0b1111;

    /// Returns the value of the `SLOTS` field.
    pub const fn slots(self) -> u8 {
        ((self.bits() >> Self::SLOTS_SHIFT) & Self::SLOTS_MASK) as u8
    }

    /// Sets the value of the `SLOTS` field.
    pub const fn set_slots(&mut self, value: u8) {
        let offset = Self::SLOTS_SHIFT;
        assert!(value & (Self::SLOTS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SLOTS_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `SLOTS` field set to the given value.
    pub const fn with_slots(mut self, value: u8) -> Self {
        self.set_slots(value);
        self
    }

    /// Returns the value of the `BUS_SLOTS` field.
    pub const fn bus_slots(self) -> u8 {
        ((self.bits() >> Self::BUS_SLOTS_SHIFT) & Self::BUS_SLOTS_MASK) as u8
    }

    /// Sets the value of the `BUS_SLOTS` field.
    pub const fn set_bus_slots(&mut self, value: u8) {
        let offset = Self::BUS_SLOTS_SHIFT;
        assert!(value & (Self::BUS_SLOTS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BUS_SLOTS_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `BUS_SLOTS` field set to the given value.
    pub const fn with_bus_slots(mut self, value: u8) -> Self {
        self.set_bus_slots(value);
        self
    }

    /// Returns the value of the `BUS_WIDTH` field.
    pub const fn bus_width(self) -> u8 {
        ((self.bits() >> Self::BUS_WIDTH_SHIFT) & Self::BUS_WIDTH_MASK) as u8
    }

    /// Sets the value of the `BUS_WIDTH` field.
    pub const fn set_bus_width(&mut self, value: u8) {
        let offset = Self::BUS_WIDTH_SHIFT;
        assert!(value & (Self::BUS_WIDTH_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BUS_WIDTH_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `BUS_WIDTH` field set to the given value.
    pub const fn with_bus_width(mut self, value: u8) -> Self {
        self.set_bus_width(value);
        self
    }

    /// Returns the value of the `THWIDTH` field.
    pub const fn thwidth(self) -> u8 {
        ((self.bits() >> Self::THWIDTH_SHIFT) & Self::THWIDTH_MASK) as u8
    }

    /// Sets the value of the `THWIDTH` field.
    pub const fn set_thwidth(&mut self, value: u8) {
        let offset = Self::THWIDTH_SHIFT;
        assert!(value & (Self::THWIDTH_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::THWIDTH_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `THWIDTH` field set to the given value.
    pub const fn with_thwidth(mut self, value: u8) -> Self {
        self.set_thwidth(value);
        self
    }

    /// Returns the value of the `EDGE` field.
    pub const fn edge(self) -> u8 {
        ((self.bits() >> Self::EDGE_SHIFT) & Self::EDGE_MASK) as u8
    }

    /// Sets the value of the `EDGE` field.
    pub const fn set_edge(&mut self, value: u8) {
        let offset = Self::EDGE_SHIFT;
        assert!(value & (Self::EDGE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EDGE_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `EDGE` field set to the given value.
    pub const fn with_edge(mut self, value: u8) -> Self {
        self.set_edge(value);
        self
    }
}

bitflags! {
    /// `PMOVSR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Pmovsr: u32 {
        /// `P<m>` bit 0.
        const P0 = 1 << 0;
        /// `P<m>` bit 1.
        const P1 = 1 << 1;
        /// `P<m>` bit 2.
        const P2 = 1 << 2;
        /// `P<m>` bit 3.
        const P3 = 1 << 3;
        /// `P<m>` bit 4.
        const P4 = 1 << 4;
        /// `P<m>` bit 5.
        const P5 = 1 << 5;
        /// `P<m>` bit 6.
        const P6 = 1 << 6;
        /// `P<m>` bit 7.
        const P7 = 1 << 7;
        /// `P<m>` bit 8.
        const P8 = 1 << 8;
        /// `P<m>` bit 9.
        const P9 = 1 << 9;
        /// `P<m>` bit 10.
        const P10 = 1 << 10;
        /// `P<m>` bit 11.
        const P11 = 1 << 11;
        /// `P<m>` bit 12.
        const P12 = 1 << 12;
        /// `P<m>` bit 13.
        const P13 = 1 << 13;
        /// `P<m>` bit 14.
        const P14 = 1 << 14;
        /// `P<m>` bit 15.
        const P15 = 1 << 15;
        /// `P<m>` bit 16.
        const P16 = 1 << 16;
        /// `P<m>` bit 17.
        const P17 = 1 << 17;
        /// `P<m>` bit 18.
        const P18 = 1 << 18;
        /// `P<m>` bit 19.
        const P19 = 1 << 19;
        /// `P<m>` bit 20.
        const P20 = 1 << 20;
        /// `P<m>` bit 21.
        const P21 = 1 << 21;
        /// `P<m>` bit 22.
        const P22 = 1 << 22;
        /// `P<m>` bit 23.
        const P23 = 1 << 23;
        /// `P<m>` bit 24.
        const P24 = 1 << 24;
        /// `P<m>` bit 25.
        const P25 = 1 << 25;
        /// `P<m>` bit 26.
        const P26 = 1 << 26;
        /// `P<m>` bit 27.
        const P27 = 1 << 27;
        /// `P<m>` bit 28.
        const P28 = 1 << 28;
        /// `P<m>` bit 29.
        const P29 = 1 << 29;
        /// `P<m>` bit 30.
        const P30 = 1 << 30;
        /// `C` bit.
        const C = 1 << 31;
    }
}

impl Pmovsr {
    /// Offset of the `P<m>` field.
    pub const P_SHIFT: u32 = 0;
    /// Offset of the `C` field.
    pub const C_SHIFT: u32 = 31;
}

bitflags! {
    /// `PMOVSSET` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Pmovsset: u32 {
        /// `P<m>` bit 0.
        const P0 = 1 << 0;
        /// `P<m>` bit 1.
        const P1 = 1 << 1;
        /// `P<m>` bit 2.
        const P2 = 1 << 2;
        /// `P<m>` bit 3.
        const P3 = 1 << 3;
        /// `P<m>` bit 4.
        const P4 = 1 << 4;
        /// `P<m>` bit 5.
        const P5 = 1 << 5;
        /// `P<m>` bit 6.
        const P6 = 1 << 6;
        /// `P<m>` bit 7.
        const P7 = 1 << 7;
        /// `P<m>` bit 8.
        const P8 = 1 << 8;
        /// `P<m>` bit 9.
        const P9 = 1 << 9;
        /// `P<m>` bit 10.
        const P10 = 1 << 10;
        /// `P<m>` bit 11.
        const P11 = 1 << 11;
        /// `P<m>` bit 12.
        const P12 = 1 << 12;
        /// `P<m>` bit 13.
        const P13 = 1 << 13;
        /// `P<m>` bit 14.
        const P14 = 1 << 14;
        /// `P<m>` bit 15.
        const P15 = 1 << 15;
        /// `P<m>` bit 16.
        const P16 = 1 << 16;
        /// `P<m>` bit 17.
        const P17 = 1 << 17;
        /// `P<m>` bit 18.
        const P18 = 1 << 18;
        /// `P<m>` bit 19.
        const P19 = 1 << 19;
        /// `P<m>` bit 20.
        const P20 = 1 << 20;
        /// `P<m>` bit 21.
        const P21 = 1 << 21;
        /// `P<m>` bit 22.
        const P22 = 1 << 22;
        /// `P<m>` bit 23.
        const P23 = 1 << 23;
        /// `P<m>` bit 24.
        const P24 = 1 << 24;
        /// `P<m>` bit 25.
        const P25 = 1 << 25;
        /// `P<m>` bit 26.
        const P26 = 1 << 26;
        /// `P<m>` bit 27.
        const P27 = 1 << 27;
        /// `P<m>` bit 28.
        const P28 = 1 << 28;
        /// `P<m>` bit 29.
        const P29 = 1 << 29;
        /// `P<m>` bit 30.
        const P30 = 1 << 30;
        /// `C` bit.
        const C = 1 << 31;
    }
}

impl Pmovsset {
    /// Offset of the `P<m>` field.
    pub const P_SHIFT: u32 = 0;
    /// Offset of the `C` field.
    pub const C_SHIFT: u32 = 31;
}

bitflags! {
    /// `PMSELR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Pmselr: u32 {
    }
}

impl Pmselr {
    /// Offset of the `SEL` field.
    pub const SEL_SHIFT: u32 = 0;
    /// Mask for the `SEL` field.
    pub const SEL_MASK: u32 = 0b1_1111;

    /// Returns the value of the `SEL` field.
    pub const fn sel(self) -> u8 {
        ((self.bits() >> Self::SEL_SHIFT) & Self::SEL_MASK) as u8
    }

    /// Sets the value of the `SEL` field.
    pub const fn set_sel(&mut self, value: u8) {
        let offset = Self::SEL_SHIFT;
        assert!(value & (Self::SEL_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SEL_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `SEL` field set to the given value.
    pub const fn with_sel(mut self, value: u8) -> Self {
        self.set_sel(value);
        self
    }
}

bitflags! {
    /// `PMSWINC` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Pmswinc: u32 {
        /// `P<m>` bit 0.
        const P0 = 1 << 0;
        /// `P<m>` bit 1.
        const P1 = 1 << 1;
        /// `P<m>` bit 2.
        const P2 = 1 << 2;
        /// `P<m>` bit 3.
        const P3 = 1 << 3;
        /// `P<m>` bit 4.
        const P4 = 1 << 4;
        /// `P<m>` bit 5.
        const P5 = 1 << 5;
        /// `P<m>` bit 6.
        const P6 = 1 << 6;
        /// `P<m>` bit 7.
        const P7 = 1 << 7;
        /// `P<m>` bit 8.
        const P8 = 1 << 8;
        /// `P<m>` bit 9.
        const P9 = 1 << 9;
        /// `P<m>` bit 10.
        const P10 = 1 << 10;
        /// `P<m>` bit 11.
        const P11 = 1 << 11;
        /// `P<m>` bit 12.
        const P12 = 1 << 12;
        /// `P<m>` bit 13.
        const P13 = 1 << 13;
        /// `P<m>` bit 14.
        const P14 = 1 << 14;
        /// `P<m>` bit 15.
        const P15 = 1 << 15;
        /// `P<m>` bit 16.
        const P16 = 1 << 16;
        /// `P<m>` bit 17.
        const P17 = 1 << 17;
        /// `P<m>` bit 18.
        const P18 = 1 << 18;
        /// `P<m>` bit 19.
        const P19 = 1 << 19;
        /// `P<m>` bit 20.
        const P20 = 1 << 20;
        /// `P<m>` bit 21.
        const P21 = 1 << 21;
        /// `P<m>` bit 22.
        const P22 = 1 << 22;
        /// `P<m>` bit 23.
        const P23 = 1 << 23;
        /// `P<m>` bit 24.
        const P24 = 1 << 24;
        /// `P<m>` bit 25.
        const P25 = 1 << 25;
        /// `P<m>` bit 26.
        const P26 = 1 << 26;
        /// `P<m>` bit 27.
        const P27 = 1 << 27;
        /// `P<m>` bit 28.
        const P28 = 1 << 28;
        /// `P<m>` bit 29.
        const P29 = 1 << 29;
        /// `P<m>` bit 30.
        const P30 = 1 << 30;
    }
}

impl Pmswinc {
    /// Offset of the `P<m>` field.
    pub const P_SHIFT: u32 = 0;
}

bitflags! {
    /// `PMUSERENR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Pmuserenr: u32 {
        /// `EN` bit.
        const EN = 1 << 0;
        /// `SW` bit.
        const SW = 1 << 1;
        /// `CR` bit.
        const CR = 1 << 2;
        /// `ER` bit.
        const ER = 1 << 3;
        /// `TID` bit.
        const TID = 1 << 6;
    }
}

impl Pmuserenr {
    /// Offset of the `EN` field.
    pub const EN_SHIFT: u32 = 0;
    /// Offset of the `SW` field.
    pub const SW_SHIFT: u32 = 1;
    /// Offset of the `CR` field.
    pub const CR_SHIFT: u32 = 2;
    /// Offset of the `ER` field.
    pub const ER_SHIFT: u32 = 3;
    /// Offset of the `TID` field.
    pub const TID_SHIFT: u32 = 6;
}

bitflags! {
    /// `PMXEVTYPER` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Pmxevtyper: u32 {
    }
}

impl Pmxevtyper {
    /// Offset of the `ETR` field.
    pub const ETR_SHIFT: u32 = 0;
    /// Mask for the `ETR` field.
    pub const ETR_MASK: u32 = 0b1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `ETR` field.
    pub const fn etr(self) -> u32 {
        (self.bits() >> Self::ETR_SHIFT) & Self::ETR_MASK
    }

    /// Sets the value of the `ETR` field.
    pub const fn set_etr(&mut self, value: u32) {
        let offset = Self::ETR_SHIFT;
        assert!(value & Self::ETR_MASK == value);
        *self =
            Self::from_bits_retain((self.bits() & !(Self::ETR_MASK << offset)) | (value << offset));
    }

    /// Returns a copy with the `ETR` field set to the given value.
    pub const fn with_etr(mut self, value: u32) -> Self {
        self.set_etr(value);
        self
    }
}

bitflags! {
    /// `PRRR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Prrr: u32 {
        /// `DS0` bit.
        const DS0 = 1 << 16;
        /// `DS1` bit.
        const DS1 = 1 << 17;
        /// `NS0` bit.
        const NS0 = 1 << 18;
        /// `NS1` bit.
        const NS1 = 1 << 19;
        /// `NOS<n>` bit 0.
        const NOS0 = 1 << 24;
        /// `NOS<n>` bit 1.
        const NOS1 = 1 << 25;
        /// `NOS<n>` bit 2.
        const NOS2 = 1 << 26;
        /// `NOS<n>` bit 3.
        const NOS3 = 1 << 27;
        /// `NOS<n>` bit 4.
        const NOS4 = 1 << 28;
        /// `NOS<n>` bit 5.
        const NOS5 = 1 << 29;
        /// `NOS<n>` bit 6.
        const NOS6 = 1 << 30;
        /// `NOS<n>` bit 7.
        const NOS7 = 1 << 31;
    }
}

impl Prrr {
    /// Offset of the `TR<n>` field.
    pub const TR_SHIFT: u32 = 0;
    /// Mask for the `TR<n>` field.
    pub const TR_MASK: u32 = 0b11;
    /// Offset of the `DS0` field.
    pub const DS0_SHIFT: u32 = 16;
    /// Offset of the `DS1` field.
    pub const DS1_SHIFT: u32 = 17;
    /// Offset of the `NS0` field.
    pub const NS0_SHIFT: u32 = 18;
    /// Offset of the `NS1` field.
    pub const NS1_SHIFT: u32 = 19;
    /// Offset of the `NOS<n>` field.
    pub const NOS_SHIFT: u32 = 24;

    /// Returns the value of the given `TR<n>` field.
    pub const fn tr(self, n: u32) -> u8 {
        assert!(n < 8);
        ((self.bits() >> (Self::TR_SHIFT + n * 2)) & Self::TR_MASK) as u8
    }

    /// Sets the value of the `TR<n>` field.
    pub const fn set_tr(&mut self, n: u32, value: u8) {
        assert!(n < 8);
        let offset = Self::TR_SHIFT + n * 2;
        assert!(value & (Self::TR_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TR_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `TR<n>` field set to the given value.
    pub const fn with_tr(mut self, n: u32, value: u8) -> Self {
        self.set_tr(n, value);
        self
    }
}

bitflags! {
    /// `RMR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Rmr: u32 {
        /// `AA64` bit.
        const AA64 = 1 << 0;
        /// `RR` bit.
        const RR = 1 << 1;
    }
}

impl Rmr {
    /// Offset of the `AA64` field.
    pub const AA64_SHIFT: u32 = 0;
    /// Offset of the `RR` field.
    pub const RR_SHIFT: u32 = 1;
}

bitflags! {
    /// `RVBAR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Rvbar: u32 {
        /// RES1 bits in the `RVBAR` register.
        const RES1 = 0b1;
    }
}

impl Rvbar {
    /// Offset of the `ResetAddress` field.
    pub const RESETADDRESS_SHIFT: u32 = 1;
    /// Mask for the `ResetAddress` field.
    pub const RESETADDRESS_MASK: u32 = 0b111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `ResetAddress` field.
    pub const fn resetaddress(self) -> u32 {
        (self.bits() >> Self::RESETADDRESS_SHIFT) & Self::RESETADDRESS_MASK
    }

    /// Sets the value of the `ResetAddress` field.
    pub const fn set_resetaddress(&mut self, value: u32) {
        let offset = Self::RESETADDRESS_SHIFT;
        assert!(value & Self::RESETADDRESS_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::RESETADDRESS_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `ResetAddress` field set to the given value.
    pub const fn with_resetaddress(mut self, value: u32) -> Self {
        self.set_resetaddress(value);
        self
    }
}

bitflags! {
    /// `SCR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Scr: u32 {
        /// `NS` bit.
        const NS = 1 << 0;
        /// `IRQ` bit.
        const IRQ = 1 << 1;
        /// `FIQ` bit.
        const FIQ = 1 << 2;
        /// `EA` bit.
        const EA = 1 << 3;
        /// `FW` bit.
        const FW = 1 << 4;
        /// `AW` bit.
        const AW = 1 << 5;
        /// `nET` bit.
        const NET = 1 << 6;
        /// `SCD` bit.
        const SCD = 1 << 7;
        /// `HCE` bit.
        const HCE = 1 << 8;
        /// `SIF` bit.
        const SIF = 1 << 9;
        /// `TWI` bit.
        const TWI = 1 << 12;
        /// `TWE` bit.
        const TWE = 1 << 13;
        /// `TERR` bit.
        const TERR = 1 << 15;
    }
}

impl Scr {
    /// Offset of the `NS` field.
    pub const NS_SHIFT: u32 = 0;
    /// Offset of the `IRQ` field.
    pub const IRQ_SHIFT: u32 = 1;
    /// Offset of the `FIQ` field.
    pub const FIQ_SHIFT: u32 = 2;
    /// Offset of the `EA` field.
    pub const EA_SHIFT: u32 = 3;
    /// Offset of the `FW` field.
    pub const FW_SHIFT: u32 = 4;
    /// Offset of the `AW` field.
    pub const AW_SHIFT: u32 = 5;
    /// Offset of the `nET` field.
    pub const NET_SHIFT: u32 = 6;
    /// Offset of the `SCD` field.
    pub const SCD_SHIFT: u32 = 7;
    /// Offset of the `HCE` field.
    pub const HCE_SHIFT: u32 = 8;
    /// Offset of the `SIF` field.
    pub const SIF_SHIFT: u32 = 9;
    /// Offset of the `TWI` field.
    pub const TWI_SHIFT: u32 = 12;
    /// Offset of the `TWE` field.
    pub const TWE_SHIFT: u32 = 13;
    /// Offset of the `TERR` field.
    pub const TERR_SHIFT: u32 = 15;
}

bitflags! {
    /// `SCTLR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Sctlr: u32 {
        /// RES1 bits in the `SCTLR` register.
        const RES1 = 0b100_0000_0000_1000_0000_0000;
        /// `M` bit.
        const M = 1 << 0;
        /// `A` bit.
        const A = 1 << 1;
        /// `C` bit.
        const C = 1 << 2;
        /// `nTLSMD` bit.
        const NTLSMD = 1 << 3;
        /// `LSMAOE` bit.
        const LSMAOE = 1 << 4;
        /// `CP15BEN` bit.
        const CP15BEN = 1 << 5;
        /// `UNK` bit.
        const UNK = 1 << 6;
        /// `ITD` bit.
        const ITD = 1 << 7;
        /// `SED` bit.
        const SED = 1 << 8;
        /// `EnRCTX` bit.
        const ENRCTX = 1 << 10;
        /// `I` bit.
        const I = 1 << 12;
        /// `V` bit.
        const V = 1 << 13;
        /// `nTWI` bit.
        const NTWI = 1 << 16;
        /// `nTWE` bit.
        const NTWE = 1 << 18;
        /// `WXN` bit.
        const WXN = 1 << 19;
        /// `UWXN` bit.
        const UWXN = 1 << 20;
        /// `SPAN` bit.
        const SPAN = 1 << 23;
        /// `EE` bit.
        const EE = 1 << 25;
        /// `TRE` bit.
        const TRE = 1 << 28;
        /// `AFE` bit.
        const AFE = 1 << 29;
        /// `TE` bit.
        const TE = 1 << 30;
        /// `DSSBS` bit.
        const DSSBS = 1 << 31;
    }
}

impl Sctlr {
    /// Offset of the `M` field.
    pub const M_SHIFT: u32 = 0;
    /// Offset of the `A` field.
    pub const A_SHIFT: u32 = 1;
    /// Offset of the `C` field.
    pub const C_SHIFT: u32 = 2;
    /// Offset of the `nTLSMD` field.
    pub const NTLSMD_SHIFT: u32 = 3;
    /// Offset of the `LSMAOE` field.
    pub const LSMAOE_SHIFT: u32 = 4;
    /// Offset of the `CP15BEN` field.
    pub const CP15BEN_SHIFT: u32 = 5;
    /// Offset of the `UNK` field.
    pub const UNK_SHIFT: u32 = 6;
    /// Offset of the `ITD` field.
    pub const ITD_SHIFT: u32 = 7;
    /// Offset of the `SED` field.
    pub const SED_SHIFT: u32 = 8;
    /// Offset of the `EnRCTX` field.
    pub const ENRCTX_SHIFT: u32 = 10;
    /// Offset of the `I` field.
    pub const I_SHIFT: u32 = 12;
    /// Offset of the `V` field.
    pub const V_SHIFT: u32 = 13;
    /// Offset of the `nTWI` field.
    pub const NTWI_SHIFT: u32 = 16;
    /// Offset of the `nTWE` field.
    pub const NTWE_SHIFT: u32 = 18;
    /// Offset of the `WXN` field.
    pub const WXN_SHIFT: u32 = 19;
    /// Offset of the `UWXN` field.
    pub const UWXN_SHIFT: u32 = 20;
    /// Offset of the `SPAN` field.
    pub const SPAN_SHIFT: u32 = 23;
    /// Offset of the `EE` field.
    pub const EE_SHIFT: u32 = 25;
    /// Offset of the `TRE` field.
    pub const TRE_SHIFT: u32 = 28;
    /// Offset of the `AFE` field.
    pub const AFE_SHIFT: u32 = 29;
    /// Offset of the `TE` field.
    pub const TE_SHIFT: u32 = 30;
    /// Offset of the `DSSBS` field.
    pub const DSSBS_SHIFT: u32 = 31;
}

bitflags! {
    /// `SDCR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Sdcr: u32 {
        /// `SPME` bit.
        const SPME = 1 << 17;
        /// `STE` bit.
        const STE = 1 << 18;
        /// `TTRF` bit.
        const TTRF = 1 << 19;
        /// `EDAD` bit.
        const EDAD = 1 << 20;
        /// `EPMAD` bit.
        const EPMAD = 1 << 21;
        /// `SCCD` bit.
        const SCCD = 1 << 23;
        /// `TDCC` bit.
        const TDCC = 1 << 27;
        /// `MTPME` bit.
        const MTPME = 1 << 28;
    }
}

impl Sdcr {
    /// Offset of the `SPD` field.
    pub const SPD_SHIFT: u32 = 14;
    /// Mask for the `SPD` field.
    pub const SPD_MASK: u32 = 0b11;
    /// Offset of the `SPME` field.
    pub const SPME_SHIFT: u32 = 17;
    /// Offset of the `STE` field.
    pub const STE_SHIFT: u32 = 18;
    /// Offset of the `TTRF` field.
    pub const TTRF_SHIFT: u32 = 19;
    /// Offset of the `EDAD` field.
    pub const EDAD_SHIFT: u32 = 20;
    /// Offset of the `EPMAD` field.
    pub const EPMAD_SHIFT: u32 = 21;
    /// Offset of the `SCCD` field.
    pub const SCCD_SHIFT: u32 = 23;
    /// Offset of the `TDCC` field.
    pub const TDCC_SHIFT: u32 = 27;
    /// Offset of the `MTPME` field.
    pub const MTPME_SHIFT: u32 = 28;

    /// Returns the value of the `SPD` field.
    pub const fn spd(self) -> u8 {
        ((self.bits() >> Self::SPD_SHIFT) & Self::SPD_MASK) as u8
    }

    /// Sets the value of the `SPD` field.
    pub const fn set_spd(&mut self, value: u8) {
        let offset = Self::SPD_SHIFT;
        assert!(value & (Self::SPD_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SPD_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `SPD` field set to the given value.
    pub const fn with_spd(mut self, value: u8) -> Self {
        self.set_spd(value);
        self
    }
}

bitflags! {
    /// `SDER` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Sder: u32 {
        /// `SUIDEN` bit.
        const SUIDEN = 1 << 0;
        /// `SUNIDEN` bit.
        const SUNIDEN = 1 << 1;
    }
}

impl Sder {
    /// Offset of the `SUIDEN` field.
    pub const SUIDEN_SHIFT: u32 = 0;
    /// Offset of the `SUNIDEN` field.
    pub const SUNIDEN_SHIFT: u32 = 1;
}

bitflags! {
    /// `TLBTR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Tlbtr: u32 {
        /// `nU` bit.
        const NU = 1 << 0;
    }
}

impl Tlbtr {
    /// Offset of the `nU` field.
    pub const NU_SHIFT: u32 = 0;
}

bitflags! {
    /// `TPIDRPRW` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Tpidrprw: u32 {
    }
}

impl Tpidrprw {
    /// Offset of the `TID` field.
    pub const TID_SHIFT: u32 = 0;
    /// Mask for the `TID` field.
    pub const TID_MASK: u32 = 0b1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `TID` field.
    pub const fn tid(self) -> u32 {
        (self.bits() >> Self::TID_SHIFT) & Self::TID_MASK
    }

    /// Sets the value of the `TID` field.
    pub const fn set_tid(&mut self, value: u32) {
        let offset = Self::TID_SHIFT;
        assert!(value & Self::TID_MASK == value);
        *self =
            Self::from_bits_retain((self.bits() & !(Self::TID_MASK << offset)) | (value << offset));
    }

    /// Returns a copy with the `TID` field set to the given value.
    pub const fn with_tid(mut self, value: u32) -> Self {
        self.set_tid(value);
        self
    }
}

bitflags! {
    /// `TPIDRURO` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Tpidruro: u32 {
    }
}

impl Tpidruro {
    /// Offset of the `TID` field.
    pub const TID_SHIFT: u32 = 0;
    /// Mask for the `TID` field.
    pub const TID_MASK: u32 = 0b1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `TID` field.
    pub const fn tid(self) -> u32 {
        (self.bits() >> Self::TID_SHIFT) & Self::TID_MASK
    }

    /// Sets the value of the `TID` field.
    pub const fn set_tid(&mut self, value: u32) {
        let offset = Self::TID_SHIFT;
        assert!(value & Self::TID_MASK == value);
        *self =
            Self::from_bits_retain((self.bits() & !(Self::TID_MASK << offset)) | (value << offset));
    }

    /// Returns a copy with the `TID` field set to the given value.
    pub const fn with_tid(mut self, value: u32) -> Self {
        self.set_tid(value);
        self
    }
}

bitflags! {
    /// `TPIDRURW` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Tpidrurw: u32 {
    }
}

impl Tpidrurw {
    /// Offset of the `TID` field.
    pub const TID_SHIFT: u32 = 0;
    /// Mask for the `TID` field.
    pub const TID_MASK: u32 = 0b1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `TID` field.
    pub const fn tid(self) -> u32 {
        (self.bits() >> Self::TID_SHIFT) & Self::TID_MASK
    }

    /// Sets the value of the `TID` field.
    pub const fn set_tid(&mut self, value: u32) {
        let offset = Self::TID_SHIFT;
        assert!(value & Self::TID_MASK == value);
        *self =
            Self::from_bits_retain((self.bits() & !(Self::TID_MASK << offset)) | (value << offset));
    }

    /// Returns a copy with the `TID` field set to the given value.
    pub const fn with_tid(mut self, value: u32) -> Self {
        self.set_tid(value);
        self
    }
}

bitflags! {
    /// `TRFCR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Trfcr: u32 {
        /// `E0TRE` bit.
        const E0TRE = 1 << 0;
        /// `E1TRE` bit.
        const E1TRE = 1 << 1;
    }
}

impl Trfcr {
    /// Offset of the `E0TRE` field.
    pub const E0TRE_SHIFT: u32 = 0;
    /// Offset of the `E1TRE` field.
    pub const E1TRE_SHIFT: u32 = 1;
    /// Offset of the `TS` field.
    pub const TS_SHIFT: u32 = 5;
    /// Mask for the `TS` field.
    pub const TS_MASK: u32 = 0b11;

    /// Returns the value of the `TS` field.
    pub const fn ts(self) -> u8 {
        ((self.bits() >> Self::TS_SHIFT) & Self::TS_MASK) as u8
    }

    /// Sets the value of the `TS` field.
    pub const fn set_ts(&mut self, value: u8) {
        let offset = Self::TS_SHIFT;
        assert!(value & (Self::TS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TS_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `TS` field set to the given value.
    pub const fn with_ts(mut self, value: u8) -> Self {
        self.set_ts(value);
        self
    }
}

bitflags! {
    /// `TTBCR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Ttbcr: u32 {
        /// `PD0` bit.
        const PD0 = 1 << 4;
        /// `PD1` bit.
        const PD1 = 1 << 5;
        /// `T2E` bit.
        const T2E = 1 << 6;
        /// `EPD0` bit.
        const EPD0 = 1 << 7;
        /// `A1` bit.
        const A1 = 1 << 22;
        /// `EPD1` bit.
        const EPD1 = 1 << 23;
        /// `EAE` bit.
        const EAE = 1 << 31;
    }
}

impl Ttbcr {
    /// Offset of the `N` field.
    pub const N_SHIFT: u32 = 0;
    /// Mask for the `N` field.
    pub const N_MASK: u32 = 0b111;
    /// Offset of the `T0SZ` field.
    pub const T0SZ_SHIFT: u32 = 0;
    /// Mask for the `T0SZ` field.
    pub const T0SZ_MASK: u32 = 0b111;
    /// Offset of the `PD0` field.
    pub const PD0_SHIFT: u32 = 4;
    /// Offset of the `PD1` field.
    pub const PD1_SHIFT: u32 = 5;
    /// Offset of the `T2E` field.
    pub const T2E_SHIFT: u32 = 6;
    /// Offset of the `EPD0` field.
    pub const EPD0_SHIFT: u32 = 7;
    /// Offset of the `IRGN0` field.
    pub const IRGN0_SHIFT: u32 = 8;
    /// Mask for the `IRGN0` field.
    pub const IRGN0_MASK: u32 = 0b11;
    /// Offset of the `ORGN0` field.
    pub const ORGN0_SHIFT: u32 = 10;
    /// Mask for the `ORGN0` field.
    pub const ORGN0_MASK: u32 = 0b11;
    /// Offset of the `SH0` field.
    pub const SH0_SHIFT: u32 = 12;
    /// Mask for the `SH0` field.
    pub const SH0_MASK: u32 = 0b11;
    /// Offset of the `T1SZ` field.
    pub const T1SZ_SHIFT: u32 = 16;
    /// Mask for the `T1SZ` field.
    pub const T1SZ_MASK: u32 = 0b111;
    /// Offset of the `A1` field.
    pub const A1_SHIFT: u32 = 22;
    /// Offset of the `EPD1` field.
    pub const EPD1_SHIFT: u32 = 23;
    /// Offset of the `IRGN1` field.
    pub const IRGN1_SHIFT: u32 = 24;
    /// Mask for the `IRGN1` field.
    pub const IRGN1_MASK: u32 = 0b11;
    /// Offset of the `ORGN1` field.
    pub const ORGN1_SHIFT: u32 = 26;
    /// Mask for the `ORGN1` field.
    pub const ORGN1_MASK: u32 = 0b11;
    /// Offset of the `SH1` field.
    pub const SH1_SHIFT: u32 = 28;
    /// Mask for the `SH1` field.
    pub const SH1_MASK: u32 = 0b11;
    /// Offset of the `EAE` field.
    pub const EAE_SHIFT: u32 = 31;

    /// Returns the value of the `N` field.
    pub const fn n(self) -> u8 {
        ((self.bits() >> Self::N_SHIFT) & Self::N_MASK) as u8
    }

    /// Sets the value of the `N` field.
    pub const fn set_n(&mut self, value: u8) {
        let offset = Self::N_SHIFT;
        assert!(value & (Self::N_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::N_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `N` field set to the given value.
    pub const fn with_n(mut self, value: u8) -> Self {
        self.set_n(value);
        self
    }

    /// Returns the value of the `T0SZ` field.
    pub const fn t0sz(self) -> u8 {
        ((self.bits() >> Self::T0SZ_SHIFT) & Self::T0SZ_MASK) as u8
    }

    /// Sets the value of the `T0SZ` field.
    pub const fn set_t0sz(&mut self, value: u8) {
        let offset = Self::T0SZ_SHIFT;
        assert!(value & (Self::T0SZ_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::T0SZ_MASK << offset)) | ((value as u32) << offset),
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
            (self.bits() & !(Self::IRGN0_MASK << offset)) | ((value as u32) << offset),
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
            (self.bits() & !(Self::ORGN0_MASK << offset)) | ((value as u32) << offset),
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
            (self.bits() & !(Self::SH0_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `SH0` field set to the given value.
    pub const fn with_sh0(mut self, value: u8) -> Self {
        self.set_sh0(value);
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
            (self.bits() & !(Self::T1SZ_MASK << offset)) | ((value as u32) << offset),
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
            (self.bits() & !(Self::IRGN1_MASK << offset)) | ((value as u32) << offset),
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
            (self.bits() & !(Self::ORGN1_MASK << offset)) | ((value as u32) << offset),
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
            (self.bits() & !(Self::SH1_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `SH1` field set to the given value.
    pub const fn with_sh1(mut self, value: u8) -> Self {
        self.set_sh1(value);
        self
    }
}

bitflags! {
    /// `TTBCR2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Ttbcr2: u32 {
        /// `HPD0` bit.
        const HPD0 = 1 << 9;
        /// `HPD1` bit.
        const HPD1 = 1 << 10;
        /// `HWU059` bit.
        const HWU059 = 1 << 11;
        /// `HWU060` bit.
        const HWU060 = 1 << 12;
        /// `HWU061` bit.
        const HWU061 = 1 << 13;
        /// `HWU062` bit.
        const HWU062 = 1 << 14;
        /// `HWU159` bit.
        const HWU159 = 1 << 15;
        /// `HWU160` bit.
        const HWU160 = 1 << 16;
        /// `HWU161` bit.
        const HWU161 = 1 << 17;
        /// `HWU162` bit.
        const HWU162 = 1 << 18;
    }
}

impl Ttbcr2 {
    /// Offset of the `HPD0` field.
    pub const HPD0_SHIFT: u32 = 9;
    /// Offset of the `HPD1` field.
    pub const HPD1_SHIFT: u32 = 10;
    /// Offset of the `HWU059` field.
    pub const HWU059_SHIFT: u32 = 11;
    /// Offset of the `HWU060` field.
    pub const HWU060_SHIFT: u32 = 12;
    /// Offset of the `HWU061` field.
    pub const HWU061_SHIFT: u32 = 13;
    /// Offset of the `HWU062` field.
    pub const HWU062_SHIFT: u32 = 14;
    /// Offset of the `HWU159` field.
    pub const HWU159_SHIFT: u32 = 15;
    /// Offset of the `HWU160` field.
    pub const HWU160_SHIFT: u32 = 16;
    /// Offset of the `HWU161` field.
    pub const HWU161_SHIFT: u32 = 17;
    /// Offset of the `HWU162` field.
    pub const HWU162_SHIFT: u32 = 18;
}

bitflags! {
    /// `TTBR0` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Ttbr0: u64 {
        /// `CnP` bit.
        const CNP = 1 << 0;
        /// `S` bit.
        const S = 1 << 1;
        /// `IMP` bit.
        const IMP = 1 << 2;
        /// `NOS` bit.
        const NOS = 1 << 5;
    }
}

impl Ttbr0 {
    /// Offset of the `CnP` field.
    pub const CNP_SHIFT: u32 = 0;
    /// Offset of the `BADDR` field.
    pub const BADDR_SHIFT: u32 = 1;
    /// Mask for the `BADDR` field.
    pub const BADDR_MASK: u64 = 0b111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;
    /// Offset of the `S` field.
    pub const S_SHIFT: u32 = 1;
    /// Offset of the `IMP` field.
    pub const IMP_SHIFT: u32 = 2;
    /// Offset of the `RGN` field.
    pub const RGN_SHIFT: u32 = 3;
    /// Mask for the `RGN` field.
    pub const RGN_MASK: u64 = 0b11;
    /// Offset of the `NOS` field.
    pub const NOS_SHIFT: u32 = 5;
    /// Offset of the `TTB0` field.
    pub const TTB0_SHIFT: u32 = 7;
    /// Mask for the `TTB0` field.
    pub const TTB0_MASK: u64 = 0b1_1111_1111_1111_1111_1111_1111;
    /// Offset of the `ASID` field.
    pub const ASID_SHIFT: u32 = 48;
    /// Mask for the `ASID` field.
    pub const ASID_MASK: u64 = 0b1111_1111;

    /// Returns the value of the `BADDR` field.
    pub const fn baddr(self) -> u64 {
        (self.bits() >> Self::BADDR_SHIFT) & Self::BADDR_MASK
    }

    /// Sets the value of the `BADDR` field.
    pub const fn set_baddr(&mut self, value: u64) {
        let offset = Self::BADDR_SHIFT;
        assert!(value & Self::BADDR_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BADDR_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `BADDR` field set to the given value.
    pub const fn with_baddr(mut self, value: u64) -> Self {
        self.set_baddr(value);
        self
    }

    /// Returns the value of the `RGN` field.
    pub const fn rgn(self) -> u8 {
        ((self.bits() >> Self::RGN_SHIFT) & Self::RGN_MASK) as u8
    }

    /// Sets the value of the `RGN` field.
    pub const fn set_rgn(&mut self, value: u8) {
        let offset = Self::RGN_SHIFT;
        assert!(value & (Self::RGN_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::RGN_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `RGN` field set to the given value.
    pub const fn with_rgn(mut self, value: u8) -> Self {
        self.set_rgn(value);
        self
    }

    /// Returns the value of the `TTB0` field.
    pub const fn ttb0(self) -> u32 {
        ((self.bits() >> Self::TTB0_SHIFT) & Self::TTB0_MASK) as u32
    }

    /// Sets the value of the `TTB0` field.
    pub const fn set_ttb0(&mut self, value: u32) {
        let offset = Self::TTB0_SHIFT;
        assert!(value & (Self::TTB0_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TTB0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `TTB0` field set to the given value.
    pub const fn with_ttb0(mut self, value: u32) -> Self {
        self.set_ttb0(value);
        self
    }

    /// Returns the value of the `ASID` field.
    pub const fn asid(self) -> u8 {
        ((self.bits() >> Self::ASID_SHIFT) & Self::ASID_MASK) as u8
    }

    /// Sets the value of the `ASID` field.
    pub const fn set_asid(&mut self, value: u8) {
        let offset = Self::ASID_SHIFT;
        assert!(value & (Self::ASID_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ASID_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `ASID` field set to the given value.
    pub const fn with_asid(mut self, value: u8) -> Self {
        self.set_asid(value);
        self
    }
}

bitflags! {
    /// `TTBR1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Ttbr1: u64 {
        /// `CnP` bit.
        const CNP = 1 << 0;
        /// `S` bit.
        const S = 1 << 1;
        /// `IMP` bit.
        const IMP = 1 << 2;
        /// `NOS` bit.
        const NOS = 1 << 5;
    }
}

impl Ttbr1 {
    /// Offset of the `CnP` field.
    pub const CNP_SHIFT: u32 = 0;
    /// Offset of the `BADDR` field.
    pub const BADDR_SHIFT: u32 = 1;
    /// Mask for the `BADDR` field.
    pub const BADDR_MASK: u64 = 0b111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;
    /// Offset of the `S` field.
    pub const S_SHIFT: u32 = 1;
    /// Offset of the `IMP` field.
    pub const IMP_SHIFT: u32 = 2;
    /// Offset of the `RGN` field.
    pub const RGN_SHIFT: u32 = 3;
    /// Mask for the `RGN` field.
    pub const RGN_MASK: u64 = 0b11;
    /// Offset of the `NOS` field.
    pub const NOS_SHIFT: u32 = 5;
    /// Offset of the `TTB1` field.
    pub const TTB1_SHIFT: u32 = 7;
    /// Mask for the `TTB1` field.
    pub const TTB1_MASK: u64 = 0b1_1111_1111_1111_1111_1111_1111;
    /// Offset of the `ASID` field.
    pub const ASID_SHIFT: u32 = 48;
    /// Mask for the `ASID` field.
    pub const ASID_MASK: u64 = 0b1111_1111;

    /// Returns the value of the `BADDR` field.
    pub const fn baddr(self) -> u64 {
        (self.bits() >> Self::BADDR_SHIFT) & Self::BADDR_MASK
    }

    /// Sets the value of the `BADDR` field.
    pub const fn set_baddr(&mut self, value: u64) {
        let offset = Self::BADDR_SHIFT;
        assert!(value & Self::BADDR_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BADDR_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `BADDR` field set to the given value.
    pub const fn with_baddr(mut self, value: u64) -> Self {
        self.set_baddr(value);
        self
    }

    /// Returns the value of the `RGN` field.
    pub const fn rgn(self) -> u8 {
        ((self.bits() >> Self::RGN_SHIFT) & Self::RGN_MASK) as u8
    }

    /// Sets the value of the `RGN` field.
    pub const fn set_rgn(&mut self, value: u8) {
        let offset = Self::RGN_SHIFT;
        assert!(value & (Self::RGN_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::RGN_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `RGN` field set to the given value.
    pub const fn with_rgn(mut self, value: u8) -> Self {
        self.set_rgn(value);
        self
    }

    /// Returns the value of the `TTB1` field.
    pub const fn ttb1(self) -> u32 {
        ((self.bits() >> Self::TTB1_SHIFT) & Self::TTB1_MASK) as u32
    }

    /// Sets the value of the `TTB1` field.
    pub const fn set_ttb1(&mut self, value: u32) {
        let offset = Self::TTB1_SHIFT;
        assert!(value & (Self::TTB1_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TTB1_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `TTB1` field set to the given value.
    pub const fn with_ttb1(mut self, value: u32) -> Self {
        self.set_ttb1(value);
        self
    }

    /// Returns the value of the `ASID` field.
    pub const fn asid(self) -> u8 {
        ((self.bits() >> Self::ASID_SHIFT) & Self::ASID_MASK) as u8
    }

    /// Sets the value of the `ASID` field.
    pub const fn set_asid(&mut self, value: u8) {
        let offset = Self::ASID_SHIFT;
        assert!(value & (Self::ASID_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ASID_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `ASID` field set to the given value.
    pub const fn with_asid(mut self, value: u8) -> Self {
        self.set_asid(value);
        self
    }
}

bitflags! {
    /// `VBAR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Vbar: u32 {
    }
}

impl Vbar {
    /// Offset of the `VBA` field.
    pub const VBA_SHIFT: u32 = 5;
    /// Mask for the `VBA` field.
    pub const VBA_MASK: u32 = 0b111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `VBA` field.
    pub const fn vba(self) -> u32 {
        (self.bits() >> Self::VBA_SHIFT) & Self::VBA_MASK
    }

    /// Sets the value of the `VBA` field.
    pub const fn set_vba(&mut self, value: u32) {
        let offset = Self::VBA_SHIFT;
        assert!(value & Self::VBA_MASK == value);
        *self =
            Self::from_bits_retain((self.bits() & !(Self::VBA_MASK << offset)) | (value << offset));
    }

    /// Returns a copy with the `VBA` field set to the given value.
    pub const fn with_vba(mut self, value: u32) -> Self {
        self.set_vba(value);
        self
    }
}

bitflags! {
    /// `VDFSR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Vdfsr: u32 {
        /// `ExT` bit.
        const EXT = 1 << 12;
    }
}

impl Vdfsr {
    /// Offset of the `ExT` field.
    pub const EXT_SHIFT: u32 = 12;
    /// Offset of the `AET` field.
    pub const AET_SHIFT: u32 = 14;
    /// Mask for the `AET` field.
    pub const AET_MASK: u32 = 0b11;

    /// Returns the value of the `AET` field.
    pub const fn aet(self) -> u8 {
        ((self.bits() >> Self::AET_SHIFT) & Self::AET_MASK) as u8
    }

    /// Sets the value of the `AET` field.
    pub const fn set_aet(&mut self, value: u8) {
        let offset = Self::AET_SHIFT;
        assert!(value & (Self::AET_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::AET_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `AET` field set to the given value.
    pub const fn with_aet(mut self, value: u8) -> Self {
        self.set_aet(value);
        self
    }
}

bitflags! {
    /// `VDISR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Vdisr: u32 {
        /// `LPAE` bit.
        const LPAE = 1 << 9;
        /// `ExT` bit.
        const EXT = 1 << 12;
        /// `A` bit.
        const A = 1 << 31;
    }
}

impl Vdisr {
    /// Offset of the `STATUS` field.
    pub const STATUS_SHIFT: u32 = 0;
    /// Mask for the `STATUS` field.
    pub const STATUS_MASK: u32 = 0b11_1111;
    /// Offset of the `LPAE` field.
    pub const LPAE_SHIFT: u32 = 9;
    /// Offset of the `ExT` field.
    pub const EXT_SHIFT: u32 = 12;
    /// Offset of the `AET` field.
    pub const AET_SHIFT: u32 = 14;
    /// Mask for the `AET` field.
    pub const AET_MASK: u32 = 0b11;
    /// Offset of the `A` field.
    pub const A_SHIFT: u32 = 31;

    /// Returns the value of the `STATUS` field.
    pub const fn status(self) -> u8 {
        ((self.bits() >> Self::STATUS_SHIFT) & Self::STATUS_MASK) as u8
    }

    /// Sets the value of the `STATUS` field.
    pub const fn set_status(&mut self, value: u8) {
        let offset = Self::STATUS_SHIFT;
        assert!(value & (Self::STATUS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::STATUS_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `STATUS` field set to the given value.
    pub const fn with_status(mut self, value: u8) -> Self {
        self.set_status(value);
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
            (self.bits() & !(Self::AET_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `AET` field set to the given value.
    pub const fn with_aet(mut self, value: u8) -> Self {
        self.set_aet(value);
        self
    }
}

bitflags! {
    /// `VMPIDR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Vmpidr: u32 {
        /// `MT` bit.
        const MT = 1 << 24;
        /// `U` bit.
        const U = 1 << 30;
        /// `M` bit.
        const M = 1 << 31;
    }
}

impl Vmpidr {
    /// Offset of the `Aff0` field.
    pub const AFF0_SHIFT: u32 = 0;
    /// Mask for the `Aff0` field.
    pub const AFF0_MASK: u32 = 0b1111_1111;
    /// Offset of the `Aff1` field.
    pub const AFF1_SHIFT: u32 = 8;
    /// Mask for the `Aff1` field.
    pub const AFF1_MASK: u32 = 0b1111_1111;
    /// Offset of the `Aff2` field.
    pub const AFF2_SHIFT: u32 = 16;
    /// Mask for the `Aff2` field.
    pub const AFF2_MASK: u32 = 0b1111_1111;
    /// Offset of the `MT` field.
    pub const MT_SHIFT: u32 = 24;
    /// Offset of the `U` field.
    pub const U_SHIFT: u32 = 30;
    /// Offset of the `M` field.
    pub const M_SHIFT: u32 = 31;

    /// Returns the value of the `Aff0` field.
    pub const fn aff0(self) -> u8 {
        ((self.bits() >> Self::AFF0_SHIFT) & Self::AFF0_MASK) as u8
    }

    /// Sets the value of the `Aff0` field.
    pub const fn set_aff0(&mut self, value: u8) {
        let offset = Self::AFF0_SHIFT;
        assert!(value & (Self::AFF0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::AFF0_MASK << offset)) | ((value as u32) << offset),
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
            (self.bits() & !(Self::AFF1_MASK << offset)) | ((value as u32) << offset),
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
            (self.bits() & !(Self::AFF2_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `Aff2` field set to the given value.
    pub const fn with_aff2(mut self, value: u8) -> Self {
        self.set_aff2(value);
        self
    }
}

bitflags! {
    /// `VPIDR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Vpidr: u32 {
    }
}

impl Vpidr {
    /// Offset of the `Revision` field.
    pub const REVISION_SHIFT: u32 = 0;
    /// Mask for the `Revision` field.
    pub const REVISION_MASK: u32 = 0b1111;
    /// Offset of the `PartNum` field.
    pub const PARTNUM_SHIFT: u32 = 4;
    /// Mask for the `PartNum` field.
    pub const PARTNUM_MASK: u32 = 0b1111_1111_1111;
    /// Offset of the `Architecture` field.
    pub const ARCHITECTURE_SHIFT: u32 = 16;
    /// Mask for the `Architecture` field.
    pub const ARCHITECTURE_MASK: u32 = 0b1111;
    /// Offset of the `Variant` field.
    pub const VARIANT_SHIFT: u32 = 20;
    /// Mask for the `Variant` field.
    pub const VARIANT_MASK: u32 = 0b1111;
    /// Offset of the `Implementer` field.
    pub const IMPLEMENTER_SHIFT: u32 = 24;
    /// Mask for the `Implementer` field.
    pub const IMPLEMENTER_MASK: u32 = 0b1111_1111;

    /// Returns the value of the `Revision` field.
    pub const fn revision(self) -> u8 {
        ((self.bits() >> Self::REVISION_SHIFT) & Self::REVISION_MASK) as u8
    }

    /// Sets the value of the `Revision` field.
    pub const fn set_revision(&mut self, value: u8) {
        let offset = Self::REVISION_SHIFT;
        assert!(value & (Self::REVISION_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::REVISION_MASK << offset)) | ((value as u32) << offset),
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
            (self.bits() & !(Self::PARTNUM_MASK << offset)) | ((value as u32) << offset),
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
            (self.bits() & !(Self::ARCHITECTURE_MASK << offset)) | ((value as u32) << offset),
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
            (self.bits() & !(Self::VARIANT_MASK << offset)) | ((value as u32) << offset),
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
            (self.bits() & !(Self::IMPLEMENTER_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `Implementer` field set to the given value.
    pub const fn with_implementer(mut self, value: u8) -> Self {
        self.set_implementer(value);
        self
    }
}

bitflags! {
    /// `VTCR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Vtcr: u32 {
        /// RES1 bits in the `VTCR` register.
        const RES1 = 0b1000_0000_0000_0000_0000_0000_0000_0000;
        /// `S` bit.
        const S = 1 << 4;
        /// `HWU59` bit.
        const HWU59 = 1 << 25;
        /// `HWU60` bit.
        const HWU60 = 1 << 26;
        /// `HWU61` bit.
        const HWU61 = 1 << 27;
        /// `HWU62` bit.
        const HWU62 = 1 << 28;
    }
}

impl Vtcr {
    /// Offset of the `T0SZ` field.
    pub const T0SZ_SHIFT: u32 = 0;
    /// Mask for the `T0SZ` field.
    pub const T0SZ_MASK: u32 = 0b1111;
    /// Offset of the `S` field.
    pub const S_SHIFT: u32 = 4;
    /// Offset of the `SL0` field.
    pub const SL0_SHIFT: u32 = 6;
    /// Mask for the `SL0` field.
    pub const SL0_MASK: u32 = 0b11;
    /// Offset of the `IRGN0` field.
    pub const IRGN0_SHIFT: u32 = 8;
    /// Mask for the `IRGN0` field.
    pub const IRGN0_MASK: u32 = 0b11;
    /// Offset of the `ORGN0` field.
    pub const ORGN0_SHIFT: u32 = 10;
    /// Mask for the `ORGN0` field.
    pub const ORGN0_MASK: u32 = 0b11;
    /// Offset of the `SH0` field.
    pub const SH0_SHIFT: u32 = 12;
    /// Mask for the `SH0` field.
    pub const SH0_MASK: u32 = 0b11;
    /// Offset of the `HWU59` field.
    pub const HWU59_SHIFT: u32 = 25;
    /// Offset of the `HWU60` field.
    pub const HWU60_SHIFT: u32 = 26;
    /// Offset of the `HWU61` field.
    pub const HWU61_SHIFT: u32 = 27;
    /// Offset of the `HWU62` field.
    pub const HWU62_SHIFT: u32 = 28;

    /// Returns the value of the `T0SZ` field.
    pub const fn t0sz(self) -> u8 {
        ((self.bits() >> Self::T0SZ_SHIFT) & Self::T0SZ_MASK) as u8
    }

    /// Sets the value of the `T0SZ` field.
    pub const fn set_t0sz(&mut self, value: u8) {
        let offset = Self::T0SZ_SHIFT;
        assert!(value & (Self::T0SZ_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::T0SZ_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `T0SZ` field set to the given value.
    pub const fn with_t0sz(mut self, value: u8) -> Self {
        self.set_t0sz(value);
        self
    }

    /// Returns the value of the `SL0` field.
    pub const fn sl0(self) -> u8 {
        ((self.bits() >> Self::SL0_SHIFT) & Self::SL0_MASK) as u8
    }

    /// Sets the value of the `SL0` field.
    pub const fn set_sl0(&mut self, value: u8) {
        let offset = Self::SL0_SHIFT;
        assert!(value & (Self::SL0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SL0_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `SL0` field set to the given value.
    pub const fn with_sl0(mut self, value: u8) -> Self {
        self.set_sl0(value);
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
            (self.bits() & !(Self::IRGN0_MASK << offset)) | ((value as u32) << offset),
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
            (self.bits() & !(Self::ORGN0_MASK << offset)) | ((value as u32) << offset),
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
            (self.bits() & !(Self::SH0_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns a copy with the `SH0` field set to the given value.
    pub const fn with_sh0(mut self, value: u8) -> Self {
        self.set_sh0(value);
        self
    }
}

bitflags! {
    /// `VTTBR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Vttbr: u64 {
        /// `CnP` bit.
        const CNP = 1 << 0;
    }
}

impl Vttbr {
    /// Offset of the `CnP` field.
    pub const CNP_SHIFT: u32 = 0;
    /// Offset of the `BADDR` field.
    pub const BADDR_SHIFT: u32 = 1;
    /// Mask for the `BADDR` field.
    pub const BADDR_MASK: u64 = 0b111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;
    /// Offset of the `VMID` field.
    pub const VMID_SHIFT: u32 = 48;
    /// Mask for the `VMID` field.
    pub const VMID_MASK: u64 = 0b1111_1111;

    /// Returns the value of the `BADDR` field.
    pub const fn baddr(self) -> u64 {
        (self.bits() >> Self::BADDR_SHIFT) & Self::BADDR_MASK
    }

    /// Sets the value of the `BADDR` field.
    pub const fn set_baddr(&mut self, value: u64) {
        let offset = Self::BADDR_SHIFT;
        assert!(value & Self::BADDR_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BADDR_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `BADDR` field set to the given value.
    pub const fn with_baddr(mut self, value: u64) -> Self {
        self.set_baddr(value);
        self
    }

    /// Returns the value of the `VMID` field.
    pub const fn vmid(self) -> u8 {
        ((self.bits() >> Self::VMID_SHIFT) & Self::VMID_MASK) as u8
    }

    /// Sets the value of the `VMID` field.
    pub const fn set_vmid(&mut self, value: u8) {
        let offset = Self::VMID_SHIFT;
        assert!(value & (Self::VMID_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VMID_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `VMID` field set to the given value.
    pub const fn with_vmid(mut self, value: u8) -> Self {
        self.set_vmid(value);
        self
    }
}
