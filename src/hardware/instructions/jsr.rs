use super::sign_extend;
use crate::hardware::register::Registers;

pub fn jsr(instr: u16, registers: &mut Registers) {
    let base_reg = (instr >> 6) & 0x7;
    let long_pc_offset = sign_extend(instr & 0x7ff, 11);
    let long_flag = (instr >> 11) & 1;
    registers.r_07 = registers.r_pc;

    if long_flag != 0 {
        let val: u32 = registers.r_pc as u32 + long_pc_offset as u32;
        if val > 65535 {
            registers.r_pc = (val - 65536) as u16;
        } else {
            registers.r_pc = val as u16;
        } /* JSR */
    } else {
        registers.r_pc = registers.get(base_reg); /* JSRR */
    }
}
