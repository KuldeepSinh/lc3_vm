use super::sign_extend;
use crate::hardware::register::condition_flag;
use crate::hardware::register::Registers;

/// An address is computed by sign-extending bits [8:0] to 16 bits and adding this value to the incremented PC.
/// This address is loaded into DR.‡ The condition codes are set, based on whether the value loaded is negative, zero, or positive.
pub fn lea(instr: u16, registers: &mut Registers) {
    let dr = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1ff, 9);
    let val: u32 = registers.r_pc as u32 + pc_offset as u32;
    registers.update(dr, val as u16);
    condition_flag::update_r_cond_register(dr, registers);
}
