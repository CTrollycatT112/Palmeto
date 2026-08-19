// SPDX-License-Identifier: GPL-2.0-or-later
//
// Purpose: This module provides a static,
//          fixed-capacity ring buffer.
//
//          A ring buffer uses a single,
//          fixed-size array connected end-to-end,
//          allowing data streaming (without memory management)
//

use crate::core::status::{KResult, Status};
pub struct RingBuffer<const CAPACITY: usize>
{
    data: [u8; CAPACITY],
    head: usize,
    tail: usize,
    full: bool,
}

impl<const CAPACITY: usize> Default for RingBuffer<CAPACITY>
{
    ///
    /// This routine returns a default, empty RingBuffer
    /// It will invoke 'Self::new()'
    ///
    fn default() -> Self
    {
        Self::new()
    }
}

impl<const CAPACITY: usize> RingBuffer<CAPACITY>
{
    ///
    /// This routine consturcts a new RingBuffer structure,
    /// it will  be completely empty,
    /// besides setting the maximum capacity to the given one.
    ///
    pub const fn new() -> Self
    {
        const
        {
            assert!(CAPACITY > 0,
                    "RINGBUFFER: CAPACITY NOT GREATER THAN ZERO");
        }
        //
        // Ring buffers start cleared
        // head, tail = 0
        // full       = false
        //
        Self
        {
            data: [0; CAPACITY],
            head: 0,
            tail: 0,
            full: false,
        }
    }

    ///
    /// This routine writes a slice of bytes to the buffer,
    /// this will overwrite the oldest data in the buffer,
    /// if the buffer is still full of course.
    ///
    /// # Arguments
    ///
    /// * bytes - A byte slice of data to write
    ///
    pub fn write(&mut self, bytes: &[u8]) -> KResult<()>
    {
        let mut overflowed = false;

        for &b in bytes
        {
            self.data[self.head] = b;
            self.head = (self.head + 1 ) % CAPACITY;

            if self.full
            {
                self.tail = (self.tail + 1 ) % CAPACITY;
                overflowed = true;
            } 
            else if self.head == self.tail 
            {
                self.full = true;
            }
        }

        if overflowed {
            Err(Status::BUFFER_OVERFLOW)
        } else {
            Ok(())
        }
    }

    ///
    /// This routines pushes a single byte into the buffer,
    /// if the buffer is already full it will return from the function
    ///
    /// # Arguments
    ///
    /// * byte - The single byte to push into the buffer
    ///
    pub fn push(&mut self, byte: u8) -> KResult<()>
    {

        if self.full
        {
            //
            // The buffer is full,
            // We cannot add anything more..
            //
            Err(Status::BUFFER_OVERFLOW)
        }
        else {
            
            self.data[self.head] = byte;
            self.head            = (self.head + 1 ) % CAPACITY;

            if self.head == self.tail
            {
                //
                // HEAD == TAIL
                // This means the buffer is full
                //
                self.full = true;
            }

            Ok(())
        }
    }

    ///
    /// This routine reads the available bytes from the buffer,
    /// and place them into the destination,
    /// it will return the number of bytes that were read.
    ///
    /// # Arguments
    ///
    /// * dest - Byte slice where read data will be stored
    ///
    pub fn read(&mut self, dest: &mut [u8]) -> usize
    {
        let mut count = 0;

        while count < dest.len() && !self.empty()
        {
            dest[count] = self.data[self.tail];

            self.tail = (self.tail + 1 ) % CAPACITY;
            self.full = false;
            count += 1;
        }

        count
    }

    ///
    /// This routine will completely clear the buffer.
    /// Use if you no longer need the stored data
    /// but you want to reuse the buffer.
    ///
    pub fn clear(&mut self)
    {
        //
        // Completely clear the ring buffer
        // 'head, tail' = 0
        // 'full'       = false
        //
        self.head = 0;
        self.tail = 0;
        self.full = false;
    }

    ///
    /// This routine returns the number of bytes in the buffer.
    ///
    pub fn len(&self) -> usize
    {
        if self.full
        {
            CAPACITY
        } else if self.head >= self.tail
        {
            self.head - self.tail
        } else {
            CAPACITY - self.tail + self.head
        }
    }

    ///
    /// This routine will return if the buffer is empty or not.
    ///
    pub fn empty(&self) -> bool
    {
        !self.full && self.head == self.tail
    }

    pub fn is_empty(&self) -> bool
    {
        self.empty()
    }

    ///
    /// This routine will return if the buffer is full or not.
    ///
    pub fn full(&self) -> bool
    {
        self.full
    }
    
    ///
    /// This routine will return the maximum capacity of the buffer.
    ///
    pub fn capacity(&self) -> usize
    {
        CAPACITY
    }
}

impl<const CAPACITY: usize> core::fmt::Write for RingBuffer<CAPACITY> {
    ///
    /// This routine will write a string slice into the buffer
    /// converting it into bytes so that it works.
    ///
    /// # Arguments
    ///
    /// * s - the string to write
    ///
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        match self.write(s.as_bytes())
        {
            Ok(_) | Err(Status::BUFFER_OVERFLOW) => Ok(()),
            Err(_) => Err(core::fmt::Error),
        }
    }
}