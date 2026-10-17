// Copyright 2026 The safe-mmio Authors.
// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

#[cfg(all(target_arch = "aarch64", not(feature = "custom-mmio")))]
pub mod aarch64;
#[cfg(feature = "custom-mmio")]
pub mod custom;
pub mod mmio_ops;
#[cfg(all(not(target_arch = "aarch64"), not(feature = "custom-mmio")))]
pub mod volatile;

#[cfg(all(target_arch = "aarch64", not(feature = "custom-mmio")))]
pub use aarch64::Ops;
#[cfg(feature = "custom-mmio")]
pub use custom::Ops;
#[cfg(all(not(target_arch = "aarch64"), not(feature = "custom-mmio")))]
pub use volatile::Ops;
