// SPDX-FileCopyrightText: Copyright The arm-sysregs Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Access to Arm A-profile EL2 system registers.

#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
pub use arm_sysregs_el2::accessors;
#[cfg(any(test, feature = "fakes"))]
pub use arm_sysregs_el2::fake;
pub use arm_sysregs_el2::helpers;
pub use arm_sysregs_el2::registers;
