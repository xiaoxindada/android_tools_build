// SPDX-FileCopyrightText: Copyright The arm-sysregs Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Access to Arm CPU system registers.

// This file is generated, do not edit manually.

// `unused_imports` is allowed because it is possible that not all of these macros are used in the
// generated output.
#[allow(unused_imports)]
use arm_sysregs_common::{read_sysreg, read_write_sysreg, write_sysreg};

read_sysreg!(amcfgr_el0: s3_3_c13_c2_1, u64: crate::registers::AmcfgrEl0, safe, crate::fake::SYSREGS);
read_sysreg!(amcg1idr_el0: s3_3_c13_c2_6, u64: crate::registers::Amcg1idrEl0, safe, crate::fake::SYSREGS);
read_sysreg!(amcgcr_el0: s3_3_c13_c2_2, u64: crate::registers::AmcgcrEl0, safe, crate::fake::SYSREGS);
read_write_sysreg!(amcntenclr0_el0: s3_3_c13_c2_4, u64: crate::registers::Amcntenclr0El0, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(amcntenclr1_el0: s3_3_c13_c3_0, u64: crate::registers::Amcntenclr1El0, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(amcntenset0_el0: s3_3_c13_c2_5, u64: crate::registers::Amcntenset0El0, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(amcntenset1_el0: s3_3_c13_c3_1, u64: crate::registers::Amcntenset1El0, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(amcr_el0: s3_3_c13_c2_0, u64: crate::registers::AmcrEl0, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(amevcntr00_el0: s3_3_c13_c4_0, u64: crate::registers::Amevcntr00El0, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(amevcntr01_el0: s3_3_c13_c4_1, u64: crate::registers::Amevcntr01El0, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(amevcntr02_el0: s3_3_c13_c4_2, u64: crate::registers::Amevcntr02El0, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(amevcntr03_el0: s3_3_c13_c4_3, u64: crate::registers::Amevcntr03El0, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(amevcntr10_el0: s3_3_c13_c12_0, u64: crate::registers::Amevcntr10El0, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(amevcntr110_el0: s3_3_c13_c13_2, u64: crate::registers::Amevcntr110El0, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(amevcntr111_el0: s3_3_c13_c13_3, u64: crate::registers::Amevcntr111El0, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(amevcntr112_el0: s3_3_c13_c13_4, u64: crate::registers::Amevcntr112El0, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(amevcntr113_el0: s3_3_c13_c13_5, u64: crate::registers::Amevcntr113El0, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(amevcntr114_el0: s3_3_c13_c13_6, u64: crate::registers::Amevcntr114El0, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(amevcntr115_el0: s3_3_c13_c13_7, u64: crate::registers::Amevcntr115El0, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(amevcntr11_el0: s3_3_c13_c12_1, u64: crate::registers::Amevcntr11El0, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(amevcntr12_el0: s3_3_c13_c12_2, u64: crate::registers::Amevcntr12El0, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(amevcntr13_el0: s3_3_c13_c12_3, u64: crate::registers::Amevcntr13El0, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(amevcntr14_el0: s3_3_c13_c12_4, u64: crate::registers::Amevcntr14El0, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(amevcntr15_el0: s3_3_c13_c12_5, u64: crate::registers::Amevcntr15El0, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(amevcntr16_el0: s3_3_c13_c12_6, u64: crate::registers::Amevcntr16El0, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(amevcntr17_el0: s3_3_c13_c12_7, u64: crate::registers::Amevcntr17El0, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(amevcntr18_el0: s3_3_c13_c13_0, u64: crate::registers::Amevcntr18El0, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(amevcntr19_el0: s3_3_c13_c13_1, u64: crate::registers::Amevcntr19El0, safe_read, safe_write, crate::fake::SYSREGS);
read_sysreg!(amevtyper00_el0: s3_3_c13_c6_0, u64: crate::registers::Amevtyper00El0, safe, crate::fake::SYSREGS);
read_sysreg!(amevtyper01_el0: s3_3_c13_c6_1, u64: crate::registers::Amevtyper01El0, safe, crate::fake::SYSREGS);
read_sysreg!(amevtyper02_el0: s3_3_c13_c6_2, u64: crate::registers::Amevtyper02El0, safe, crate::fake::SYSREGS);
read_sysreg!(amevtyper03_el0: s3_3_c13_c6_3, u64: crate::registers::Amevtyper03El0, safe, crate::fake::SYSREGS);
read_write_sysreg!(amuserenr_el0: s3_3_c13_c2_3, u64: crate::registers::AmuserenrEl0, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(cntfrq_el0, u64: crate::registers::CntfrqEl0, safe_read, safe_write, crate::fake::SYSREGS);
read_sysreg!(cntpctss_el0: s3_3_c14_c0_5, u64: crate::registers::CntpctssEl0, safe, crate::fake::SYSREGS);
read_sysreg!(cntpct_el0, u64: crate::registers::CntpctEl0, safe, crate::fake::SYSREGS);
read_write_sysreg!(cntp_ctl_el0, u64: crate::registers::CntpCtlEl0, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(cntp_cval_el0, u64: crate::registers::CntpCvalEl0, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(cntp_tval_el0, u64: crate::registers::CntpTvalEl0, safe_read, safe_write, crate::fake::SYSREGS);
read_sysreg!(cntvctss_el0: s3_3_c14_c0_6, u64: crate::registers::CntvctssEl0, safe, crate::fake::SYSREGS);
read_sysreg!(cntvct_el0, u64: crate::registers::CntvctEl0, safe, crate::fake::SYSREGS);
read_write_sysreg!(cntv_ctl_el0, u64: crate::registers::CntvCtlEl0, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(cntv_cval_el0, u64: crate::registers::CntvCvalEl0, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(cntv_tval_el0, u64: crate::registers::CntvTvalEl0, safe_read, safe_write, crate::fake::SYSREGS);
read_sysreg!(ctr_el0, u64: crate::registers::CtrEl0, safe, crate::fake::SYSREGS);
read_sysreg!(currentel, u64: crate::registers::Currentel, safe, crate::fake::SYSREGS);
read_write_sysreg!(daif, u64: crate::registers::Daif, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(dit: s3_3_c4_c2_5, u64: crate::registers::Dit, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(fpcr, u64: crate::registers::Fpcr, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(fpmr: s3_3_c4_c4_2, u64: crate::registers::Fpmr, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(fpsr, u64: crate::registers::Fpsr, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(gcspr_el0: s3_3_c2_c5_1, u64: crate::registers::GcsprEl0, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(pmcr_el0: s3_3_c9_c12_0, u64: crate::registers::PmcrEl0, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(por_el0: s3_3_c10_c2_4, u64: crate::registers::PorEl0, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(svcr: s3_3_c4_c2_2, u64: crate::registers::Svcr, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(tpidrro_el0, u64: crate::registers::TpidrroEl0, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(tpidr_el0, u64: crate::registers::TpidrEl0, safe_read, crate::fake::SYSREGS);
