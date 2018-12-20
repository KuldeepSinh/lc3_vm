use super::sign_extend;
use crate::hardware::register::Registers;
use crate::hardware::Memory;

pub fn str(instr: u16, registers: &mut Registers, memory: &mut Memory) {
    let dr = (instr >> 9) & 0x7;
    let sr1 = (instr >> 6) & 0x7;
    let pc_offset = sign_extend(instr & 0x3F, 6);
    memory.write(
        registers.get(sr1) as usize + pc_offset as usize,
        registers.get(dr),
    );
}
