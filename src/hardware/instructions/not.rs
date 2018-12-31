use crate::hardware::register::condition_flag;
use crate::hardware::register::Registers;

/// The bit-wise complement of the contents of SR is stored in DR.
/// The condi- tion codes are set, based on whether the binary value produced,
/// taken as a 2’s complement integer, is negative, zero, or positive.
pub fn not(instr: u16, registers: &mut Registers) {
    let dr = (instr >> 9) & 0x7;
    let sr1 = (instr >> 6) & 0x7;
    registers.update(dr, !registers.get(sr1));

    condition_flag::update_r_cond_register(dr, registers);
}
