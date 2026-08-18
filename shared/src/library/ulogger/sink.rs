// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2026 Trollycat
//
// Purpose: A 'sink' is the output target,
//          for example,
//          fbcon and serial will be registered as a 'sink',
//          this design allows us to avoid pulling in fbcon and serial inside the shared/ crate,
//          which avoids errors and stuff... I guess?
//
use spin::Mutex;

use crate::core::status::{KResult, Status};

//
// Hard-coded 4 sink limit
// I mean I don't think it's possible to have +4 sinks???
// Atleast for my O/S..? Maybe idk
//
const MAX_SINKS: usize = 4;

//
// Might be one of the ugliest lines ever..
//
static SINKS: Mutex<[Option<&'static dyn LogSink>; MAX_SINKS]> = 
    Mutex::new([None; MAX_SINKS]);

//
// Anything that wants to apart of the logger,
// must register using this trait,
// otherwise it's a bum..
//
pub trait LogSink: Send + Sync
{
    fn write(&self, data: &[u8]);
}

pub fn register_sink(sink: &'static dyn LogSink) -> KResult<()> 
{
    let mut sinks = SINKS.lock();

    for slot in sinks.iter_mut() {
        if slot.is_none() {
            *slot = Some(sink);
            return Ok(());
        }
    }

    Err(Status::INSUFFICIENT_RESOURCES)
}

pub fn dispatch(data: &[u8]) {

    let sinks = SINKS.lock();
    
    for slot in sinks.iter() {
        if let Some(sink) = slot {
            sink.write(data);
        }
    }
}