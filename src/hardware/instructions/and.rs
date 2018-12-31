use super::sign_extend;
use crate::hardware::register::condition_flag;
use crate::hardware::register::Registers;

/// If bit [5] is 0, the second source operand is obtained from SR2.
/// If bit [5] is 1, the second source operand is obtained by sign-extending the imm5 field to 16 bits.
/// In either case, the second source operand and the contents of SR1 are bit- wise ANDed,
/// and the result stored in DR. The condition codes are set, based on whether the binary value produced,
/// taken as a 2’s complement integer, is negative, zero, or positive.
pub fn and(instr: u16, registers: &mut Registers) {
    let dr = (instr >> 9) & 0x7;
    let sr1 = (instr >> 6) & 0x7;
    let imm_flag = (instr >> 5) & 0x1;

    if imm_flag == 1 {
        let imm5 = sign_extend(instr & 0x1F, 5);
        registers.update(dr, registers.get(sr1) & imm5);
    } else {
        let sr2 = instr & 0x7;
        registers.update(dr, registers.get(sr1) & registers.get(sr2));
    }
    condition_flag::update_r_cond_register(dr, registers);
}
