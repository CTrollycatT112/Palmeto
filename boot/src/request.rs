// SPDX-License-Identifier: Apache-2.0
//
// Copyright (c) 2026 Palmeto OS Team
//
// Author:      Trollycat
//
// Module:      Limine request handler
//
// Description: This file handles limine requests
//
use limine::request::{HhdmRequest, ExecutableCmdlineRequest};

#[used]
pub static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();

#[used]
pub static CMDLINE_REQUEST: ExecutableCmdlineRequest = ExecutableCmdlineRequest::new();
