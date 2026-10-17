// SPDX-FileCopyrightText: Copyright The arm-sysregs Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Access to Arm A-profile CPU system registers.

#![cfg_attr(not(any(test, feature = "fakes")), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod aarch32;
pub mod el0;
#[cfg(feature = "el1")]
pub mod el1;
#[cfg(feature = "el2")]
pub mod el2;
#[cfg(feature = "el3")]
pub mod el3;
mod version;

pub use arm_sysregs_common::types;
#[cfg(any(test, feature = "fakes", target_arch = "aarch64", target_arch = "arm"))]
pub use arm_sysregs_common::{read_sysreg, read_write_sysreg, write_sysreg};
pub use version::AARCHMRS_VERSION;
