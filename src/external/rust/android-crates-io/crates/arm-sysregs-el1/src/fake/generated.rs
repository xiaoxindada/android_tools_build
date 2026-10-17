// SPDX-FileCopyrightText: Copyright The arm-sysregs Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

// This file is generated, do not edit manually.

use crate::registers::{
    ApiakeyhiEl1, ApiakeyloEl1, CcsidrEl1, ClidrEl1, CntkctlEl1, CntpsCtlEl1, CntpsCvalEl1,
    CntpsTvalEl1, ContextidrEl1, CpacrEl1, CsselrEl1, DisrEl1, ElrEl1, EsrEl1, FarEl1, GcrEl1,
    GcscrEl1, Gcscre0El1, GcsprEl1, IccAp1r0El1, IccAsgi1rEl1, IccBpr0El1, IccBpr1El1, IccCtlrEl1,
    IccDirEl1, IccEoir0El1, IccEoir1El1, IccHppir0El1, IccHppir1El1, IccIar0El1, IccIar1El1,
    IccIgrpen0El1, IccIgrpen1El1, IccNmiar1El1, IccPmrEl1, IccRprEl1, IccSgi0rEl1, IccSgi1rEl1,
    IccSreEl1, IdAa64dfr0El1, IdAa64dfr1El1, IdAa64isar1El1, IdAa64isar2El1, IdAa64isar3El1,
    IdAa64mmfr0El1, IdAa64mmfr1El1, IdAa64mmfr2El1, IdAa64mmfr3El1, IdAa64mmfr4El1, IdAa64pfr0El1,
    IdAa64pfr1El1, IdAa64pfr2El1, IdAa64smfr0El1, IsrEl1, MairEl1, MdccintEl1, MdscrEl1, MidrEl1,
    MpamidrEl1, MpidrEl1, ParEl1, PfarEl1, PirEl1, Pire0El1, PorEl1, RgsrEl1, S2porEl1, Sctlr2El1,
    SctlrEl1, SpEl1, SpsrEl1, Tcr2El1, TcrEl1, TfsrEl1, Tfsre0El1, TpidrEl1, Ttbr0El1, Ttbr1El1,
    VbarEl1,
};

