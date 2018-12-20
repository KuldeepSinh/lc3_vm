use super::update_flags;
use crate::hardware::register::Registers;

pub fn not(instr: u16, registers: &mut Registers) {
    let dr = (instr >> 9) & 0x7;
    let sr1 = (instr >> 6) & 0x7;
    registers.update(dr, !registers.get(sr1));

    update_flags(dr, registers);
}
