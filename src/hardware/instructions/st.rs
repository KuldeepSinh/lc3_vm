use super::sign_extend;
use crate::hardware::register::Registers;
use crate::hardware::Memory;

/// The contents of the register specified by SR are stored in the memory location
/// whose address is computed by sign-extending bits [8:0] to 16 bits and adding this value to the incremented PC.
pub fn st(instr: u16, registers: &mut Registers, memory: &mut Memory) {
    let dr = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1ff, 9);
    let val: u32 = registers.r_pc as u32 + pc_offset as u32;
    let val: u16 = val as u16;
    memory.write(val as usize, registers.get(dr));
}
