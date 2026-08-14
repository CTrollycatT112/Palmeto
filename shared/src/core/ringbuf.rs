// SPDX-License-Identifier: GPL-3.0-or-later
//
// Copyright (c) 2026 Trollycat
//
// Purpose: This module provides static ring buffers,
//          for the ulogger..
//

// I think it's normal to use them as static?
// I could change this with heap allocation,
// but from what i saw linux uses static ones..?

pub struct RingBuffer<const CAPACITY: usize>
{
    data: [u8; CAPACITY],
    head: usize,
    tail: usize,
    full: bool,
}

impl<const CAPACITY: usize> RingBuffer<CAPACITY>
{
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

    pub fn write(&mut self, bytes: &[u8])
    {
        for &b in bytes
        {
            self.data[self.head] = b;
            self.head = (self.head + 1 ) % CAPACITY;

            if self.full
            {
                self.tail = (self.tail + 1 ) % CAPACITY;
            } 
            else if self.head == self.tail 
            {
                self.full = true;
            }
        }
    }

    pub fn push(&mut self, byte: u8) -> bool
    {

        if self.full
        {
            //
            // The buffer is full,
            // We cannot add anything more..
            //
            return false;
        }

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

        true
    }

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

    pub fn empty(&self) -> bool
    {
        !self.full && self.head == self.tail
    }

    pub fn full(&self) -> bool
    {
        self.full
    }
    
    pub fn capacity(&self) -> usize
    {
        CAPACITY
    }
}

impl<const CAPACITY: usize> core::fmt::Write for RingBuffer<CAPACITY> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write(s.as_bytes());
        Ok(())
    }
}