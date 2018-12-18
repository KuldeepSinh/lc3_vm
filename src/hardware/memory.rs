//set memory size
pub const MEMORY_SIZE: usize = std::u16::MAX as usize;
pub struct Memory {
    pub cells: [u16; MEMORY_SIZE],
}
