// Copyright 2025 The safe-mmio Authors.
// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

use crate::backend::mmio_ops::MmioOps;

macro_rules! asm_read {
    ($ins:literal, $reg:literal, $src:expr) => {{
        let value;
        // SAFETY: Caller guarantees src is valid and aligned.
        unsafe {
            core::arch::asm!(
                concat!($ins, " {value:", $reg, "}, [{ptr}]"),
                value = out(reg) value,
                ptr = in(reg) $src,
            );
        }
        value
    }};
}

macro_rules! asm_write {
    ($ins:literal, $reg:literal, $dst:expr, $value:expr) => {
        // SAFETY: Caller guarantees dst is valid and aligned.
        unsafe {
            core::arch::asm!(
                concat!($ins, " {value:", $reg, "}, [{ptr}]"),
                value = in(reg) $value,
                ptr = in(reg) $dst,
            );
        }
    };
}

/// MmioOps backend using aarch64 inline assembly for MMIO access.
pub struct Ops;

impl MmioOps for Ops {
    unsafe fn read_u8(src: *const u8) -> u8 {
        asm_read!("ldrb", "w", src)
    }

    unsafe fn read_u16(src: *const u16) -> u16 {
        asm_read!("ldrh", "w", src)
    }

    unsafe fn read_u32(src: *const u32) -> u32 {
        asm_read!("ldr", "w", src)
    }

    unsafe fn read_u64(src: *const u64) -> u64 {
        asm_read!("ldr", "x", src)
    }

    unsafe fn write_u8(dst: *mut u8, value: u8) {
        asm_write!("strb", "w", dst, value);
    }

    unsafe fn write_u16(dst: *mut u16, value: u16) {
        asm_write!("strh", "w", dst, value);
    }

    unsafe fn write_u32(dst: *mut u32, value: u32) {
        asm_write!("str", "w", dst, value);
    }

    unsafe fn write_u64(dst: *mut u64, value: u64) {
        asm_write!("str", "x", dst, value);
    }
}
