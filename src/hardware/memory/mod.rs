//! `Memory` : LC-3 has 65,536 memory locations (the maximum that is addressable by a 16-bit unsigned integer 2^16),
//! each of which stores a 16-bit value. This means it can store a total of only 128kb.

use crate::sys::getchar::get_char;
use crate::sys::select::{self, FdSet};
use crate::sys::time::TimeVal;
use crate::sys::time::TimeValLike;
use libc::STDIN_FILENO;

/// `MEMORY_SIZE` is a constant to represent size of memory in LC-3.
pub const MEMORY_SIZE: usize = u16::MAX as usize + 1;

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
/// Memory-mapped I/O Input and output are handled by load/store (LDI/STI, LDR/STR) instructions using memory addresses
/// to designate each I/O device register.
/// Addresses xFE00 through xFFFF have been allocated to represent the addresses of I/O devices.
/// The LC-3 has two memory mapped registers that need to be implemented.
/// They are the keyboard status register (KBSR) and keyboard data register (KBDR).
/// The KBSR indicates whether a key has been pressed, and the KBDR identifies which key was pressed.
pub enum MemoryMappedReg {
    /// keyboard status: The KBSR indicates whether a key has been pressed
    Kbsr = 0xFE00, /* keyboard status */
    /// keyboard data: The KBDR identifies which key was pressed
    Kbdr = 0xFE02, /* keyboard data */
}

fn check_key() -> bool {
    let mut fd = FdSet::new();
    fd.insert(STDIN_FILENO);

    let mut timeout = TimeVal::seconds(0);
    match select::select(1, &mut fd, None, None, &mut timeout) {
        Err(_) => false,
        _ => true,
    }
}

#[cfg(test)]
mod memory_test {
    use super::*;

    const EXPECTED_MEMORY_SIZE: usize = 65536;

    #[test]
    fn memory_size() {
        let memory = Memory::new();
        assert_eq!(memory.cells.len(), EXPECTED_MEMORY_SIZE);
    }

    #[test]
    fn memory_size_constant() {
        assert_eq!(MEMORY_SIZE, EXPECTED_MEMORY_SIZE);
    }
}
