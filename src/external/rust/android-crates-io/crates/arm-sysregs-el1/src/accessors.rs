// SPDX-FileCopyrightText: Copyright The arm-sysregs Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Access to Arm CPU system registers.

// This file is generated, do not edit manually.

// `unused_imports` is allowed because it is possible that not all of these macros are used in the
// generated output.
#[allow(unused_imports)]
use arm_sysregs_common::{read_sysreg, read_write_sysreg, write_sysreg};

read_write_sysreg!(actlr_el1, u64, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(afsr0_el1, u64, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(afsr1_el1, u64, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(amair_el1, u64, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(apiakeyhi_el1: s3_0_c2_c1_1, u64: crate::registers::ApiakeyhiEl1, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(apiakeylo_el1: s3_0_c2_c1_0, u64: crate::registers::ApiakeyloEl1, safe_read, crate::fake::SYSREGS);
read_sysreg!(ccsidr_el1, u64: crate::registers::CcsidrEl1, safe, crate::fake::SYSREGS);
read_sysreg!(clidr_el1, u64: crate::registers::ClidrEl1, safe, crate::fake::SYSREGS);
read_write_sysreg!(cntkctl_el1, u64: crate::registers::CntkctlEl1, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(cntps_ctl_el1, u64: crate::registers::CntpsCtlEl1, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(cntps_cval_el1, u64: crate::registers::CntpsCvalEl1, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(cntps_tval_el1, u64: crate::registers::CntpsTvalEl1, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(contextidr_el1, u64: crate::registers::ContextidrEl1, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(cpacr_el1, u64: crate::registers::CpacrEl1, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(csselr_el1, u64: crate::registers::CsselrEl1, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(disr_el1: s3_0_c12_c1_1, u64: crate::registers::DisrEl1, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(elr_el1, u64: crate::registers::ElrEl1, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(esr_el1, u64: crate::registers::EsrEl1, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(far_el1, u64: crate::registers::FarEl1, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(gcr_el1: s3_0_c1_c0_6, u64: crate::registers::GcrEl1, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(gcscre0_el1: s3_0_c2_c5_2, u64: crate::registers::Gcscre0El1, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(gcscr_el1: s3_0_c2_c5_0, u64: crate::registers::GcscrEl1, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(gcspr_el1: s3_0_c2_c5_1, u64: crate::registers::GcsprEl1, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(icc_ap0r0_el1: s3_0_c12_c8_4, u64, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(icc_ap0r1_el1: s3_0_c12_c8_5, u64, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(icc_ap0r2_el1: s3_0_c12_c8_6, u64, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(icc_ap0r3_el1: s3_0_c12_c8_7, u64, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(icc_ap1r0_el1: s3_0_c12_c9_0, u64: crate::registers::IccAp1r0El1, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(icc_ap1r1_el1: s3_0_c12_c9_1, u64, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(icc_ap1r2_el1: s3_0_c12_c9_2, u64, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(icc_ap1r3_el1: s3_0_c12_c9_3, u64, safe_read, crate::fake::SYSREGS);
write_sysreg!(icc_asgi1r_el1: s3_0_c12_c11_6, u64: crate::registers::IccAsgi1rEl1, safe, crate::fake::SYSREGS);
read_write_sysreg!(icc_bpr0_el1: s3_0_c12_c8_3, u64: crate::registers::IccBpr0El1, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(icc_bpr1_el1: s3_0_c12_c12_3, u64: crate::registers::IccBpr1El1, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(icc_ctlr_el1: s3_0_c12_c12_4, u64: crate::registers::IccCtlrEl1, safe_read, safe_write, crate::fake::SYSREGS);
write_sysreg!(icc_dir_el1: s3_0_c12_c11_1, u64: crate::registers::IccDirEl1, safe, crate::fake::SYSREGS);
write_sysreg!(icc_eoir0_el1: s3_0_c12_c8_1, u64: crate::registers::IccEoir0El1, safe, crate::fake::SYSREGS);
write_sysreg!(icc_eoir1_el1: s3_0_c12_c12_1, u64: crate::registers::IccEoir1El1, safe, crate::fake::SYSREGS);
read_sysreg!(icc_hppir0_el1: s3_0_c12_c8_2, u64: crate::registers::IccHppir0El1, safe, crate::fake::SYSREGS);
read_sysreg!(icc_hppir1_el1: s3_0_c12_c12_2, u64: crate::registers::IccHppir1El1, safe, crate::fake::SYSREGS);
read_sysreg!(icc_iar0_el1: s3_0_c12_c8_0, u64: crate::registers::IccIar0El1, safe, crate::fake::SYSREGS);
read_sysreg!(icc_iar1_el1: s3_0_c12_c12_0, u64: crate::registers::IccIar1El1, safe, crate::fake::SYSREGS);
read_write_sysreg!(icc_igrpen0_el1: s3_0_c12_c12_6, u64: crate::registers::IccIgrpen0El1, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(icc_igrpen1_el1: s3_0_c12_c12_7, u64: crate::registers::IccIgrpen1El1, safe_read, safe_write, crate::fake::SYSREGS);
read_sysreg!(icc_nmiar1_el1: s3_0_c12_c9_5, u64: crate::registers::IccNmiar1El1, safe, crate::fake::SYSREGS);
read_write_sysreg!(icc_pmr_el1: s3_0_c4_c6_0, u64: crate::registers::IccPmrEl1, safe_read, safe_write, crate::fake::SYSREGS);
read_sysreg!(icc_rpr_el1: s3_0_c12_c11_3, u64: crate::registers::IccRprEl1, safe, crate::fake::SYSREGS);
write_sysreg!(icc_sgi0r_el1: s3_0_c12_c11_7, u64: crate::registers::IccSgi0rEl1, safe, crate::fake::SYSREGS);
write_sysreg!(icc_sgi1r_el1: s3_0_c12_c11_5, u64: crate::registers::IccSgi1rEl1, safe, crate::fake::SYSREGS);
read_write_sysreg!(icc_sre_el1: s3_0_c12_c12_5, u64: crate::registers::IccSreEl1, safe_read, crate::fake::SYSREGS);
read_sysreg!(id_aa64dfr0_el1, u64: crate::registers::IdAa64dfr0El1, safe, crate::fake::SYSREGS);
read_sysreg!(id_aa64dfr1_el1, u64: crate::registers::IdAa64dfr1El1, safe, crate::fake::SYSREGS);
read_sysreg!(id_aa64isar1_el1, u64: crate::registers::IdAa64isar1El1, safe, crate::fake::SYSREGS);
read_sysreg!(id_aa64isar2_el1, u64: crate::registers::IdAa64isar2El1, safe, crate::fake::SYSREGS);
read_sysreg!(id_aa64isar3_el1, u64: crate::registers::IdAa64isar3El1, safe, crate::fake::SYSREGS);
read_sysreg!(id_aa64mmfr0_el1, u64: crate::registers::IdAa64mmfr0El1, safe, crate::fake::SYSREGS);
read_sysreg!(id_aa64mmfr1_el1, u64: crate::registers::IdAa64mmfr1El1, safe, crate::fake::SYSREGS);
read_sysreg!(id_aa64mmfr2_el1, u64: crate::registers::IdAa64mmfr2El1, safe, crate::fake::SYSREGS);
read_sysreg!(id_aa64mmfr3_el1, u64: crate::registers::IdAa64mmfr3El1, safe, crate::fake::SYSREGS);
read_sysreg!(id_aa64mmfr4_el1, u64: crate::registers::IdAa64mmfr4El1, safe, crate::fake::SYSREGS);
read_sysreg!(id_aa64pfr0_el1, u64: crate::registers::IdAa64pfr0El1, safe, crate::fake::SYSREGS);
read_sysreg!(id_aa64pfr1_el1, u64: crate::registers::IdAa64pfr1El1, safe, crate::fake::SYSREGS);
read_sysreg!(id_aa64pfr2_el1, u64: crate::registers::IdAa64pfr2El1, safe, crate::fake::SYSREGS);
read_sysreg!(id_aa64smfr0_el1: s3_0_c0_c4_5, u64: crate::registers::IdAa64smfr0El1, safe, crate::fake::SYSREGS);
read_sysreg!(isr_el1, u64: crate::registers::IsrEl1, safe, crate::fake::SYSREGS);
read_write_sysreg!(mair_el1, u64: crate::registers::MairEl1, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(mdccint_el1, u64: crate::registers::MdccintEl1, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(mdscr_el1, u64: crate::registers::MdscrEl1, safe_read, safe_write, crate::fake::SYSREGS);
read_sysreg!(midr_el1, u64: crate::registers::MidrEl1, safe, crate::fake::SYSREGS);
read_sysreg!(mpamidr_el1: s3_0_c10_c4_4, u64: crate::registers::MpamidrEl1, safe, crate::fake::SYSREGS);
read_sysreg!(mpidr_el1, u64: crate::registers::MpidrEl1, safe, crate::fake::SYSREGS);
read_write_sysreg!(par_el1, u64: crate::registers::ParEl1, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(pfar_el1: s3_0_c6_c0_5, u64: crate::registers::PfarEl1, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(pire0_el1: s3_0_c10_c2_2, u64: crate::registers::Pire0El1, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(pir_el1: s3_0_c10_c2_3, u64: crate::registers::PirEl1, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(por_el1: s3_0_c10_c2_4, u64: crate::registers::PorEl1, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(rgsr_el1: s3_0_c1_c0_5, u64: crate::registers::RgsrEl1, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(s2por_el1: s3_0_c10_c2_5, u64: crate::registers::S2porEl1, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(sctlr2_el1: s3_0_c1_c0_3, u64: crate::registers::Sctlr2El1, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(sctlr_el1, u64: crate::registers::SctlrEl1, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(spsr_el1, u64: crate::registers::SpsrEl1, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(sp_el1, u64: crate::registers::SpEl1, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(tcr2_el1: s3_0_c2_c0_3, u64: crate::registers::Tcr2El1, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(tcr_el1, u64: crate::registers::TcrEl1, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(tfsre0_el1: s3_0_c5_c6_1, u64: crate::registers::Tfsre0El1, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(tfsr_el1: s3_0_c5_c6_0, u64: crate::registers::TfsrEl1, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(tpidr_el1, u64: crate::registers::TpidrEl1, safe_read, crate::fake::SYSREGS);
read_write_sysreg! {
    /// # Safety
    ///
    /// The base address must point to a valid and properly aligned translation table.
    ttbr0_el1, u64: crate::registers::Ttbr0El1, safe_read, crate::fake::SYSREGS
}
read_write_sysreg! {
    /// # Safety
    ///
    /// The base address must point to a valid and properly aligned translation table.
    ttbr1_el1, u64: crate::registers::Ttbr1El1, safe_read, crate::fake::SYSREGS
}
read_write_sysreg! {
    /// # Safety
    ///
    /// The base address must point to a valid exception vector.
    vbar_el1, u64: crate::registers::VbarEl1, safe_read, crate::fake::SYSREGS
}
