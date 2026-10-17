//! Drivers for specific VirtIO devices.

pub mod blk;
pub mod console;
pub mod gpu;
pub mod input;

pub mod net;

pub mod rng;

pub mod socket;
pub mod sound;

pub(crate) mod common;
