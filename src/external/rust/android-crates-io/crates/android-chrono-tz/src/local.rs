// Copyright 2026 The android-chrono-tz Authors.
// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

//! `Local` timezone type.

use crate::{Tz, TzOffset, inner::TzInner};
use chrono::{DateTime, MappedLocalTime, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use libc::time_t;
use std::sync::Arc;

/// The local system timezone as seen in Settings, from the `persist.sys.timezone` property.
///
/// Note that this ignores the `TZ` environment variable.
#[derive(Copy, Clone, Debug)]
pub struct Local;

impl Local {
    /// Returns a `DateTime<Local>` for the current date, time and timezone offset.
    pub fn now() -> DateTime<Self> {
        Utc::now().with_timezone(&Self)
    }
}

impl TimeZone for Local {
    type Offset = TzOffset;

    fn from_offset(_offset: &TzOffset) -> Self {
        Self
    }

    fn offset_from_local_date(&self, local: &NaiveDate) -> MappedLocalTime<TzOffset> {
        self.offset_from_local_datetime(&local.and_time(NaiveTime::MIN))
    }

    fn offset_from_local_datetime(&self, local: &NaiveDateTime) -> MappedLocalTime<TzOffset> {
        Tz::local().unwrap().offset_from_local_datetime(local)
    }

    fn offset_from_utc_date(&self, utc: &NaiveDate) -> TzOffset {
        self.offset_from_utc_datetime(&utc.and_time(NaiveTime::MIN))
    }

    fn offset_from_utc_datetime(&self, utc: &NaiveDateTime) -> TzOffset {
        TzOffset {
            timezone: Arc::new(TzInner::local().unwrap()),
            time: utc.and_utc().timestamp() as time_t,
        }
    }
}
