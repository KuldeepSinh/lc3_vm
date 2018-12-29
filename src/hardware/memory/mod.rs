//! `Memory` : LC-3 has 65,536 memory locations (the maximum that is addressable by a 16-bit unsigned integer 2^16),
//! each of which stores a 16-bit value. This means it can store a total of only 128kb.

use crate::sys::getchar::get_char;
use crate::sys::select::{self, FdSet};
use crate::sys::time::TimeVal;
use crate::sys::time::TimeValLike;
use libc::STDIN_FILENO;

/// `MEMORY_SIZE` is a constant to represent size of memory in LC-3.
pub const MEMORY_SIZE: usize = std::u16::MAX as usize;

/// `Memory` : LC-3 has 65,536 memory locations (the maximum that is addressable by a 16-bit unsigned integer 2^16),
/// each of which stores a 16-bit value. This means it can store a total of only 128kb.

#[derive(Copy)]
pub struct Memory {
    /// Memory is an array of `u16` cells, with length = 65,536.
    pub cells: [u16; MEMORY_SIZE],
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            cells: [0; MEMORY_SIZE],
        }
    }
    pub fn write(&mut self, address: usize, value: u16) {
        self.cells[(address as u16) as usize] = value;
    }

    pub fn read(&mut self, address: u16) -> u16 {
        if address == MemoryMappedReg::Kbsr as u16 {
            if check_key() {
                self.cells[MemoryMappedReg::Kbsr as usize] = 1 << 15;
                self.cells[MemoryMappedReg::Kbdr as usize] = get_char() as u16;
            } else {
                self.cells[MemoryMappedReg::Kbsr as usize] = 0;
            }
        }
        self.cells[address as usize]
    }
}

impl Clone for Memory {
    fn clone(&self) -> Memory {
        *self
    }
}
pub enum MemoryMappedReg {
    Kbsr = 0xFE00, /* keyboard status */
    Kbdr = 0xFE02, /* keyboard data */
}

fn check_key() -> bool {
    let mut fd = FdSet::new();
    fd.insert(STDIN_FILENO);

    let mut timeout = TimeVal::seconds(10);
    match select::select(1, &mut fd, None, None, &mut timeout) {
        Err(_) => false,
        _ => true,
    }
}
