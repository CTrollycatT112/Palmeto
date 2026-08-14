// SPDX-License-Identifier: GPL-3.0-or-later
//
// Copyright (c) 2026 Trollycat
//
// Purpose: This is the entry point file for the O/S,
//          It contains "_start"
//          It will handle VERY early initialization

#![no_std]
#![no_main]

mod reloc;
mod dtbinit;

//
// !!! KERNEL IMPORTS
//
use kernel::fbcon;
use kernel::arch;
use kernel::arch::arm64::assembly::instructions;
use kernel::arch::arm64::exception::timer;

//
// !!! SHARED IMPORTS
//
use shared::{debug, fatal, println};
use shared::core::requests::{HHDM_REQUEST, DTB_REQUEST, CMDLINE_REQUEST, DATE_AT_BOOT_REQUEST};

//
// !!! RUST IMPORTS
//
use core::panic::PanicInfo;

pub fn _init_boot_time()
{
    if let Some(response) = DATE_AT_BOOT_REQUEST.response()
    {
        let boot_time_seconds = response.timestamp;
        
        match timer::set_manual_unix_time_ms((boot_time_seconds as u64) * 1_000)
        {
            Ok(()) =>
            {

                let mut dt = timer::DateTime
                {
                    year: 0,
                    month: 0,
                    day: 0,
                    hour: 0,
                    minute: 0,
                    second: 0,
                };

                if timer::now_datetime(&mut dt, true)
                {
                    let mut buf = [0u8; 64];

                    timer::datetime_to_string(&dt, &mut buf, 20);
                
                    if let Ok(time_str) = core::str::from_utf8(&buf)
                    {
                        debug!("SYSTEM TIME: {}", time_str.trim_end_matches('\0'));
                    }
                }
            }
            Err(status) => {
                debug!("FAILED TO SET BOOT TIME, STATUS: {:?}", status);
            }
        }
    } else {
        fatal!("LIMINE FAILED TO PROVIDE DATE");
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {

    let runtime_pc: u64;
    unsafe {
        core::arch::asm!("adrp {}, .", out(reg) runtime_pc);
        reloc::apply_runtime_relocations(runtime_pc);
    }

    arch::init();
    fbcon::initialize();
    fbcon::reset_display();

    let hhdm_offset = HHDM_REQUEST
        .response()
        .map(|r| r.offset)
        .unwrap_or(0);

    let mut serial_ok = false;
    
    match DTB_REQUEST.response() 
    {
        Some(dtb_resp) => 
        {
            let dtb_ptr = dtb_resp.dtb_ptr as *const u8;

            if let Ok(()) = dtbinit::init_dtb(dtb_ptr, hhdm_offset) {
                serial_ok = true;
            }

            if !serial_ok
            {
                fatal!("COULD NOT FIND SERIAL IN DTB");
            }
        }
        None => {
            fatal!("COULD NOT GET DTB_RESPONSE");
        }
    }

    _init_boot_time();

    if let Some(cmd_response) = CMDLINE_REQUEST.response() {
        let raw_ptr: *const u8 = cmd_response.cmdline().as_ptr();

        if !raw_ptr.is_null() {
            let mut len = 0;
            unsafe {
                while *raw_ptr.offset(len) != 0 {
                    len += 1;
                }
                
                let byte_slice = core::slice::from_raw_parts(raw_ptr, len as usize);
                if let Ok(cmd_str) = core::str::from_utf8(byte_slice) {
                    kernel::cmdline::parse_and_store(cmd_str);
                    debug!("COMMAND LINE ARGUMENTS: {}", cmd_str);
                }
            }
        }
    }

    let resp = HHDM_REQUEST.response().unwrap_or_else(|| {
        fatal!("COULD NOT GET HHDM_RESPONSE");
    });
    let _ = resp.offset;

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