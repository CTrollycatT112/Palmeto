// SPDX-License-Identifier: Apache-2.0
//
// Copyright (c) 2026 Trollycat
//
// Purpose: This module parses the Device tree blob,
//          each module will handle it's own detection,
//          but this will be an 'interface',
//          the main function only needs to call 'init_dtb',
//          and the 'init_dtb' will call individual functions
//

use drivers::tty::serial;
use shared::core::types::status::{KResult, Status};

pub fn init_dtb(dtb: *const u8, 
                hhdm_offset: u64
) -> KResult<()>  
{
    if dtb.is_null()
    {
        return Err(Status::INVALID_PARAMETER);
    }

    let fdt = match unsafe { fdt::Fdt::from_ptr(dtb) } {
        Ok(fdt) => fdt,
        Err(_) => return Err(Status::FILE_CORRUPT_ERROR),
    };

    for node in fdt.all_nodes()
    {
        let Some(compatible) = node.compatible() else { continue };
        let Some(reg) = node.reg().and_then(|mut r| r.next()) else {continue};

        let phys = reg.starting_address as u64;
        if phys == 0
        {
            continue;
        }

        let vaddr = phys + hhdm_offset;

        if let Ok(()) = serial::init_from_dtb(&compatible, vaddr) {
            return Ok(());
        }
    }

    Err(Status::NO_SUCH_DEVICE)
}