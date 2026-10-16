// Copyright 2026, The Android Open Source Project
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Modprobe modules when selected via kcmdlinectrl.

use log::warn;
use std::fs::OpenOptions;
use std::io::{Error, ErrorKind, Read, Result, Write};

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("kcmdlinemodprobe.hpp");
        fn LoadRustBinder() -> bool;
    }
}

fn should_load_rust_binder() -> Result<bool> {
    let mut buf = [0; 16];

    // Reading the actual kernel parameter ensures that we take BoardConfig into account.
    let file = OpenOptions::new().read(true).write(true).open("/sys/module/binder/parameters/impl");

    let mut file = match file {
        Ok(file) => file,
        Err(err) if err.kind() == ErrorKind::NotFound => return Ok(false),
        Err(err) => return Err(err),
    };

    // Check the value of binder.impl=
    let len = file.read(&mut buf)?;
    match &buf[..len] {
        b"rust\n" => { /* continue below */ }
        b"c\n" => return Ok(false),
        _ => return Err(Error::other("unknown binder.impl value")),
    }

    // Check whether Rust Binder was already loaded.
    //
    // This skips the modprobe on platforms where rust_binder.ko is modprobed using another
    // mechanism (such as during first-stage-init).
    match file.write(b"invalid value") {
        // Not writable means that Binder is already loaded.
        Err(err) if err.kind() == ErrorKind::PermissionDenied => Ok(false),
        // Writable but invalid value means that Binder is not loaded yet.
        Err(err) if err.kind() == ErrorKind::InvalidInput => Ok(true),
        Err(err) => Err(err),
        Ok(_) => Err(Error::other("should not accept invalid value")),
    }
}

fn real_main() -> i32 {
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Info)
            .with_tag("kcmdlinemodprobe"),
    );

    match should_load_rust_binder() {
        Ok(true) => match ffi::LoadRustBinder() {
            true => 0,
            false => 1,
        },
        Ok(false) => 0,
        Err(err) => {
            warn!("failed to check driver status: {err}");
            1
        }
    }
}

fn main() {
    std::process::exit(real_main());
}
