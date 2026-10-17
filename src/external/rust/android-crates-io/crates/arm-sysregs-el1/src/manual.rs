// SPDX-FileCopyrightText: Copyright The arm-sysregs Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Manually implemented methods for EL1 system register types.

#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
use crate::accessors::read_mpidr_el1;
use crate::registers::{
    ClidrEl1, CsselrEl1, EsrEl1, IdAa64dfr0El1, IdAa64dfr1El1, IdAa64mmfr0El1, IdAa64mmfr1El1,
    IdAa64mmfr2El1, IdAa64mmfr3El1, IdAa64mmfr4El1, IdAa64pfr0El1, IdAa64pfr1El1, IdAa64pfr2El1,
    MpidrEl1, SpsrEl1,
};
use arm_sysregs_common::types::{CacheLevel, CacheType};
use core::fmt::{self, Debug, Formatter};

impl ClidrEl1 {
    /// Returns the inner cache boundary level.
    pub fn icb_level(self) -> Option<CacheLevel> {
        let icb = self.icb();
        if icb != 0 {
            Some(CacheLevel(icb as u8))
        } else {
            None
        }
    }

    /// Returns Cache Type [1-7] fields.
    pub fn cache_type(self, level: CacheLevel) -> CacheType {
        self.ctype(level.level().into()).try_into().unwrap()
    }
}

impl CsselrEl1 {
    /// Creates new instance. TnD is only valid if FEAT_MTE2 is implemented.
    pub fn new(tnd: bool, level: CacheLevel, ind: bool) -> Self {
        let mut instance = Self::from_bits_retain(u64::from(level) << 1);

        if ind {
            instance |= Self::IND;
        } else if tnd {
            // TnD is only valid if InD is not set.
            instance |= Self::TND;
        }

        instance
    }

    /// Returns the cache level of requested cache.
    pub fn cache_level(self) -> CacheLevel {
        CacheLevel(self.level() + 1)
    }
}

impl EsrEl1 {
    /// Mask for the parts of an ESR value containing the opcode.
    pub const ISS_SYSREG_OPCODE_MASK: Self = Self::from_bits_retain(0x003f_fc1e);
}

impl Debug for EsrEl1 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "EsrEl1({:#x})", self.bits())
    }
}

impl IdAa64dfr0El1 {
    const SYS_REG_TRACE_IMPLEMENTED: u8 = 0b0001;
    const SPE_IMPLEMENTED: u8 = 0b0001;
    const TRF_IMPLEMENTED: u8 = 0b0001;
    const TRBE_IMPLEMENTED: u8 = 0b0001;
    const MTPMU_IMPLEMENTED: u8 = 0b0001;
    const BRBE_IMPLEMENTED: u8 = 0b0001;
    const BRBEV1P1_IMPLEMENTED: u8 = 0b0010;

    /// Indicates whether FEAT_BRBE is supported.
    pub fn is_feat_brbe_present(self) -> bool {
        self.brbe() >= Self::BRBE_IMPLEMENTED
    }

    /// Indicates whether FEAT_BRBEv1p1 is supported.
    /// If FEAT_BRBEv1p1 is supported, FEAT_BRBE is also supported.
    pub fn is_feat_brbe_v1p1_present(self) -> bool {
        self.brbe() >= Self::BRBEV1P1_IMPLEMENTED
    }

    /// Trace support. Indicates whether System register interface to a PE trace unit is
    /// implemented.
    pub fn is_feat_sys_reg_trace_present(self) -> bool {
        self.tracever() >= Self::SYS_REG_TRACE_IMPLEMENTED
    }

    /// Indicates whether Armv8.1 Statistical Profiling Extension is implemented.
    pub fn is_feat_spe_present(self) -> bool {
        self.pmsver() >= Self::SPE_IMPLEMENTED
    }

    /// Indicates whether Armv8.4 Self-hosted Trace Extension is implemented.
    pub fn is_feat_trf_present(self) -> bool {
        self.tracefilt() >= Self::TRF_IMPLEMENTED
    }

