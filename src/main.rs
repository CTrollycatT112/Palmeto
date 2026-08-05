#![no_std]
#![no_main]

mod drivers;

use drivers::tty::serial::SerialHardware;

use core::panic::PanicInfo;
use core::fmt::Write;

use limine::request::HhdmRequest;

#[used]
static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    if let Some(hhdm_response) = HHDM_REQUEST.response() 
    {
        let mut serial = unsafe
        {
            SerialHardware::new(hhdm_response.offset)
        };

        let cpu_id = 0;
        let _ = write!(serial, "KERNEL BOOTING...\n");
        let _ = write!(serial, "CPU: #{}\n", cpu_id);
        let _ = write!(serial, "OFFSET: {:#X}\n", hhdm_response.offset);
    }

    loop {
        core::hint::spin_loop();
    }
}

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}