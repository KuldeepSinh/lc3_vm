use super::sign_extend;
use crate::hardware::register::condition_flag;
use crate::hardware::register::Registers;
use crate::hardware::Memory;

/// An address is computed by sign-extending bits [5:0] to 16 bits
/// and adding this value to the contents of the register specified by bits [8:6].
/// The contents of memory at this address are loaded into DR.
/// The condition codes are set, based on whether the value loaded is negative, zero, or positive.
pub fn ldr(instr: u16, registers: &mut Registers, memory: &mut Memory) {
    let dr = (instr >> 9) & 0x7;
    let sr1 = (instr >> 6) & 0x7;
    let offset = sign_extend(instr & 0x3F, 6);
    let val: u32 = registers.get(sr1) as u32 + offset as u32;
    registers.update(dr, memory.read(val as u16));
    condition_flag::update_r_cond_register(dr, registers);
}
