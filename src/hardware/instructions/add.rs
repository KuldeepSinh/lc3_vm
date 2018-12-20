//! An `instr` is a command which tells the CPU to do some fundamental task,
//! instrs have both an `opcode` which indicates the kind of task to perform and
//! a set of `parameters` which provide inputs to the task being performed.

use super::sign_extend;
use super::update_flags;
use crate::hardware::register::Registers;

pub fn add(instr: u16, registers: &mut Registers) {
    /* destination register (DR) */
    let dr = (instr >> 9) & 0x7;
    /* first operand (SR1) */
    let sr1 = (instr >> 6) & 0x7;
    /* whether we are in immediate mode */
    let imm_flag = (instr >> 5) & 0x1;
    if imm_flag == 1 {
        let imm5 = sign_extend(instr & 0x1F, 5);
        registers.update(dr, registers.get(sr1) + imm5);
    } else {
        /* first operand (SR2) */
        let sr2 = instr & 0x7;
        registers.update(dr, registers.get(sr1) + registers.get(sr2));
    }
    update_flags(dr, registers);
}
