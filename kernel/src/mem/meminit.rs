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
// NOTE:
//  This entire system is basically just a copy of what polaris did,
//  only to get me started.
//
//  A little bit of time and this will be reworked.
//  Here's a list of some things i would like to do..

//
// Use a buddy allocator instead of a bitmap
//

//
// Add support for 1G huge pages
//

//
// Add tagging to the allocator (sinpired by nT)
//

//
// Split logic into better APIs / modules
//

//
// Git rid of bump allocator and bad code
//

//
// TODO:
//
#![allow(unused)]

use shared::core::requests::{HHDM_REQUEST,
                             MEMMAP_REQUEST};

use limine::memmap;

use core::sync::atomic::Ordering;

use crate::mem::{memory::{ADDRESS_MASK,
                          BIG_ALLOC_BASE, 
                          HUGE_PAGE_2MB, 
                          PAGE_SIZE, 
                          PFN_DATABASE_BASE, 
                          PteFlags, 
                          STACK_ALLOC_BASE}, phys::{PMM, 
                       PageUsage}, virt::{HHDM_OFFSET, PageTable}
            };


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

fn address_to_indices(virt: u64) -> (usize, usize, usize, usize) {
    (
        ((virt >> 39) & 0x1FF) as usize,
        ((virt >> 30) & 0x1FF) as usize,
        ((virt >> 21) & 0x1FF) as usize,
        ((virt >> 12) & 0x1FF) as usize,
    )
}

fn get_next_level(level: &mut Table, index: usize, alloc: bool) -> Option<*mut Table> {
    let entry = &mut level.entries[index];

    if entry.is_present() {
        return Some((entry.addr() + HHDM_OFFSET.load(Ordering::Relaxed)) as *mut Table);
    }

    if !alloc {
        return None;
    }

    let next_level_phys = PMM.lock().as_mut().unwrap().alloc(PageUsage::PageTable)?;

    unsafe {
        let next_level_ptr = (next_level_phys + HHDM_OFFSET.load(Ordering::Relaxed)) as *mut u8;
        core::ptr::write_bytes(next_level_ptr, 0, PAGE_SIZE);
    }

    entry.set(
        next_level_phys,
        PteFlags::PRESENT | PteFlags::TABLE_BLOCK | PteFlags::ACCESSED,
    );

    Some((next_level_phys + HHDM_OFFSET.load(Ordering::Relaxed)) as *mut Table)
}

impl PageTable {
    pub fn new() -> Self 
    {
        let dir = PMM
            .lock()
            .as_mut()
            .unwrap()
            .alloc(PageUsage::PageTable)
            .expect("Failed to allocate page for page table?");

        unsafe 
        {
            let dir_ptr = (dir + (HHDM_OFFSET.load(Ordering::Relaxed))) as *mut u8;
            core::ptr::write_bytes(dir_ptr, 0, PAGE_SIZE);
        }

        Self { directory: dir }
    }

    fn top_level(&self) -> &mut Table {
        unsafe { &mut *((self.directory + HHDM_OFFSET.load(Ordering::Relaxed)) as *mut Table) }
    }

    pub fn set(&self) {
        unsafe {
            core::arch::asm!(
                "msr ttbr1_el1, {reg}",
                "dsb ish",
                "isb",
                "tlbi vmalle1",
                "dsb ish",
                "isb",
                reg = in(reg) self.directory,
                options(nostack, preserves_flags)
            );
        }
    }

    pub fn map(&mut self, virt: u64, phys: u64, flags: PteFlags) -> Option<()> {
        let (i4, i3, i2, i1) = address_to_indices(virt);

        let pdpt = unsafe { &mut *get_next_level(self.top_level(), i4, true)? };
        let pd = unsafe { &mut *get_next_level(pdpt, i3, true)? };
        let pt = unsafe { &mut *get_next_level(pd, i2, true)? };

        pt.entries[i1].set(phys, flags | PteFlags::PRESENT | PteFlags::TABLE_BLOCK | PteFlags::ACCESSED);
        Some(())
    }

    pub fn map_large(&mut self, virt: u64, phys: u64, flags: PteFlags) -> Option<()> {
        let (i4, i3, i2, _i1) = address_to_indices(virt);

        let pdpt = unsafe { &mut *get_next_level(self.top_level(), i4, true)? };
        let pd = unsafe { &mut *get_next_level(pdpt, i3, true)? };

        let entry = &mut pd.entries[i2];
        let block_flags = (flags | PteFlags::PRESENT | PteFlags::ACCESSED).bits() & !(1 << 1);
        entry.0 = (phys & 0x0000_ffff_ffff_f000) | block_flags;

        Some(())
    }

    pub fn unmap(&mut self, virt: u64) -> Option<u64> {
        let (i4, i3, i2, i1) = address_to_indices(virt);

        let pdpt = unsafe { &mut *get_next_level(self.top_level(), i4, false)? };
        let pd = unsafe { &mut *get_next_level(pdpt, i3, false)? };
        let pt = unsafe { &mut *get_next_level(pd, i2, false)? };

        let entry = &mut pt.entries[i1];
        if !entry.is_present() {
            return None;
        }

        let phys = entry.addr();
        *entry = Entry(0);

        Some(phys)
    }

    pub fn remap(&mut self, virt: u64, flags: PteFlags) -> Option<()> {
        let (i4, i3, i2, i1) = address_to_indices(virt);

        let pdpt = unsafe { &mut *get_next_level(self.top_level(), i4, false)? };
        let pd = unsafe { &mut *get_next_level(pdpt, i3, false)? };
        let pt = unsafe { &mut *get_next_level(pd, i2, false)? };

        let entry = &mut pt.entries[i1];
        if !entry.is_present() {
            return None;
        }

        let phys = entry.addr();
        entry.set(phys, flags | PteFlags::PRESENT);

        Some(())
    }
}

unsafe extern "C" {
    static __start_text:   [u8; 0];
    static __stop_text:    [u8; 0];
    static __start_rodata: [u8; 0];
    static __stop_rodata:  [u8; 0];
    static __start_data:   [u8; 0];
    static __stop_data:    [u8; 0];
}

fn align_down(x: u64, align: u64) -> u64 {
    x & !(align - 1)
}
fn align_up(x: u64, align: u64) -> u64 {
    (x + align - 1) & !(align - 1)
}

pub unsafe fn init(
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