# Changelog

## 0.5.0

- Refactor to split the codebase into more manageable modules
- Update Rust edition to 2024 and version to 1.90
- FF-A v1.3 ALP4 related updates:
  - Add flags for FFA_VERSION
  - Extend direct request/response register usage
  - Fix number registers used for SMC32/64 calls

## 0.4.1

- Improve performance of `Interface::{from_regs, to_regs}`.
- Derive common traits for error types.
- Check the 16 byte alignment of the 'Endpoint memory access descriptor array offset'.

## 0.4.0

- Implement `FFA_MEM_FRAG_RX`, `FFA_MEM_FRAG_TX`, `FFA_MEM_OP_PAUSE` and `FFA_MEM_OP_RESUME`
  interface handling.
- Add support for returning `NOT_SUPPORTED` as a response to `FFA_VERSION`.
- Fix `FFA_RXTX_UNMAP` and `FFA_MSG_SEND2` argument handling.
- Fix UUID handling in boot info descriptor.
- Cover crate with unit tests.

## 0.3.1

- Move repository under arm-firmware-crates namespace.
- Adopt Linux Foundation's guidance on copyrights
- Apply Arm Trademark Guidance
- Remove security incident process

## 0.3.0

- Refactor FFA_CONSOLE_LOG handling.
- Add more types for the various encodings of FFA_SUCCESS.
- Add support for handling multiple endpoints in the FF-A memory relinquish descriptor.
- Add new types for flags in various FF-A interfaces.
- Minor fixes, add more trait derives, missing doc comments, etc.

## 0.2.1

Minor bugfixes for parse_console_log() function and VersionOut interface handling.

## 0.2.0

- Implement missing FF-A interfaces:
  - FFA_SECONDARY_EP_REGISTER,
  - FFA_PARTITION_INFO_GET_REGS,
  - FFA_EL3_INTR_HANDLE.
- Add types for the various encodings of FFA_SUCCESS.
- Add types to support FF-A framework messages.
- Additional checks and bugfixes.

## 0.1.0

Initial release.
