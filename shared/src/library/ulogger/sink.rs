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

///
/// This routine registers a new logging sink,
/// anything that wants standard log macros to print to it,
/// needs to register as a sink.
///
/// # Arguments
///
/// * sink - A static reference to an object that has 'LogSink' trait
///
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

///
/// This routine will dispatch a stream of log data to all registed sinks.
///
/// # Arguments
///
/// * data - A byte slice that has the log message to send to each sink
///
pub fn dispatch(data: &[u8]) {

    let sinks = SINKS.lock();
    
    for sink in sinks.iter().filter_map(Option::as_ref) {
        sink.write(data);
    }
}