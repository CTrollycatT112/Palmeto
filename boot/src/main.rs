// SPDX-License-Identifier: Apache-2.0
//
// Copyright (c) 2026 Palmeto OS Team
//
// Author:      Trollycat
//
// Module:      Entry point
//
// Description: This is the entry point file for the O/S,
//              It contains "_start" (or sometimes known as 'kmain')
//              It will handle VERY early initialization

#![no_std]
#![no_main]

use kernel::arch;
use drivers::println;
use core::panic::PanicInfo;
use limine::request::{HhdmRequest, ExecutableCmdlineRequest};

#[used]
static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();

#[used]
static CMDLINE_REQUEST: ExecutableCmdlineRequest = ExecutableCmdlineRequest::new();

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    arch::init();
    
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
                }
            }
        }
    }

    if let Some(hhdm_response) = HHDM_REQUEST.response() {
        drivers::tty::serial::init(hhdm_response.offset);
    }

    println!("\nKERNEL BOOTING...");
    println!("CPU: #0");

    if let Some(resp) = HHDM_REQUEST.response() {
        println!("HHDM OFFSET: {:#X}", resp.offset);
    }

    loop {
        core::hint::spin_loop();
    }
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    println!("PANIC YOU NOOB....");
    println!("{info}");

    loop {
        core::hint::spin_loop();
    }
}
