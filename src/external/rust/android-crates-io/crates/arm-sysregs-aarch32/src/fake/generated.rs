// SPDX-FileCopyrightText: Copyright The arm-sysregs Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

// This file is generated, do not edit manually.

#[cfg(feature = "el2")]
use crate::registers::ElrHyp;
use crate::registers::{
    Amcfgr, Amcgcr, Amcntenclr0, Amcntenclr1, Amcntenset0, Amcntenset1, Amcr, Amevcntr00,
    Amevcntr01, Amevcntr02, Amevcntr03, Amevtyper00, Amevtyper01, Amevtyper02, Amevtyper03,
    Amevtyper10, Amevtyper11, Amevtyper12, Amevtyper13, Amevtyper14, Amevtyper15, Amevtyper16,
    Amevtyper17, Amevtyper18, Amevtyper19, Amevtyper110, Amevtyper111, Amevtyper112, Amevtyper113,
    Amevtyper114, Amevtyper115, Amuserenr, Ccsidr, Ccsidr2, Clidr, Cntfrq, Cnthctl, CnthpCtl,
    CnthpCval, CnthpTval, CnthpsCtl, CnthpsCval, CnthpsTval, CnthvCtl, CnthvCval, CnthvTval,
    CnthvsCtl, CnthvsCval, CnthvsTval, Cntkctl, CntpCtl, CntpCval, CntpTval, Cntpct, Cntpctss,
    CntvCtl, CntvCval, CntvTval, Cntvct, Cntvctss, Cntvoff, Contextidr, Cpacr, Csselr, Ctr, Dacr,
    Dbgauthstatus, Dbgclaimclr, Dbgclaimset, Dbgdccint, Dbgdevid, Dbgdevid1, Dbgdidr, Dbgdrar,
    Dbgdscrext, Dbgdscrint, Dbgdtrrxext, Dbgdtrrxint, Dbgdtrtxext, Dbgdtrtxint, Dbgosdlr,
    Dbgoseccr, Dbgoslar, Dbgoslsr, Dbgprcr, Dbgvcr, Dfar, Dfsr, Disr, Dlr, Dspsr, Dspsr2, Erridr,
    Errselr, Erxaddr, Erxaddr2, Erxctlr, Erxctlr2, Erxfr, Erxfr2, Erxmisc0, Erxmisc1, Erxmisc2,
    Erxmisc3, Erxmisc4, Erxmisc5, Erxmisc6, Erxmisc7, Erxstatus, Hcptr, Hcr, Hcr2, Hdcr, Hdfar,
    Hifar, Hmair0, Hmair1, Hpfar, Hrmr, Hsctlr, Hsr, Htcr, Htpidr, Htrfcr, Httbr, Hvbar, IccAsgi1r,
    IccBpr0, IccBpr1, IccCtlr, IccDir, IccEoir0, IccEoir1, IccHppir0, IccHppir1, IccHsre, IccIar0,
    IccIar1, IccIgrpen0, IccIgrpen1, IccMctlr, IccMgrpen1, IccMsre, IccPmr, IccRpr, IccSgi0r,
    IccSgi1r, IccSre, IdDfr0, IdDfr1, IdIsar0, IdIsar1, IdIsar2, IdIsar3, IdIsar4, IdIsar5,
    IdIsar6, IdMmfr0, IdMmfr1, IdMmfr2, IdMmfr3, IdMmfr4, IdMmfr5, IdPfr0, IdPfr1, IdPfr2, Ifar,
    Ifsr, Isr, Mair0, Mair1, Midr, Mpidr, Mvbar, Nmrr, Nsacr, Par, Pmccfiltr, Pmccntr, Pmceid0,
    Pmceid1, Pmceid2, Pmceid3, Pmcntenclr, Pmcntenset, Pmcr, Pmintenclr, Pmintenset, Pmmir, Pmovsr,
    Pmovsset, Pmselr, Pmswinc, Pmuserenr, Pmxevtyper, Prrr, Rmr, Rvbar, Scr, Sctlr, Sdcr, Sder,
    Tlbtr, Tpidrprw, Tpidruro, Tpidrurw, Trfcr, Ttbcr, Ttbcr2, Ttbr0, Ttbr1, Vbar, Vdfsr, Vdisr,
    Vmpidr, Vpidr, Vtcr, Vttbr,
};

