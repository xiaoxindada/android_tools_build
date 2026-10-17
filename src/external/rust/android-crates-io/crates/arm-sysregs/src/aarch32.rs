// SPDX-FileCopyrightText: Copyright The arm-sysregs Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Access to Arm A-profile AArch32 system registers.

#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
pub use arm_sysregs_aarch32::accessors;
#[cfg(any(test, feature = "fakes"))]
pub use arm_sysregs_aarch32::fake;
pub use arm_sysregs_aarch32::helpers;
pub use arm_sysregs_aarch32::registers;
