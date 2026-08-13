// SPDX-License-Identifier: GPL-3.0-or-later
//
// Copyright (c) 2026 Trollycat
//
// Purpose: This file handles limine requests
//

use limine::request::{HhdmRequest, 
                      DtbRequest,
                      ExecutableCmdlineRequest,
                      FramebufferRequest,
                      DateAtBootRequest
};

#[used]
#[unsafe(link_section = ".requests")]
pub static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();

#[used]
#[unsafe(link_section = ".requests")]
pub static DTB_REQUEST: DtbRequest = DtbRequest::new();

#[used]
#[unsafe(link_section = ".requests")]
pub static CMDLINE_REQUEST: ExecutableCmdlineRequest = ExecutableCmdlineRequest::new();

#[used]
#[unsafe(link_section = ".requests")]
pub static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[used]
#[unsafe(link_section = ".requests")]
pub static DATE_AT_BOOT_REQUEST: DateAtBootRequest = DateAtBootRequest::new();