    /// Indicates whether Trace Buffer Extension is implemented.
    pub fn is_feat_trbe_present(self) -> bool {
        self.tracebuffer() >= Self::TRBE_IMPLEMENTED
    }

    /// Indicates whether Multi Threaded PMU Extension is implemented.
    pub fn is_feat_mtpmu_present(self) -> bool {
        self.mtpmu() >= Self::MTPMU_IMPLEMENTED
    }
}

impl IdAa64dfr1El1 {
    const EBEP_IMPLEMENTED: u8 = 0b1;

    /// Indicates whether FEAT_EBEP is implemented.
    pub fn is_feat_ebep_present(self) -> bool {
        self.ebep() >= Self::EBEP_IMPLEMENTED
    }
}

impl IdAa64mmfr0El1 {
    const FGT_IMPLEMENTED: u8 = 0b0001;
    const FGT2_IMPLEMENTED: u8 = 0b0010;

    /// Indicates whether Fine Grain Traps Extension is implemented.
    pub fn is_feat_fgt_present(self) -> bool {
        self.fgt() >= Self::FGT_IMPLEMENTED
    }

    /// Indicates whether Fine Grain Traps 2 Extension is implemented.
    pub fn is_feat_fgt2_present(self) -> bool {
        self.fgt() >= Self::FGT2_IMPLEMENTED
    }
}

impl IdAa64mmfr1El1 {
    const VH_IMPLEMENTED: u8 = 0b0001;
    const HCX_IMPLEMENTED: u8 = 0b0001;

    /// Indicates presence of FEAT_VHE.
    pub fn is_feat_vhe_present(self) -> bool {
        self.vh() >= Self::VH_IMPLEMENTED
    }

    /// Indicates presence of FEAT_HCX.
    pub fn is_feat_hcx_present(self) -> bool {
        self.hcx() >= Self::HCX_IMPLEMENTED
    }
}

impl IdAa64mmfr2El1 {
    const CCIDX_64_BIT: u8 = 0b0001;

    /// Checks whether 64-bit format is implemented for all levels of the CCSIDR_EL1.
    pub fn has_64_bit_ccsidr_el1(self) -> bool {
        self.ccidx() >= Self::CCIDX_64_BIT
    }
}

impl IdAa64mmfr3El1 {
    const TCRX_IMPLEMENTED: u8 = 0b0001;
    const S1PIE_IMPLEMENTED: u8 = 0b0001;
    const S1POE_IMPLEMENTED: u8 = 0b0001;
    const S2PIE_IMPLEMENTED: u8 = 0b0001;
    const S2POE_IMPLEMENTED: u8 = 0b0001;
    const SCTLR2_IMPLEMENTED: u8 = 0b0001;

    /// Indicates presence of FEAT_SCTLR2.
    pub fn is_feat_sctlr2_present(self) -> bool {
        self.sctlrx() >= Self::SCTLR2_IMPLEMENTED
    }

    /// Indicates whether FEAT_S1PIE is implemented.
    pub fn is_feat_s1pie_present(self) -> bool {
        self.s1pie() >= Self::S1PIE_IMPLEMENTED
    }

    /// Indicates whether FEAT_S1POE is implemented.
    pub fn is_feat_s1poe_present(self) -> bool {
        self.s1poe() >= Self::S1POE_IMPLEMENTED
    }

    /// Indicates whether FEAT_S2PIE is implemented.
    pub fn is_feat_s2pie_present(self) -> bool {
        self.s2pie() >= Self::S2PIE_IMPLEMENTED
    }

    /// Indicates whether FEAT_S2POE is implemented.
    pub fn is_feat_s2poe_present(self) -> bool {
        self.s2poe() >= Self::S2POE_IMPLEMENTED
    }

