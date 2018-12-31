use crate::hardware::register::Registers;

/// The program unconditionally jumps to the location specified by the contents of the base register.
/// Bits [8:6] identify the base register.
/// `RET` is listed as a separate instruction in the specification, since it is a different keyword in assembly.
/// However, it is actually a special case of JMP. RET happens whenever R1 is 7.
pub fn jmp(instr: u16, registers: &mut Registers) {
    // also handles RET
    let base_reg = (instr >> 6) & 0x7;
    registers.r_pc = registers.get(base_reg);
}
