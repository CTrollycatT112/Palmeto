// SPDX-License-Identifier: GPL-3.0-or-later
//
// Copyright (c) 2026 Trollycat
//
// Purpose: Instead of booting from 0xffffffff80000000,
//          modern kernels use 'relocation',
//          the problem is hackers could easily attack,
//          since the address is always 0xffffffff80000000,
//          relocating solves this issue,
//          by booting from a random address everytime

#[repr(C)]
struct Elf64Rela {
    offset: u64,
    info:   u64,
    addend: i64,
}

const R_AARCH64_RELATIVE: u64 = 1027;

unsafe extern "C" {
    static __rela_start: Elf64Rela;
    static __rela_end:   Elf64Rela;
}

unsafe fn apply_runtime_relocations(actual_kernel_base: u64) {
    const LINK_BASE: u64 = 0xffffffff80000000;
    let slide_offset     = actual_kernel_base.wrapping_sub(LINK_BASE);
    
    if slide_offset == 0 
    {
        return;
    }

    unsafe 
    {
        let mut current = &__rela_start as *const Elf64Rela;
        let end         = &__rela_end as *const Elf64Rela;

        while current < end 
        {
            let entry = &*current;

            if (entry.info & 0xffffffff) == R_AARCH64_RELATIVE 
            {
                let pointer_address  = (entry.offset + slide_offset) as *mut u64;
                let corrected_target = (entry.addend as u64) + slide_offset;
                *pointer_address     = corrected_target;
            }

            current = current.add(1);
        }
    }
}

pub fn init()
{
    let pc: u64;

    unsafe
    {
        core::arch::asm!("adrp {}, .", out(reg) pc);
        apply_runtime_relocations(pc);
    }
}