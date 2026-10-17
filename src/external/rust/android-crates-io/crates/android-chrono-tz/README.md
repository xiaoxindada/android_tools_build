# android-chrono-tz

[![crates.io page](https://img.shields.io/crates/v/android-chrono-tz.svg)](https://crates.io/crates/android-chrono-tz)
[![docs.rs page](https://docs.rs/android-chrono-tz/badge.svg)](https://docs.rs/android-chrono-tz)

This crates provides a chrono `TimeZone` implementation to correctly fetch the local timezone on
Android, using the `localtime_rz` and `mktime_z` functions from Bionic.

Unlike `localtime` and `mktime` these functions are fully thread-safe. They were added in Android
15 (API level 35) so won't work in earlier Android versions. If you're using this crate in an
Android app then ensure your `minSdk` is set to 35 or higher.

It also provides access to arbitrary timezones from the system timezone database, by Olson ID.

This is not an officially supported Google product. This project is not eligible for the
[Google Open Source Software Vulnerability Rewards Program](https://bughunters.google.com/open-source-security).

## Development

Install the Android NDK and `cargo-ndk`, e.g.:

```sh
$ sudo apt install google-android-ndk-r28c-installer
$ cargo install cargo-ndk
$ rustup target add aarch64-linux-android
```

To run tests on a connected Android device:

```sh
$ CARGO_NDK_PLATFORM=35 ANDROID_NDK_HOME=/usr/lib/android-ndk cargo ndk test --target aarch64-linux-android
```

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

If you want to contribute to the project, see details of
[how we accept contributions](CONTRIBUTING.md).
