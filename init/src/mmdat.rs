// SPDX-License-Identifier: GPL-2.0
//
// Purpose: This module provides common memory-management data
//
use kernel::mm::PhysAddr;

//
// TODO
//
#[allow(dead_code)]

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub enum PhysMemoryUsage {
    #[default]
    Reserved,
    Reclaimable,
    Usable,
}

//
// TODO
//
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct PhysMemory {
    pub address: PhysAddr,
    pub length: usize,
    pub usage: PhysMemoryUsage,
}

//
// TODO
//
#[allow(dead_code)]
impl PhysMemory {
    ///
    /// This routine constructs an empty PhysMemory structure,
    /// you can fill the structure later
    ///
    pub const fn empty() -> Self {
        Self {
            address: PhysAddr::null(),
            length: 0,
            usage: PhysMemoryUsage::Reserved,
        }
    }
}