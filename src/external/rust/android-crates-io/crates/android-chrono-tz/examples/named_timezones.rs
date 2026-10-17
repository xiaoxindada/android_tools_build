// Copyright 2026 The android-chrono-tz Authors.
// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

//! Example to try some named timezones.
//!
//! To run on an attached Android device:
//!
//! ```sh
//! CARGO_NDK_PLATFORM=35 ANDROID_NDK_HOME=/usr/lib/android-ndk cargo ndk run --target aarch64-linux-android --example named_timezones
//! ```

use android_chrono_tz::Tz;
use chrono::TimeZone;

fn main() {
    let timezone = Tz::new("Europe/London").unwrap();
    println!("Timezone: {timezone:?}");
    let summer = timezone.with_ymd_and_hms(2026, 4, 1, 0, 0, 0).unwrap();
    println!("Summer time {summer}");
    let winter = timezone.with_ymd_and_hms(2026, 3, 1, 0, 0, 0).unwrap();
    println!("Winter time {winter}");

    let local = Tz::local();
    println!("Local timezone: {local:?}");
}
