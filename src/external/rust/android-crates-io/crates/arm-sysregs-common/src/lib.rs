// SPDX-FileCopyrightText: Copyright The arm-sysregs Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

#![cfg_attr(not(any(test, feature = "fakes")), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(all(not(any(test, feature = "fakes")), target_arch = "arm"))]
pub mod aarch32;
#[cfg(all(not(any(test, feature = "fakes")), target_arch = "aarch64"))]
pub mod aarch64;
#[cfg(any(test, feature = "fakes"))]
pub mod fake;
pub mod macros;
pub mod types;

#[doc(hidden)]
pub use paste as _paste;