    /// Indicates presence of FEAT_TCR2.
    pub fn is_feat_tcr2_present(self) -> bool {
        self.tcrx() >= Self::TCRX_IMPLEMENTED
    }
}

impl IdAa64mmfr4El1 {
    const FGWTE3_IMPLEMENTED: u8 = 0b0001;
    const RME_GDI_IMPLEMENTED: u8 = 0b0001;

    /// Indicates whether FEAT_FGWTE3 is implemented.
    pub fn is_feat_fgwte3_present(self) -> bool {
        self.fgwte3() >= Self::FGWTE3_IMPLEMENTED
    }

    /// Indicates whether FEAT_RME_GDI is implemented.
    pub fn is_feat_rme_gdi_present(self) -> bool {
        self.rmegdi() >= Self::RME_GDI_IMPLEMENTED
    }
}

impl IdAa64pfr0El1 {
    const SVE_IMPLEMENTED: u8 = 0b0001;
    const MPAM_IMPLEMENTED: u8 = 0b0001;
    const AMUV1_IMPLEMENTED: u8 = 0b0001;
    const AMUV1P1_IMPLEMENTED: u8 = 0b0010;
    const DIT_IMPLEMENTED: u8 = 0b0001;
    const RME_IMPLEMENTED: u8 = 0b0001;
    const RME_GPC2_IMPLEMENTED: u8 = 0b0010;
    const RME_GPC3_IMPLEMENTED: u8 = 0b0011;
    const SEL2_IMPLEMENTED: u8 = 0b0001;

    /// Indicates whether DIT Extension is implemented.
    pub fn is_feat_dit_present(self) -> bool {
        self.dit() >= Self::DIT_IMPLEMENTED
    }

    /// Indicates whether SVE is implemented.
    pub fn is_feat_sve_present(self) -> bool {
        self.sve() >= Self::SVE_IMPLEMENTED
    }

    /// Indicates whether MPAM Extension is implemented.
    pub fn is_feat_mpam_present(self) -> bool {
        self.mpam() >= Self::MPAM_IMPLEMENTED
    }

    /// Indicates whether AMUv1 Extension is implemented.
    pub fn is_feat_amuv1_present(&self) -> bool {
        self.amu() >= Self::AMUV1_IMPLEMENTED
    }

    /// Indicates whether AMUv1p1 Extension is implemented.
    pub fn is_feat_amuv1p1_present(&self) -> bool {
        self.amu() >= Self::AMUV1P1_IMPLEMENTED
    }

    /// Indicates whether FEAT_RME is implemented.
    pub fn is_feat_rme_present(self) -> bool {
        self.rme() >= Self::RME_IMPLEMENTED
    }

    /// Indicates whether FEAT_RME_GPC2 is implemented.
    pub fn is_feat_rme_gpc2_present(self) -> bool {
        self.rme() >= Self::RME_GPC2_IMPLEMENTED
    }

    /// Indicates whether FEAT_RME_GPC3 is implemented.
    pub fn is_feat_rme_gpc3_present(self) -> bool {
        self.rme() >= Self::RME_GPC3_IMPLEMENTED
    }

    /// Indicates whether FEAT_SEL2 is implemented.
    pub fn is_feat_sel2_present(self) -> bool {
        self.sel2() >= Self::SEL2_IMPLEMENTED
    }
}

impl IdAa64pfr1El1 {
    const SSBS_IMPLEMENTED: u8 = 0b0001;
    const MTE_IMPLEMENTED: u8 = 0b0001;
    const MTE2_IMPLEMENTED: u8 = 0b0010;
    const SME_IMPLEMENTED: u8 = 0b0001;
    const SME2_IMPLEMENTED: u8 = 0b0010;
    const NMI_IMPLEMENTED: u8 = 0b0001;
    const GCS_IMPLEMENTED: u8 = 0b0001;
    const PFAR_IMPLEMENTED: u8 = 0b0001;

    /// Indicates whether FEAT_PFAR is implemented.
    pub fn is_feat_pfar_present(self) -> bool {
        self.pfar() >= Self::PFAR_IMPLEMENTED
    }

