// Copyright 2026 The android-chrono-tz Authors.
// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

//! Example to print offsets of the local timezone from UTC at various times, from both UTC and the
//! local timezone.
//!
//! To run on an attached Android device:
//!
//! ```sh
//! CARGO_NDK_PLATFORM=35 ANDROID_NDK_HOME=/usr/lib/android-ndk cargo ndk run --target aarch64-linux-android --example local_offsets
//! ```

use android_chrono_tz::Local;
use chrono::{NaiveDate, TimeZone, Utc};

fn main() {
    println!("Local time: {}", Local::now());

    let winter = Utc.with_ymd_and_hms(2026, 3, 1, 12, 0, 0).unwrap();
    println!(
        "Offset at {winter}: {}",
        Local.offset_from_utc_datetime(&winter.naive_utc())
    );

    let summer = Utc.with_ymd_and_hms(2026, 4, 1, 12, 0, 0).unwrap();
    println!(
        "Offset at {summer}: {}",
        Local.offset_from_utc_datetime(&summer.naive_utc())
    );

    let winter_local = Local.with_ymd_and_hms(2026, 3, 1, 12, 0, 0).unwrap();
    println!(
        "Offset at {winter_local}: {:?}",
        Local.offset_from_local_datetime(&winter_local.naive_local())
    );

    let summer_local = Local.with_ymd_and_hms(2026, 4, 1, 12, 0, 0).unwrap();
    println!(
        "Offset at {summer_local}: {:?}",
        Local.offset_from_local_datetime(&summer_local.naive_local())
    );

    // Start of summer time in the UK and western Europe. This should be `None`.
    let start_local = NaiveDate::from_ymd_opt(2026, 3, 29)
        .unwrap()
        .and_hms_opt(1, 30, 0)
        .unwrap();
    println!(
        "At start of DST {start_local}: {:?}",
        Local.offset_from_local_datetime(&start_local)
    );

    // End of summer time in the UK and western Europe. This should be `Ambiguous`.
    let end_local = NaiveDate::from_ymd_opt(2026, 10, 25)
        .unwrap()
        .and_hms_opt(1, 30, 0)
        .unwrap();
    println!(
        "At end of DST {end_local}: {:?}",
        Local.offset_from_local_datetime(&end_local)
    );
}
