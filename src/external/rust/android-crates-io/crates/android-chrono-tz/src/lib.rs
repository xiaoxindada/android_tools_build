// Copyright 2026 The android-chrono-tz Authors.
// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

//! This crates provides [`Local`], a chrono `TimeZone` implementation to correctly fetch the local
//! timezone on Android, using the `localtime_rz` and `mktime_z` functions from Bionic.
//!
//! Unlike `localtime` and `mktime` these functions are fully thread-safe. They were added in
//! Android 15 (API level 35) so won't work in earlier Android versions. If you're using this crate
//! in an Android app then ensure your `minSdk` is set to 35 or higher.
//!
//! It also provides [`Tz`] to use arbitrary timezones from the system timezone database, by Olson
//! ID.

mod inner;
mod local;
mod sys;

use crate::inner::TzInner;
pub use crate::local::Local;
use chrono::{FixedOffset, MappedLocalTime, NaiveDate, NaiveDateTime, NaiveTime, Offset, TimeZone};
use libc::time_t;
use std::{
    ffi::NulError,
    fmt::{self, Debug, Display, Formatter},
    io,
    sync::Arc,
};
use thiserror::Error;

/// An error getting a timezone from the system database.
#[derive(Debug, Error)]
pub enum TzError {
    /// The given timezone ID string contained a NUL byte.
    #[error("Invalid string for timezone ID: {0}")]
    InvalidId(#[from] NulError),
    /// There was an error allocating a timezone with the given ID.
    #[error("Error allocating timezone: {0}")]
    Io(#[from] io::Error),
}

/// An offset for a timezone from the system database.
#[derive(Clone, Debug)]
pub struct TzOffset {
    timezone: Arc<TzInner>,
    time: time_t,
}

impl TzOffset {
    /// Returns the name of the timezone offset.
    pub fn name(&self) -> &str {
        self.timezone.name_at_utc_timestamp(self.time)
    }
}

impl Display for TzOffset {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl Offset for TzOffset {
    fn fix(&self) -> FixedOffset {
        self.timezone.offset_at_utc_timestamp(self.time)
    }
}

/// A named timezone.
#[derive(Clone, Debug)]
pub struct Tz {
    timezone: Arc<TzInner>,
}

impl Tz {
    /// Returns a new `Tz` for the timezone with the given Olson ID.
    pub fn new(olson_id: &str) -> Result<Self, TzError> {
        Ok(Self {
            timezone: Arc::new(TzInner::new(olson_id)?),
        })
    }

    /// Returns a new `Tz` for the current local system timezone as seen in Settings, from the
    /// `persist.sys.timezone` property.
    ///
    /// Note that this ignores the `TZ` environment variable.
    pub fn local() -> Result<Self, TzError> {
        Ok(Self {
            timezone: Arc::new(TzInner::local()?),
        })
    }
}

impl TimeZone for Tz {
    type Offset = TzOffset;

    fn from_offset(offset: &TzOffset) -> Self {
        Self {
            timezone: offset.timezone.clone(),
        }
    }

    fn offset_from_local_date(&self, local: &NaiveDate) -> MappedLocalTime<TzOffset> {
        self.offset_from_local_datetime(&local.and_time(NaiveTime::MIN))
    }

    fn offset_from_local_datetime(&self, local: &NaiveDateTime) -> MappedLocalTime<TzOffset> {
        self.timezone
            .timestamp_from_local_datetime(local)
            .map(|time| TzOffset {
                timezone: self.timezone.clone(),
                time,
            })
    }

    fn offset_from_utc_date(&self, utc: &NaiveDate) -> TzOffset {
        self.offset_from_utc_datetime(&utc.and_time(NaiveTime::MIN))
    }

    fn offset_from_utc_datetime(&self, utc: &NaiveDateTime) -> TzOffset {
        TzOffset {
            timezone: self.timezone.clone(),
            time: utc.and_utc().timestamp() as time_t,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn london_names() {
        let london = Tz::new("Europe/London").unwrap();

        let winter = london.with_ymd_and_hms(2026, 3, 1, 0, 0, 0).unwrap();
        assert_eq!(winter.offset().name(), "GMT");

        let summer = london.with_ymd_and_hms(2026, 4, 1, 0, 0, 0).unwrap();
        assert_eq!(summer.offset().name(), "BST");
    }

    #[test]
    fn london_offsets() {
        let london = Tz::new("Europe/London").unwrap();

        let winter = london.with_ymd_and_hms(2026, 3, 1, 0, 0, 0).unwrap();
        assert_eq!(winter.offset().fix(), FixedOffset::east_opt(0).unwrap());

        let summer = london.with_ymd_and_hms(2026, 4, 1, 0, 0, 0).unwrap();
        assert_eq!(summer.offset().fix(), FixedOffset::east_opt(3600).unwrap());
    }

    #[test]
    fn dst_boundaries() {
        let london = Tz::new("Europe/London").unwrap();

        // Start of BST.
        let start_local = NaiveDate::from_ymd_opt(2026, 3, 29)
            .unwrap()
            .and_hms_opt(1, 30, 0)
            .unwrap();
        assert_eq!(
            london
                .offset_from_local_datetime(&start_local)
                .map(|offset| offset.fix()),
            MappedLocalTime::None
        );

        // End of BST.
        let end_local = NaiveDate::from_ymd_opt(2026, 10, 25)
            .unwrap()
            .and_hms_opt(1, 30, 0)
            .unwrap();
        assert_eq!(
            london
                .offset_from_local_datetime(&end_local)
                .map(|offset| offset.fix()),
            MappedLocalTime::Ambiguous(
                FixedOffset::east_opt(0).unwrap(),
                FixedOffset::east_opt(3600).unwrap()
            )
        );
    }

    #[test]
    fn offsets_from_utc() {
        let london = Tz::new("Europe/London").unwrap();

        let winter = NaiveDate::from_ymd_opt(2026, 3, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        assert_eq!(
            london.offset_from_utc_datetime(&winter).fix(),
            FixedOffset::east_opt(0).unwrap()
        );

        let summer = NaiveDate::from_ymd_opt(2026, 4, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        assert_eq!(
            london.offset_from_utc_datetime(&summer).fix(),
            FixedOffset::east_opt(3600).unwrap()
        );
    }

    #[test]
    fn local_same() {
        let local = Local.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap();
        let local_from_tz = Tz::local()
            .unwrap()
            .with_ymd_and_hms(2026, 1, 1, 0, 0, 0)
            .unwrap();
        assert_eq!(local, local_from_tz);
    }
}
