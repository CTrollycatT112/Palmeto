use crate::arch::arm64::exception::exceptions::{ExceptionHandlers, RegisterStateRef};
use crate::exception_handlers;

pub struct KernelExceptionHandlers;

#[allow(unused)]
impl ExceptionHandlers for KernelExceptionHandlers {
    extern "C" fn sync_current(register_state: RegisterStateRef) {
        let esr: u64;
        let far: u64;
        unsafe {
            core::arch::asm!("mrs {}, esr_el1", out(reg) esr);
            core::arch::asm!("mrs {}, far_el1", out(reg) far);
        }


        for (i, reg) in register_state.registers.iter().enumerate() {
            if (i + 1) % 2 == 0 {
            }
        }

        panic!("Unrecoverable CPU exception");
    }
}

exception_handlers!(KernelExceptionHandlers);

pub fn init() {
    unsafe {
        core::arch::asm!(
            "adr x0, vector_table_el1",
            "msr vbar_el1, x0",
            options(nomem, nostack)
        );
    }
}