// SPDX-License-Identifier: Apache-2.0
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

use shared::{debug, fatal, println};

use shared::core::requests::{HHDM_REQUEST, DTB_REQUEST, CMDLINE_REQUEST};

use kernel::arch;
use kernel::fbcon;

use core::panic::PanicInfo;

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

    debug!("FBCON INITIALIZED...");

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

    for i in 0..51
    {
        debug!("VALUE: {}", i);
    }

    loop {
        core::hint::spin_loop();
    }
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    println!("=========================");
    println!("   KERNEL PANIC: ");
    println!("          {info}       ");

    loop {
        core::hint::spin_loop();
    }
}