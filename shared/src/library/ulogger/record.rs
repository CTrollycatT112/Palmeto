// SPDX-License-Identifier: GPL-2.0-or-later
//
// Purpose: This module provides the level enum,
//          as well as the LogRecord<>
//

use crate::core::color;

//
// WOW.. I love #[derive] :::: hehe
//
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level
{
    Trace = 0,
    Debug = 1,
    Info  = 2,
    Warn  = 3,
    Error = 4,
    Fatal = 5,
}

impl Level {
    ///
    /// This routine converts the logging levels to a string,
    /// so that you can print it out.
    /// 
    /// (Example: ) Trace => "TRACE"
    ///
    pub const fn as_str(&self) -> &'static str
    {
        match self {
            Level::Trace => "TRACE",
            Level::Debug => "DEBUG",
            Level::Info  => "INFO",
            Level::Warn  => "WARN",
            Level::Error => "ERROR",
            Level::Fatal => "FATAL",
        }
    }

    ///
    /// This routine converts the logging levels to a color,
    /// used for the logger.
    ///
    pub const fn color_code(&self) -> &'static str
    {
        match self {
            Level::Trace => color::ANSI_TRACE,
            Level::Debug => color::ANSI_DEBUG,
            Level::Info  => color::ANSI_INFO,
            Level::Warn  => color::ANSI_WARN,
            Level::Error => color::ANSI_ERROR,
            Level::Fatal => color::ANSI_FATAL,
        }
    }
}

pub struct LogRecord<'a>
{
    pub level: Level,
    pub args:  core::fmt::Arguments<'a>,
    pub file:  &'a str,
    pub line:  u32,
}