use super::sign_extend;
use super::update_flags;
use crate::hardware::register::Registers;
use crate::hardware::Memory;

pub fn ldi(instr: u16, registers: &mut Registers, memory: &mut Memory) {
    let dr = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1ff, 9);
    let first_read = memory.read(registers.r_pc + pc_offset);
    registers.update(dr, memory.read(first_read));
    update_flags(dr, registers);
}
