// SPDX-FileCopyrightText: Copyright The arm-sysregs Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Access to Arm A-profile EL1 system registers.

#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
pub use arm_sysregs_el1::accessors;
#[cfg(any(test, feature = "fakes"))]
pub use arm_sysregs_el1::fake;
pub use arm_sysregs_el1::helpers;
pub use arm_sysregs_el1::registers;
