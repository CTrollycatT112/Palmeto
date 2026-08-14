// SPDX-License-Identifier: GPL-3.0-or-later
//
// Copyright (c) 2026 Trollycat
//
// Purpose: ARM64 'Generic Interrupt Controller',
//          there is versions to this,
//          '1, '2', '3',
//          the libre potato and qemu use gicv2
//          which makes everything so much easier...
//
//          I think gicv3 is mainly for servers?
//          So we don't need different drivers,
//          If qemu used gicv3 i would have to write another..
//
use shared::core::types::status::{KResult};
use shared::{debug};

pub const COMPATIBLE_STRINGS: &[&str] = &[
    "arm,gic-v2",
    "arm,gic-400",
    "arm,cortex-a15-gic",
    "arm,gic",
];

pub fn try_init_node(node: &fdt::node::FdtNode) -> KResult<()> 
{
    //
    // TODO
    //
    let _ = node;
    debug!("FOUND GENERIC INTERRUPT CONTROLLER... V2");
    Ok(())
}