use super::sign_extend;
use super::update_flags;
use crate::hardware::register::Registers;

pub fn lea(instr: u16, registers: &mut Registers) {
    let dr = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1ff, 9);
    let val: u32 = registers.r_pc as u32 + pc_offset as u32;
    registers.update(dr, val as u16);
    update_flags(dr, registers);
}
