// SPDX-FileCopyrightText: Copyright The arm-sysregs Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Example to log all readable system register values.

// This file is generated, do not edit manually.

#![no_std]
#![cfg_attr(not(any(test, feature = "fakes")), no_main)]

#[cfg(all(target_arch = "aarch64", target_os = "none"))]
use aarch64_rt::entry;
#[cfg(not(any(test, feature = "fakes")))]
use core::panic::PanicInfo;
use log::info;

#[cfg(all(target_arch = "aarch64", target_os = "none"))]
entry!(entry);
#[cfg_attr(any(test, feature = "fakes"), allow(unused))]
fn entry(_: u64, _: u64, _: u64, _: u64) -> ! {
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "actlr = {:?}",
        arm_sysregs::aarch32::accessors::read_actlr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "actlr2 = {:?}",
        arm_sysregs::aarch32::accessors::read_actlr2()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "adfsr = {:?}",
        arm_sysregs::aarch32::accessors::read_adfsr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("aidr = {:?}", arm_sysregs::aarch32::accessors::read_aidr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "aifsr = {:?}",
        arm_sysregs::aarch32::accessors::read_aifsr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "amair0 = {:?}",
        arm_sysregs::aarch32::accessors::read_amair0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "amair1 = {:?}",
        arm_sysregs::aarch32::accessors::read_amair1()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "amcfgr = {:?}",
        arm_sysregs::aarch32::accessors::read_amcfgr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "amcgcr = {:?}",
        arm_sysregs::aarch32::accessors::read_amcgcr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "amcntenclr0 = {:?}",
        arm_sysregs::aarch32::accessors::read_amcntenclr0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "amcntenclr1 = {:?}",
        arm_sysregs::aarch32::accessors::read_amcntenclr1()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "amcntenset0 = {:?}",
        arm_sysregs::aarch32::accessors::read_amcntenset0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "amcntenset1 = {:?}",
        arm_sysregs::aarch32::accessors::read_amcntenset1()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("amcr = {:?}", arm_sysregs::aarch32::accessors::read_amcr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "amevcntr00 = {:?}",
        arm_sysregs::aarch32::accessors::read_amevcntr00()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "amevcntr01 = {:?}",
        arm_sysregs::aarch32::accessors::read_amevcntr01()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "amevcntr02 = {:?}",
        arm_sysregs::aarch32::accessors::read_amevcntr02()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "amevcntr03 = {:?}",
        arm_sysregs::aarch32::accessors::read_amevcntr03()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "amevtyper00 = {:?}",
        arm_sysregs::aarch32::accessors::read_amevtyper00()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "amevtyper01 = {:?}",
        arm_sysregs::aarch32::accessors::read_amevtyper01()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "amevtyper02 = {:?}",
        arm_sysregs::aarch32::accessors::read_amevtyper02()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "amevtyper03 = {:?}",
        arm_sysregs::aarch32::accessors::read_amevtyper03()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "amevtyper10 = {:?}",
        arm_sysregs::aarch32::accessors::read_amevtyper10()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "amevtyper11 = {:?}",
        arm_sysregs::aarch32::accessors::read_amevtyper11()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "amevtyper110 = {:?}",
        arm_sysregs::aarch32::accessors::read_amevtyper110()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "amevtyper111 = {:?}",
        arm_sysregs::aarch32::accessors::read_amevtyper111()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "amevtyper112 = {:?}",
        arm_sysregs::aarch32::accessors::read_amevtyper112()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "amevtyper113 = {:?}",
        arm_sysregs::aarch32::accessors::read_amevtyper113()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "amevtyper114 = {:?}",
        arm_sysregs::aarch32::accessors::read_amevtyper114()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "amevtyper115 = {:?}",
        arm_sysregs::aarch32::accessors::read_amevtyper115()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "amevtyper12 = {:?}",
        arm_sysregs::aarch32::accessors::read_amevtyper12()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "amevtyper13 = {:?}",
        arm_sysregs::aarch32::accessors::read_amevtyper13()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "amevtyper14 = {:?}",
        arm_sysregs::aarch32::accessors::read_amevtyper14()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "amevtyper15 = {:?}",
        arm_sysregs::aarch32::accessors::read_amevtyper15()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "amevtyper16 = {:?}",
        arm_sysregs::aarch32::accessors::read_amevtyper16()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "amevtyper17 = {:?}",
        arm_sysregs::aarch32::accessors::read_amevtyper17()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "amevtyper18 = {:?}",
        arm_sysregs::aarch32::accessors::read_amevtyper18()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "amevtyper19 = {:?}",
        arm_sysregs::aarch32::accessors::read_amevtyper19()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "amuserenr = {:?}",
        arm_sysregs::aarch32::accessors::read_amuserenr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "ccsidr = {:?}",
        arm_sysregs::aarch32::accessors::read_ccsidr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "ccsidr2 = {:?}",
        arm_sysregs::aarch32::accessors::read_ccsidr2()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "clidr = {:?}",
        arm_sysregs::aarch32::accessors::read_clidr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "cntfrq = {:?}",
        arm_sysregs::aarch32::accessors::read_cntfrq()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "cnthctl = {:?}",
        arm_sysregs::aarch32::accessors::read_cnthctl()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "cnthps_ctl = {:?}",
        arm_sysregs::aarch32::accessors::read_cnthps_ctl()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "cnthps_cval = {:?}",
        arm_sysregs::aarch32::accessors::read_cnthps_cval()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "cnthps_tval = {:?}",
        arm_sysregs::aarch32::accessors::read_cnthps_tval()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "cnthp_ctl = {:?}",
        arm_sysregs::aarch32::accessors::read_cnthp_ctl()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "cnthp_cval = {:?}",
        arm_sysregs::aarch32::accessors::read_cnthp_cval()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "cnthp_tval = {:?}",
        arm_sysregs::aarch32::accessors::read_cnthp_tval()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "cnthvs_ctl = {:?}",
        arm_sysregs::aarch32::accessors::read_cnthvs_ctl()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "cnthvs_cval = {:?}",
        arm_sysregs::aarch32::accessors::read_cnthvs_cval()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "cnthvs_tval = {:?}",
        arm_sysregs::aarch32::accessors::read_cnthvs_tval()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "cnthv_ctl = {:?}",
        arm_sysregs::aarch32::accessors::read_cnthv_ctl()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "cnthv_cval = {:?}",
        arm_sysregs::aarch32::accessors::read_cnthv_cval()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "cnthv_tval = {:?}",
        arm_sysregs::aarch32::accessors::read_cnthv_tval()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "cntkctl = {:?}",
        arm_sysregs::aarch32::accessors::read_cntkctl()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "cntpct = {:?}",
        arm_sysregs::aarch32::accessors::read_cntpct()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "cntpctss = {:?}",
        arm_sysregs::aarch32::accessors::read_cntpctss()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "cntp_ctl = {:?}",
        arm_sysregs::aarch32::accessors::read_cntp_ctl()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "cntp_cval = {:?}",
        arm_sysregs::aarch32::accessors::read_cntp_cval()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "cntp_tval = {:?}",
        arm_sysregs::aarch32::accessors::read_cntp_tval()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "cntvct = {:?}",
        arm_sysregs::aarch32::accessors::read_cntvct()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "cntvctss = {:?}",
        arm_sysregs::aarch32::accessors::read_cntvctss()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "cntvoff = {:?}",
        arm_sysregs::aarch32::accessors::read_cntvoff()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "cntv_ctl = {:?}",
        arm_sysregs::aarch32::accessors::read_cntv_ctl()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "cntv_cval = {:?}",
        arm_sysregs::aarch32::accessors::read_cntv_cval()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "cntv_tval = {:?}",
        arm_sysregs::aarch32::accessors::read_cntv_tval()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "contextidr = {:?}",
        arm_sysregs::aarch32::accessors::read_contextidr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "cpacr = {:?}",
        arm_sysregs::aarch32::accessors::read_cpacr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "csselr = {:?}",
        arm_sysregs::aarch32::accessors::read_csselr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("ctr = {:?}", arm_sysregs::aarch32::accessors::read_ctr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("dacr = {:?}", arm_sysregs::aarch32::accessors::read_dacr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "dbgauthstatus = {:?}",
        arm_sysregs::aarch32::accessors::read_dbgauthstatus()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "dbgclaimclr = {:?}",
        arm_sysregs::aarch32::accessors::read_dbgclaimclr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "dbgclaimset = {:?}",
        arm_sysregs::aarch32::accessors::read_dbgclaimset()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "dbgdccint = {:?}",
        arm_sysregs::aarch32::accessors::read_dbgdccint()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "dbgdevid = {:?}",
        arm_sysregs::aarch32::accessors::read_dbgdevid()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "dbgdevid1 = {:?}",
        arm_sysregs::aarch32::accessors::read_dbgdevid1()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "dbgdevid2 = {:?}",
        arm_sysregs::aarch32::accessors::read_dbgdevid2()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "dbgdidr = {:?}",
        arm_sysregs::aarch32::accessors::read_dbgdidr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "dbgdrar = {:?}",
        arm_sysregs::aarch32::accessors::read_dbgdrar()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "dbgdsar = {:?}",
        arm_sysregs::aarch32::accessors::read_dbgdsar()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "dbgdscrext = {:?}",
        arm_sysregs::aarch32::accessors::read_dbgdscrext()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "dbgdscrint = {:?}",
        arm_sysregs::aarch32::accessors::read_dbgdscrint()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "dbgdtrrxext = {:?}",
        arm_sysregs::aarch32::accessors::read_dbgdtrrxext()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "dbgdtrrxint = {:?}",
        arm_sysregs::aarch32::accessors::read_dbgdtrrxint()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "dbgdtrtxext = {:?}",
        arm_sysregs::aarch32::accessors::read_dbgdtrtxext()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "dbgosdlr = {:?}",
        arm_sysregs::aarch32::accessors::read_dbgosdlr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "dbgoseccr = {:?}",
        arm_sysregs::aarch32::accessors::read_dbgoseccr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "dbgoslsr = {:?}",
        arm_sysregs::aarch32::accessors::read_dbgoslsr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "dbgprcr = {:?}",
        arm_sysregs::aarch32::accessors::read_dbgprcr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "dbgvcr = {:?}",
        arm_sysregs::aarch32::accessors::read_dbgvcr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "dbgwfar = {:?}",
        arm_sysregs::aarch32::accessors::read_dbgwfar()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("dfar = {:?}", arm_sysregs::aarch32::accessors::read_dfar());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("dfsr = {:?}", arm_sysregs::aarch32::accessors::read_dfsr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("disr = {:?}", arm_sysregs::aarch32::accessors::read_disr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("dlr = {:?}", arm_sysregs::aarch32::accessors::read_dlr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "dspsr = {:?}",
        arm_sysregs::aarch32::accessors::read_dspsr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "dspsr2 = {:?}",
        arm_sysregs::aarch32::accessors::read_dspsr2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "arm"), feature = "el2"))]
    info!(
        "elr_hyp = {:?}",
        arm_sysregs::aarch32::accessors::read_elr_hyp()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "erridr = {:?}",
        arm_sysregs::aarch32::accessors::read_erridr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "errselr = {:?}",
        arm_sysregs::aarch32::accessors::read_errselr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "erxaddr = {:?}",
        arm_sysregs::aarch32::accessors::read_erxaddr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "erxaddr2 = {:?}",
        arm_sysregs::aarch32::accessors::read_erxaddr2()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "erxctlr = {:?}",
        arm_sysregs::aarch32::accessors::read_erxctlr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "erxctlr2 = {:?}",
        arm_sysregs::aarch32::accessors::read_erxctlr2()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "erxfr = {:?}",
        arm_sysregs::aarch32::accessors::read_erxfr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "erxfr2 = {:?}",
        arm_sysregs::aarch32::accessors::read_erxfr2()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "erxmisc0 = {:?}",
        arm_sysregs::aarch32::accessors::read_erxmisc0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "erxmisc1 = {:?}",
        arm_sysregs::aarch32::accessors::read_erxmisc1()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "erxmisc2 = {:?}",
        arm_sysregs::aarch32::accessors::read_erxmisc2()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "erxmisc3 = {:?}",
        arm_sysregs::aarch32::accessors::read_erxmisc3()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "erxmisc4 = {:?}",
        arm_sysregs::aarch32::accessors::read_erxmisc4()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "erxmisc5 = {:?}",
        arm_sysregs::aarch32::accessors::read_erxmisc5()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "erxmisc6 = {:?}",
        arm_sysregs::aarch32::accessors::read_erxmisc6()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "erxmisc7 = {:?}",
        arm_sysregs::aarch32::accessors::read_erxmisc7()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "erxstatus = {:?}",
        arm_sysregs::aarch32::accessors::read_erxstatus()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "fcseidr = {:?}",
        arm_sysregs::aarch32::accessors::read_fcseidr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("hacr = {:?}", arm_sysregs::aarch32::accessors::read_hacr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "hactlr = {:?}",
        arm_sysregs::aarch32::accessors::read_hactlr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "hactlr2 = {:?}",
        arm_sysregs::aarch32::accessors::read_hactlr2()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "hadfsr = {:?}",
        arm_sysregs::aarch32::accessors::read_hadfsr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "haifsr = {:?}",
        arm_sysregs::aarch32::accessors::read_haifsr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "hamair0 = {:?}",
        arm_sysregs::aarch32::accessors::read_hamair0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "hamair1 = {:?}",
        arm_sysregs::aarch32::accessors::read_hamair1()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "hcptr = {:?}",
        arm_sysregs::aarch32::accessors::read_hcptr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("hcr = {:?}", arm_sysregs::aarch32::accessors::read_hcr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("hcr2 = {:?}", arm_sysregs::aarch32::accessors::read_hcr2());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("hdcr = {:?}", arm_sysregs::aarch32::accessors::read_hdcr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "hdfar = {:?}",
        arm_sysregs::aarch32::accessors::read_hdfar()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "hifar = {:?}",
        arm_sysregs::aarch32::accessors::read_hifar()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "hmair0 = {:?}",
        arm_sysregs::aarch32::accessors::read_hmair0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "hmair1 = {:?}",
        arm_sysregs::aarch32::accessors::read_hmair1()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "hpfar = {:?}",
        arm_sysregs::aarch32::accessors::read_hpfar()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("hrmr = {:?}", arm_sysregs::aarch32::accessors::read_hrmr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "hsctlr = {:?}",
        arm_sysregs::aarch32::accessors::read_hsctlr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("hsr = {:?}", arm_sysregs::aarch32::accessors::read_hsr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("hstr = {:?}", arm_sysregs::aarch32::accessors::read_hstr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("htcr = {:?}", arm_sysregs::aarch32::accessors::read_htcr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "htpidr = {:?}",
        arm_sysregs::aarch32::accessors::read_htpidr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "htrfcr = {:?}",
        arm_sysregs::aarch32::accessors::read_htrfcr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "httbr = {:?}",
        arm_sysregs::aarch32::accessors::read_httbr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "hvbar = {:?}",
        arm_sysregs::aarch32::accessors::read_hvbar()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "icc_bpr0 = {:?}",
        arm_sysregs::aarch32::accessors::read_icc_bpr0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "icc_bpr1 = {:?}",
        arm_sysregs::aarch32::accessors::read_icc_bpr1()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "icc_ctlr = {:?}",
        arm_sysregs::aarch32::accessors::read_icc_ctlr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "icc_hppir0 = {:?}",
        arm_sysregs::aarch32::accessors::read_icc_hppir0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "icc_hppir1 = {:?}",
        arm_sysregs::aarch32::accessors::read_icc_hppir1()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "icc_hsre = {:?}",
        arm_sysregs::aarch32::accessors::read_icc_hsre()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "icc_iar0 = {:?}",
        arm_sysregs::aarch32::accessors::read_icc_iar0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "icc_iar1 = {:?}",
        arm_sysregs::aarch32::accessors::read_icc_iar1()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "icc_igrpen0 = {:?}",
        arm_sysregs::aarch32::accessors::read_icc_igrpen0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "icc_igrpen1 = {:?}",
        arm_sysregs::aarch32::accessors::read_icc_igrpen1()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "icc_mctlr = {:?}",
        arm_sysregs::aarch32::accessors::read_icc_mctlr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "icc_mgrpen1 = {:?}",
        arm_sysregs::aarch32::accessors::read_icc_mgrpen1()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "icc_msre = {:?}",
        arm_sysregs::aarch32::accessors::read_icc_msre()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "icc_pmr = {:?}",
        arm_sysregs::aarch32::accessors::read_icc_pmr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "icc_rpr = {:?}",
        arm_sysregs::aarch32::accessors::read_icc_rpr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "icc_sre = {:?}",
        arm_sysregs::aarch32::accessors::read_icc_sre()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "id_afr0 = {:?}",
        arm_sysregs::aarch32::accessors::read_id_afr0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "id_dfr0 = {:?}",
        arm_sysregs::aarch32::accessors::read_id_dfr0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "id_dfr1 = {:?}",
        arm_sysregs::aarch32::accessors::read_id_dfr1()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "id_isar0 = {:?}",
        arm_sysregs::aarch32::accessors::read_id_isar0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "id_isar1 = {:?}",
        arm_sysregs::aarch32::accessors::read_id_isar1()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "id_isar2 = {:?}",
        arm_sysregs::aarch32::accessors::read_id_isar2()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "id_isar3 = {:?}",
        arm_sysregs::aarch32::accessors::read_id_isar3()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "id_isar4 = {:?}",
        arm_sysregs::aarch32::accessors::read_id_isar4()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "id_isar5 = {:?}",
        arm_sysregs::aarch32::accessors::read_id_isar5()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "id_isar6 = {:?}",
        arm_sysregs::aarch32::accessors::read_id_isar6()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "id_mmfr0 = {:?}",
        arm_sysregs::aarch32::accessors::read_id_mmfr0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "id_mmfr1 = {:?}",
        arm_sysregs::aarch32::accessors::read_id_mmfr1()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "id_mmfr2 = {:?}",
        arm_sysregs::aarch32::accessors::read_id_mmfr2()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "id_mmfr3 = {:?}",
        arm_sysregs::aarch32::accessors::read_id_mmfr3()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "id_mmfr4 = {:?}",
        arm_sysregs::aarch32::accessors::read_id_mmfr4()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "id_mmfr5 = {:?}",
        arm_sysregs::aarch32::accessors::read_id_mmfr5()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "id_pfr0 = {:?}",
        arm_sysregs::aarch32::accessors::read_id_pfr0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "id_pfr1 = {:?}",
        arm_sysregs::aarch32::accessors::read_id_pfr1()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "id_pfr2 = {:?}",
        arm_sysregs::aarch32::accessors::read_id_pfr2()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("ifar = {:?}", arm_sysregs::aarch32::accessors::read_ifar());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("ifsr = {:?}", arm_sysregs::aarch32::accessors::read_ifsr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("isr = {:?}", arm_sysregs::aarch32::accessors::read_isr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("jidr = {:?}", arm_sysregs::aarch32::accessors::read_jidr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("jmcr = {:?}", arm_sysregs::aarch32::accessors::read_jmcr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "joscr = {:?}",
        arm_sysregs::aarch32::accessors::read_joscr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "mair0 = {:?}",
        arm_sysregs::aarch32::accessors::read_mair0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "mair1 = {:?}",
        arm_sysregs::aarch32::accessors::read_mair1()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("midr = {:?}", arm_sysregs::aarch32::accessors::read_midr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "mpidr = {:?}",
        arm_sysregs::aarch32::accessors::read_mpidr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "mvbar = {:?}",
        arm_sysregs::aarch32::accessors::read_mvbar()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("nmrr = {:?}", arm_sysregs::aarch32::accessors::read_nmrr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "nsacr = {:?}",
        arm_sysregs::aarch32::accessors::read_nsacr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("par = {:?}", arm_sysregs::aarch32::accessors::read_par());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "pmccfiltr = {:?}",
        arm_sysregs::aarch32::accessors::read_pmccfiltr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "pmccntr = {:?}",
        arm_sysregs::aarch32::accessors::read_pmccntr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "pmceid0 = {:?}",
        arm_sysregs::aarch32::accessors::read_pmceid0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "pmceid1 = {:?}",
        arm_sysregs::aarch32::accessors::read_pmceid1()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "pmceid2 = {:?}",
        arm_sysregs::aarch32::accessors::read_pmceid2()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "pmceid3 = {:?}",
        arm_sysregs::aarch32::accessors::read_pmceid3()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "pmcntenclr = {:?}",
        arm_sysregs::aarch32::accessors::read_pmcntenclr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "pmcntenset = {:?}",
        arm_sysregs::aarch32::accessors::read_pmcntenset()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("pmcr = {:?}", arm_sysregs::aarch32::accessors::read_pmcr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "pmintenclr = {:?}",
        arm_sysregs::aarch32::accessors::read_pmintenclr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "pmintenset = {:?}",
        arm_sysregs::aarch32::accessors::read_pmintenset()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "pmmir = {:?}",
        arm_sysregs::aarch32::accessors::read_pmmir()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "pmovsr = {:?}",
        arm_sysregs::aarch32::accessors::read_pmovsr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "pmovsset = {:?}",
        arm_sysregs::aarch32::accessors::read_pmovsset()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "pmselr = {:?}",
        arm_sysregs::aarch32::accessors::read_pmselr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "pmuserenr = {:?}",
        arm_sysregs::aarch32::accessors::read_pmuserenr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "pmxevtyper = {:?}",
        arm_sysregs::aarch32::accessors::read_pmxevtyper()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("prrr = {:?}", arm_sysregs::aarch32::accessors::read_prrr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "revidr = {:?}",
        arm_sysregs::aarch32::accessors::read_revidr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("rmr = {:?}", arm_sysregs::aarch32::accessors::read_rmr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "rvbar = {:?}",
        arm_sysregs::aarch32::accessors::read_rvbar()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("scr = {:?}", arm_sysregs::aarch32::accessors::read_scr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "sctlr = {:?}",
        arm_sysregs::aarch32::accessors::read_sctlr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("sdcr = {:?}", arm_sysregs::aarch32::accessors::read_sdcr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("sder = {:?}", arm_sysregs::aarch32::accessors::read_sder());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "tcmtr = {:?}",
        arm_sysregs::aarch32::accessors::read_tcmtr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "tlbtr = {:?}",
        arm_sysregs::aarch32::accessors::read_tlbtr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "tpidrprw = {:?}",
        arm_sysregs::aarch32::accessors::read_tpidrprw()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "tpidruro = {:?}",
        arm_sysregs::aarch32::accessors::read_tpidruro()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "tpidrurw = {:?}",
        arm_sysregs::aarch32::accessors::read_tpidrurw()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "trfcr = {:?}",
        arm_sysregs::aarch32::accessors::read_trfcr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "ttbcr = {:?}",
        arm_sysregs::aarch32::accessors::read_ttbcr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "ttbcr2 = {:?}",
        arm_sysregs::aarch32::accessors::read_ttbcr2()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "ttbr0 = {:?}",
        arm_sysregs::aarch32::accessors::read_ttbr0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "ttbr1 = {:?}",
        arm_sysregs::aarch32::accessors::read_ttbr1()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("vbar = {:?}", arm_sysregs::aarch32::accessors::read_vbar());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "vdfsr = {:?}",
        arm_sysregs::aarch32::accessors::read_vdfsr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "vdisr = {:?}",
        arm_sysregs::aarch32::accessors::read_vdisr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "vmpidr = {:?}",
        arm_sysregs::aarch32::accessors::read_vmpidr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "vpidr = {:?}",
        arm_sysregs::aarch32::accessors::read_vpidr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("vtcr = {:?}", arm_sysregs::aarch32::accessors::read_vtcr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!(
        "vttbr = {:?}",
        arm_sysregs::aarch32::accessors::read_vttbr()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!(
        "amcfgr_el0 = {:?}",
        arm_sysregs::el0::accessors::read_amcfgr_el0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!(
        "amcg1idr_el0 = {:?}",
        arm_sysregs::el0::accessors::read_amcg1idr_el0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!(
        "amcgcr_el0 = {:?}",
        arm_sysregs::el0::accessors::read_amcgcr_el0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!(
        "amcntenclr0_el0 = {:?}",
        arm_sysregs::el0::accessors::read_amcntenclr0_el0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!(
        "amcntenclr1_el0 = {:?}",
        arm_sysregs::el0::accessors::read_amcntenclr1_el0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!(
        "amcntenset0_el0 = {:?}",
        arm_sysregs::el0::accessors::read_amcntenset0_el0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!(
        "amcntenset1_el0 = {:?}",
        arm_sysregs::el0::accessors::read_amcntenset1_el0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!(
        "amcr_el0 = {:?}",
        arm_sysregs::el0::accessors::read_amcr_el0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!(
        "amevcntr00_el0 = {:?}",
        arm_sysregs::el0::accessors::read_amevcntr00_el0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!(
        "amevcntr01_el0 = {:?}",
        arm_sysregs::el0::accessors::read_amevcntr01_el0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!(
        "amevcntr02_el0 = {:?}",
        arm_sysregs::el0::accessors::read_amevcntr02_el0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!(
        "amevcntr03_el0 = {:?}",
        arm_sysregs::el0::accessors::read_amevcntr03_el0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!(
        "amevcntr10_el0 = {:?}",
        arm_sysregs::el0::accessors::read_amevcntr10_el0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!(
        "amevcntr110_el0 = {:?}",
        arm_sysregs::el0::accessors::read_amevcntr110_el0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!(
        "amevcntr111_el0 = {:?}",
        arm_sysregs::el0::accessors::read_amevcntr111_el0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!(
        "amevcntr112_el0 = {:?}",
        arm_sysregs::el0::accessors::read_amevcntr112_el0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!(
        "amevcntr113_el0 = {:?}",
        arm_sysregs::el0::accessors::read_amevcntr113_el0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!(
        "amevcntr114_el0 = {:?}",
        arm_sysregs::el0::accessors::read_amevcntr114_el0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!(
        "amevcntr115_el0 = {:?}",
        arm_sysregs::el0::accessors::read_amevcntr115_el0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!(
        "amevcntr11_el0 = {:?}",
        arm_sysregs::el0::accessors::read_amevcntr11_el0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!(
        "amevcntr12_el0 = {:?}",
        arm_sysregs::el0::accessors::read_amevcntr12_el0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!(
        "amevcntr13_el0 = {:?}",
        arm_sysregs::el0::accessors::read_amevcntr13_el0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!(
        "amevcntr14_el0 = {:?}",
        arm_sysregs::el0::accessors::read_amevcntr14_el0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!(
        "amevcntr15_el0 = {:?}",
        arm_sysregs::el0::accessors::read_amevcntr15_el0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!(
        "amevcntr16_el0 = {:?}",
        arm_sysregs::el0::accessors::read_amevcntr16_el0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!(
        "amevcntr17_el0 = {:?}",
        arm_sysregs::el0::accessors::read_amevcntr17_el0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!(
        "amevcntr18_el0 = {:?}",
        arm_sysregs::el0::accessors::read_amevcntr18_el0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!(
        "amevcntr19_el0 = {:?}",
        arm_sysregs::el0::accessors::read_amevcntr19_el0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!(
        "amevtyper00_el0 = {:?}",
        arm_sysregs::el0::accessors::read_amevtyper00_el0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!(
        "amevtyper01_el0 = {:?}",
        arm_sysregs::el0::accessors::read_amevtyper01_el0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!(
        "amevtyper02_el0 = {:?}",
        arm_sysregs::el0::accessors::read_amevtyper02_el0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!(
        "amevtyper03_el0 = {:?}",
        arm_sysregs::el0::accessors::read_amevtyper03_el0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!(
        "amuserenr_el0 = {:?}",
        arm_sysregs::el0::accessors::read_amuserenr_el0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!(
        "cntfrq_el0 = {:?}",
        arm_sysregs::el0::accessors::read_cntfrq_el0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!(
        "cntpctss_el0 = {:?}",
        arm_sysregs::el0::accessors::read_cntpctss_el0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!(
        "cntpct_el0 = {:?}",
        arm_sysregs::el0::accessors::read_cntpct_el0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!(
        "cntp_ctl_el0 = {:?}",
        arm_sysregs::el0::accessors::read_cntp_ctl_el0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!(
        "cntp_cval_el0 = {:?}",
        arm_sysregs::el0::accessors::read_cntp_cval_el0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!(
        "cntp_tval_el0 = {:?}",
        arm_sysregs::el0::accessors::read_cntp_tval_el0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!(
        "cntvctss_el0 = {:?}",
        arm_sysregs::el0::accessors::read_cntvctss_el0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!(
        "cntvct_el0 = {:?}",
        arm_sysregs::el0::accessors::read_cntvct_el0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!(
        "cntv_ctl_el0 = {:?}",
        arm_sysregs::el0::accessors::read_cntv_ctl_el0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!(
        "cntv_cval_el0 = {:?}",
        arm_sysregs::el0::accessors::read_cntv_cval_el0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!(
        "cntv_tval_el0 = {:?}",
        arm_sysregs::el0::accessors::read_cntv_tval_el0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!(
        "ctr_el0 = {:?}",
        arm_sysregs::el0::accessors::read_ctr_el0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!(
        "currentel = {:?}",
        arm_sysregs::el0::accessors::read_currentel()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!("daif = {:?}", arm_sysregs::el0::accessors::read_daif());
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!("dit = {:?}", arm_sysregs::el0::accessors::read_dit());
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!("fpcr = {:?}", arm_sysregs::el0::accessors::read_fpcr());
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!("fpmr = {:?}", arm_sysregs::el0::accessors::read_fpmr());
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!("fpsr = {:?}", arm_sysregs::el0::accessors::read_fpsr());
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!(
        "gcspr_el0 = {:?}",
        arm_sysregs::el0::accessors::read_gcspr_el0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!(
        "pmcr_el0 = {:?}",
        arm_sysregs::el0::accessors::read_pmcr_el0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!(
        "por_el0 = {:?}",
        arm_sysregs::el0::accessors::read_por_el0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!("svcr = {:?}", arm_sysregs::el0::accessors::read_svcr());
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!(
        "tpidrro_el0 = {:?}",
        arm_sysregs::el0::accessors::read_tpidrro_el0()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!(
        "tpidr_el0 = {:?}",
        arm_sysregs::el0::accessors::read_tpidr_el0()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "actlr_el1 = {:?}",
        arm_sysregs::el1::accessors::read_actlr_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "afsr0_el1 = {:?}",
        arm_sysregs::el1::accessors::read_afsr0_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "afsr1_el1 = {:?}",
        arm_sysregs::el1::accessors::read_afsr1_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "amair_el1 = {:?}",
        arm_sysregs::el1::accessors::read_amair_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "apiakeyhi_el1 = {:?}",
        arm_sysregs::el1::accessors::read_apiakeyhi_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "apiakeylo_el1 = {:?}",
        arm_sysregs::el1::accessors::read_apiakeylo_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "ccsidr_el1 = {:?}",
        arm_sysregs::el1::accessors::read_ccsidr_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "clidr_el1 = {:?}",
        arm_sysregs::el1::accessors::read_clidr_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "cntkctl_el1 = {:?}",
        arm_sysregs::el1::accessors::read_cntkctl_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "cntps_ctl_el1 = {:?}",
        arm_sysregs::el1::accessors::read_cntps_ctl_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "cntps_cval_el1 = {:?}",
        arm_sysregs::el1::accessors::read_cntps_cval_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "cntps_tval_el1 = {:?}",
        arm_sysregs::el1::accessors::read_cntps_tval_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "contextidr_el1 = {:?}",
        arm_sysregs::el1::accessors::read_contextidr_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "cpacr_el1 = {:?}",
        arm_sysregs::el1::accessors::read_cpacr_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "csselr_el1 = {:?}",
        arm_sysregs::el1::accessors::read_csselr_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "disr_el1 = {:?}",
        arm_sysregs::el1::accessors::read_disr_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "elr_el1 = {:?}",
        arm_sysregs::el1::accessors::read_elr_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "esr_el1 = {:?}",
        arm_sysregs::el1::accessors::read_esr_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "far_el1 = {:?}",
        arm_sysregs::el1::accessors::read_far_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "gcr_el1 = {:?}",
        arm_sysregs::el1::accessors::read_gcr_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "gcscre0_el1 = {:?}",
        arm_sysregs::el1::accessors::read_gcscre0_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "gcscr_el1 = {:?}",
        arm_sysregs::el1::accessors::read_gcscr_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "gcspr_el1 = {:?}",
        arm_sysregs::el1::accessors::read_gcspr_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "icc_ap0r0_el1 = {:?}",
        arm_sysregs::el1::accessors::read_icc_ap0r0_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "icc_ap0r1_el1 = {:?}",
        arm_sysregs::el1::accessors::read_icc_ap0r1_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "icc_ap0r2_el1 = {:?}",
        arm_sysregs::el1::accessors::read_icc_ap0r2_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "icc_ap0r3_el1 = {:?}",
        arm_sysregs::el1::accessors::read_icc_ap0r3_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "icc_ap1r0_el1 = {:?}",
        arm_sysregs::el1::accessors::read_icc_ap1r0_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "icc_ap1r1_el1 = {:?}",
        arm_sysregs::el1::accessors::read_icc_ap1r1_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "icc_ap1r2_el1 = {:?}",
        arm_sysregs::el1::accessors::read_icc_ap1r2_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "icc_ap1r3_el1 = {:?}",
        arm_sysregs::el1::accessors::read_icc_ap1r3_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "icc_bpr0_el1 = {:?}",
        arm_sysregs::el1::accessors::read_icc_bpr0_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "icc_bpr1_el1 = {:?}",
        arm_sysregs::el1::accessors::read_icc_bpr1_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "icc_ctlr_el1 = {:?}",
        arm_sysregs::el1::accessors::read_icc_ctlr_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "icc_hppir0_el1 = {:?}",
        arm_sysregs::el1::accessors::read_icc_hppir0_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "icc_hppir1_el1 = {:?}",
        arm_sysregs::el1::accessors::read_icc_hppir1_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "icc_iar0_el1 = {:?}",
        arm_sysregs::el1::accessors::read_icc_iar0_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "icc_iar1_el1 = {:?}",
        arm_sysregs::el1::accessors::read_icc_iar1_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "icc_igrpen0_el1 = {:?}",
        arm_sysregs::el1::accessors::read_icc_igrpen0_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "icc_igrpen1_el1 = {:?}",
        arm_sysregs::el1::accessors::read_icc_igrpen1_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "icc_nmiar1_el1 = {:?}",
        arm_sysregs::el1::accessors::read_icc_nmiar1_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "icc_pmr_el1 = {:?}",
        arm_sysregs::el1::accessors::read_icc_pmr_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "icc_rpr_el1 = {:?}",
        arm_sysregs::el1::accessors::read_icc_rpr_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "icc_sre_el1 = {:?}",
        arm_sysregs::el1::accessors::read_icc_sre_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "id_aa64dfr0_el1 = {:?}",
        arm_sysregs::el1::accessors::read_id_aa64dfr0_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "id_aa64dfr1_el1 = {:?}",
        arm_sysregs::el1::accessors::read_id_aa64dfr1_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "id_aa64isar1_el1 = {:?}",
        arm_sysregs::el1::accessors::read_id_aa64isar1_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "id_aa64isar2_el1 = {:?}",
        arm_sysregs::el1::accessors::read_id_aa64isar2_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "id_aa64isar3_el1 = {:?}",
        arm_sysregs::el1::accessors::read_id_aa64isar3_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "id_aa64mmfr0_el1 = {:?}",
        arm_sysregs::el1::accessors::read_id_aa64mmfr0_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "id_aa64mmfr1_el1 = {:?}",
        arm_sysregs::el1::accessors::read_id_aa64mmfr1_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "id_aa64mmfr2_el1 = {:?}",
        arm_sysregs::el1::accessors::read_id_aa64mmfr2_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "id_aa64mmfr3_el1 = {:?}",
        arm_sysregs::el1::accessors::read_id_aa64mmfr3_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "id_aa64mmfr4_el1 = {:?}",
        arm_sysregs::el1::accessors::read_id_aa64mmfr4_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "id_aa64pfr0_el1 = {:?}",
        arm_sysregs::el1::accessors::read_id_aa64pfr0_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "id_aa64pfr1_el1 = {:?}",
        arm_sysregs::el1::accessors::read_id_aa64pfr1_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "id_aa64pfr2_el1 = {:?}",
        arm_sysregs::el1::accessors::read_id_aa64pfr2_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "id_aa64smfr0_el1 = {:?}",
        arm_sysregs::el1::accessors::read_id_aa64smfr0_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "isr_el1 = {:?}",
        arm_sysregs::el1::accessors::read_isr_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "mair_el1 = {:?}",
        arm_sysregs::el1::accessors::read_mair_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "mdccint_el1 = {:?}",
        arm_sysregs::el1::accessors::read_mdccint_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "mdscr_el1 = {:?}",
        arm_sysregs::el1::accessors::read_mdscr_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "midr_el1 = {:?}",
        arm_sysregs::el1::accessors::read_midr_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "mpamidr_el1 = {:?}",
        arm_sysregs::el1::accessors::read_mpamidr_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "mpidr_el1 = {:?}",
        arm_sysregs::el1::accessors::read_mpidr_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "par_el1 = {:?}",
        arm_sysregs::el1::accessors::read_par_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "pfar_el1 = {:?}",
        arm_sysregs::el1::accessors::read_pfar_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "pire0_el1 = {:?}",
        arm_sysregs::el1::accessors::read_pire0_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "pir_el1 = {:?}",
        arm_sysregs::el1::accessors::read_pir_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "por_el1 = {:?}",
        arm_sysregs::el1::accessors::read_por_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "rgsr_el1 = {:?}",
        arm_sysregs::el1::accessors::read_rgsr_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "s2por_el1 = {:?}",
        arm_sysregs::el1::accessors::read_s2por_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "sctlr2_el1 = {:?}",
        arm_sysregs::el1::accessors::read_sctlr2_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "sctlr_el1 = {:?}",
        arm_sysregs::el1::accessors::read_sctlr_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "spsr_el1 = {:?}",
        arm_sysregs::el1::accessors::read_spsr_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!("sp_el1 = {:?}", arm_sysregs::el1::accessors::read_sp_el1());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "tcr2_el1 = {:?}",
        arm_sysregs::el1::accessors::read_tcr2_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "tcr_el1 = {:?}",
        arm_sysregs::el1::accessors::read_tcr_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "tfsre0_el1 = {:?}",
        arm_sysregs::el1::accessors::read_tfsre0_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "tfsr_el1 = {:?}",
        arm_sysregs::el1::accessors::read_tfsr_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "tpidr_el1 = {:?}",
        arm_sysregs::el1::accessors::read_tpidr_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "ttbr0_el1 = {:?}",
        arm_sysregs::el1::accessors::read_ttbr0_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "ttbr1_el1 = {:?}",
        arm_sysregs::el1::accessors::read_ttbr1_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "vbar_el1 = {:?}",
        arm_sysregs::el1::accessors::read_vbar_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "actlr_el2 = {:?}",
        arm_sysregs::el2::accessors::read_actlr_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "afsr0_el2 = {:?}",
        arm_sysregs::el2::accessors::read_afsr0_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "afsr1_el2 = {:?}",
        arm_sysregs::el2::accessors::read_afsr1_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "amair_el2 = {:?}",
        arm_sysregs::el2::accessors::read_amair_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "amevcntvoff00_el2 = {:?}",
        arm_sysregs::el2::accessors::read_amevcntvoff00_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "amevcntvoff010_el2 = {:?}",
        arm_sysregs::el2::accessors::read_amevcntvoff010_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "amevcntvoff011_el2 = {:?}",
        arm_sysregs::el2::accessors::read_amevcntvoff011_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "amevcntvoff012_el2 = {:?}",
        arm_sysregs::el2::accessors::read_amevcntvoff012_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "amevcntvoff013_el2 = {:?}",
        arm_sysregs::el2::accessors::read_amevcntvoff013_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "amevcntvoff014_el2 = {:?}",
        arm_sysregs::el2::accessors::read_amevcntvoff014_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "amevcntvoff015_el2 = {:?}",
        arm_sysregs::el2::accessors::read_amevcntvoff015_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "amevcntvoff01_el2 = {:?}",
        arm_sysregs::el2::accessors::read_amevcntvoff01_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "amevcntvoff02_el2 = {:?}",
        arm_sysregs::el2::accessors::read_amevcntvoff02_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "amevcntvoff03_el2 = {:?}",
        arm_sysregs::el2::accessors::read_amevcntvoff03_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "amevcntvoff04_el2 = {:?}",
        arm_sysregs::el2::accessors::read_amevcntvoff04_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "amevcntvoff05_el2 = {:?}",
        arm_sysregs::el2::accessors::read_amevcntvoff05_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "amevcntvoff06_el2 = {:?}",
        arm_sysregs::el2::accessors::read_amevcntvoff06_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "amevcntvoff07_el2 = {:?}",
        arm_sysregs::el2::accessors::read_amevcntvoff07_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "amevcntvoff08_el2 = {:?}",
        arm_sysregs::el2::accessors::read_amevcntvoff08_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "amevcntvoff09_el2 = {:?}",
        arm_sysregs::el2::accessors::read_amevcntvoff09_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "amevcntvoff10_el2 = {:?}",
        arm_sysregs::el2::accessors::read_amevcntvoff10_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "amevcntvoff110_el2 = {:?}",
        arm_sysregs::el2::accessors::read_amevcntvoff110_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "amevcntvoff111_el2 = {:?}",
        arm_sysregs::el2::accessors::read_amevcntvoff111_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "amevcntvoff112_el2 = {:?}",
        arm_sysregs::el2::accessors::read_amevcntvoff112_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "amevcntvoff113_el2 = {:?}",
        arm_sysregs::el2::accessors::read_amevcntvoff113_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "amevcntvoff114_el2 = {:?}",
        arm_sysregs::el2::accessors::read_amevcntvoff114_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "amevcntvoff115_el2 = {:?}",
        arm_sysregs::el2::accessors::read_amevcntvoff115_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "amevcntvoff11_el2 = {:?}",
        arm_sysregs::el2::accessors::read_amevcntvoff11_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "amevcntvoff12_el2 = {:?}",
        arm_sysregs::el2::accessors::read_amevcntvoff12_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "amevcntvoff13_el2 = {:?}",
        arm_sysregs::el2::accessors::read_amevcntvoff13_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "amevcntvoff14_el2 = {:?}",
        arm_sysregs::el2::accessors::read_amevcntvoff14_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "amevcntvoff15_el2 = {:?}",
        arm_sysregs::el2::accessors::read_amevcntvoff15_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "amevcntvoff16_el2 = {:?}",
        arm_sysregs::el2::accessors::read_amevcntvoff16_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "amevcntvoff17_el2 = {:?}",
        arm_sysregs::el2::accessors::read_amevcntvoff17_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "amevcntvoff18_el2 = {:?}",
        arm_sysregs::el2::accessors::read_amevcntvoff18_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "amevcntvoff19_el2 = {:?}",
        arm_sysregs::el2::accessors::read_amevcntvoff19_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "brbcr_el2 = {:?}",
        arm_sysregs::el2::accessors::read_brbcr_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "cnthctl_el2 = {:?}",
        arm_sysregs::el2::accessors::read_cnthctl_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "cnthps_ctl_el2 = {:?}",
        arm_sysregs::el2::accessors::read_cnthps_ctl_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "cnthps_cval_el2 = {:?}",
        arm_sysregs::el2::accessors::read_cnthps_cval_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "cnthps_tval_el2 = {:?}",
        arm_sysregs::el2::accessors::read_cnthps_tval_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "cnthp_ctl_el2 = {:?}",
        arm_sysregs::el2::accessors::read_cnthp_ctl_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "cnthp_cval_el2 = {:?}",
        arm_sysregs::el2::accessors::read_cnthp_cval_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "cnthp_tval_el2 = {:?}",
        arm_sysregs::el2::accessors::read_cnthp_tval_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "cnthvs_ctl_el2 = {:?}",
        arm_sysregs::el2::accessors::read_cnthvs_ctl_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "cnthvs_cval_el2 = {:?}",
        arm_sysregs::el2::accessors::read_cnthvs_cval_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "cnthvs_tval_el2 = {:?}",
        arm_sysregs::el2::accessors::read_cnthvs_tval_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "cnthv_ctl_el2 = {:?}",
        arm_sysregs::el2::accessors::read_cnthv_ctl_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "cnthv_cval_el2 = {:?}",
        arm_sysregs::el2::accessors::read_cnthv_cval_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "cnthv_tval_el2 = {:?}",
        arm_sysregs::el2::accessors::read_cnthv_tval_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "cntpoff_el2 = {:?}",
        arm_sysregs::el2::accessors::read_cntpoff_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "cntvoff_el2 = {:?}",
        arm_sysregs::el2::accessors::read_cntvoff_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "contextidr_el2 = {:?}",
        arm_sysregs::el2::accessors::read_contextidr_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "cptr_el2 = {:?}",
        arm_sysregs::el2::accessors::read_cptr_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "elr_el2 = {:?}",
        arm_sysregs::el2::accessors::read_elr_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "esr_el2 = {:?}",
        arm_sysregs::el2::accessors::read_esr_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "far_el2 = {:?}",
        arm_sysregs::el2::accessors::read_far_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "gcscr_el2 = {:?}",
        arm_sysregs::el2::accessors::read_gcscr_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "gcspr_el2 = {:?}",
        arm_sysregs::el2::accessors::read_gcspr_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "hacr_el2 = {:?}",
        arm_sysregs::el2::accessors::read_hacr_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "hafgrtr_el2 = {:?}",
        arm_sysregs::el2::accessors::read_hafgrtr_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "hcrx_el2 = {:?}",
        arm_sysregs::el2::accessors::read_hcrx_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "hcr_el2 = {:?}",
        arm_sysregs::el2::accessors::read_hcr_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "hdfgrtr2_el2 = {:?}",
        arm_sysregs::el2::accessors::read_hdfgrtr2_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "hdfgrtr_el2 = {:?}",
        arm_sysregs::el2::accessors::read_hdfgrtr_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "hdfgwtr2_el2 = {:?}",
        arm_sysregs::el2::accessors::read_hdfgwtr2_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "hdfgwtr_el2 = {:?}",
        arm_sysregs::el2::accessors::read_hdfgwtr_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "hfgitr2_el2 = {:?}",
        arm_sysregs::el2::accessors::read_hfgitr2_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "hfgitr_el2 = {:?}",
        arm_sysregs::el2::accessors::read_hfgitr_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "hfgrtr2_el2 = {:?}",
        arm_sysregs::el2::accessors::read_hfgrtr2_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "hfgrtr_el2 = {:?}",
        arm_sysregs::el2::accessors::read_hfgrtr_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "hfgwtr2_el2 = {:?}",
        arm_sysregs::el2::accessors::read_hfgwtr2_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "hfgwtr_el2 = {:?}",
        arm_sysregs::el2::accessors::read_hfgwtr_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "hpfar_el2 = {:?}",
        arm_sysregs::el2::accessors::read_hpfar_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "hstr_el2 = {:?}",
        arm_sysregs::el2::accessors::read_hstr_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "icc_sre_el2 = {:?}",
        arm_sysregs::el2::accessors::read_icc_sre_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "ich_hcr_el2 = {:?}",
        arm_sysregs::el2::accessors::read_ich_hcr_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "ich_vmcr_el2 = {:?}",
        arm_sysregs::el2::accessors::read_ich_vmcr_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "mair_el2 = {:?}",
        arm_sysregs::el2::accessors::read_mair_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "mdcr_el2 = {:?}",
        arm_sysregs::el2::accessors::read_mdcr_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "mpam2_el2 = {:?}",
        arm_sysregs::el2::accessors::read_mpam2_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "mpamhcr_el2 = {:?}",
        arm_sysregs::el2::accessors::read_mpamhcr_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "mpamvpm0_el2 = {:?}",
        arm_sysregs::el2::accessors::read_mpamvpm0_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "mpamvpm1_el2 = {:?}",
        arm_sysregs::el2::accessors::read_mpamvpm1_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "mpamvpm2_el2 = {:?}",
        arm_sysregs::el2::accessors::read_mpamvpm2_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "mpamvpm3_el2 = {:?}",
        arm_sysregs::el2::accessors::read_mpamvpm3_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "mpamvpm4_el2 = {:?}",
        arm_sysregs::el2::accessors::read_mpamvpm4_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "mpamvpm5_el2 = {:?}",
        arm_sysregs::el2::accessors::read_mpamvpm5_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "mpamvpm6_el2 = {:?}",
        arm_sysregs::el2::accessors::read_mpamvpm6_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "mpamvpm7_el2 = {:?}",
        arm_sysregs::el2::accessors::read_mpamvpm7_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "mpamvpmv_el2 = {:?}",
        arm_sysregs::el2::accessors::read_mpamvpmv_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "pfar_el2 = {:?}",
        arm_sysregs::el2::accessors::read_pfar_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "pire0_el2 = {:?}",
        arm_sysregs::el2::accessors::read_pire0_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "pir_el2 = {:?}",
        arm_sysregs::el2::accessors::read_pir_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "por_el2 = {:?}",
        arm_sysregs::el2::accessors::read_por_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "s2pir_el2 = {:?}",
        arm_sysregs::el2::accessors::read_s2pir_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "sctlr2_el2 = {:?}",
        arm_sysregs::el2::accessors::read_sctlr2_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "sctlr_el2 = {:?}",
        arm_sysregs::el2::accessors::read_sctlr_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "spsr_el2 = {:?}",
        arm_sysregs::el2::accessors::read_spsr_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("sp_el2 = {:?}", arm_sysregs::el2::accessors::read_sp_el2());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "tcr2_el2 = {:?}",
        arm_sysregs::el2::accessors::read_tcr2_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "tcr_el2 = {:?}",
        arm_sysregs::el2::accessors::read_tcr_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "tfsr_el2 = {:?}",
        arm_sysregs::el2::accessors::read_tfsr_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "tpidr_el2 = {:?}",
        arm_sysregs::el2::accessors::read_tpidr_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "ttbr0_el2 = {:?}",
        arm_sysregs::el2::accessors::read_ttbr0_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "ttbr1_el2 = {:?}",
        arm_sysregs::el2::accessors::read_ttbr1_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "vbar_el2 = {:?}",
        arm_sysregs::el2::accessors::read_vbar_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "vdisr_el2 = {:?}",
        arm_sysregs::el2::accessors::read_vdisr_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "vmpidr_el2 = {:?}",
        arm_sysregs::el2::accessors::read_vmpidr_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "vpidr_el2 = {:?}",
        arm_sysregs::el2::accessors::read_vpidr_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "vsesr_el2 = {:?}",
        arm_sysregs::el2::accessors::read_vsesr_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "vtcr_el2 = {:?}",
        arm_sysregs::el2::accessors::read_vtcr_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!(
        "vttbr_el2 = {:?}",
        arm_sysregs::el2::accessors::read_vttbr_el2()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
    info!(
        "cptr_el3 = {:?}",
        arm_sysregs::el3::accessors::read_cptr_el3()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
    info!(
        "esr_el3 = {:?}",
        arm_sysregs::el3::accessors::read_esr_el3()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
    info!(
        "fgwte3_el3 = {:?}",
        arm_sysregs::el3::accessors::read_fgwte3_el3()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
    info!(
        "gpccr_el3 = {:?}",
        arm_sysregs::el3::accessors::read_gpccr_el3()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
    info!(
        "gptbr_el3 = {:?}",
        arm_sysregs::el3::accessors::read_gptbr_el3()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
    info!(
        "icc_ctlr_el3 = {:?}",
        arm_sysregs::el3::accessors::read_icc_ctlr_el3()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
    info!(
        "icc_igrpen1_el3 = {:?}",
        arm_sysregs::el3::accessors::read_icc_igrpen1_el3()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
    info!(
        "icc_sre_el3 = {:?}",
        arm_sysregs::el3::accessors::read_icc_sre_el3()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
    info!(
        "mair_el3 = {:?}",
        arm_sysregs::el3::accessors::read_mair_el3()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
    info!(
        "mdcr_el3 = {:?}",
        arm_sysregs::el3::accessors::read_mdcr_el3()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
    info!(
        "mpam3_el3 = {:?}",
        arm_sysregs::el3::accessors::read_mpam3_el3()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
    info!(
        "pir_el3 = {:?}",
        arm_sysregs::el3::accessors::read_pir_el3()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
    info!(
        "por_el3 = {:?}",
        arm_sysregs::el3::accessors::read_por_el3()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
    info!(
        "scr_el3 = {:?}",
        arm_sysregs::el3::accessors::read_scr_el3()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
    info!(
        "sctlr2_el3 = {:?}",
        arm_sysregs::el3::accessors::read_sctlr2_el3()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
    info!(
        "sctlr_el3 = {:?}",
        arm_sysregs::el3::accessors::read_sctlr_el3()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
    info!(
        "smcr_el3 = {:?}",
        arm_sysregs::el3::accessors::read_smcr_el3()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
    info!(
        "spsr_el3 = {:?}",
        arm_sysregs::el3::accessors::read_spsr_el3()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
    info!(
        "tcr_el3 = {:?}",
        arm_sysregs::el3::accessors::read_tcr_el3()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
    info!(
        "tpidr_el3 = {:?}",
        arm_sysregs::el3::accessors::read_tpidr_el3()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
    info!(
        "ttbr0_el3 = {:?}",
        arm_sysregs::el3::accessors::read_ttbr0_el3()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
    info!(
        "zcr_el3 = {:?}",
        arm_sysregs::el3::accessors::read_zcr_el3()
    );
    loop {}
}

#[cfg(any(test, feature = "fakes"))]
fn main() {}

#[cfg(not(any(test, feature = "fakes")))]
#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
    loop {}
}
