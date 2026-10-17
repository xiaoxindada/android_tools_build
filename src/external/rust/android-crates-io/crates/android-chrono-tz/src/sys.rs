// Copyright 2026 The android-chrono-tz Authors.
// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

//! Declarations for timezone-related functions from Bionic.

use libc::{c_char, c_int, time_t, tm};

/// Opaque type returned by `tzalloc`.
pub enum Timezone {}

/// Opaque pointer type returned by `tzalloc`.
#[allow(non_camel_case_types)]
pub type timezone_t = *mut Timezone;

// Timezone access functions added in API level 35. These are prefereable to `localtime_r` /
// `mktime` if they are available, as they don't access environment variables and so avoid any
// potential thread-safety issues with them.
unsafe extern "C" {
    /// Converts the number of seconds since the Unix epoch in `t` to a broken-down time in `tm`, in
    /// the given timezone.
    ///
    /// # Safety
    ///
    /// * `tz` must have been allocated by `tzalloc` and not yet freed.
    /// * `t` must point to a valid `time_t` which is not mutated during this call.
    /// * `tm` must point to a valid `tm` which is not accessed during this call.
    pub unsafe fn localtime_rz(tz: timezone_t, t: *const time_t, tm: *mut tm) -> *mut tm;

    /// Converts the broken-down time in `tm` into the number of seconds since the Unix epoch, in
    /// the given timezone.
    ///
    /// Returns the time in seconds on success, or -1 on failure.
    ///
    /// # Safety
    ///
    /// * `tz` must have been allocated by `tzalloc` and not yet freed.
    /// * `tm` must point to a valid `tm` which is not accessed during this call.
    pub unsafe fn mktime_z(tz: timezone_t, tm: *mut tm) -> time_t;

    /// Allocates a timezone corresponding to the given Olson ID.
    ///
    /// If `id` is null then returns the system timezone.
    ///
    /// # Safety
    ///
    /// `id` must either be null or a valid NUL-terminated C string.
    pub unsafe fn tzalloc(id: *const c_char) -> timezone_t;

    /// Frees a timezone object returned by `tzalloc`.
    ///
    /// # Safety
    ///
    /// `tz` must previously have been allocated by `tzalloc` and not yet freed.
    pub unsafe fn tzfree(tz: timezone_t);
}

pub const DST_IN_EFFECT: c_int = 1;
pub const DST_NOT_IN_EFFECT: c_int = 0;
pub const DST_UNKNOWN: c_int = -1;
