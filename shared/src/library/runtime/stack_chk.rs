// SPDX-License-Identifier: GPL-2.0
//
// Purpose: This file handles __stack related routines,
//          This would've been unneeded,
//          but flanterm for whatever reason relies on it,
//          so this is a dummy version..
//

#[unsafe(no_mangle)]
pub static __stack_chk_guard: usize = 0x5905_fde0_90cc_0aaf;

///
/// This rountine handles stack protector guard fails.
/// 
/// This routine will be automatically caleld by the compiler,
/// if a stack smashing attack or buffer overflow happens,
/// it will trigger a kernel panic.
///
#[unsafe(no_mangle)]
pub extern "C" fn __stack_chk_fail() -> ! {
    panic!("__STACK_CHECK_FAIL");
}
