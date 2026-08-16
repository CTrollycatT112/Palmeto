// SPDX-License-Identifier: GPL-3.0-or-later
//
// Copyright (c) 2026 Trollycat
//
// Purpose: This module provides the entire memory system with constants,
//          some of which will be set during run-time ->
//
//          Sadly we cannot hard-code the values,
//          as I enabled relocating and KASLR.
//          
//          So they must be determined at run-time
//
use core::sync::atomic::{AtomicU64};

use bitflags::bitflags;

pub const PAGE_SIZE:        usize = 4096;
pub const PAGE_SHIFT:       usize = 12;
pub const STACK_SIZE:       usize = PAGE_SIZE * 16;

pub const HUGE_PAGE_2MB:    u64 = 0x200000;
pub const ADDRESS_MASK:     u64 = 0x0000_fffffffff000;

pub static PFN_DATABASE_BASE: AtomicU64 = AtomicU64::new(0);
pub static BIG_ALLOC_BASE:    AtomicU64 = AtomicU64::new(0);
pub static STACK_ALLOC_BASE:  AtomicU64 = AtomicU64::new(0);

bitflags!
{
    #[derive(Clone, Copy)]
    pub struct PteFlags: u64
    {
        const PRESENT     = 1 << 0;
        const TABLE_BLOCK = 1 << 1;

        const WRITABLE    = 0;
        const NO_EXECUTE  = 1 << 54;
        const ACCESSED    = 1 << 10;
        const SHAREABLE   = 3 << 8;
    }
}