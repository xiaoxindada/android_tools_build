// SPDX-FileCopyrightText: Copyright The arm-sysregs Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

/// Generates a public function named `read_$ident` to read the system register `$sysreg` as a value
/// of type `$type`.
///
/// `safe` should only be specified for system registers which are indeed safe to read.
#[cfg(not(any(test, feature = "fakes")))]
#[macro_export]
macro_rules! read_sysreg {
    ($sysreg:ident, $type:ty, safe $(, $fake_sysregs:expr)?) => {
        $crate::_paste::paste! {
            #[doc = "Returns the value of the `"]
            #[doc = stringify!($sysreg)]
            #[doc = "` system register."]
            #[inline(always)]
            pub fn [< read_ $sysreg >]() -> $type {
                let value: $type;
                // SAFETY: The macro call site's author (see below) has determined that it is
                // always safe to read the given `$sysreg.`
                unsafe {
                    core::arch::asm!(
                        concat!("mrs {value}, ", stringify!($sysreg)),
                        options(nomem, nostack, preserves_flags),
                        value = out(reg) value,
                    );
                }
                value
            }
        }
    };
    ($(#[$attributes:meta])* $sysreg:ident, $type:ty $(, $fake_sysregs:expr)?) => {
        $crate::_paste::paste! {
            #[doc = "Returns the value of the `"]
            #[doc = stringify!($sysreg)]
            #[doc = "` system register."]
            $(#[$attributes])*
            #[inline(always)]
            pub unsafe fn [< read_ $sysreg >]() -> $type {
                let value: $type;
                // SAFETY: The caller promises that it is safe to read the given `$sysreg`.
                unsafe {
                    core::arch::asm!(
                        concat!("mrs {value}, ", stringify!($sysreg)),
                        options(nomem, nostack, preserves_flags),
                        value = out(reg) value,
                    );
                }
                value
            }
        }
    };
    ($sysreg:ident, $type:ty : $bitflags_type:ty, safe $(, $fake_sysregs:expr)?) => {
        $crate::_paste::paste! {
            #[doc = "Returns the value of the `"]
            #[doc = stringify!($sysreg)]
            #[doc = "` system register."]
            #[inline(always)]
            pub fn [< read_ $sysreg >]() -> $bitflags_type {
                let value: $type;
                // SAFETY: The macro call site's author (see below) has determined that it is
                // always safe to read the given `$sysreg.`
                unsafe {
                    core::arch::asm!(
                        concat!("mrs {value}, ", stringify!($sysreg)),
                        options(nomem, nostack, preserves_flags),
                        value = out(reg) value,
                    );
                }
                <$bitflags_type>::from_bits_retain(value)
            }
        }
    };
    ($(#[$attributes:meta])* $sysreg:ident, $type:ty : $bitflags_type:ty $(, $fake_sysregs:expr)?) => {
        $crate::_paste::paste! {
            #[doc = "Returns the value of the `"]
            #[doc = stringify!($sysreg)]
            #[doc = "` system register."]
            $(#[$attributes])*
            #[inline(always)]
            pub unsafe fn [< read_ $sysreg >]() -> $bitflags_type {
                let value: $type;
                // SAFETY: The caller promises that it is safe to read the given `$sysreg`.
                unsafe {
                    core::arch::asm!(
                        concat!("mrs {value}, ", stringify!($sysreg)),
                        options(nomem, nostack, preserves_flags),
                        value = out(reg) value,
                    );
                }
                <$bitflags_type>::from_bits_retain(value)
            }
        }
    };
    ($sysreg:ident : ($coproc:ident, $opc1:literal, $crm:ident, $crn:ident, $opc2:literal), u32, safe $(, $fake_sysregs:expr)?) => {
        $crate::_paste::paste! {
            #[doc = "Returns the value of the `"]
            #[doc = stringify!($sysreg)]
            #[doc = "` system register."]
            #[inline(always)]
            pub fn [< read_ $sysreg >]() -> u32 {
                let value: u32;
                // SAFETY: The macro call site's author (see below) has determined that it is
                // always safe to read the given `$sysreg.`
                unsafe {
                    core::arch::asm!(
                        concat!(
                            "mrc ",
                            stringify!($coproc), ", ",
                            stringify!($opc1), ", ",
                            "{value}, ",
                            stringify!($crn), ",",
                            stringify!($crm), ",",
                            stringify!($opc2)
                        ),
                        options(nomem, nostack, preserves_flags),
                        value = out(reg) value,
                    );
                }
                value
            }
        }
    };
    ($(#[$attributes:meta])* $sysreg:ident : ($coproc:ident, $opc1:literal, $crm:ident, $crn:ident, $opc2:literal), u32 $(, $fake_sysregs:expr)?) => {
        $crate::_paste::paste! {
            #[doc = "Returns the value of the `"]
            #[doc = stringify!($sysreg)]
            #[doc = "` system register."]
            $(#[$attributes])*
            #[inline(always)]
            pub unsafe fn [< read_ $sysreg >]() -> u32 {
                let value: u32;
                // SAFETY: The caller promises that it is safe to read the given `$sysreg`.
                unsafe {
                    core::arch::asm!(
                        concat!(
                            "mrc ",
                            stringify!($coproc), ", ",
                            stringify!($opc1), ", ",
                            "{value}, ",
                            stringify!($crn), ",",
                            stringify!($crm), ",",
                            stringify!($opc2)
                        ),
                        options(nomem, nostack, preserves_flags),
                        value = out(reg) value,
                    );
                }
                value
            }
        }
    };
    ($sysreg:ident : ($coproc:ident, $opc1:literal, $crm:ident, $crn:ident, $opc2:literal), u32 : $bitflags_type:ty, safe $(, $fake_sysregs:expr)?) => {
        $crate::_paste::paste! {
            #[doc = "Returns the value of the `"]
            #[doc = stringify!($sysreg)]
            #[doc = "` system register."]
            #[inline(always)]
            pub fn [< read_ $sysreg >]() -> $bitflags_type {
                let value: u32;
                // SAFETY: The macro call site's author (see below) has determined that it is
                // always safe to read the given `$sysreg.`
                unsafe {
                    core::arch::asm!(
                        concat!(
                            "mrc ",
                            stringify!($coproc), ", ",
                            stringify!($opc1), ", ",
                            "{value}, ",
                            stringify!($crn), ",",
                            stringify!($crm), ",",
                            stringify!($opc2)
                        ),
                        options(nomem, nostack, preserves_flags),
                        value = out(reg) value,
                    );
                }
                <$bitflags_type>::from_bits_retain(value)
            }
        }
    };
    ($(#[$attributes:meta])* $sysreg:ident : ($coproc:ident, $opc1:literal, $crm:ident, $crn:ident, $opc2:literal), u32 : $bitflags_type:ty $(, $fake_sysregs:expr)?) => {
        $crate::_paste::paste! {
            #[doc = "Returns the value of the `"]
            #[doc = stringify!($sysreg)]
            #[doc = "` system register."]
            $(#[$attributes])*
            #[inline(always)]
            pub unsafe fn [< read_ $sysreg >]() -> $bitflags_type {
                let value: u32;
                // SAFETY: The caller promises that it is safe to read the given `$sysreg`.
                unsafe {
                    core::arch::asm!(
                        concat!(
                            "mrc ",
                            stringify!($coproc), ", ",
                            stringify!($opc1), ", ",
                            "{value}, ",
                            stringify!($crn), ",",
                            stringify!($crm), ",",
                            stringify!($opc2)
                        ),
                        options(nomem, nostack, preserves_flags),
                        value = out(reg) value,
                    );
                }
                <$bitflags_type>::from_bits_retain(value)
            }
        }
    };
    ($sysreg:ident : ($coproc:ident, $opc1:literal, $crm:ident), u64, safe $(, $fake_sysregs:expr)?) => {
        $crate::_paste::paste! {
            #[doc = "Returns the value of the `"]
            #[doc = stringify!($sysreg)]
            #[doc = "` system register."]
            #[inline(always)]
            pub fn [< read_ $sysreg >]() -> u64 {
                let value_low: u32;
                let value_high: u32;
                // SAFETY: The macro call site's author (see below) has determined that it is
                // always safe to read the given `$sysreg.`
                unsafe {
                    core::arch::asm!(
                        concat!(
                            "mrc ",
                            stringify!($coproc), ", ",
                            stringify!($opc1), ", ",
                            "{value_low}, ",
                            "{value_high}, ",
                            stringify!($crm)
                        ),
                        options(nomem, nostack, preserves_flags),
                        value_low = out(reg) value_low,
                        value_high = out(reg) value_high,
                    );
                }
                ((value_high as u64) << 32) | (value_low as u64)
            }
        }
    };
    ($(#[$attributes:meta])* $sysreg:ident : ($coproc:ident, $opc1:literal, $crm:ident), u64 $(, $fake_sysregs:expr)?) => {
        $crate::_paste::paste! {
            #[doc = "Returns the value of the `"]
            #[doc = stringify!($sysreg)]
            #[doc = "` system register."]
            $(#[$attributes])*
            #[inline(always)]
            pub unsafe fn [< read_ $sysreg >]() -> u64 {
                let value_low: u32;
                let value_high: u32;
                // SAFETY: The caller promises that it is safe to read the given `$sysreg`.
                unsafe {
                    core::arch::asm!(
                        concat!(
                            "mrc ",
                            stringify!($coproc), ", ",
                            stringify!($opc1), ", ",
                            "{value_low}, ",
                            "{value_high}, ",
                            stringify!($crm)
                        ),
                        options(nomem, nostack, preserves_flags),
                        value_low = out(reg) value_low,
                        value_high = out(reg) value_high,
                    );
                }
                ((value_high as u64) << 32) | (value_low as u64)
            }
        }
    };
    ($sysreg:ident : ($coproc:ident, $opc1:literal, $crm:ident), u64 : $bitflags_type:ty, safe $(, $fake_sysregs:expr)?) => {
        $crate::_paste::paste! {
            #[doc = "Returns the value of the `"]
            #[doc = stringify!($sysreg)]
            #[doc = "` system register."]
            #[inline(always)]
            pub fn [< read_ $sysreg >]() -> $bitflags_type {
                let value_low: u32;
                let value_high: u32;
                // SAFETY: The macro call site's author (see below) has determined that it is
                // always safe to read the given `$sysreg.`
                unsafe {
                    core::arch::asm!(
                        concat!(
                            "mrc ",
                            stringify!($coproc), ", ",
                            stringify!($opc1), ", ",
                            "{value_low}, ",
                            "{value_high}, ",
                            stringify!($crm)
                        ),
                        options(nomem, nostack, preserves_flags),
                        value_low = out(reg) value_low,
                        value_high = out(reg) value_high,
                    );
                }
                <$bitflags_type>::from_bits_retain(((value_high as u64) << 32) | (value_low as u64))
            }
        }
    };
    ($(#[$attributes:meta])* $sysreg:ident : ($coproc:ident, $opc1:literal, $crm:ident), u64 : $bitflags_type:ty $(, $fake_sysregs:expr)?) => {
        $crate::_paste::paste! {
            #[doc = "Returns the value of the `"]
            #[doc = stringify!($sysreg)]
            #[doc = "` system register."]
            $(#[$attributes])*
            #[inline(always)]
            pub unsafe fn [< read_ $sysreg >]() -> $bitflags_type {
                let value_low: u32;
                let value_high: u32;
                // SAFETY: The caller promises that it is safe to read the given `$sysreg`.
                unsafe {
                    core::arch::asm!(
                        concat!(
                            "mrc ",
                            stringify!($coproc), ", ",
                            stringify!($opc1), ", ",
                            "{value_low}, ",
                            "{value_high}, ",
                            stringify!($crm)
                        ),
                        options(nomem, nostack, preserves_flags),
                        value_low = out(reg) value_low,
                        value_high = out(reg) value_high,
                    );
                }
                <$bitflags_type>::from_bits_retain(((value_high as u64) << 32) | (value_low as u64))
            }
        }
    };
}

/// Generates a public function named `write_$sysreg` to write a value of type `$type` to the system
/// register `$sysreg`.
///
/// `safe` should only be specified for system registers which are indeed safe to write any value
/// to.
#[cfg(not(any(test, feature = "fakes")))]
#[macro_export]
macro_rules! write_sysreg {
    ($sysreg:ident, $type:ty, safe $(, $fake_sysregs:expr)?) => {
        $crate::_paste::paste! {
            #[doc = "Writes `value` to the `"]
            #[doc = stringify!($sysreg)]
            #[doc = "` system register."]
            #[inline(always)]
            pub fn [< write_ $sysreg >](value: $type) {
                // SAFETY: The macro call site's author (see below) has determined that it is safe
                // to write any value to the given `$sysreg.`
                unsafe {
                    core::arch::asm!(
                        concat!("msr ", stringify!($sysreg), ", {value}"),
                        options(nomem, nostack, preserves_flags),
                        value = in(reg) value,
                    );
                }
            }
        }
    };
    (
        $(#[$attributes:meta])*
        $sysreg:ident, $type:ty $(, $fake_sysregs:expr)?
    ) => {
        $crate::_paste::paste! {
            #[doc = "Writes `value` to the `"]
            #[doc = stringify!($sysreg)]
            #[doc = "` system register."]
            $(#[$attributes])*
            #[inline(always)]
            pub unsafe fn [< write_ $sysreg >](value: $type) {
                // SAFETY: The caller promises that it is safe to write `value` to the given `$sysreg`.
                unsafe {
                    core::arch::asm!(
                        concat!("msr ", stringify!($sysreg), ", {value}"),
                        options(nostack, preserves_flags),
                        value = in(reg) value,
                    );
                }
            }
        }
    };
    ($sysreg:ident, $type:ty : $bitflags_type:ty, safe $(, $fake_sysregs:expr)?) => {
        $crate::_paste::paste! {
            #[doc = "Writes `value` to the `"]
            #[doc = stringify!($sysreg)]
            #[doc = "` system register."]
            #[inline(always)]
            pub fn [< write_ $sysreg >](value: $bitflags_type) {
                let value: $type = value.bits();
                // SAFETY: The macro call site's author (see below) has determined that it is safe
                // to write any value to the given `$sysreg.`
                unsafe {
                    core::arch::asm!(
                        concat!("msr ", stringify!($sysreg), ", {value}"),
                        options(nomem, nostack, preserves_flags),
                        value = in(reg) value,
                    );
                }
            }
        }
    };
    (
        $(#[$attributes:meta])*
        $sysreg:ident, $type:ty : $bitflags_type:ty $(, $fake_sysregs:expr)?
    ) => {
        $crate::_paste::paste! {
            #[doc = "Writes `value` to the `"]
            #[doc = stringify!($sysreg)]
            #[doc = "` system register."]
            $(#[$attributes])*
            #[inline(always)]
            pub unsafe fn [< write_ $sysreg >](value: $bitflags_type) {
                let value: $type = value.bits();
                // SAFETY: The caller promises that it is safe to write `value` to the given `$sysreg`.
                unsafe {
                    core::arch::asm!(
                        concat!("msr ", stringify!($sysreg), ", {value}"),
                        options(nostack, preserves_flags),
                        value = in(reg) value,
                    );
                }
            }
        }
    };
    ($sysreg:ident : ($coproc:ident, $opc1:literal, $crm:ident, $crn:ident, $opc2:literal), u32, safe $(, $fake_sysregs:expr)?) => {
        $crate::_paste::paste! {
            #[doc = "Writes `value` to the `"]
            #[doc = stringify!($sysreg)]
            #[doc = "` system register."]
            #[inline(always)]
            pub fn [< write_ $sysreg >](value: u32) {
                // SAFETY: The macro call site's author (see below) has determined that it is safe
                // to write any value to the given `$sysreg.`
                unsafe {
                    core::arch::asm!(
                        concat!(
                            "mcr ",
                            stringify!($coproc), ", ",
                            stringify!($opc1), ", ",
                            "{value}, ",
                            stringify!($crn), ", ",
                            stringify!($crm), ", ",
                            stringify!($opc2)
                        ),
                        options(nomem, nostack, preserves_flags),
                        value = in(reg) value,
                    );
                }
            }
        }
    };
    (
        $(#[$attributes:meta])*
        $sysreg:ident : ($coproc:ident, $opc1:literal, $crm:ident, $crn:ident, $opc2:literal), u32 $(, $fake_sysregs:expr)?
    ) => {
        $crate::_paste::paste! {
            #[doc = "Writes `value` to the `"]
            #[doc = stringify!($sysreg)]
            #[doc = "` system register."]
            $(#[$attributes])*
            #[inline(always)]
            pub unsafe fn [< write_ $sysreg >](value: u32) {
                // SAFETY: The caller promises that it is safe to write `value` to the given `$sysreg`.
                unsafe {
                    core::arch::asm!(
                        concat!(
                            "mcr ",
                            stringify!($coproc), ", ",
                            stringify!($opc1), ", ",
                            "{value}, ",
                            stringify!($crn), ", ",
                            stringify!($crm), ", ",
                            stringify!($opc2)
                        ),
                        options(nostack, preserves_flags),
                        value = in(reg) value,
                    );
                }
            }
        }
    };
    ($sysreg:ident : ($coproc:ident, $opc1:literal, $crm:ident, $crn:ident, $opc2:literal), u32 : $bitflags_type:ty, safe $(, $fake_sysregs:expr)?) => {
        $crate::_paste::paste! {
            #[doc = "Writes `value` to the `"]
            #[doc = stringify!($sysreg)]
            #[doc = "` system register."]
            #[inline(always)]
            pub fn [< write_ $sysreg >](value: $bitflags_type) {
                let value: u32 = value.bits();
                // SAFETY: The macro call site's author (see below) has determined that it is safe
                // to write any value to the given `$sysreg.`
                unsafe {
                    core::arch::asm!(
                        concat!(
                            "mcr ",
                            stringify!($coproc), ", ",
                            stringify!($opc1), ", ",
                            "{value}, ",
                            stringify!($crn), ", ",
                            stringify!($crm), ", ",
                            stringify!($opc2)
                        ),
                        options(nomem, nostack, preserves_flags),
                        value = in(reg) value,
                    );
                }
            }
        }
    };
    (
        $(#[$attributes:meta])*
        $sysreg:ident : ($coproc:ident, $opc1:literal, $crm:ident, $crn:ident, $opc2:literal), u32 : $bitflags_type:ty $(, $fake_sysregs:expr)?
    ) => {
        $crate::_paste::paste! {
            #[doc = "Writes `value` to the `"]
            #[doc = stringify!($sysreg)]
            #[doc = "` system register."]
            $(#[$attributes])*
            #[inline(always)]
            pub unsafe fn [< write_ $sysreg >](value: $bitflags_type) {
                let value: u32 = value.bits();
                // SAFETY: The caller promises that it is safe to write `value` to the given `$sysreg`.
                unsafe {
                    core::arch::asm!(
                        concat!(
                            "mcr ",
                            stringify!($coproc), ", ",
                            stringify!($opc1), ", ",
                            "{value}, ",
                            stringify!($crn), ", ",
                            stringify!($crm), ", ",
                            stringify!($opc2)
                        ),
                        options(nostack, preserves_flags),
                        value = in(reg) value,
                    );
                }
            }
        }
    };
    ($sysreg:ident : ($coproc:ident, $opc1:literal, $crm:ident), u64, safe $(, $fake_sysregs:expr)?) => {
        $crate::_paste::paste! {
            #[doc = "Writes `value` to the `"]
            #[doc = stringify!($sysreg)]
            #[doc = "` system register."]
            #[inline(always)]
            pub fn [< write_ $sysreg >](value: u64) {
                let value_low = value as u32;
                let value_high = (value >> 32) as u32;
                // SAFETY: The macro call site's author (see below) has determined that it is safe
                // to write any value to the given `$sysreg.`
                unsafe {
                    core::arch::asm!(
                        concat!(
                            "mcrr ",
                            stringify!($coproc), ", ",
                            stringify!($opc1), ", ",
                            "{value_low}, ",
                            "{value_high}, ",
                            stringify!($crm)
                        ),
                        options(nomem, nostack, preserves_flags),
                        value_low = in(reg) value_low,
                        value_high = in(reg) value_high,
                    );
                }
            }
        }
    };
    (
        $(#[$attributes:meta])*
        $sysreg:ident : ($coproc:ident, $opc1:literal, $crm:ident), u64 $(, $fake_sysregs:expr)?
    ) => {
        $crate::_paste::paste! {
            #[doc = "Writes `value` to the `"]
            #[doc = stringify!($sysreg)]
            #[doc = "` system register."]
            $(#[$attributes])*
            #[inline(always)]
            pub unsafe fn [< write_ $sysreg >](value: u64) {
                let value_low = value as u32;
                let value_high = (value >> 32) as u32;
                // SAFETY: The caller promises that it is safe to write `value` to the given `$sysreg`.
                unsafe {
                    core::arch::asm!(
                        concat!(
                            "mcrr ",
                            stringify!($coproc), ", ",
                            stringify!($opc1), ", ",
                            "{value_low}, ",
                            "{value_high}, ",
                            stringify!($crm)
                        ),
                        options(nostack, preserves_flags),
                        value_low = in(reg) value_low,
                        value_high = in(reg) value_high,
                    );
                }
            }
        }
    };
    ($sysreg:ident : ($coproc:ident, $opc1:literal, $crm:ident), u64 : $bitflags_type:ty, safe $(, $fake_sysregs:expr)?) => {
        $crate::_paste::paste! {
            #[doc = "Writes `value` to the `"]
            #[doc = stringify!($sysreg)]
            #[doc = "` system register."]
            #[inline(always)]
            pub fn [< write_ $sysreg >](value: $bitflags_type) {
                let value: u64 = value.bits();
                let value_low = value as u32;
                let value_high = (value >> 32) as u32;
                // SAFETY: The macro call site's author (see below) has determined that it is safe
                // to write any value to the given `$sysreg.`
                unsafe {
                    core::arch::asm!(
                        concat!(
                            "mcrr ",
                            stringify!($coproc), ", ",
                            stringify!($opc1), ", ",
                            "{value_low}, ",
                            "{value_high}, ",
                            stringify!($crm)
                        ),
                        options(nomem, nostack, preserves_flags),
                        value_low = in(reg) value_low,
                        value_high = in(reg) value_high,
                    );
                }
            }
        }
    };
    (
        $(#[$attributes:meta])*
        $sysreg:ident : ($coproc:ident, $opc1:literal, $crm:ident), u64 : $bitflags_type:ty $(, $fake_sysregs:expr)?
    ) => {
        $crate::_paste::paste! {
            #[doc = "Writes `value` to the `"]
            #[doc = stringify!($sysreg)]
            #[doc = "` system register."]
            $(#[$attributes])*
            #[inline(always)]
            pub unsafe fn [< write_ $sysreg >](value: $bitflags_type) {
                let value: u64 = value.bits();
                let value_low = value as u32;
                let value_high = (value >> 32) as u32;
                // SAFETY: The caller promises that it is safe to write `value` to the given `$sysreg`.
                unsafe {
                    core::arch::asm!(
                        concat!(
                            "mcrr ",
                            stringify!($coproc), ", ",
                            stringify!($opc1), ", ",
                            "{value_low}, ",
                            "{value_high}, ",
                            stringify!($crm)
                        ),
                        options(nostack, preserves_flags),
                        value_low = in(reg) value_low,
                        value_high = in(reg) value_high,
                    );
                }
            }
        }
    };
}
