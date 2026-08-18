// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2026 Trollycat
//
// Purpose:  A user can pass arguments to limine,
//           This structure stores the possble arguments,
//           Then the parser will handle It,
//           Any driver or crate might need the config,
//           So this is within the 'shared' crate

#[derive(Debug, Clone, Copy)]
pub struct BootConfiguration
{
    pub max_memory:   Option<usize>,
    pub disable_smp: bool,
    pub serial_baud: u32, 
}

impl Default for BootConfiguration
{
    ///
    /// This routine is related to the 'Default' trait,
    /// If the command line has no arguments,
    /// it will use default settings.
    /// 
    /// These are:
    ///     no limit to memory,
    ///     multi-core allowed,
    ///     serial-buad default (115200)
    ///
    fn default() -> Self
    {
        Self
        {
            max_memory: None,
            disable_smp: false,
            serial_baud: 115200,
        }
    }
}