// SPDX-License-Identifier: GPL-2.0-or-later
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
    static __start_text: u8;
}

///
/// This routine handles relocating the kernel image,
/// and adjusts addresses to match runtime load addresses.
///
/// # Arguments
///
/// * actual_kernel_base - virtual memory address where the kernel was loaded (by Limine)
///
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

///
/// This routine calls the internal initialization functions,
/// so that the main function can remain clean
///
pub fn init()
{
    let actual_kernel_base: u64;

    unsafe
    {
        core::arch::asm!("adrp {}, __start_text", out(reg) actual_kernel_base);
        apply_runtime_relocations(actual_kernel_base);
    }
}