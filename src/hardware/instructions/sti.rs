use super::sign_extend;
use crate::hardware::register::Registers;
use crate::hardware::Memory;

/// The contents of the register specified by SR are stored in the memory location
/// whose address is obtained as follows: Bits [8:0] are sign-extended to 16 bits and added to the incremented PC.
/// What is in memory at this address is the address of the location to which the data in SR is stored.
pub fn sti(instr: u16, registers: &mut Registers, memory: &mut Memory) {
    let dr = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1ff, 9);
    let val: u32 = registers.r_pc as u32 + pc_offset as u32;
    let val: u16 = val as u16;
    let adrs = memory.read(val) as usize;
    memory.write(adrs, registers.get(dr));
}