/// A set of fake system registers.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub struct SystemRegisters {
    /// Fake value for the `ACTLR_EL1` system register.
    pub actlr_el1: u64,
    /// Fake value for the `AFSR0_EL1` system register.
    pub afsr0_el1: u64,
    /// Fake value for the `AFSR1_EL1` system register.
    pub afsr1_el1: u64,
    /// Fake value for the `AMAIR_EL1` system register.
    pub amair_el1: u64,
    /// Fake value for the `APIAKeyHi_EL1` system register.
    pub apiakeyhi_el1: ApiakeyhiEl1,
    /// Fake value for the `APIAKeyLo_EL1` system register.
    pub apiakeylo_el1: ApiakeyloEl1,
    /// Fake value for the `CCSIDR_EL1` system register.
    pub ccsidr_el1: CcsidrEl1,
    /// Fake value for the `CLIDR_EL1` system register.
    pub clidr_el1: ClidrEl1,
    /// Fake value for the `CNTKCTL_EL1` system register.
    pub cntkctl_el1: CntkctlEl1,
    /// Fake value for the `CNTPS_CTL_EL1` system register.
    pub cntps_ctl_el1: CntpsCtlEl1,
    /// Fake value for the `CNTPS_CVAL_EL1` system register.
    pub cntps_cval_el1: CntpsCvalEl1,
    /// Fake value for the `CNTPS_TVAL_EL1` system register.
    pub cntps_tval_el1: CntpsTvalEl1,
    /// Fake value for the `CONTEXTIDR_EL1` system register.
    pub contextidr_el1: ContextidrEl1,
    /// Fake value for the `CPACR_EL1` system register.
    pub cpacr_el1: CpacrEl1,
    /// Fake value for the `CSSELR_EL1` system register.
    pub csselr_el1: CsselrEl1,
    /// Fake value for the `DISR_EL1` system register.
    pub disr_el1: DisrEl1,
    /// Fake value for the `ELR_EL1` system register.
    pub elr_el1: ElrEl1,
    /// Fake value for the `ESR_EL1` system register.
    pub esr_el1: EsrEl1,
    /// Fake value for the `FAR_EL1` system register.
    pub far_el1: FarEl1,
    /// Fake value for the `GCR_EL1` system register.
    pub gcr_el1: GcrEl1,
    /// Fake value for the `GCSCRE0_EL1` system register.
    pub gcscre0_el1: Gcscre0El1,
    /// Fake value for the `GCSCR_EL1` system register.
    pub gcscr_el1: GcscrEl1,
    /// Fake value for the `GCSPR_EL1` system register.
    pub gcspr_el1: GcsprEl1,
    /// Fake value for the `ICC_AP0R0_EL1` system register.
    pub icc_ap0r0_el1: u64,
    /// Fake value for the `ICC_AP0R1_EL1` system register.
    pub icc_ap0r1_el1: u64,
    /// Fake value for the `ICC_AP0R2_EL1` system register.
    pub icc_ap0r2_el1: u64,
    /// Fake value for the `ICC_AP0R3_EL1` system register.
    pub icc_ap0r3_el1: u64,
    /// Fake value for the `ICC_AP1R0_EL1` system register.
    pub icc_ap1r0_el1: IccAp1r0El1,
    /// Fake value for the `ICC_AP1R1_EL1` system register.
    pub icc_ap1r1_el1: u64,
    /// Fake value for the `ICC_AP1R2_EL1` system register.
    pub icc_ap1r2_el1: u64,
    /// Fake value for the `ICC_AP1R3_EL1` system register.
    pub icc_ap1r3_el1: u64,
    /// Fake value for the `ICC_ASGI1R_EL1` system register.
    pub icc_asgi1r_el1: IccAsgi1rEl1,
    /// Fake value for the `ICC_BPR0_EL1` system register.
    pub icc_bpr0_el1: IccBpr0El1,
    /// Fake value for the `ICC_BPR1_EL1` system register.
    pub icc_bpr1_el1: IccBpr1El1,
    /// Fake value for the `ICC_CTLR_EL1` system register.
    pub icc_ctlr_el1: IccCtlrEl1,
    /// Fake value for the `ICC_DIR_EL1` system register.
    pub icc_dir_el1: IccDirEl1,
    /// Fake value for the `ICC_EOIR0_EL1` system register.
    pub icc_eoir0_el1: IccEoir0El1,
    /// Fake value for the `ICC_EOIR1_EL1` system register.
    pub icc_eoir1_el1: IccEoir1El1,
    /// Fake value for the `ICC_HPPIR0_EL1` system register.
    pub icc_hppir0_el1: IccHppir0El1,
    /// Fake value for the `ICC_HPPIR1_EL1` system register.
    pub icc_hppir1_el1: IccHppir1El1,
    /// Fake value for the `ICC_IAR0_EL1` system register.
    pub icc_iar0_el1: IccIar0El1,
    /// Fake value for the `ICC_IAR1_EL1` system register.
    pub icc_iar1_el1: IccIar1El1,
    /// Fake value for the `ICC_IGRPEN0_EL1` system register.
    pub icc_igrpen0_el1: IccIgrpen0El1,
    /// Fake value for the `ICC_IGRPEN1_EL1` system register.
    pub icc_igrpen1_el1: IccIgrpen1El1,
    /// Fake value for the `ICC_NMIAR1_EL1` system register.
    pub icc_nmiar1_el1: IccNmiar1El1,
    /// Fake value for the `ICC_PMR_EL1` system register.
    pub icc_pmr_el1: IccPmrEl1,
    /// Fake value for the `ICC_RPR_EL1` system register.
    pub icc_rpr_el1: IccRprEl1,
    /// Fake value for the `ICC_SGI0R_EL1` system register.
    pub icc_sgi0r_el1: IccSgi0rEl1,
    /// Fake value for the `ICC_SGI1R_EL1` system register.
    pub icc_sgi1r_el1: IccSgi1rEl1,
    /// Fake value for the `ICC_SRE_EL1` system register.
    pub icc_sre_el1: IccSreEl1,
    /// Fake value for the `ID_AA64DFR0_EL1` system register.
    pub id_aa64dfr0_el1: IdAa64dfr0El1,
    /// Fake value for the `ID_AA64DFR1_EL1` system register.
    pub id_aa64dfr1_el1: IdAa64dfr1El1,
    /// Fake value for the `ID_AA64ISAR1_EL1` system register.
    pub id_aa64isar1_el1: IdAa64isar1El1,
    /// Fake value for the `ID_AA64ISAR2_EL1` system register.
    pub id_aa64isar2_el1: IdAa64isar2El1,
    /// Fake value for the `ID_AA64ISAR3_EL1` system register.
    pub id_aa64isar3_el1: IdAa64isar3El1,
    /// Fake value for the `ID_AA64MMFR0_EL1` system register.
    pub id_aa64mmfr0_el1: IdAa64mmfr0El1,
    /// Fake value for the `ID_AA64MMFR1_EL1` system register.
    pub id_aa64mmfr1_el1: IdAa64mmfr1El1,
    /// Fake value for the `ID_AA64MMFR2_EL1` system register.
    pub id_aa64mmfr2_el1: IdAa64mmfr2El1,
    /// Fake value for the `ID_AA64MMFR3_EL1` system register.
    pub id_aa64mmfr3_el1: IdAa64mmfr3El1,
    /// Fake value for the `ID_AA64MMFR4_EL1` system register.
    pub id_aa64mmfr4_el1: IdAa64mmfr4El1,
    /// Fake value for the `ID_AA64PFR0_EL1` system register.
    pub id_aa64pfr0_el1: IdAa64pfr0El1,
    /// Fake value for the `ID_AA64PFR1_EL1` system register.
    pub id_aa64pfr1_el1: IdAa64pfr1El1,
    /// Fake value for the `ID_AA64PFR2_EL1` system register.
    pub id_aa64pfr2_el1: IdAa64pfr2El1,
    /// Fake value for the `ID_AA64SMFR0_EL1` system register.
    pub id_aa64smfr0_el1: IdAa64smfr0El1,
    /// Fake value for the `ISR_EL1` system register.
    pub isr_el1: IsrEl1,
    /// Fake value for the `MAIR_EL1` system register.
    pub mair_el1: MairEl1,
    /// Fake value for the `MDCCINT_EL1` system register.
    pub mdccint_el1: MdccintEl1,
    /// Fake value for the `MDSCR_EL1` system register.
    pub mdscr_el1: MdscrEl1,
    /// Fake value for the `MIDR_EL1` system register.
    pub midr_el1: MidrEl1,
    /// Fake value for the `MPAMIDR_EL1` system register.
    pub mpamidr_el1: MpamidrEl1,
    /// Fake value for the `MPIDR_EL1` system register.
    pub mpidr_el1: MpidrEl1,
    /// Fake value for the `PAR_EL1` system register.
    pub par_el1: ParEl1,
    /// Fake value for the `PFAR_EL1` system register.
    pub pfar_el1: PfarEl1,
    /// Fake value for the `PIRE0_EL1` system register.
    pub pire0_el1: Pire0El1,
    /// Fake value for the `PIR_EL1` system register.
    pub pir_el1: PirEl1,
    /// Fake value for the `POR_EL1` system register.
    pub por_el1: PorEl1,
    /// Fake value for the `RGSR_EL1` system register.
    pub rgsr_el1: RgsrEl1,
    /// Fake value for the `S2POR_EL1` system register.
    pub s2por_el1: S2porEl1,
    /// Fake value for the `SCTLR2_EL1` system register.
    pub sctlr2_el1: Sctlr2El1,
    /// Fake value for the `SCTLR_EL1` system register.
    pub sctlr_el1: SctlrEl1,
    /// Fake value for the `SPSR_EL1` system register.
    pub spsr_el1: SpsrEl1,
    /// Fake value for the `SP_EL1` system register.
    pub sp_el1: SpEl1,
    /// Fake value for the `TCR2_EL1` system register.
    pub tcr2_el1: Tcr2El1,
    /// Fake value for the `TCR_EL1` system register.
    pub tcr_el1: TcrEl1,
    /// Fake value for the `TFSRE0_EL1` system register.
    pub tfsre0_el1: Tfsre0El1,
    /// Fake value for the `TFSR_EL1` system register.
    pub tfsr_el1: TfsrEl1,
    /// Fake value for the `TPIDR_EL1` system register.
    pub tpidr_el1: TpidrEl1,
    /// Fake value for the `TTBR0_EL1` system register.
    pub ttbr0_el1: Ttbr0El1,
    /// Fake value for the `TTBR1_EL1` system register.
    pub ttbr1_el1: Ttbr1El1,
    /// Fake value for the `VBAR_EL1` system register.
    pub vbar_el1: VbarEl1,
}

