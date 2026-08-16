// SPDX-License-Identifier: GPL-3.0-or-later
//
// Copyright (c) 2026 Trollycat
//
// Purpose: This module manages routine for virtual memory,
//          acting as a 'virtual memory manager'.
//
//          For a clean guide on virtual memory:
//              https://en.wikipedia.org/wiki/Virtual_memory
//
use spin::Mutex;

use core::sync::atomic::AtomicU64;

use crate::mem::memory::PteFlags;

pub static HHDM_OFFSET:          AtomicU64 = AtomicU64::new(0);
pub static KERNEL_ADDRESS_SPACE: Mutex<Option<AddressSpace>> = Mutex::new(None);

pub(crate) struct PageTable
{
    pub(crate) directory: u64,
}

pub struct AddressSpace
{
    pub(crate) page_table: PageTable,
}

impl AddressSpace {
    pub fn new() -> Self {
        Self {
            page_table: PageTable::new(),
        }
    }

    pub fn map(&mut self, virt: u64, phys: u64, flags: PteFlags) {
        self.page_table.map(virt, phys, flags);
    }

    pub fn unmap(&mut self, virt: u64) -> Option<u64> {
        self.page_table.unmap(virt)
    }

    pub unsafe fn set(&self) {
        self.page_table.set();
    }
}