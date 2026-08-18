// SPDX-License-Identifier: GPL-3.0-or-later
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
use kernel::arch::arm64::exception::{timer, intrcntrl};

use shared::{core::{requests::{DTB_REQUEST, HHDM_REQUEST}, 
             status::{KResult, Status}}, fatal};

fn internal_init_dtb(dtb: *const u8, 
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
        if let Some(compatible) = node.compatible() {
            if compatible.all().any(|c| intrcntrl::COMPATIBLE_STRINGS.contains(&c)) 
            {
                intrcntrl::try_init_node(&node)?;
                break;
            }
        }
    }

    for node in fdt.all_nodes() {
        let Some(compatible) = node.compatible() else { continue };

        let matches = |strings: &[&str]| compatible.all().any(|c| strings.contains(&c));

        if matches(serial::COMPATIBLE_STRINGS) {
            serial::try_init_node(&node, hhdm_offset)?;
        } else if matches(timer::COMPATIBLE_STRINGS) {
            timer::try_init_node(&node)?;
        }
    }

    Ok(())
}

pub fn init() -> KResult<()>
{
    let hhdm_offset = HHDM_REQUEST
        .response()
        .map(|r| r.offset)
        .unwrap_or(0);

    let dtb_resp = match DTB_REQUEST.response() 
    {
        Some(resp) => resp,
        None => {
            fatal!("COULD NOT GET DTB_RESPONSE");
        }
    };

    let dtb_ptr = dtb_resp.dtb_ptr as *const u8;
    internal_init_dtb(dtb_ptr, hhdm_offset)?;

    Ok(())
}