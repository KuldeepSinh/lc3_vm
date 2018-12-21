use super::sign_extend;
use crate::hardware::register::Registers;

pub fn jsr(instr: u16, registers: &mut Registers) {
    let base_reg = (instr >> 6) & 0x7;
    let long_pc_offset = sign_extend(instr & 0x7ff, 11);
    let long_flag = (instr >> 11) & 1;
    registers.r_07 = registers.r_pc;

    if long_flag != 0 {
        registers.r_pc = registers.r_pc + long_pc_offset; /* JSR */
    } else {
        registers.r_pc = registers.get(base_reg); /* JSRR */
    }
}
