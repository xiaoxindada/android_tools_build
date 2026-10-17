// SPDX-FileCopyrightText: Copyright The arm-sysregs Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Access to Arm CPU system registers.

#![cfg_attr(not(any(test, feature = "fakes")), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
pub mod accessors;
#[cfg(any(test, feature = "fakes"))]
pub mod fake;
pub mod helpers;
mod manual;
pub mod registers;
