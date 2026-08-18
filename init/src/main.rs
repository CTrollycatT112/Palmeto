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
use kernel::arch::arm64::assembly::instructions;

//
// !!! SHARED IMPORTS
//
use shared::{println};

//
// !!! RUST IMPORTS
//
use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    
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

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    fbcon::set_kern_panic_color();

    let esr = unsafe { instructions::read_esr_el1() };
    let elr = unsafe { instructions::read_elr_el1() };
    let far = unsafe { instructions::read_far_el1() };
    let ec  = (esr >> 26) & 0x3F;

    println!("          KERNEL PANIC            ");
    println!("REASON: {info}");
    println!("EXCEPTION:");
    println!("  ESR_EL1: {esr:#018X} (Class: {ec:#04X})");
    println!("  ELR_EL1: {elr:#018X}");
    println!("  FAR_EL1: {far:#018X}");

    println!("STACK TRACE:");

    let mut fp: u64;

    unsafe {
        core::arch::asm!("mov {}, x29", out(reg) fp);
    }

    for _ in 0..10 {
        if fp == 0 {
            break;
        }
        let prev_fp = unsafe { *(fp as *const u64) };
        let lr = unsafe { *((fp + 8) as *const u64) };

        if lr == 0 {
            break;
        }

        println!("  at {lr:#018X}");
        fp = prev_fp;
    }

    loop {
        core::hint::spin_loop();
    }
}