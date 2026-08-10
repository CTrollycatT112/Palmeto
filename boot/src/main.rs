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
mod dtbinit;

use kernel::arch;
use kernel::fbcon;
use drivers::println;
use request::{HHDM_REQUEST, DTB_REQUEST, CMDLINE_REQUEST};

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub static __stack_chk_guard: usize = 0x5905_fde0_90cc_0aaf;

#[unsafe(no_mangle)]
pub extern "C" fn __stack_chk_fail() -> ! {
    panic!("Stack smashing detected!");
}

fn num_to_str(mut num: u8, buf: &mut [u8]) -> &str {
    if num == 0 {
        buf[0] = b'0';
        return core::str::from_utf8(&buf[0..1]).unwrap();
    }
    
    let mut idx = buf.len();
    while num > 0 {
        idx -= 1;
        buf[idx] = b'0' + (num % 10);
        num /= 10;
    }
    
    core::str::from_utf8(&buf[idx..]).unwrap()
}

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

    fbcon::initialize();
    fbcon::reset_display();
    
    for i in 0..100
    {
        let mut buf = [0u8; 4];
        let s = num_to_str(i, &mut buf);
        fbcon::write_string(s);
        fbcon::write_string("\n");
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