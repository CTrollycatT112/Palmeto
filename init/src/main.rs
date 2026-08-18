// SPDX-License-Identifier: GPL-3.0-or-later
//
// Copyright (c) 2026 Trollycat
//
// Purpose: This is the entry point file for the O/S,
//          It contains "_start"
//          It will handle VERY early initialization

#![no_std]
#![no_main]

//
// !!! MODULES
//
mod panic;
mod reloc;
mod mmdat;
mod dtbinit;
mod timinit;
mod cmdinit;

//
// !!! KERNEL IMPORTS
//
use kernel::fbcon;
use kernel::arch;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    //
    // RELOCATION HANDLING
    //
    let runtime_pc: u64;
    unsafe {
        core::arch::asm!("adrp {}, .", out(reg) runtime_pc);
        reloc::apply_runtime_relocations(runtime_pc);
    }

    //
    // ARCHITECTURE INITALIZATION
    //
    arch::init();

    //
    // FRAMEBUFFER CONSOLE INITIALIZATION
    //
    fbcon::initialize();
    fbcon::reset_display();

    //
    // DEVICE TREE BLOB INITIALIZATION
    //
    dtbinit::init().expect("Failed to initialize DTB...");
    
    //
    // TIMER INITIALIZATION
    //
    timinit::init_time();
    
    //
    // COMMAND LINE INITIALIZATION
    //
    cmdinit::init();

    loop {
        core::hint::spin_loop();
    }
}