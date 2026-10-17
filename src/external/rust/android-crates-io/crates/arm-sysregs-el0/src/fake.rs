// SPDX-FileCopyrightText: Copyright The arm-sysregs Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Fake implementations of Aarch64 EL0 system register getters and setters for unit tests.

pub use self::generated::SystemRegisters;
use std::sync::Mutex;

mod generated;

/// Values of fake system registers.
pub static SYSREGS: Mutex<SystemRegisters> = Mutex::new(SystemRegisters::new());

impl SystemRegisters {
    /// Resets the fake system registers to their initial state.
    pub fn reset(&mut self) {
        *self = Self::new();
    }
}

impl Default for SystemRegisters {
    fn default() -> Self {
        Self::new()
    }
}
