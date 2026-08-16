// SPDX-License-Identifier: GPL-3.0-or-later
//
// Copyright (c) 2026 Trollycat
//
// Purpose: This module handles initialization of the memory subsystem,
//          It will setup the base for everything,
//          and the constants inside memory.rs
//
//          It's important that the initialization routines are called first,
//          otherwise variables will be uninitialized.
//

//
// TODO:
//
#![allow(unused)]

use shared::core::requests::{HHDM_REQUEST,
                             MEMMAP_REQUEST};

use limine::memmap;

use core::sync::atomic::Ordering;

use crate::mem::memory::{PFN_DATABASE_BASE,
                         BIG_ALLOC_BASE,
                         STACK_ALLOC_BASE,
                         PteFlags,
                         ADDRESS_MASK,
                         PAGE_SIZE,
                         HUGE_PAGE_2MB};


#[repr(transparent)]
#[derive(Clone, Copy)]
struct Entry(u64);

impl Entry
{
    fn is_present(&self) -> bool
    {
        (self.0 & PteFlags::PRESENT.bits()) != 0
    }

    fn addr(&self) -> u64
    {
        self.0 & ADDRESS_MASK
    }

    fn set(&mut self, phys: u64, flags: PteFlags)
    {
        self.0 = (phys & ADDRESS_MASK) | flags.bits();
    }
}

#[repr(C, align(4096))]
struct Table
{
    entries: [Entry; 512],
}

unsafe extern "C" {
    static __start_text: [u8; 0];
    static __stop_text: [u8; 0];
    static __start_rodata: [u8; 0];
    static __stop_rodata: [u8; 0];
    static __start_data: [u8; 0];
    static __stop_data: [u8; 0];
}

fn align_down(x: u64, align: u64) -> u64 {
    x & !(align - 1)
}
fn align_up(x: u64, align: u64) -> u64 {
    (x + align - 1) & !(align - 1)
}

pub unsafe fn init_memory_manager(
    entries: &[&memmap::Entry],
    hhdm_offset: u64,
    phys_kernel_base: u64,
    virt_kernel_base: u64,
)
{
    let highest = entries
        .iter()
        .filter(|e| e.type_ == memmap::MEMMAP_USABLE)
        .map(|e| e.base + e.length)
        .max()
        .expect("No usable memory found!");

    let pfn_base   = hhdm_offset + highest + 0x1000;
    let big_alloc  = pfn_base    + (1u64  << 30);
    let stack_base = big_alloc   + (16u64 << 30);

    PFN_DATABASE_BASE.store(pfn_base, Ordering::Release);
    BIG_ALLOC_BASE.store(big_alloc, Ordering::Release);
    STACK_ALLOC_BASE.store(stack_base, Ordering::Relaxed);

    //
    // TODO:
    //  ALLOCATE AND ZERO OUT TRANSLATION TABLE
    //
    let root_table_phys = 0x77;

    //
    // TODO:
    //  MAP HHDM (2MB HUGE PAGES)
    //

    //
    // TODO:
    //  PROTECT BINARY SECTIONS
    //

    unsafe {
        core::arch::asm!(
            "msr ttbr1_el1, {reg:x}",
            "dsb ish",
            "isb",
            "tlbi vmalle1",
            "dsb ish",
            "isb",
            reg = in(reg) root_table_phys,
            options(nostack, preserves_flags)
        );
    }
}