use crate::arch::arm64::exceptions::{ExceptionHandlers, RegisterStateRef};
use crate::exception_handlers;
use crate::{print, println};

pub struct KernelExceptionHandlers;

impl ExceptionHandlers for KernelExceptionHandlers {
    extern "C" fn sync_current(register_state: RegisterStateRef) {
        let esr: u64;
        let far: u64;
        unsafe {
            core::arch::asm!("mrs {}, esr_el1", out(reg) esr);
            core::arch::asm!("mrs {}, far_el1", out(reg) far);
        }

        println!("\nCPU EXCEPTION: Synchronous");
        println!("PC  (ELR_EL1) : {:#018x}", register_state.elr);
        println!("FAR (FAR_EL1) : {:#018x}", far);
        println!("ESR (ESR_EL1) : {:#018x}", esr);
        println!("SPSR          : {:#018x}", register_state.spsr);

        for (i, reg) in register_state.registers.iter().enumerate() {
            print!("x{:02}: {:#018x}  ", i, reg);
            if (i + 1) % 2 == 0 {
                println!();
            }
        }
        println!();

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