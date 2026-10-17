// SPDX-FileCopyrightText: Copyright The arm-sysregs Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

/// Generates public functions named `read_$sysreg` and `write_$sysreg` to read or write
/// (respectively) a value of type `$type` from/to the system register `$sysreg`.
///
/// `safe_read` and `safe_write` should only be specified for system registers which are indeed safe
/// to read from or write any value to.
#[macro_export]
macro_rules! read_write_sysreg {
    ($sysreg:ident $(: $asm_sysreg:ident)?, $type:ident $(: $bitflags_type:ty)?, safe_read, safe_write $(, $fake_sysregs:expr)?) => {
        $crate::read_sysreg!($sysreg $(: $asm_sysreg)?, $type $(: $bitflags_type)?, safe $(, $fake_sysregs)?);
        $crate::write_sysreg!($sysreg $(: $asm_sysreg)?, $type $(: $bitflags_type)?, safe $(, $fake_sysregs)?);
    };
    ($sysreg:ident : ($coproc:ident, $opc1:literal, $crm:ident, $crn:ident, $opc2:literal), $type:ident $(: $bitflags_type:ty)?, safe_read, safe_write $(, $fake_sysregs:expr)?) => {
        $crate::read_sysreg!($sysreg : ($coproc, $opc1, $crm, $crn, $opc2), $type $(: $bitflags_type)?, safe $(, $fake_sysregs)?);
        $crate::write_sysreg!($sysreg : ($coproc, $opc1, $crm, $crn, $opc2), $type $(: $bitflags_type)?, safe $(, $fake_sysregs)?);
    };
    ($sysreg:ident : ($coproc:ident, $opc1:literal, $crm:ident), $type:ident $(: $bitflags_type:ty)?, safe_read, safe_write $(, $fake_sysregs:expr)?) => {
        $crate::read_sysreg!($sysreg : ($coproc, $opc1, $crm), $type $(: $bitflags_type)?, safe $(, $fake_sysregs)?);
        $crate::write_sysreg!($sysreg : ($coproc, $opc1, $crm), $type $(: $bitflags_type)?, safe $(, $fake_sysregs)?);
    };
    (
        $(#[$attributes:meta])*
        $sysreg:ident $(: $asm_sysreg:ident)?, $type:ident $(: $bitflags_type:ty)?, safe_read $(, $fake_sysregs:expr)?
    ) => {
        $crate::read_sysreg!($sysreg $(: $asm_sysreg)?, $type $(: $bitflags_type)?, safe $(, $fake_sysregs)?);
        $crate::write_sysreg! {
            $(#[$attributes])*
            $sysreg $(: $asm_sysreg)?, $type $(: $bitflags_type)? $(, $fake_sysregs)?
        }
    };
    (
        $(#[$attributes:meta])*
        $sysreg:ident : ($coproc:ident, $opc1:literal, $crm:ident, $crn:ident, $opc2:literal), $type:ident $(: $bitflags_type:ty)?, safe_read $(, $fake_sysregs:expr)?
    ) => {
        $crate::read_sysreg!($sysreg : ($coproc, $opc1, $crm, $crn, $opc2), $type $(: $bitflags_type)?, safe $(, $fake_sysregs)?);
        $crate::write_sysreg! {
            $(#[$attributes])*
            $sysreg : ($coproc, $opc1, $crm, $crn, $opc2), $type $(: $bitflags_type)? $(, $fake_sysregs)?
        }
    };
    (
        $(#[$attributes:meta])*
        $sysreg:ident : ($coproc:ident, $opc1:literal, $crm:ident), $type:ident $(: $bitflags_type:ty)?, safe_read $(, $fake_sysregs:expr)?
    ) => {
        $crate::read_sysreg!($sysreg : ($coproc, $opc1, $crm), $type $(: $bitflags_type)?, safe $(, $fake_sysregs)?);
        $crate::write_sysreg! {
            $(#[$attributes])*
            $sysreg : ($coproc, $opc1, $crm), $type $(: $bitflags_type)? $(, $fake_sysregs)?
        }
    };
}
