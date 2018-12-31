use super::sign_extend;
use crate::hardware::register::condition_flag;
use crate::hardware::register::Registers;
use crate::hardware::Memory;

/// An address is computed by sign-extending bits [8:0] to 16 bits and adding this value to the incremented PC.
/// What is stored in memory at this address is the address of the data to be loaded into DR.
/// The condition codes are set, based on whether the value loaded is negative, zero, or positive.
pub fn ldi(instr: u16, registers: &mut Registers, memory: &mut Memory) {
    let dr = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1ff, 9);
    let first_read = memory.read(registers.r_pc + pc_offset);
    registers.update(dr, memory.read(first_read));
    condition_flag::update_r_cond_register(dr, registers);
}
