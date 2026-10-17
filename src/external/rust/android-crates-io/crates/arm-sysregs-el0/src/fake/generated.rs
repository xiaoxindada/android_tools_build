// SPDX-FileCopyrightText: Copyright The arm-sysregs Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

// This file is generated, do not edit manually.

use crate::registers::{
    AmcfgrEl0, Amcg1idrEl0, AmcgcrEl0, Amcntenclr0El0, Amcntenclr1El0, Amcntenset0El0,
    Amcntenset1El0, AmcrEl0, Amevcntr00El0, Amevcntr01El0, Amevcntr02El0, Amevcntr03El0,
    Amevcntr10El0, Amevcntr11El0, Amevcntr12El0, Amevcntr13El0, Amevcntr14El0, Amevcntr15El0,
    Amevcntr16El0, Amevcntr17El0, Amevcntr18El0, Amevcntr19El0, Amevcntr110El0, Amevcntr111El0,
    Amevcntr112El0, Amevcntr113El0, Amevcntr114El0, Amevcntr115El0, Amevtyper00El0, Amevtyper01El0,
    Amevtyper02El0, Amevtyper03El0, AmuserenrEl0, CntfrqEl0, CntpCtlEl0, CntpCvalEl0, CntpTvalEl0,
    CntpctEl0, CntpctssEl0, CntvCtlEl0, CntvCvalEl0, CntvTvalEl0, CntvctEl0, CntvctssEl0, CtrEl0,
    Currentel, Daif, Dit, Fpcr, Fpmr, Fpsr, GcsprEl0, PmcrEl0, PorEl0, Svcr, TpidrEl0, TpidrroEl0,
};