/// A set of fake system registers.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub struct SystemRegisters {
    /// Fake value for the `ACTLR` system register.
    pub actlr: u32,
    /// Fake value for the `ACTLR2` system register.
    pub actlr2: u32,
    /// Fake value for the `ADFSR` system register.
    pub adfsr: u32,
    /// Fake value for the `AIDR` system register.
    pub aidr: u32,
    /// Fake value for the `AIFSR` system register.
    pub aifsr: u32,
    /// Fake value for the `AMAIR0` system register.
    pub amair0: u32,
    /// Fake value for the `AMAIR1` system register.
    pub amair1: u32,
    /// Fake value for the `AMCFGR` system register.
    pub amcfgr: Amcfgr,
    /// Fake value for the `AMCGCR` system register.
    pub amcgcr: Amcgcr,
    /// Fake value for the `AMCNTENCLR0` system register.
    pub amcntenclr0: Amcntenclr0,
    /// Fake value for the `AMCNTENCLR1` system register.
    pub amcntenclr1: Amcntenclr1,
    /// Fake value for the `AMCNTENSET0` system register.
    pub amcntenset0: Amcntenset0,
    /// Fake value for the `AMCNTENSET1` system register.
    pub amcntenset1: Amcntenset1,
    /// Fake value for the `AMCR` system register.
    pub amcr: Amcr,
    /// Fake value for the `AMEVCNTR00` system register.
    pub amevcntr00: Amevcntr00,
    /// Fake value for the `AMEVCNTR01` system register.
    pub amevcntr01: Amevcntr01,
    /// Fake value for the `AMEVCNTR02` system register.
    pub amevcntr02: Amevcntr02,
    /// Fake value for the `AMEVCNTR03` system register.
    pub amevcntr03: Amevcntr03,
    /// Fake value for the `AMEVTYPER00` system register.
    pub amevtyper00: Amevtyper00,
    /// Fake value for the `AMEVTYPER01` system register.
    pub amevtyper01: Amevtyper01,
    /// Fake value for the `AMEVTYPER02` system register.
    pub amevtyper02: Amevtyper02,
    /// Fake value for the `AMEVTYPER03` system register.
    pub amevtyper03: Amevtyper03,
    /// Fake value for the `AMEVTYPER10` system register.
    pub amevtyper10: Amevtyper10,
    /// Fake value for the `AMEVTYPER11` system register.
    pub amevtyper11: Amevtyper11,
    /// Fake value for the `AMEVTYPER110` system register.
    pub amevtyper110: Amevtyper110,
    /// Fake value for the `AMEVTYPER111` system register.
    pub amevtyper111: Amevtyper111,
    /// Fake value for the `AMEVTYPER112` system register.
    pub amevtyper112: Amevtyper112,
    /// Fake value for the `AMEVTYPER113` system register.
    pub amevtyper113: Amevtyper113,
    /// Fake value for the `AMEVTYPER114` system register.
    pub amevtyper114: Amevtyper114,
    /// Fake value for the `AMEVTYPER115` system register.
    pub amevtyper115: Amevtyper115,
    /// Fake value for the `AMEVTYPER12` system register.
    pub amevtyper12: Amevtyper12,
    /// Fake value for the `AMEVTYPER13` system register.
    pub amevtyper13: Amevtyper13,
    /// Fake value for the `AMEVTYPER14` system register.
    pub amevtyper14: Amevtyper14,
    /// Fake value for the `AMEVTYPER15` system register.
    pub amevtyper15: Amevtyper15,
    /// Fake value for the `AMEVTYPER16` system register.
    pub amevtyper16: Amevtyper16,
    /// Fake value for the `AMEVTYPER17` system register.
    pub amevtyper17: Amevtyper17,
    /// Fake value for the `AMEVTYPER18` system register.
    pub amevtyper18: Amevtyper18,
    /// Fake value for the `AMEVTYPER19` system register.
    pub amevtyper19: Amevtyper19,
    /// Fake value for the `AMUSERENR` system register.
    pub amuserenr: Amuserenr,
    /// Fake value for the `CCSIDR` system register.
    pub ccsidr: Ccsidr,
    /// Fake value for the `CCSIDR2` system register.
    pub ccsidr2: Ccsidr2,
    /// Fake value for the `CLIDR` system register.
    pub clidr: Clidr,
    /// Fake value for the `CNTFRQ` system register.
    pub cntfrq: Cntfrq,
    /// Fake value for the `CNTHCTL` system register.
    pub cnthctl: Cnthctl,
    /// Fake value for the `CNTHPS_CTL` system register.
    pub cnthps_ctl: CnthpsCtl,
    /// Fake value for the `CNTHPS_CVAL` system register.
    pub cnthps_cval: CnthpsCval,
    /// Fake value for the `CNTHPS_TVAL` system register.
    pub cnthps_tval: CnthpsTval,
    /// Fake value for the `CNTHP_CTL` system register.
    pub cnthp_ctl: CnthpCtl,
    /// Fake value for the `CNTHP_CVAL` system register.
    pub cnthp_cval: CnthpCval,
    /// Fake value for the `CNTHP_TVAL` system register.
    pub cnthp_tval: CnthpTval,
    /// Fake value for the `CNTHVS_CTL` system register.
    pub cnthvs_ctl: CnthvsCtl,
    /// Fake value for the `CNTHVS_CVAL` system register.
    pub cnthvs_cval: CnthvsCval,
    /// Fake value for the `CNTHVS_TVAL` system register.
    pub cnthvs_tval: CnthvsTval,
    /// Fake value for the `CNTHV_CTL` system register.
    pub cnthv_ctl: CnthvCtl,
    /// Fake value for the `CNTHV_CVAL` system register.
    pub cnthv_cval: CnthvCval,
    /// Fake value for the `CNTHV_TVAL` system register.
    pub cnthv_tval: CnthvTval,
    /// Fake value for the `CNTKCTL` system register.
    pub cntkctl: Cntkctl,
    /// Fake value for the `CNTPCT` system register.
    pub cntpct: Cntpct,
    /// Fake value for the `CNTPCTSS` system register.
    pub cntpctss: Cntpctss,
    /// Fake value for the `CNTP_CTL` system register.
    pub cntp_ctl: CntpCtl,
    /// Fake value for the `CNTP_CVAL` system register.
    pub cntp_cval: CntpCval,
    /// Fake value for the `CNTP_TVAL` system register.
    pub cntp_tval: CntpTval,
    /// Fake value for the `CNTVCT` system register.
    pub cntvct: Cntvct,
    /// Fake value for the `CNTVCTSS` system register.
    pub cntvctss: Cntvctss,
    /// Fake value for the `CNTVOFF` system register.
    pub cntvoff: Cntvoff,
    /// Fake value for the `CNTV_CTL` system register.
    pub cntv_ctl: CntvCtl,
    /// Fake value for the `CNTV_CVAL` system register.
    pub cntv_cval: CntvCval,
    /// Fake value for the `CNTV_TVAL` system register.
    pub cntv_tval: CntvTval,
    /// Fake value for the `CONTEXTIDR` system register.
    pub contextidr: Contextidr,
    /// Fake value for the `CPACR` system register.
    pub cpacr: Cpacr,
    /// Fake value for the `CSSELR` system register.
    pub csselr: Csselr,
    /// Fake value for the `CTR` system register.
    pub ctr: Ctr,
    /// Fake value for the `DACR` system register.
    pub dacr: Dacr,
    /// Fake value for the `DBGAUTHSTATUS` system register.
    pub dbgauthstatus: Dbgauthstatus,
    /// Fake value for the `DBGCLAIMCLR` system register.
    pub dbgclaimclr: Dbgclaimclr,
    /// Fake value for the `DBGCLAIMSET` system register.
    pub dbgclaimset: Dbgclaimset,
    /// Fake value for the `DBGDCCINT` system register.
    pub dbgdccint: Dbgdccint,
    /// Fake value for the `DBGDEVID` system register.
    pub dbgdevid: Dbgdevid,
    /// Fake value for the `DBGDEVID1` system register.
    pub dbgdevid1: Dbgdevid1,
    /// Fake value for the `DBGDEVID2` system register.
    pub dbgdevid2: u32,
    /// Fake value for the `DBGDIDR` system register.
    pub dbgdidr: Dbgdidr,
    /// Fake value for the `DBGDRAR` system register.
    pub dbgdrar: Dbgdrar,
    /// Fake value for the `DBGDSAR` system register.
    pub dbgdsar: u64,
    /// Fake value for the `DBGDSCRext` system register.
    pub dbgdscrext: Dbgdscrext,
    /// Fake value for the `DBGDSCRint` system register.
    pub dbgdscrint: Dbgdscrint,
    /// Fake value for the `DBGDTRRXext` system register.
    pub dbgdtrrxext: Dbgdtrrxext,
    /// Fake value for the `DBGDTRRXint` system register.
    pub dbgdtrrxint: Dbgdtrrxint,
    /// Fake value for the `DBGDTRTXext` system register.
    pub dbgdtrtxext: Dbgdtrtxext,
    /// Fake value for the `DBGDTRTXint` system register.
    pub dbgdtrtxint: Dbgdtrtxint,
    /// Fake value for the `DBGOSDLR` system register.
    pub dbgosdlr: Dbgosdlr,
    /// Fake value for the `DBGOSECCR` system register.
    pub dbgoseccr: Dbgoseccr,
    /// Fake value for the `DBGOSLAR` system register.
    pub dbgoslar: Dbgoslar,
    /// Fake value for the `DBGOSLSR` system register.
    pub dbgoslsr: Dbgoslsr,
    /// Fake value for the `DBGPRCR` system register.
    pub dbgprcr: Dbgprcr,
    /// Fake value for the `DBGVCR` system register.
    pub dbgvcr: Dbgvcr,
    /// Fake value for the `DBGWFAR` system register.
    pub dbgwfar: u32,
    /// Fake value for the `DFAR` system register.
    pub dfar: Dfar,
    /// Fake value for the `DFSR` system register.
    pub dfsr: Dfsr,
    /// Fake value for the `DISR` system register.
    pub disr: Disr,
    /// Fake value for the `DLR` system register.
    pub dlr: Dlr,
    /// Fake value for the `DSPSR` system register.
    pub dspsr: Dspsr,
    /// Fake value for the `DSPSR2` system register.
    pub dspsr2: Dspsr2,
    #[cfg(feature = "el2")]
    /// Fake value for the `ELR_hyp` system register.
    pub elr_hyp: ElrHyp,
    /// Fake value for the `ERRIDR` system register.
    pub erridr: Erridr,
    /// Fake value for the `ERRSELR` system register.
    pub errselr: Errselr,
    /// Fake value for the `ERXADDR` system register.
    pub erxaddr: Erxaddr,
    /// Fake value for the `ERXADDR2` system register.
    pub erxaddr2: Erxaddr2,
    /// Fake value for the `ERXCTLR` system register.
    pub erxctlr: Erxctlr,
    /// Fake value for the `ERXCTLR2` system register.
    pub erxctlr2: Erxctlr2,
    /// Fake value for the `ERXFR` system register.
    pub erxfr: Erxfr,
    /// Fake value for the `ERXFR2` system register.
    pub erxfr2: Erxfr2,
    /// Fake value for the `ERXMISC0` system register.
    pub erxmisc0: Erxmisc0,
    /// Fake value for the `ERXMISC1` system register.
    pub erxmisc1: Erxmisc1,
    /// Fake value for the `ERXMISC2` system register.
    pub erxmisc2: Erxmisc2,
    /// Fake value for the `ERXMISC3` system register.
    pub erxmisc3: Erxmisc3,
    /// Fake value for the `ERXMISC4` system register.
    pub erxmisc4: Erxmisc4,
    /// Fake value for the `ERXMISC5` system register.
    pub erxmisc5: Erxmisc5,
    /// Fake value for the `ERXMISC6` system register.
    pub erxmisc6: Erxmisc6,
    /// Fake value for the `ERXMISC7` system register.
    pub erxmisc7: Erxmisc7,
    /// Fake value for the `ERXSTATUS` system register.
    pub erxstatus: Erxstatus,
    /// Fake value for the `FCSEIDR` system register.
    pub fcseidr: u32,
    /// Fake value for the `HACR` system register.
    pub hacr: u32,
    /// Fake value for the `HACTLR` system register.
    pub hactlr: u32,
    /// Fake value for the `HACTLR2` system register.
    pub hactlr2: u32,
    /// Fake value for the `HADFSR` system register.
    pub hadfsr: u32,
    /// Fake value for the `HAIFSR` system register.
    pub haifsr: u32,
    /// Fake value for the `HAMAIR0` system register.
    pub hamair0: u32,
    /// Fake value for the `HAMAIR1` system register.
    pub hamair1: u32,
    /// Fake value for the `HCPTR` system register.
    pub hcptr: Hcptr,
    /// Fake value for the `HCR` system register.
    pub hcr: Hcr,
    /// Fake value for the `HCR2` system register.
    pub hcr2: Hcr2,
    /// Fake value for the `HDCR` system register.
    pub hdcr: Hdcr,
    /// Fake value for the `HDFAR` system register.
    pub hdfar: Hdfar,
    /// Fake value for the `HIFAR` system register.
    pub hifar: Hifar,
    /// Fake value for the `HMAIR0` system register.
    pub hmair0: Hmair0,
    /// Fake value for the `HMAIR1` system register.
    pub hmair1: Hmair1,
    /// Fake value for the `HPFAR` system register.
    pub hpfar: Hpfar,
    /// Fake value for the `HRMR` system register.
    pub hrmr: Hrmr,
    /// Fake value for the `HSCTLR` system register.
    pub hsctlr: Hsctlr,
    /// Fake value for the `HSR` system register.
    pub hsr: Hsr,
    /// Fake value for the `HSTR` system register.
    pub hstr: u32,
    /// Fake value for the `HTCR` system register.
    pub htcr: Htcr,
    /// Fake value for the `HTPIDR` system register.
    pub htpidr: Htpidr,
    /// Fake value for the `HTRFCR` system register.
    pub htrfcr: Htrfcr,
    /// Fake value for the `HTTBR` system register.
    pub httbr: Httbr,
    /// Fake value for the `HVBAR` system register.
    pub hvbar: Hvbar,
    /// Fake value for the `ICC_ASGI1R` system register.
    pub icc_asgi1r: IccAsgi1r,
    /// Fake value for the `ICC_BPR0` system register.
    pub icc_bpr0: IccBpr0,
    /// Fake value for the `ICC_BPR1` system register.
    pub icc_bpr1: IccBpr1,
    /// Fake value for the `ICC_CTLR` system register.
    pub icc_ctlr: IccCtlr,
    /// Fake value for the `ICC_DIR` system register.
    pub icc_dir: IccDir,
    /// Fake value for the `ICC_EOIR0` system register.
    pub icc_eoir0: IccEoir0,
    /// Fake value for the `ICC_EOIR1` system register.
    pub icc_eoir1: IccEoir1,
    /// Fake value for the `ICC_HPPIR0` system register.
    pub icc_hppir0: IccHppir0,
    /// Fake value for the `ICC_HPPIR1` system register.
    pub icc_hppir1: IccHppir1,
    /// Fake value for the `ICC_HSRE` system register.
    pub icc_hsre: IccHsre,
    /// Fake value for the `ICC_IAR0` system register.
    pub icc_iar0: IccIar0,
    /// Fake value for the `ICC_IAR1` system register.
    pub icc_iar1: IccIar1,
    /// Fake value for the `ICC_IGRPEN0` system register.
    pub icc_igrpen0: IccIgrpen0,
    /// Fake value for the `ICC_IGRPEN1` system register.
    pub icc_igrpen1: IccIgrpen1,
    /// Fake value for the `ICC_MCTLR` system register.
    pub icc_mctlr: IccMctlr,
    /// Fake value for the `ICC_MGRPEN1` system register.
    pub icc_mgrpen1: IccMgrpen1,
    /// Fake value for the `ICC_MSRE` system register.
    pub icc_msre: IccMsre,
    /// Fake value for the `ICC_PMR` system register.
    pub icc_pmr: IccPmr,
    /// Fake value for the `ICC_RPR` system register.
    pub icc_rpr: IccRpr,
    /// Fake value for the `ICC_SGI0R` system register.
    pub icc_sgi0r: IccSgi0r,
    /// Fake value for the `ICC_SGI1R` system register.
    pub icc_sgi1r: IccSgi1r,
    /// Fake value for the `ICC_SRE` system register.
    pub icc_sre: IccSre,
    /// Fake value for the `ID_AFR0` system register.
    pub id_afr0: u32,
    /// Fake value for the `ID_DFR0` system register.
    pub id_dfr0: IdDfr0,
    /// Fake value for the `ID_DFR1` system register.
    pub id_dfr1: IdDfr1,
    /// Fake value for the `ID_ISAR0` system register.
    pub id_isar0: IdIsar0,
    /// Fake value for the `ID_ISAR1` system register.
    pub id_isar1: IdIsar1,
    /// Fake value for the `ID_ISAR2` system register.
    pub id_isar2: IdIsar2,
    /// Fake value for the `ID_ISAR3` system register.
    pub id_isar3: IdIsar3,
    /// Fake value for the `ID_ISAR4` system register.
    pub id_isar4: IdIsar4,
    /// Fake value for the `ID_ISAR5` system register.
    pub id_isar5: IdIsar5,
    /// Fake value for the `ID_ISAR6` system register.
    pub id_isar6: IdIsar6,
    /// Fake value for the `ID_MMFR0` system register.
    pub id_mmfr0: IdMmfr0,
    /// Fake value for the `ID_MMFR1` system register.
    pub id_mmfr1: IdMmfr1,
    /// Fake value for the `ID_MMFR2` system register.
    pub id_mmfr2: IdMmfr2,
    /// Fake value for the `ID_MMFR3` system register.
    pub id_mmfr3: IdMmfr3,
    /// Fake value for the `ID_MMFR4` system register.
    pub id_mmfr4: IdMmfr4,
    /// Fake value for the `ID_MMFR5` system register.
    pub id_mmfr5: IdMmfr5,
    /// Fake value for the `ID_PFR0` system register.
    pub id_pfr0: IdPfr0,
    /// Fake value for the `ID_PFR1` system register.
    pub id_pfr1: IdPfr1,
    /// Fake value for the `ID_PFR2` system register.
    pub id_pfr2: IdPfr2,
    /// Fake value for the `IFAR` system register.
    pub ifar: Ifar,
    /// Fake value for the `IFSR` system register.
    pub ifsr: Ifsr,
    /// Fake value for the `ISR` system register.
    pub isr: Isr,
    /// Fake value for the `JIDR` system register.
    pub jidr: u32,
    /// Fake value for the `JMCR` system register.
    pub jmcr: u32,
    /// Fake value for the `JOSCR` system register.
    pub joscr: u32,
    /// Fake value for the `MAIR0` system register.
    pub mair0: Mair0,
    /// Fake value for the `MAIR1` system register.
    pub mair1: Mair1,
    /// Fake value for the `MIDR` system register.
    pub midr: Midr,
    /// Fake value for the `MPIDR` system register.
    pub mpidr: Mpidr,
    /// Fake value for the `MVBAR` system register.
    pub mvbar: Mvbar,
    /// Fake value for the `NMRR` system register.
    pub nmrr: Nmrr,
    /// Fake value for the `NSACR` system register.
    pub nsacr: Nsacr,
    /// Fake value for the `PAR` system register.
    pub par: Par,
    /// Fake value for the `PMCCFILTR` system register.
    pub pmccfiltr: Pmccfiltr,
    /// Fake value for the `PMCCNTR` system register.
    pub pmccntr: Pmccntr,
    /// Fake value for the `PMCEID0` system register.
    pub pmceid0: Pmceid0,
    /// Fake value for the `PMCEID1` system register.
    pub pmceid1: Pmceid1,
    /// Fake value for the `PMCEID2` system register.
    pub pmceid2: Pmceid2,
    /// Fake value for the `PMCEID3` system register.
    pub pmceid3: Pmceid3,
    /// Fake value for the `PMCNTENCLR` system register.
    pub pmcntenclr: Pmcntenclr,
    /// Fake value for the `PMCNTENSET` system register.
    pub pmcntenset: Pmcntenset,
    /// Fake value for the `PMCR` system register.
    pub pmcr: Pmcr,
    /// Fake value for the `PMINTENCLR` system register.
    pub pmintenclr: Pmintenclr,
    /// Fake value for the `PMINTENSET` system register.
    pub pmintenset: Pmintenset,
    /// Fake value for the `PMMIR` system register.
    pub pmmir: Pmmir,
    /// Fake value for the `PMOVSR` system register.
    pub pmovsr: Pmovsr,
    /// Fake value for the `PMOVSSET` system register.
    pub pmovsset: Pmovsset,
    /// Fake value for the `PMSELR` system register.
    pub pmselr: Pmselr,
    /// Fake value for the `PMSWINC` system register.
    pub pmswinc: Pmswinc,
    /// Fake value for the `PMUSERENR` system register.
    pub pmuserenr: Pmuserenr,
    /// Fake value for the `PMXEVTYPER` system register.
    pub pmxevtyper: Pmxevtyper,
    /// Fake value for the `PRRR` system register.
    pub prrr: Prrr,
    /// Fake value for the `REVIDR` system register.
    pub revidr: u32,
    /// Fake value for the `RMR` system register.
    pub rmr: Rmr,
    /// Fake value for the `RVBAR` system register.
    pub rvbar: Rvbar,
    /// Fake value for the `SCR` system register.
    pub scr: Scr,
    /// Fake value for the `SCTLR` system register.
    pub sctlr: Sctlr,
    /// Fake value for the `SDCR` system register.
    pub sdcr: Sdcr,
    /// Fake value for the `SDER` system register.
    pub sder: Sder,
    /// Fake value for the `TCMTR` system register.
    pub tcmtr: u32,
    /// Fake value for the `TLBTR` system register.
    pub tlbtr: Tlbtr,
    /// Fake value for the `TPIDRPRW` system register.
    pub tpidrprw: Tpidrprw,
    /// Fake value for the `TPIDRURO` system register.
    pub tpidruro: Tpidruro,
    /// Fake value for the `TPIDRURW` system register.
    pub tpidrurw: Tpidrurw,
    /// Fake value for the `TRFCR` system register.
    pub trfcr: Trfcr,
    /// Fake value for the `TTBCR` system register.
    pub ttbcr: Ttbcr,
    /// Fake value for the `TTBCR2` system register.
    pub ttbcr2: Ttbcr2,
    /// Fake value for the `TTBR0` system register.
    pub ttbr0: Ttbr0,
    /// Fake value for the `TTBR1` system register.
    pub ttbr1: Ttbr1,
    /// Fake value for the `VBAR` system register.
    pub vbar: Vbar,
    /// Fake value for the `VDFSR` system register.
    pub vdfsr: Vdfsr,
    /// Fake value for the `VDISR` system register.
    pub vdisr: Vdisr,
    /// Fake value for the `VMPIDR` system register.
    pub vmpidr: Vmpidr,
    /// Fake value for the `VPIDR` system register.
    pub vpidr: Vpidr,
    /// Fake value for the `VTCR` system register.
    pub vtcr: Vtcr,
    /// Fake value for the `VTTBR` system register.
    pub vttbr: Vttbr,
}

