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
