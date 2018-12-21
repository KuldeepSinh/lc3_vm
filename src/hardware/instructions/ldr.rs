use super::sign_extend;
use super::update_flags;
use crate::hardware::register::Registers;
use crate::hardware::Memory;

pub fn ldr(instr: u16, registers: &mut Registers, memory: &mut Memory) {
    let dr = (instr >> 9) & 0x7;
    let sr1 = (instr >> 6) & 0x7;
    let offset = sign_extend(instr & 0x3F, 6);
    registers.update(dr, memory.read(registers.get(sr1) + offset));
    update_flags(dr, registers);
}