    /// Indicates whether FEAT_SSBS is implemented.
    pub fn is_feat_ssbs_present(self) -> bool {
        self.ssbs() >= Self::SSBS_IMPLEMENTED
    }

    /// Indicates whether FEAT_MTE is implemented.
    pub fn is_feat_mte_present(self) -> bool {
        self.mte() >= Self::MTE_IMPLEMENTED
    }

    /// Indicates whether FEAT_MTE2 is implemented.
    pub fn is_feat_mte2_present(self) -> bool {
        self.mte() >= Self::MTE2_IMPLEMENTED
    }

    /// Indicates whether FEAT_SME is implemented.
    pub fn is_feat_sme_present(self) -> bool {
        self.sme() >= Self::SME_IMPLEMENTED
    }

    /// Indicates whether FEAT_SME2 is implemented.
    pub fn is_feat_sme2_present(self) -> bool {
        self.sme() >= Self::SME2_IMPLEMENTED
    }

    /// Indicates whether FEAT_NMI is implemented.
    pub fn is_feat_nmi_present(self) -> bool {
        self.nmi() >= Self::NMI_IMPLEMENTED
    }

    /// Indicates whether FEAT_GCS is implemented.
    pub fn is_feat_gcs_present(self) -> bool {
        self.gcs() >= Self::GCS_IMPLEMENTED
    }
}

impl IdAa64pfr2El1 {
    const FPMR_IMPLEMENTED: u8 = 0b0001;

    /// Indicates whether FEAT_FPMR is implemented.
    pub fn is_feat_fpmr_present(self) -> bool {
        self.fpmr() >= Self::FPMR_IMPLEMENTED
    }
}

impl MpidrEl1 {
    /// Size in bits of the affinity fields.
    pub const AFFINITY_BITS: usize = 8;

    /// Converts a PSCI MPIDR value into the equivalent `MpidrEL1` value.
    ///
    /// This reads the MT and U bits from the current CPU's MPIDR_EL1 value and combines them with
    /// the affinity values from the given `psci_mpidr`.
    ///
    /// This assumes that the MPIDR_EL1 values of all CPUs in a system have the same values for the
    /// MT and U bits.
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    pub fn from_psci_mpidr(psci_mpidr: u64) -> Self {
        let mpidr_el1 = read_mpidr_el1();
        Self::from_bits_retain(psci_mpidr) | (mpidr_el1 & (Self::MT | Self::U))
    }
}

impl SpsrEl1 {
    /// All of the N, Z, C and V bits.
    pub const NZCV: Self = Self::V.union(Self::C).union(Self::Z).union(Self::N);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]

    fn debug_mpidr_el1() {
        assert_eq!(format!("{:?}", MpidrEl1::empty()), "MpidrEl1(0x0)");
        assert_eq!(
            format!("{:?}", MpidrEl1::MT | MpidrEl1::U),
            "MpidrEl1(MT | U)"
        );
        assert_eq!(
            format!("{:?}", MpidrEl1::from_bits_retain(0x12_4134_5678)),
            "MpidrEl1(MT | U | 0x1200345678)"
        );
    }

    #[test]
    fn debug_spsr_el1() {
        assert_eq!(format!("{:?}", SpsrEl1::empty()), "SpsrEl1(0x0)");
        assert_eq!(format!("{:?}", SpsrEl1::NZCV), "SpsrEl1(V | C | Z | N)");
    }

    #[test]
    fn debug_esr_el1() {
        assert_eq!(format!("{:?}", EsrEl1::empty()), "EsrEl1(0x0)");
        assert_eq!(format!("{:?}", EsrEl1::IL), "EsrEl1(0x2000000)");
        assert_eq!(
            format!("{:?}", EsrEl1::ISS_SYSREG_OPCODE_MASK),
            "EsrEl1(0x3ffc1e)"
        );
    }
}
