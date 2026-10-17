# Changelog

## 0.4.0

### New features

- Added GCSCRE0_EL1 and GCSPR_EL0 registers.
- Added a `AARCHMRS_VERSION` module to the top-level `arm-sysregs` crate, generated from the JSON
  input.

### Breaking changes

- Changed the public API: moved `read_` and `write_` functions to the `accessors` submodule, system
  register types to the `registers` submodule, and public helper functions to the `helpers` module.
- Split into crates by exception level and architecture; arm-sysregs acts as a facade crate. EL- and
  architecture-specific registers, accessors, fakes and helpers can now be accessed via
  `arm_sysregs::elx::` or `arm_sysregs::aarch32`. The `manual` modules are no longer public. Types
  not semantically belonging to specific ELs (`Shareability`, `Cacheability`, etc.) are accessible
  through `arm_sysregs::types`.

## 0.3.3

### Bugfixes

- Regenerated the code using AARCHMRS 2026-03, because the 0.3.2 release accidentally used AARCHMRS 2026-06.

## 0.3.2

### New features

- Added `is_feat_sel2_present()` to `IdAa64pfr0El1`.
- Added the `FGWTE3_EL3` register. Added `is_feat_fgwte3_present()` to `IdAa64Mmfr4El1`.
- Added `GCSPR_EL1` and `GCSPR_EL2` registers.

### Bugfixes

- Fixed the gating of `is_feat_pauth_lr_present()` and its underlying accessor functions.

## 0.3.1

### New features

- Added `is_feat_*_present` for `FEAT_RME`, `FEAT_RME_GPC2` and `FEAT_RME_GPC3`.
- Added `ID_AA64ISAR3_EL1` register.
- Added `is_feat_pauth_lr_present()` function.
- Added FEAT_AMUv1p1 registers.

## 0.3.0

### New features

- Updated to AARCHMRS 2026-03 release.
- Added FEAT_PFAR `PFAR_EL1` and `PFAR_EL2` registers.
- Added FEAT_FPMR registers (`FPCR`, `FPMR`, `FPSR`), and `ID_AA64PFR2_EL1`.
- Added registers for FEAT_S1PIE, FEAT_S1POE, FEAT_S2PIE, and FEAT_S2POE.
- Added FEAT_BRBE register `BRBCR_EL2`, and query functions for FEAT_BRBE and FEAT_BRBEv1p1.
- Added FEAT_SCTLR2 registers.
- Added DAIF register.
- Added `ID_AA64MMFR4_EL1` register, and query function for FEAT_RME_GDI.

### Bugfixes

- Fixed feature detection helpers for ID register fields to treat later feature versions as also
  implementing earlier versions, following the Arm ARM ID scheme principles.

### Breaking changes

- Armv8.9 FEAT_Debugv8p9 `Dspsr2` PPEND and `SpsrElx` PPEND bits removed.
- Armv8.9 FEAT_FGT2 `Hdfgrtr2El2` NPMIAR_EL1 and `Hdfgwtr2El2` NPMIAR_EL1 bits removed, `Hfgwtr2El2` NLDSTT_EL1 and `Hfgwtr2El2` ACTLR_EL1 bits added.
- FEAT_SEBEP identification register fields and system registers removed from architecture.
- Armv8.6 FEAT_MPAMv0p1 `MpamhcrEl2` VPMEN / VMMEN / SMVPMEN / SMVMMEN bits removed.
- `TcrEl2` TVAD and VTB fields removed.
- Use type aliasing for identical registers, rather than generate different types.
- Split AMU feature discovery to distinguish between FEAT_AMUv1 and FEAT_AMUv1p1.

## 0.2.9

### Bugfixes

- Fixed accessors on AArch32; they were all broken because CRn and CRm were swapped.

## 0.2.8

### Bugfixes

- Made functions to write `AMEVCNTR0<n>` and `AMEVCNTR1<n>_EL0` safe.

## 0.2.7

### New features

- Added AMU registers.

## 0.2.6

### New features

- Added more GIC registers.
- Made some GIC registers safe to write.
- Added `with_*` methods to set fields taking `self` by value and returning it. This allows a less
  verbose builder pattern for register values.

### Bugfixes

- Fixed compilation error for output of `read_write_sysregs!` on AArch32 in some cases.

## 0.2.5

### New features

- Added SVCR register.
- Added TPIDR_EL3 register.
- Added Generic Timer registers.
- Added GIC registers.
- Added more fields to `MpamhcrEl2`.

## 0.2.4

### Bugfixes

- Fixed accessors for `ID_AA64SMFR0_EL1` and `TTBR1_EL2` raw encoding name in assembly, to avoid
  compilation errors when the assembler doesn't recognise the system register names.

## 0.2.3

### New features

- Added AMU feature detection for the `ID_AA64PFR0_EL1` register.
- Added FGT feature registers.

## 0.2.2

### New features

- Register structs now implement `Default`.
- Register structs now have setters for multibit fields.

## 0.2.1

### Bugfixes

- Fixed `IdAa64mmfr0El1::FGT2_SUPPORTED` definition.

### New features

- Added AArch32 support to sysregs macros.
- Added many AArch32 system registers.

## 0.2.0

### Breaking changes

- Added `CurrentEL` register.
- Added `GPCCR_EL3` and `GPTBR_EL3` registers.
- Added `CNTPCT_EL0`, `ID_AA64ISAR1_EL1`, `ID_AA64ISAR2_EL1` and `SCTLR2_EL3` registers.
- Marked fake `SystemRegisters` struct as `#[non_exhaustive]`.

### Other changes

- Fixed warnings in Rustdoc.
- Included all fearures in docs.rs build.
- Documented feature flags in README.

## 0.1.0

Initial release.
