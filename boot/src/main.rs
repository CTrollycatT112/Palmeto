#![no_std]
#![no_main]

use kernel::arch;

use drivers::{println};

use core::panic::PanicInfo;

use limine::request::HhdmRequest;

#[used]
static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    arch::init();
    
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

//
// TODO:
//  IMPLEMENT A LOGGER,
//  AND PROPERLY PANIC,
//  WITH WHAT WENT WRONG, AND MORE INFORMATION
//
#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    println!("PANIC YOU NOOB....");
    println!("{info}");

    loop {
        core::hint::spin_loop();
    }
}