/// A set of fake system registers.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub struct SystemRegisters {
    /// Fake value for the `AMCFGR_EL0` system register.
    pub amcfgr_el0: AmcfgrEl0,
    /// Fake value for the `AMCG1IDR_EL0` system register.
    pub amcg1idr_el0: Amcg1idrEl0,
    /// Fake value for the `AMCGCR_EL0` system register.
    pub amcgcr_el0: AmcgcrEl0,
    /// Fake value for the `AMCNTENCLR0_EL0` system register.
    pub amcntenclr0_el0: Amcntenclr0El0,
    /// Fake value for the `AMCNTENCLR1_EL0` system register.
    pub amcntenclr1_el0: Amcntenclr1El0,
    /// Fake value for the `AMCNTENSET0_EL0` system register.
    pub amcntenset0_el0: Amcntenset0El0,
    /// Fake value for the `AMCNTENSET1_EL0` system register.
    pub amcntenset1_el0: Amcntenset1El0,
    /// Fake value for the `AMCR_EL0` system register.
    pub amcr_el0: AmcrEl0,
    /// Fake value for the `AMEVCNTR00_EL0` system register.
    pub amevcntr00_el0: Amevcntr00El0,
    /// Fake value for the `AMEVCNTR01_EL0` system register.
    pub amevcntr01_el0: Amevcntr01El0,
    /// Fake value for the `AMEVCNTR02_EL0` system register.
    pub amevcntr02_el0: Amevcntr02El0,
    /// Fake value for the `AMEVCNTR03_EL0` system register.
    pub amevcntr03_el0: Amevcntr03El0,
    /// Fake value for the `AMEVCNTR10_EL0` system register.
    pub amevcntr10_el0: Amevcntr10El0,
    /// Fake value for the `AMEVCNTR110_EL0` system register.
    pub amevcntr110_el0: Amevcntr110El0,
    /// Fake value for the `AMEVCNTR111_EL0` system register.
    pub amevcntr111_el0: Amevcntr111El0,
    /// Fake value for the `AMEVCNTR112_EL0` system register.
    pub amevcntr112_el0: Amevcntr112El0,
    /// Fake value for the `AMEVCNTR113_EL0` system register.
    pub amevcntr113_el0: Amevcntr113El0,
    /// Fake value for the `AMEVCNTR114_EL0` system register.
    pub amevcntr114_el0: Amevcntr114El0,
    /// Fake value for the `AMEVCNTR115_EL0` system register.
    pub amevcntr115_el0: Amevcntr115El0,
    /// Fake value for the `AMEVCNTR11_EL0` system register.
    pub amevcntr11_el0: Amevcntr11El0,
    /// Fake value for the `AMEVCNTR12_EL0` system register.
    pub amevcntr12_el0: Amevcntr12El0,
    /// Fake value for the `AMEVCNTR13_EL0` system register.
    pub amevcntr13_el0: Amevcntr13El0,
    /// Fake value for the `AMEVCNTR14_EL0` system register.
    pub amevcntr14_el0: Amevcntr14El0,
    /// Fake value for the `AMEVCNTR15_EL0` system register.
    pub amevcntr15_el0: Amevcntr15El0,
    /// Fake value for the `AMEVCNTR16_EL0` system register.
    pub amevcntr16_el0: Amevcntr16El0,
    /// Fake value for the `AMEVCNTR17_EL0` system register.
    pub amevcntr17_el0: Amevcntr17El0,
    /// Fake value for the `AMEVCNTR18_EL0` system register.
    pub amevcntr18_el0: Amevcntr18El0,
    /// Fake value for the `AMEVCNTR19_EL0` system register.
    pub amevcntr19_el0: Amevcntr19El0,
    /// Fake value for the `AMEVTYPER00_EL0` system register.
    pub amevtyper00_el0: Amevtyper00El0,
    /// Fake value for the `AMEVTYPER01_EL0` system register.
    pub amevtyper01_el0: Amevtyper01El0,
    /// Fake value for the `AMEVTYPER02_EL0` system register.
    pub amevtyper02_el0: Amevtyper02El0,
    /// Fake value for the `AMEVTYPER03_EL0` system register.
    pub amevtyper03_el0: Amevtyper03El0,
    /// Fake value for the `AMUSERENR_EL0` system register.
    pub amuserenr_el0: AmuserenrEl0,
    /// Fake value for the `CNTFRQ_EL0` system register.
    pub cntfrq_el0: CntfrqEl0,
    /// Fake value for the `CNTPCTSS_EL0` system register.
    pub cntpctss_el0: CntpctssEl0,
    /// Fake value for the `CNTPCT_EL0` system register.
    pub cntpct_el0: CntpctEl0,
    /// Fake value for the `CNTP_CTL_EL0` system register.
    pub cntp_ctl_el0: CntpCtlEl0,
    /// Fake value for the `CNTP_CVAL_EL0` system register.
    pub cntp_cval_el0: CntpCvalEl0,
    /// Fake value for the `CNTP_TVAL_EL0` system register.
    pub cntp_tval_el0: CntpTvalEl0,
    /// Fake value for the `CNTVCTSS_EL0` system register.
    pub cntvctss_el0: CntvctssEl0,
    /// Fake value for the `CNTVCT_EL0` system register.
    pub cntvct_el0: CntvctEl0,
    /// Fake value for the `CNTV_CTL_EL0` system register.
    pub cntv_ctl_el0: CntvCtlEl0,
    /// Fake value for the `CNTV_CVAL_EL0` system register.
    pub cntv_cval_el0: CntvCvalEl0,
    /// Fake value for the `CNTV_TVAL_EL0` system register.
    pub cntv_tval_el0: CntvTvalEl0,
    /// Fake value for the `CTR_EL0` system register.
    pub ctr_el0: CtrEl0,
    /// Fake value for the `CurrentEL` system register.
    pub currentel: Currentel,
    /// Fake value for the `DAIF` system register.
    pub daif: Daif,
    /// Fake value for the `DIT` system register.
    pub dit: Dit,
    /// Fake value for the `FPCR` system register.
    pub fpcr: Fpcr,
    /// Fake value for the `FPMR` system register.
    pub fpmr: Fpmr,
    /// Fake value for the `FPSR` system register.
    pub fpsr: Fpsr,
    /// Fake value for the `GCSPR_EL0` system register.
    pub gcspr_el0: GcsprEl0,
    /// Fake value for the `PMCR_EL0` system register.
    pub pmcr_el0: PmcrEl0,
    /// Fake value for the `POR_EL0` system register.
    pub por_el0: PorEl0,
    /// Fake value for the `SVCR` system register.
    pub svcr: Svcr,
    /// Fake value for the `TPIDRRO_EL0` system register.
    pub tpidrro_el0: TpidrroEl0,
    /// Fake value for the `TPIDR_EL0` system register.
    pub tpidr_el0: TpidrEl0,
}

