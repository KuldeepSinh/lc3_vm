use super::sign_extend;
use crate::hardware::register::condition_flag;
use crate::hardware::register::Registers;
use crate::hardware::Memory;

pub fn ldr(instr: u16, registers: &mut Registers, memory: &mut Memory) {
    let dr = (instr >> 9) & 0x7;
    let sr1 = (instr >> 6) & 0x7;
    let offset = sign_extend(instr & 0x3F, 6);
    let val: u32 = registers.get(sr1) as u32 + offset as u32;
    registers.update(dr, memory.read(val as u16));
    condition_flag::update_flags(dr, registers);
}
