// SPDX-License-Identifier: GPL-3.0-or-later
//
// Copyright (c) 2026 Trollycat
//
// Purpose: Color constants for serial and fbcon
//

//
// FBCON
//
pub const FBCON_COLOR_BLACK: u32 = 0x00000000;
pub const FBCON_COLOR_BLUE: u32 = 0x000000AA;
pub const FBCON_COLOR_GREEN: u32 = 0x0000AA00;
pub const FBCON_COLOR_CYAN: u32 = 0x0000AAAA;
pub const FBCON_COLOR_RED: u32 = 0x00AA0000;
pub const FBCON_COLOR_MAGENTA: u32 = 0x00AA00AA;
pub const FBCON_COLOR_BROWN: u32 = 0x00AA5500;
pub const FBCON_COLOR_LIGHT_GRAY: u32 = 0x00AAAAAA;
pub const FBCON_COLOR_DARK_GRAY: u32 = 0x00555555;
pub const FBCON_COLOR_LIGHT_BLUE: u32 = 0x005555FF;
pub const FBCON_COLOR_LIGHT_GREEN: u32 = 0x0055FF55;
pub const FBCON_COLOR_LIGHT_CYAN: u32 = 0x0055FFFF;
pub const FBCON_COLOR_LIGHT_RED: u32 = 0x00FF5555;
pub const FBCON_COLOR_LIGHT_MAGENTA: u32 = 0x00FF55FF;
pub const FBCON_COLOR_YELLOW: u32 = 0x00FFFF55;
pub const FBCON_COLOR_WHITE: u32 =  0x00FFFFFF;

//
// ANSI (SERIAL)
//
pub const ANSI_RESET: &str = "\x1b[0m";
pub const ANSI_TRACE: &str = "\x1b[37m";
pub const ANSI_DEBUG: &str = "\x1b[36m";
pub const ANSI_INFO: &str = "\x1b[32m";
pub const ANSI_WARN: &str = "\x1b[33m";
pub const ANSI_ERROR: &str = "\x1b[31m";
pub const ANSI_FATAL: &str = "\x1b[35m";