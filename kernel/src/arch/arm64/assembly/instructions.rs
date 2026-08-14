// SPDX-License-Identifier: GPL-3.0-or-later
//
// Copyright (c) 2026 Trollycat
//
// Purpose: This file uses raw assembly instructions,
//          wrapping them in functions,
//          AARCH64 instructions are a little weird..
//          I'm used to X86_64...
//

#![allow(unused)]

use core::arch::{asm, global_asm};

pub struct ArmCpuIdResult {
    pub midr: u64,
    pub mpidr: u64,
    pub id_aa64pfr0: u64,
}

#[inline]
pub unsafe fn read_cpu_features() -> ArmCpuIdResult {
    let midr: u64;
    let mpidr: u64;
    let id_aa64pfr0: u64;
    unsafe {
        asm!(
            "mrs {0}, midr_el1",
            "mrs {1}, mpidr_el1",
            "mrs {2}, id_aa64pfr0_el1",
            out(reg) midr,
            out(reg) mpidr,
            out(reg) id_aa64pfr0,
        );
    }
    ArmCpuIdResult { midr, mpidr, id_aa64pfr0 }
}

#[inline]
pub unsafe fn read_sctlr_el1() -> u64 {
    let value: u64;
    unsafe {
        asm!("mrs {0}, sctlr_el1", out(reg) value);
    }
    value
}

#[inline]
pub unsafe fn write_sctlr_el1(value: u64) {
    unsafe {
        asm!("msr sctlr_el1, {0}", in(reg) value);
    }
}

#[inline]
pub fn rdtsc() -> u64 {
    let value: u64;
    unsafe {
        asm!("mrs {0}, cntvct_el0", out(reg) value);
    }
    value
}

#[inline]
pub unsafe fn save_fp_context(memory: *mut u128) {
    unsafe {
        asm!(
            "stp q0, q1, [{0}]",
            "stp q2, q3, [{0}, #32]",
            in(reg) memory
        );
    }
}

#[inline]
pub unsafe fn mmio_read8(address: usize) -> u8 {
    unsafe {
        core::ptr::read_volatile(address as *const u8)
    }
}

#[inline]
pub unsafe fn mmio_write8(address: usize, value: u8) {
    unsafe {
        core::ptr::write_volatile(address as *mut u8, value);
    }
}

#[inline]
pub unsafe fn read_tls_pointer() -> u64 {
    let value: u64;
    unsafe {
        asm!("mrs {0}, tpidr_el0", out(reg) value);
    }
    value
}

#[inline]
pub unsafe fn write_tls_pointer(value: u64) {
    unsafe {
        asm!("msr tpidr_el0, {0}", in(reg) value);
    }
}

#[inline]
pub unsafe fn cli() {
    unsafe {
        asm!("msr daifset, #2");
    }
}

#[inline]
pub unsafe fn sti() {
    unsafe {
        asm!("msr daifclr, #2", options(nostack, preserves_flags));
    }
}

#[inline]
pub unsafe fn halt() {
    unsafe {
        asm!("wfi");
    }
}

pub fn halt_forever() {
    loop {
        unsafe {
            cli();
            halt();
        }
    }
}

#[inline]
pub unsafe fn interrupt_state() -> bool {
    let value: u64;
    unsafe {
        asm!("mrs {0}, daif", out(reg) value);
    }
    (value & (1 << 7)) == 0
}

#[inline]
pub unsafe fn toggle_interrupts(state: bool) -> bool {
    let current_state = unsafe { interrupt_state() };
    if state {
        unsafe { sti(); }
    } else {
        unsafe { cli(); }
    }
    current_state
}
