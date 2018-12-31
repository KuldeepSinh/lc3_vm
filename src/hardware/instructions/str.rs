use super::sign_extend;
use crate::hardware::register::Registers;
use crate::hardware::Memory;

/// The contents of the register specified by SR are stored in the memory location
/// whose address is computed by sign-extending bits [5:0] to 16 bits
/// and adding this value to the contents of the register specified by bits [8:6].
pub fn str(instr: u16, registers: &mut Registers, memory: &mut Memory) {
    let dr = (instr >> 9) & 0x7;
    let sr1 = (instr >> 6) & 0x7;
    let offset = sign_extend(instr & 0x3F, 6);
    let val: u32 = registers.get(sr1) as u32 + offset as u32;
    let val: u16 = val as u16;
    memory.write(val as usize, registers.get(dr));
}