impl SystemRegisters {
    pub(crate) const fn new() -> Self {
        Self {
            amcfgr_el0: AmcfgrEl0::empty(),
            amcg1idr_el0: Amcg1idrEl0::empty(),
            amcgcr_el0: AmcgcrEl0::empty(),
            amcntenclr0_el0: Amcntenclr0El0::empty(),
            amcntenclr1_el0: Amcntenclr1El0::empty(),
            amcntenset0_el0: Amcntenset0El0::empty(),
            amcntenset1_el0: Amcntenset1El0::empty(),
            amcr_el0: AmcrEl0::empty(),
            amevcntr00_el0: Amevcntr00El0::empty(),
            amevcntr01_el0: Amevcntr01El0::empty(),
            amevcntr02_el0: Amevcntr02El0::empty(),
            amevcntr03_el0: Amevcntr03El0::empty(),
            amevcntr10_el0: Amevcntr10El0::empty(),
            amevcntr110_el0: Amevcntr110El0::empty(),
            amevcntr111_el0: Amevcntr111El0::empty(),
            amevcntr112_el0: Amevcntr112El0::empty(),
            amevcntr113_el0: Amevcntr113El0::empty(),
            amevcntr114_el0: Amevcntr114El0::empty(),
            amevcntr115_el0: Amevcntr115El0::empty(),
            amevcntr11_el0: Amevcntr11El0::empty(),
            amevcntr12_el0: Amevcntr12El0::empty(),
            amevcntr13_el0: Amevcntr13El0::empty(),
            amevcntr14_el0: Amevcntr14El0::empty(),
            amevcntr15_el0: Amevcntr15El0::empty(),
            amevcntr16_el0: Amevcntr16El0::empty(),
            amevcntr17_el0: Amevcntr17El0::empty(),
            amevcntr18_el0: Amevcntr18El0::empty(),
            amevcntr19_el0: Amevcntr19El0::empty(),
            amevtyper00_el0: Amevtyper00El0::empty(),
            amevtyper01_el0: Amevtyper01El0::empty(),
            amevtyper02_el0: Amevtyper02El0::empty(),
            amevtyper03_el0: Amevtyper03El0::empty(),
            amuserenr_el0: AmuserenrEl0::empty(),
            cntfrq_el0: CntfrqEl0::empty(),
            cntpctss_el0: CntpctssEl0::empty(),
            cntpct_el0: CntpctEl0::empty(),
            cntp_ctl_el0: CntpCtlEl0::empty(),
            cntp_cval_el0: CntpCvalEl0::empty(),
            cntp_tval_el0: CntpTvalEl0::empty(),
            cntvctss_el0: CntvctssEl0::empty(),
            cntvct_el0: CntvctEl0::empty(),
            cntv_ctl_el0: CntvCtlEl0::empty(),
            cntv_cval_el0: CntvCvalEl0::empty(),
            cntv_tval_el0: CntvTvalEl0::empty(),
            ctr_el0: CtrEl0::empty(),
            currentel: Currentel::empty(),
            daif: Daif::empty(),
            dit: Dit::empty(),
            fpcr: Fpcr::empty(),
            fpmr: Fpmr::empty(),
            fpsr: Fpsr::empty(),
            gcspr_el0: GcsprEl0::empty(),
            pmcr_el0: PmcrEl0::empty(),
            por_el0: PorEl0::empty(),
            svcr: Svcr::empty(),
            tpidrro_el0: TpidrroEl0::empty(),
            tpidr_el0: TpidrEl0::empty(),
        }
    }
}
