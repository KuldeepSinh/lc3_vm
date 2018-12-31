use super::sign_extend;
use crate::hardware::register::Registers;

/// First, the incremented PC is saved in R7.
/// This is the linkage back to the calling routine.
/// Then the PC is loaded with the address of the first instruction of the subroutine,
/// causing an unconditional jump to that address.
/// The address of the subroutine is obtained from the base register (if bit [11] is 0),
/// or the address is computed by sign-extending bits [10:0] and adding this value to the incremented PC (if bit [11] is 1).

pub fn jsr(instr: u16, registers: &mut Registers) {
    let base_reg = (instr >> 6) & 0x7;
    let long_pc_offset = sign_extend(instr & 0x7ff, 11);
    let long_flag = (instr >> 11) & 1;
    registers.r_07 = registers.r_pc;

    if long_flag != 0 {
        let val: u32 = registers.r_pc as u32 + long_pc_offset as u32;
        registers.r_pc = val as u16; /* JSR */
    } else {
        registers.r_pc = registers.get(base_reg); /* JSRR */
    }
}
