// Copyright 2026 The android-chrono-tz Authors.
// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

use crate::{
    TzError,
    sys::{
        DST_IN_EFFECT, DST_NOT_IN_EFFECT, DST_UNKNOWN, localtime_rz, mktime_z, timezone_t, tzalloc,
        tzfree,
    },
};
use chrono::{Datelike, FixedOffset, MappedLocalTime, NaiveDateTime, Timelike};
use libc::time_t;
use std::{
    ffi::{CStr, CString},
    io,
    ptr::null,
};

#[derive(Debug)]
pub struct TzInner {
    timezone: timezone_t,
}

impl TzInner {
    /// Returns a new `TzInner` for the timezone with the given Olson ID.
    pub fn new(olson_id: &str) -> Result<Self, TzError> {
        let id = CString::new(olson_id)?;
        // SAFETY: The id is a valid C string because it comes from a CString.
        let timezone = unsafe { tzalloc(id.as_ptr()) };
        if timezone.is_null() {
            Err(TzError::Io(io::Error::last_os_error()))
        } else {
            Ok(Self { timezone })
        }
    }

    /// Returns a new `TzInner` for the current local system timezone as seen in Settings, from the
    /// `persist.sys.timezone` property.
    ///
    /// Note that this ignores the `TZ` environment variable.
    pub fn local() -> Result<Self, TzError> {
        // SAFETY: tzalloc accepts a null pointer to use the current system timezone.
        let timezone = unsafe { tzalloc(null()) };
        if timezone.is_null() {
            Err(TzError::Io(io::Error::last_os_error()))
        } else {
            Ok(Self { timezone })
        }
    }

    /// Returns the name of the timezone at the given UTC time in seconds since the Unix epoch.
    pub fn name_at_utc_timestamp(&self, time: time_t) -> &str {
        let result = self.localtime_rz(time).expect("localtime_rz failed");
        // SAFETY: `tm_zone` should have been set to a valid C string pointer by `localtime_rz`.
        let tm_zone = unsafe { CStr::from_ptr(result.tm_zone) };
        tm_zone.to_str().unwrap()
    }

    /// Returns the offset from UTC which this timezone applies at the given UTC time in seconds
    /// since the Unix epoch.
    pub fn offset_at_utc_timestamp(&self, time: time_t) -> FixedOffset {
        let result = self.localtime_rz(time).expect("localtime_rz failed");
        FixedOffset::east_opt(
            result
                .tm_gmtoff
                .try_into()
                .expect("localtime_r returned invalid UTC offset"),
        )
        .expect("localtime_r returned invalid UTC offset")
    }

    /// Returns the UTC time in seconds since the Unix epoch corresponding to the given local time
    /// in this timezone.
    pub fn timestamp_from_local_datetime(&self, local: &NaiveDateTime) -> MappedLocalTime<time_t> {
        // Calling mktime with different isdst values allows us to detect the ambiguous case where DST
        // ends.
        let (utc_timestamp, _isdst) = self.mktime_with_dst(local, DST_UNKNOWN);
        let (utc_timestamp_without_dst, isdst0) = self.mktime_with_dst(local, DST_NOT_IN_EFFECT);
        let (utc_timestamp_with_dst, isdst1) = self.mktime_with_dst(local, DST_IN_EFFECT);
        if utc_timestamp == -1 {
            MappedLocalTime::None
        } else if isdst0 == isdst1 {
            MappedLocalTime::Single(utc_timestamp)
        } else {
            MappedLocalTime::Ambiguous(utc_timestamp_without_dst, utc_timestamp_with_dst)
        }
    }

    /// Calls `localtime_rz` for this timezone with an empty `tm` struct, and returns the resulting
    /// `tm` struct.
    fn localtime_rz(&self, time: time_t) -> Result<libc::tm, io::Error> {
        let mut result = libc::tm {
            tm_sec: 0,
            tm_min: 0,
            tm_hour: 0,
            tm_mday: 0,
            tm_mon: 0,
            tm_year: 0,
            tm_wday: 0,
            tm_yday: 0,
            tm_isdst: 0,
            tm_gmtoff: 0,
            tm_zone: null(),
        };

        // SAFETY: `timezone` was returned by `tzalloc`, and the other pointers are valid for the
        // duration of the call because they come from references.
        if unsafe { localtime_rz(self.timezone, &time, &mut result) }.is_null() {
            Err(io::Error::last_os_error())
        } else {
            Ok(result)
        }
    }

    /// Calls `mktime_z` from libc with the given local date-time and isdst value.
    ///
    /// Returns the timestamp and new isdst value set by `mktime_z`.
    fn mktime_with_dst(&self, local: &NaiveDateTime, isdst: i32) -> (time_t, i32) {
        let mut tm = libc::tm {
            tm_sec: local.second() as i32,
            tm_min: local.minute() as i32,
            tm_hour: local.hour() as i32,
            tm_mday: local.day() as i32,
            tm_mon: local.month0() as i32,
            tm_year: local.year() - 1900,
            tm_wday: 0,
            tm_yday: 0,
            tm_isdst: isdst,
            tm_gmtoff: 0,
            tm_zone: null(),
        };

        // SAFETY: mktime_z only accesses the struct tm it is passed during the call, and doesn't store
        // the pointer to access later. The tm_zone it sets in tm may only be valid as long as the
        // timezone is, but that's fine as we don't access tm_zone.
        let timestamp = unsafe { mktime_z(self.timezone, &mut tm) };

        (timestamp, tm.tm_isdst)
    }
}

impl Drop for TzInner {
    fn drop(&mut self) {
        // SAFETY: The timezone must be valid because it was allocated with `tzalloc`, and won't be
        // used anymore as this `Tz` is being dropped.
        unsafe { tzfree(self.timezone) }
    }
}

// SAFETY: `timezone_t` can be passed between threads.
unsafe impl Send for TzInner {}

// SAFETY: `TzInner` doesn't expose any methods to mutate the timezone, so it is safe to use from
// multiple threads simultaneously.
unsafe impl Sync for TzInner {}
