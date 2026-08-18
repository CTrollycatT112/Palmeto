// SPDX-License-Identifier: GPL-2.0
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
mod relocate;
mod mmdat;
mod dtbinit;
mod timinit;
mod cmdinit;
mod bootinfo;

//
// !!! KERNEL IMPORTS
//
use kernel::fbcon;
use kernel::arch;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    //
    // RELOCATE INITIALIZATION
    //
    relocate::init();
    
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
    // BOOT INFORMATION FILLING
    //
    bootinfo::fill_info();

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