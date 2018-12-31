use super::sign_extend;
use crate::hardware::register::Registers;

/// The condition codes specified by the state of bits [11:9] are tested.
// If bit [11] is set, N is tested; if bit [11] is clear, N is not tested.
/// If bit [10] is set, Z is tested, etc. If any of the condition codes tested is set,
/// the program branches to the location specified by adding the sign-extended PCoffset9 field to the incremented PC.
pub fn br(instr: u16, registers: &mut Registers) {
    let pc_offset = sign_extend((instr) & 0x1ff, 9);
    let cond_flag = (instr >> 9) & 0x7;
    if cond_flag & registers.r_cond != 0 {
        let val: u32 = registers.r_pc as u32 + pc_offset as u32;
        registers.r_pc = val as u16;
    }
}
