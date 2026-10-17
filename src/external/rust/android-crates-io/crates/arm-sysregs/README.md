# Armv8-A CPU system register access

Helper crate to access Arm A-profile CPU system registers.

## Implemented features

## Feature flags

The system registers for each Exception Level are guarded by a corresponding feature flag. Each EL
feature implies the lower ELs.

- `el3` gives access to all supported system registers.
- `el2` gives access to EL2, EL1 and EL0 system registers.
- `el1` gives access to EL1 and EL0 system registers.

Without any feature flags only EL0 system registers are included.

For unit testing, the `fakes` feature can be used. This replaces the assembly code for reading and
writing system registers with accesses to a set of fake system registers, stored in `fake::SYSREGS`
in the EL- and architecture-specific submodules.

## Future plans

## License

The project is MIT and Apache-2.0 dual licensed, see `LICENSE-Apache-2.0` and `LICENSE-MIT`.

## Maintainers

arm-sysregs is a trustedfirmware.org maintained project. All contributions are ultimately merged by the maintainers
listed below.

* Bálint Dobszay <balint.dobszay@arm.com>
  [balint-dobszay-arm](https://github.com/balint-dobszay-arm)
* Imre Kis <imre.kis@arm.com>
  [imre-kis-arm](https://github.com/imre-kis-arm)
* Sandrine Afsa <sandrine.afsa@arm.com>
  [sandrine-bailleux-arm](https://github.com/sandrine-bailleux-arm)

## Contributing

Please follow the directions of the [Trusted Firmware Processes](https://trusted-firmware-docs.readthedocs.io/en/latest/generic_processes/index.html)

Contributions are handled through [review.trustedfirmware.org](https://review.trustedfirmware.org/q/project:arm-firmware-crates/arm-sysregs).

## Arm trademark notice

Arm is a registered trademark of Arm Limited (or its subsidiaries or affiliates).

This project uses some of the Arm product, service or technology trademarks, as listed in the
[Trademark List][1], in accordance with the [Arm Trademark Use Guidelines][2].

Subsequent uses of these trademarks throughout this repository do not need to be prefixed with the
Arm word trademark.

[1]: https://www.arm.com/company/policies/trademarks/arm-trademark-list
[2]: https://www.arm.com/company/policies/trademarks/guidelines-trademarks

--------------

*Copyright The arm-sysregs Contributors.*
