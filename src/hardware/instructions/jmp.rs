use crate::hardware::register::Registers;

pub fn jmp(instr: u16, registers: &mut Registers) {
    let r1 = (instr >> 6) & 0x7;
    registers.r_pc = registers.get(r1);
}
