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

mod reloc;
mod request;

use kernel::arch;
use drivers::println;
use request::{HHDM_REQUEST, DTB_REQUEST, CMDLINE_REQUEST};

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let runtime_pc: u64;
    unsafe {
        core::arch::asm!("adrp {}, .", out(reg) runtime_pc);
        reloc::apply_runtime_relocations(runtime_pc);
    }

    arch::init();

    let hhdm_offset = HHDM_REQUEST
            .response()
            .map(|r| r.offset)
            .unwrap_or(0);

    let serial_ok;
    match DTB_REQUEST.response() 
    {
        Some(dtb_resp) => 
        {
            let dtb_ptr = dtb_resp.dtb_ptr as *const u8;
            serial_ok = 
                drivers::tty::serial::init_from_dtb
                (dtb_ptr, hhdm_offset);

            if !serial_ok
            {
                //
                // TODO:
                //  ONCE FLANTERM IS WORKING,
                //  WE SHOULD PRINT TO THE SCREEN INSTEAD,
                //  BUT WE CAN'T PRINT TO CONSOLE,
                //  AS THIS LITERALLY MEANS WE HAVE NO SERIAL..
                //
                core::hint::spin_loop();
            }
        }
        None => {
            core::hint::spin_loop();
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
                }
            }
        }
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