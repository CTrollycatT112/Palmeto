// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2026 Trollycat
//
// Purpose: This file handles __stack related routines,
//          This would've been unneeded,
//          but flanterm for whatever reason relies on it,
//          so this is a dummy version..
//

#[unsafe(no_mangle)]
pub static __stack_chk_guard: usize = 0x5905_fde0_90cc_0aaf;

#[unsafe(no_mangle)]
pub extern "C" fn __stack_chk_fail() -> ! {
    panic!("__STACK_CHECK_FAIL");
}
