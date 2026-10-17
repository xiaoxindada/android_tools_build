// SPDX-FileCopyrightText: Copyright The arm-sysregs Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Types used by the system register crates.

use num_enum::{IntoPrimitive, TryFromPrimitive};

/// An AArch64 exception level.
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum ExceptionLevel {
    /// Exception level 0.
    El0 = 0,
    /// Exception level 1.
    El1 = 1,
    /// Exception level 2.
    El2 = 2,
    /// Exception level 3.
    El3 = 3,
}

/// Values for SPSEL.
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum StackPointer {
    /// Use SP_EL0.
    El0 = 0,
    /// Use SP_EL1, SP_EL2 or SP_EL3 according to the current exception level.
    ElX = 1,
}

/// Allowed Shareability attributes.
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum Shareability {
    /// Non-shareable.
    Non = 0b00,
    /// Outer-shareable.
    Outer = 0b10,
    /// Inner-shareable.
    Inner = 0b11,
}

/// Allowed Cacheability attributes.
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum Cacheability {
    /// Normal memory, Non-cacheable.
    Non = 0b00,
    /// Normal memory, Write-Back Read-Allocate Write-Allocate Cacheable.
    WriteBackAllocate = 0b01,
    /// Normal memory, Write-Through Read-Allocate No Write-Allocate Cacheable.
    WriteThrough = 0b10,
    /// Normal memory, Write-Back Read-Allocate No Write-Allocate Cacheable.
    WriteBackNoAllocate = 0b11,
}

/// Cache type enum.
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum CacheType {
    /// No cache.
    NoCache = 0b000,
    /// Instruction cache only.
    InstructionOnly = 0b001,
    /// Data cache only.
    DataOnly = 0b010,
    /// Separate instruction and data caches.
    SeparateInstructionAndData = 0b011,
    /// Unified cache.
    Unified = 0b100,
}

/// Wrapper type for describing cache level in a human readable format, e.g. L3 cache = `CacheLevel(3)`
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CacheLevel(pub u8);

impl CacheLevel {
    /// Creates new instance.
    pub fn new(level: u8) -> Self {
        assert!((1..8).contains(&level));
        Self(level)
    }

    /// Returns the level value.
    pub fn level(&self) -> u8 {
        self.0
    }
}

impl From<CacheLevel> for u32 {
    fn from(value: CacheLevel) -> Self {
        (value.0 - 1).into()
    }
}

impl From<CacheLevel> for u64 {
    fn from(value: CacheLevel) -> Self {
        u32::from(value).into()
    }
}
