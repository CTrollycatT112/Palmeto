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
use core::ptr::{read_volatile, write_volatile, addr_of_mut, addr_of};

use shared::core::status::{Status, KResult};
use shared::{trace};

static mut GICD_BASE: usize = 0;
static mut GICC_BASE: usize = 0;

pub const COMPATIBLE_STRINGS: &[&str] = &[
    "arm,gic-v2",
    "arm,gic-400",
    "arm,cortex-a15-gic",
    "arm,gic",
];

pub fn try_init_node(node: &fdt::node::FdtNode) -> KResult<()> 
{
    let mut reg_iter = node.reg().ok_or(Status::FILE_CORRUPT_ERROR)?;
    
    //
    // DISTRIBUTER
    //
    let dist_reg = reg_iter.next().ok_or(Status::FILE_CORRUPT_ERROR)?;
    let dist_base = dist_reg.starting_address as usize;
    
    //
    // CPU
    //
    let cpu_reg = reg_iter.next().ok_or(Status::FILE_CORRUPT_ERROR)?;
    let cpu_base = cpu_reg.starting_address as usize;

    init(dist_base, cpu_base)
}

pub fn init(dist_base: usize, cpu_base: usize) -> KResult<()> 
{

    if dist_base == 0 || cpu_base == 0 {
        return Err(Status::INVALID_PARAMETER);
    }

    unsafe {

        *addr_of_mut!(GICD_BASE) = dist_base;
        *addr_of_mut!(GICC_BASE) = cpu_base;

        trace!("INIT GICV2...");
        trace!("DISTRIBUTER BASE: 0x{:x}", *addr_of!(GICD_BASE));
        trace!("CPU BASE: 0x{:x}", *addr_of!(GICC_BASE));

        let dist_ctrl = dist_base as *mut u32;
        write_volatile(dist_ctrl, 1);

        let cpu_ctrl = cpu_base as *mut u32;
        write_volatile(cpu_ctrl, 1);

        let cpu_pmr = (cpu_base + 0x4) as *mut u32;
        write_volatile(cpu_pmr, 0xFF);
    }

    Ok(())
}

pub fn enable_irq(irq_id: usize)
{
    unsafe 
    {
        let dist_base = *addr_of!(GICD_BASE);

        let reg_offset = 0x100  + (irq_id / 32) * 4;
        let bit_index  = irq_id % 32;

        let addr = (dist_base + reg_offset) as *mut u32;
        let val  = read_volatile(addr);

        write_volatile(addr, val | (1 << bit_index));
    }
}

pub fn read_and_ack_interrupt() -> u32 {
    unsafe {
        let base = *addr_of!(GICC_BASE);
        let iar_addr = (base + 0xC) as *const u32;
        read_volatile(iar_addr)
    }
}

pub fn parse_interrupt(node: &fdt::node::FdtNode, index: usize) -> KResult<usize> 
{
    let prop  = node.property("interrupts").ok_or(Status::FILE_CORRUPT_ERROR)?;
    let chunk = prop.value.chunks_exact(12).nth(index).ok_or(Status::FILE_CORRUPT_ERROR)?;
    
    let irq_type = u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
    let irq_id   = u32::from_be_bytes([chunk[4], chunk[5], chunk[6], chunk[7]]) as usize;
    
    match irq_type {
        0 => Ok(irq_id + 32), // SPI...
        1 => Ok(irq_id + 16), // PPI...
        _ => Err(Status::FILE_CORRUPT_ERROR), // DOESN'T EXIST :P
    }
}

pub fn end_of_interrupt(interrupt_id: u32) {
    unsafe {
        let base = *addr_of!(GICC_BASE);
        let eoir_addr = (base + 0x10) as *mut u32;
        write_volatile(eoir_addr, interrupt_id);
    }
}