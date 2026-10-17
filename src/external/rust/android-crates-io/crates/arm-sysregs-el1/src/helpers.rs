// SPDX-FileCopyrightText: Copyright The arm-sysregs Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Helper functions for EL1 system registers.

#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
use crate::accessors::{read_id_aa64isar1_el1, read_id_aa64isar2_el1, read_id_aa64isar3_el1};

/// Indicates whether FEAT_PAuth_LR is implemented.
///
/// The presence of FEAT_PAuth_LR is indicated by multiple ID register fields.
/// * ID_AA64ISAR3_EL1.PACM.
/// * ID_AA64ISAR1_EL1.APA.
/// * ID_AA64ISAR1_EL1.API.
/// * ID_AA64ISAR2_EL1.APA3.
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
pub fn is_feat_pauth_lr_present() -> bool {
    const PACM_IMPLEMENTED: u8 = 0b0001;
    const PAUTH_LR_IMPLEMENTED: u8 = 0b0110;

    let id_aa64isar1_el1 = read_id_aa64isar1_el1();

    read_id_aa64isar3_el1().pacm() >= PACM_IMPLEMENTED
        || id_aa64isar1_el1.apa() == PAUTH_LR_IMPLEMENTED
        || id_aa64isar1_el1.api() == PAUTH_LR_IMPLEMENTED
        || read_id_aa64isar2_el1().apa3() == PAUTH_LR_IMPLEMENTED
}
