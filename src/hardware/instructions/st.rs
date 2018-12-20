use super::sign_extend;
use crate::hardware::register::Registers;
use crate::hardware::Memory;

pub fn st(instr: u16, registers: &mut Registers, memory: &mut Memory) {
    let dr = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1ff, 9);
    memory.write(
        registers.r_pc as usize + pc_offset as usize,
        registers.get(dr),
    );
}
