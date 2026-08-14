// SPDX-License-Identifier: GPL-3.0-or-later
//
// Copyright (c) 2026 Trollycat
//
// Purpose: 'API' for interrupt handling,
//          We use a Dispatch table approach,
//          An interrupt will register itself (like the timer)
//
use spin::Mutex;

const MAX_IRQS: usize = 1024;

type IrqHandler = fn();

static IRQ_TABLE: Mutex<[Option<IrqHandler>; MAX_IRQS]> = Mutex::new(
    [None; MAX_IRQS]
);

pub fn register_handler(irq: usize, handler: IrqHandler)
{
    if irq < MAX_IRQS
    {
        let mut table = IRQ_TABLE.lock();
        table[irq]    = Some(handler);
    }
}

pub fn dispatch(irq: usize)
{
    let handler = {
        let table = IRQ_TABLE.lock();
        table[irq]
    };

    if let Some(func) = handler
    {
        func();
    } else {
        shared::debug!("SPURIOUS OR UNHANDLED IRQ: {}", irq);
    }
}