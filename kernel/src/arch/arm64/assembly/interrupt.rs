// SPDX-License-Identifier: Apache-2.0
//
// Copyright (c) 2026 Trollycat
//
// Purpose: This file has assembly instruction wrappers,
//          for interrupt handling...
//          instructions.rs already has assembly wrappers,
//          but this is another layer for clean 'interrupts'
//

#![allow(unused)]

use super::instructions;

pub unsafe fn enable_interrupts()
{
    unsafe
    {
        instructions::sti();
    }
}

pub unsafe fn disable_interrupts()
{
    unsafe
    {
        instructions::cli();
    }
}

pub unsafe fn get_interrupt_state() -> bool
{
    unsafe
    {
        return instructions::interrupt_state();
    }
}

pub unsafe fn toggle_interrupts(state: bool) -> bool
{
    unsafe
    {
        return instructions::toggle_interrupts(state);
    }
}