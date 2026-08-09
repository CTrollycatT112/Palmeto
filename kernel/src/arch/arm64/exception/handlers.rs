// SPDX-License-Identifier: Apache-2.0
//
// Copyright (c) 2026 Palmeto OS Team
//
// Author:      Trollycat
//
// Module:      Exception Handlers
//
// Description: This is where any exception handlers go,
//              if you want a certain function called instead of panic,
//              you register it here,
//              said function will be called upon the exception
//


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