impl SystemRegisters {
    pub(crate) const fn new() -> Self {
        Self {
            actlr: 0,
            actlr2: 0,
            adfsr: 0,
            aidr: 0,
            aifsr: 0,
            amair0: 0,
            amair1: 0,
            amcfgr: Amcfgr::empty(),
            amcgcr: Amcgcr::empty(),
            amcntenclr0: Amcntenclr0::empty(),
            amcntenclr1: Amcntenclr1::empty(),
            amcntenset0: Amcntenset0::empty(),
            amcntenset1: Amcntenset1::empty(),
            amcr: Amcr::empty(),
            amevcntr00: Amevcntr00::empty(),
            amevcntr01: Amevcntr01::empty(),
            amevcntr02: Amevcntr02::empty(),
            amevcntr03: Amevcntr03::empty(),
            amevtyper00: Amevtyper00::empty(),
            amevtyper01: Amevtyper01::empty(),
            amevtyper02: Amevtyper02::empty(),
            amevtyper03: Amevtyper03::empty(),
            amevtyper10: Amevtyper10::empty(),
            amevtyper11: Amevtyper11::empty(),
            amevtyper110: Amevtyper110::empty(),
            amevtyper111: Amevtyper111::empty(),
            amevtyper112: Amevtyper112::empty(),
            amevtyper113: Amevtyper113::empty(),
            amevtyper114: Amevtyper114::empty(),
            amevtyper115: Amevtyper115::empty(),
            amevtyper12: Amevtyper12::empty(),
            amevtyper13: Amevtyper13::empty(),
            amevtyper14: Amevtyper14::empty(),
            amevtyper15: Amevtyper15::empty(),
            amevtyper16: Amevtyper16::empty(),
            amevtyper17: Amevtyper17::empty(),
            amevtyper18: Amevtyper18::empty(),
            amevtyper19: Amevtyper19::empty(),
            amuserenr: Amuserenr::empty(),
            ccsidr: Ccsidr::empty(),
            ccsidr2: Ccsidr2::empty(),
            clidr: Clidr::empty(),
            cntfrq: Cntfrq::empty(),
            cnthctl: Cnthctl::empty(),
            cnthps_ctl: CnthpsCtl::empty(),
            cnthps_cval: CnthpsCval::empty(),
            cnthps_tval: CnthpsTval::empty(),
            cnthp_ctl: CnthpCtl::empty(),
            cnthp_cval: CnthpCval::empty(),
            cnthp_tval: CnthpTval::empty(),
            cnthvs_ctl: CnthvsCtl::empty(),
            cnthvs_cval: CnthvsCval::empty(),
            cnthvs_tval: CnthvsTval::empty(),
            cnthv_ctl: CnthvCtl::empty(),
            cnthv_cval: CnthvCval::empty(),
            cnthv_tval: CnthvTval::empty(),
            cntkctl: Cntkctl::empty(),
            cntpct: Cntpct::empty(),
            cntpctss: Cntpctss::empty(),
            cntp_ctl: CntpCtl::empty(),
            cntp_cval: CntpCval::empty(),
            cntp_tval: CntpTval::empty(),
            cntvct: Cntvct::empty(),
            cntvctss: Cntvctss::empty(),
            cntvoff: Cntvoff::empty(),
            cntv_ctl: CntvCtl::empty(),
            cntv_cval: CntvCval::empty(),
            cntv_tval: CntvTval::empty(),
            contextidr: Contextidr::empty(),
            cpacr: Cpacr::empty(),
            csselr: Csselr::empty(),
            ctr: Ctr::empty(),
            dacr: Dacr::empty(),
            dbgauthstatus: Dbgauthstatus::empty(),
            dbgclaimclr: Dbgclaimclr::empty(),
            dbgclaimset: Dbgclaimset::empty(),
            dbgdccint: Dbgdccint::empty(),
            dbgdevid: Dbgdevid::empty(),
            dbgdevid1: Dbgdevid1::empty(),
            dbgdevid2: 0,
            dbgdidr: Dbgdidr::empty(),
            dbgdrar: Dbgdrar::empty(),
            dbgdsar: 0,
            dbgdscrext: Dbgdscrext::empty(),
            dbgdscrint: Dbgdscrint::empty(),
            dbgdtrrxext: Dbgdtrrxext::empty(),
            dbgdtrrxint: Dbgdtrrxint::empty(),
            dbgdtrtxext: Dbgdtrtxext::empty(),
            dbgdtrtxint: Dbgdtrtxint::empty(),
            dbgosdlr: Dbgosdlr::empty(),
            dbgoseccr: Dbgoseccr::empty(),
            dbgoslar: Dbgoslar::empty(),
            dbgoslsr: Dbgoslsr::empty(),
            dbgprcr: Dbgprcr::empty(),
            dbgvcr: Dbgvcr::empty(),
            dbgwfar: 0,
            dfar: Dfar::empty(),
            dfsr: Dfsr::empty(),
            disr: Disr::empty(),
            dlr: Dlr::empty(),
            dspsr: Dspsr::empty(),
            dspsr2: Dspsr2::empty(),
            #[cfg(feature = "el2")]
            elr_hyp: ElrHyp::empty(),
            erridr: Erridr::empty(),
            errselr: Errselr::empty(),
            erxaddr: Erxaddr::empty(),
            erxaddr2: Erxaddr2::empty(),
            erxctlr: Erxctlr::empty(),
            erxctlr2: Erxctlr2::empty(),
            erxfr: Erxfr::empty(),
            erxfr2: Erxfr2::empty(),
            erxmisc0: Erxmisc0::empty(),
            erxmisc1: Erxmisc1::empty(),
            erxmisc2: Erxmisc2::empty(),
            erxmisc3: Erxmisc3::empty(),
            erxmisc4: Erxmisc4::empty(),
            erxmisc5: Erxmisc5::empty(),
            erxmisc6: Erxmisc6::empty(),
            erxmisc7: Erxmisc7::empty(),
            erxstatus: Erxstatus::empty(),
            fcseidr: 0,
            hacr: 0,
            hactlr: 0,
            hactlr2: 0,
            hadfsr: 0,
            haifsr: 0,
            hamair0: 0,
            hamair1: 0,
            hcptr: Hcptr::empty(),
            hcr: Hcr::empty(),
            hcr2: Hcr2::empty(),
            hdcr: Hdcr::empty(),
            hdfar: Hdfar::empty(),
            hifar: Hifar::empty(),
            hmair0: Hmair0::empty(),
            hmair1: Hmair1::empty(),
            hpfar: Hpfar::empty(),
            hrmr: Hrmr::empty(),
            hsctlr: Hsctlr::empty(),
            hsr: Hsr::empty(),
            hstr: 0,
            htcr: Htcr::empty(),
            htpidr: Htpidr::empty(),
            htrfcr: Htrfcr::empty(),
            httbr: Httbr::empty(),
            hvbar: Hvbar::empty(),
            icc_asgi1r: IccAsgi1r::empty(),
            icc_bpr0: IccBpr0::empty(),
            icc_bpr1: IccBpr1::empty(),
            icc_ctlr: IccCtlr::empty(),
            icc_dir: IccDir::empty(),
            icc_eoir0: IccEoir0::empty(),
            icc_eoir1: IccEoir1::empty(),
            icc_hppir0: IccHppir0::empty(),
            icc_hppir1: IccHppir1::empty(),
            icc_hsre: IccHsre::empty(),
            icc_iar0: IccIar0::empty(),
            icc_iar1: IccIar1::empty(),
            icc_igrpen0: IccIgrpen0::empty(),
            icc_igrpen1: IccIgrpen1::empty(),
            icc_mctlr: IccMctlr::empty(),
            icc_mgrpen1: IccMgrpen1::empty(),
            icc_msre: IccMsre::empty(),
            icc_pmr: IccPmr::empty(),
            icc_rpr: IccRpr::empty(),
            icc_sgi0r: IccSgi0r::empty(),
            icc_sgi1r: IccSgi1r::empty(),
            icc_sre: IccSre::empty(),
            id_afr0: 0,
            id_dfr0: IdDfr0::empty(),
            id_dfr1: IdDfr1::empty(),
            id_isar0: IdIsar0::empty(),
            id_isar1: IdIsar1::empty(),
            id_isar2: IdIsar2::empty(),
            id_isar3: IdIsar3::empty(),
            id_isar4: IdIsar4::empty(),
            id_isar5: IdIsar5::empty(),
            id_isar6: IdIsar6::empty(),
            id_mmfr0: IdMmfr0::empty(),
            id_mmfr1: IdMmfr1::empty(),
            id_mmfr2: IdMmfr2::empty(),
            id_mmfr3: IdMmfr3::empty(),
            id_mmfr4: IdMmfr4::empty(),
            id_mmfr5: IdMmfr5::empty(),
            id_pfr0: IdPfr0::empty(),
            id_pfr1: IdPfr1::empty(),
            id_pfr2: IdPfr2::empty(),
            ifar: Ifar::empty(),
            ifsr: Ifsr::empty(),
            isr: Isr::empty(),
            jidr: 0,
            jmcr: 0,
            joscr: 0,
            mair0: Mair0::empty(),
            mair1: Mair1::empty(),
            midr: Midr::empty(),
            mpidr: Mpidr::empty(),
            mvbar: Mvbar::empty(),
            nmrr: Nmrr::empty(),
            nsacr: Nsacr::empty(),
            par: Par::empty(),
            pmccfiltr: Pmccfiltr::empty(),
            pmccntr: Pmccntr::empty(),
            pmceid0: Pmceid0::empty(),
            pmceid1: Pmceid1::empty(),
            pmceid2: Pmceid2::empty(),
            pmceid3: Pmceid3::empty(),
            pmcntenclr: Pmcntenclr::empty(),
            pmcntenset: Pmcntenset::empty(),
            pmcr: Pmcr::empty(),
            pmintenclr: Pmintenclr::empty(),
            pmintenset: Pmintenset::empty(),
            pmmir: Pmmir::empty(),
            pmovsr: Pmovsr::empty(),
            pmovsset: Pmovsset::empty(),
            pmselr: Pmselr::empty(),
            pmswinc: Pmswinc::empty(),
            pmuserenr: Pmuserenr::empty(),
            pmxevtyper: Pmxevtyper::empty(),
            prrr: Prrr::empty(),
            revidr: 0,
            rmr: Rmr::empty(),
            rvbar: Rvbar::empty(),
            scr: Scr::empty(),
            sctlr: Sctlr::empty(),
            sdcr: Sdcr::empty(),
            sder: Sder::empty(),
            tcmtr: 0,
            tlbtr: Tlbtr::empty(),
            tpidrprw: Tpidrprw::empty(),
            tpidruro: Tpidruro::empty(),
            tpidrurw: Tpidrurw::empty(),
            trfcr: Trfcr::empty(),
            ttbcr: Ttbcr::empty(),
            ttbcr2: Ttbcr2::empty(),
            ttbr0: Ttbr0::empty(),
            ttbr1: Ttbr1::empty(),
            vbar: Vbar::empty(),
            vdfsr: Vdfsr::empty(),
            vdisr: Vdisr::empty(),
            vmpidr: Vmpidr::empty(),
            vpidr: Vpidr::empty(),
            vtcr: Vtcr::empty(),
            vttbr: Vttbr::empty(),
        }
    }
}
