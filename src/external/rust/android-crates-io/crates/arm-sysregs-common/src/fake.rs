// SPDX-FileCopyrightText: Copyright The arm-sysregs Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Fake implementations of system register getters and setters for unit tests.

/// Generates a public function named `read_$sysreg` to read the fake system register `$sysreg` of
/// type `$type`.
#[macro_export]
macro_rules! read_sysreg {
    ($sysreg:ident $(: $asm_sysreg:tt)?, $type:ty, safe, $fake_sysregs:expr) => {
        $crate::_paste::paste! {
            #[doc = "Returns the value of the `"]
            #[doc = stringify!($sysreg)]
            #[doc = "` system register."]
            pub fn [< read_ $sysreg >]() -> $type {
                $fake_sysregs.lock().unwrap().$sysreg
            }
        }
    };
    ($(#[$attributes:meta])* $sysreg:ident $(: $asm_sysreg:tt)?, $type:ty, $fake_sysregs:expr) => {
        $crate::_paste::paste! {
            #[doc = "Returns the value of the `"]
            #[doc = stringify!($sysreg)]
            #[doc = "` system register."]
            $(#[$attributes])*
            pub unsafe fn [< read_ $sysreg >]() -> $type {
                $fake_sysregs.lock().unwrap().$sysreg
            }
        }
    };
    ($sysreg:ident $(: $asm_sysreg:tt)?, $type:ty : $bitflags_type:ty, safe, $fake_sysregs:expr) => {
        $crate::_paste::paste! {
            #[doc = "Returns the value of the `"]
            #[doc = stringify!($sysreg)]
            #[doc = "` system register."]
            pub fn [< read_ $sysreg >]() -> $bitflags_type {
                $fake_sysregs.lock().unwrap().$sysreg
            }
        }
    };
    ($(#[$attributes:meta])* $sysreg:ident $(: $asm_sysreg:tt)?, $type:ty : $bitflags_type:ty, $fake_sysregs:expr) => {
        $crate::_paste::paste! {
            #[doc = "Returns the value of the `"]
            #[doc = stringify!($sysreg)]
            #[doc = "` system register."]
            $(#[$attributes])*
            pub unsafe fn [< read_ $sysreg >]() -> $bitflags_type {
                $fake_sysregs.lock().unwrap().$sysreg
            }
        }
    };
}

/// Generates a public function named `write_$sysreg` to write to the fake system register `$sysreg`
/// of type `$type`.
#[macro_export]
macro_rules! write_sysreg {
    ($sysreg:ident $(: $asm_sysreg:tt)?, $type:ty, safe, $fake_sysregs:expr) => {
        $crate::_paste::paste! {
            #[doc = "Writes `value` to the `"]
            #[doc = stringify!($sysreg)]
            #[doc = "` system register."]
            pub fn [< write_ $sysreg >](value: $type) {
                $fake_sysregs.lock().unwrap().$sysreg = value;
            }
        }
    };
    (
        $(#[$attributes:meta])*
        $sysreg:ident $(: $asm_sysreg:tt)?, $type:ty, $fake_sysregs:expr
    ) => {
        $crate::_paste::paste! {
            #[doc = "Writes `value` to the `"]
            #[doc = stringify!($sysreg)]
            #[doc = "` system register."]
            $(#[$attributes])*
            pub unsafe fn [< write_ $sysreg >](value: $type) {
                $fake_sysregs.lock().unwrap().$sysreg = value;
            }
        }
    };
    ($sysreg:ident $(: $asm_sysreg:tt)?, $type:ty : $bitflags_type:ty, safe, $fake_sysregs:expr) => {
        $crate::_paste::paste! {
            #[doc = "Writes `value` to the `"]
            #[doc = stringify!($sysreg)]
            #[doc = "` system register."]
            pub fn [< write_ $sysreg >](value: $bitflags_type) {
                $fake_sysregs.lock().unwrap().$sysreg = value;
            }
        }
    };
    (
        $(#[$attributes:meta])*
        $sysreg:ident $(: $asm_sysreg:tt)?, $type:ty : $bitflags_type:ty, $fake_sysregs:expr
    ) => {
        $crate::_paste::paste! {
            #[doc = "Writes `value` to the `"]
            #[doc = stringify!($sysreg)]
            #[doc = "` system register."]
            $(#[$attributes])*
            pub unsafe fn [< write_ $sysreg >](value: $bitflags_type) {
                $fake_sysregs.lock().unwrap().$sysreg = value;
            }
        }
    };
}
