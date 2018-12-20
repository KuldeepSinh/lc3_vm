use super::sign_extend;
use crate::hardware::register::Registers;

pub fn br(instr: u16, registers: &mut Registers) {
    let pc_offset = sign_extend((instr) & 0x1ff, 9);
    let cond_flag = (instr >> 9) & 0x7;
    if cond_flag & registers.r_cond != 0 {
        registers.r_pc = registers.r_pc + pc_offset;
    }
}
