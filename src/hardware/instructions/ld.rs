// use super::sign_extend;
// use super::update_flags;
// use crate::hardware::register::Registers;
//
// pub fn ld(instr: u16, registers: &mut Registers) {
//     let dr = (instr >> 9) & 0x7;
//     let pc_offset = sign_extend(instr & 0x1ff, 9);
//     //reg[r0] = mem_read(reg[R_PC] + pc_offset);
//     registers.update(dr, registers.get(sr1) & registers.get(sr2));
//     update_flags(dr, registers);
// }
