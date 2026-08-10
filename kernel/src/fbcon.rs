// SPDX-License-Identifier: Apache-2.0

pub mod alloc;
pub mod framebuffer;
pub mod fbfont;

use core::fmt::{self, Write};
use spin::Mutex;

use flanterm::fb::{FlantermFb, Font, Rotation};

use shared::color::{FBCON_COLOR_BLUE, FBCON_COLOR_WHITE};

pub use framebuffer::{fill_display, query_framebuffer_information};

use crate::fbcon::framebuffer::FRAMEBUFFER_REQUEST;

pub const HIDE_CURSOR: &str = "\x1b[?25l";
pub const SHOW_CURSOR: &str = "\x1b[?25h";
pub const CLEAR_SCREEN_HOME_CURSOR: &str = "\x1b[H\x1b[2J";

pub struct TermWrapper(pub FlantermFb<'static>);
unsafe impl Send for TermWrapper {}

static FBCON_TERM: Mutex<Option<TermWrapper>> = Mutex::new(None);

pub fn initialize() {

    if let Some(resp) = FRAMEBUFFER_REQUEST.response()
        && let Some(fb) = resp.framebuffers().first()
    {
        fill_display(
            0,
            0,
            fb.width as u32,
            fb.height as u32,
            FBCON_COLOR_BLUE,
        );

        let default_bg = FBCON_COLOR_BLUE;
        let default_fg = FBCON_COLOR_WHITE;

        let custom_font = Font {
            font: &fbfont::FBCON_DISPLAY_FONT.0,
            width: 8,
            height: 16,
            spacing: 0,
        };
        
        let framebuffer: *mut() = fb.address();

        let term = FlantermFb::new(
            unsafe
            {&mut *core::ptr::slice_from_raw_parts_mut(framebuffer as *mut u32, fb.pitch as usize * fb.height as usize)},
            fb.width as usize,
            fb.height as usize,
            fb.pitch as usize,
            fb.red_mask_size,
            fb.red_mask_shift,
            fb.green_mask_size,
            fb.green_mask_shift,
            fb.blue_mask_size,
            fb.blue_mask_shift,
            Some(custom_font),
            1,
            1,
            None,
            None,
            None,
            Some(default_bg),
            None,
            Some(default_fg),
            None,
            0,
            Rotation::Rot0,
        )
        .expect("Failed to initialize Flanterm");

        *FBCON_TERM.lock() = Some(TermWrapper(term));

        write_string(HIDE_CURSOR);
    }
}

pub fn reset_display() {
    write_string(CLEAR_SCREEN_HOME_CURSOR);
}

pub fn write_char(character: char) {
    let mut buf = [0u8; 4];
    let encoded = character.encode_utf8(&mut buf);
    write_string(encoded);
}

pub fn write_string(s: &str) {
    if let Some(wrapper) = FBCON_TERM.lock().as_mut() {
        let mut buf = [0u8; 4];
        for c in s.chars() {
            if c == '\n' {
                let _ = wrapper.0.write_str("\r\n");
            } else {
                let encoded = c.encode_utf8(&mut buf);
                let _ = wrapper.0.write_str(encoded);
            }
        }
    }
}

pub struct FbConsole;

impl fmt::Write for FbConsole {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        write_string(s);
        Ok(())
    }
}