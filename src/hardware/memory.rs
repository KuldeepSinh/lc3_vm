//! `Memory` : LC-3 has 65,536 memory locations (the maximum that is addressable by a 16-bit unsigned integer 2^16),
//! each of which stores a 16-bit value. This means it can store a total of only 128kb.

/// `MEMORY_SIZE` is a constant to represent size of memory in LC-3.
pub const MEMORY_SIZE: usize = std::u16::MAX as usize;

/// `Memory` : LC-3 has 65,536 memory locations (the maximum that is addressable by a 16-bit unsigned integer 2^16),
/// each of which stores a 16-bit value. This means it can store a total of only 128kb.

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
        self.cells[address] = value;
    }

    pub fn read(&mut self, address: u16) -> u16 {
        if address == MemoryMappedReg::Kbsr as u16 {
            if check_key() {
                self.cells[MemoryMappedReg::Kbsr as usize] = 1 << 15;
                self.cells[MemoryMappedReg::Kbdr as usize] = get_char();
            } else {
                self.cells[MemoryMappedReg::Kbsr as usize] = 0;
            }
        }
        self.cells[address as usize]
    }
}
pub enum MemoryMappedReg {
    Kbsr = 0xFE00, /* keyboard status */
    Kbdr = 0xFE02, /* keyboard data */
}

fn check_key() -> bool {
    unimplemented!("fn check_key() is not implemented yet.");
}

fn get_char() -> u16 {
    unimplemented!("fn get_char() is not implemented yet.");
}