impl SystemRegisters {
    pub(crate) const fn new() -> Self {
        Self {
            actlr_el1: 0,
            afsr0_el1: 0,
            afsr1_el1: 0,
            amair_el1: 0,
            apiakeyhi_el1: ApiakeyhiEl1::empty(),
            apiakeylo_el1: ApiakeyloEl1::empty(),
            ccsidr_el1: CcsidrEl1::empty(),
            clidr_el1: ClidrEl1::empty(),
            cntkctl_el1: CntkctlEl1::empty(),
            cntps_ctl_el1: CntpsCtlEl1::empty(),
            cntps_cval_el1: CntpsCvalEl1::empty(),
            cntps_tval_el1: CntpsTvalEl1::empty(),
            contextidr_el1: ContextidrEl1::empty(),
            cpacr_el1: CpacrEl1::empty(),
            csselr_el1: CsselrEl1::empty(),
            disr_el1: DisrEl1::empty(),
            elr_el1: ElrEl1::empty(),
            esr_el1: EsrEl1::empty(),
            far_el1: FarEl1::empty(),
            gcr_el1: GcrEl1::empty(),
            gcscre0_el1: Gcscre0El1::empty(),
            gcscr_el1: GcscrEl1::empty(),
            gcspr_el1: GcsprEl1::empty(),
            icc_ap0r0_el1: 0,
            icc_ap0r1_el1: 0,
            icc_ap0r2_el1: 0,
            icc_ap0r3_el1: 0,
            icc_ap1r0_el1: IccAp1r0El1::empty(),
            icc_ap1r1_el1: 0,
            icc_ap1r2_el1: 0,
            icc_ap1r3_el1: 0,
            icc_asgi1r_el1: IccAsgi1rEl1::empty(),
            icc_bpr0_el1: IccBpr0El1::empty(),
            icc_bpr1_el1: IccBpr1El1::empty(),
            icc_ctlr_el1: IccCtlrEl1::empty(),
            icc_dir_el1: IccDirEl1::empty(),
            icc_eoir0_el1: IccEoir0El1::empty(),
            icc_eoir1_el1: IccEoir1El1::empty(),
            icc_hppir0_el1: IccHppir0El1::empty(),
            icc_hppir1_el1: IccHppir1El1::empty(),
            icc_iar0_el1: IccIar0El1::empty(),
            icc_iar1_el1: IccIar1El1::empty(),
            icc_igrpen0_el1: IccIgrpen0El1::empty(),
            icc_igrpen1_el1: IccIgrpen1El1::empty(),
            icc_nmiar1_el1: IccNmiar1El1::empty(),
            icc_pmr_el1: IccPmrEl1::empty(),
            icc_rpr_el1: IccRprEl1::empty(),
            icc_sgi0r_el1: IccSgi0rEl1::empty(),
            icc_sgi1r_el1: IccSgi1rEl1::empty(),
            icc_sre_el1: IccSreEl1::empty(),
            id_aa64dfr0_el1: IdAa64dfr0El1::empty(),
            id_aa64dfr1_el1: IdAa64dfr1El1::empty(),
            id_aa64isar1_el1: IdAa64isar1El1::empty(),
            id_aa64isar2_el1: IdAa64isar2El1::empty(),
            id_aa64isar3_el1: IdAa64isar3El1::empty(),
            id_aa64mmfr0_el1: IdAa64mmfr0El1::empty(),
            id_aa64mmfr1_el1: IdAa64mmfr1El1::empty(),
            id_aa64mmfr2_el1: IdAa64mmfr2El1::empty(),
            id_aa64mmfr3_el1: IdAa64mmfr3El1::empty(),
            id_aa64mmfr4_el1: IdAa64mmfr4El1::empty(),
            id_aa64pfr0_el1: IdAa64pfr0El1::empty(),
            id_aa64pfr1_el1: IdAa64pfr1El1::empty(),
            id_aa64pfr2_el1: IdAa64pfr2El1::empty(),
            id_aa64smfr0_el1: IdAa64smfr0El1::empty(),
            isr_el1: IsrEl1::empty(),
            mair_el1: MairEl1::empty(),
            mdccint_el1: MdccintEl1::empty(),
            mdscr_el1: MdscrEl1::empty(),
            midr_el1: MidrEl1::empty(),
            mpamidr_el1: MpamidrEl1::empty(),
            mpidr_el1: MpidrEl1::empty(),
            par_el1: ParEl1::empty(),
            pfar_el1: PfarEl1::empty(),
            pire0_el1: Pire0El1::empty(),
            pir_el1: PirEl1::empty(),
            por_el1: PorEl1::empty(),
            rgsr_el1: RgsrEl1::empty(),
            s2por_el1: S2porEl1::empty(),
            sctlr2_el1: Sctlr2El1::empty(),
            sctlr_el1: SctlrEl1::empty(),
            spsr_el1: SpsrEl1::empty(),
            sp_el1: SpEl1::empty(),
            tcr2_el1: Tcr2El1::empty(),
            tcr_el1: TcrEl1::empty(),
            tfsre0_el1: Tfsre0El1::empty(),
            tfsr_el1: TfsrEl1::empty(),
            tpidr_el1: TpidrEl1::empty(),
            ttbr0_el1: Ttbr0El1::empty(),
            ttbr1_el1: Ttbr1El1::empty(),
            vbar_el1: VbarEl1::empty(),
        }
    }
}
