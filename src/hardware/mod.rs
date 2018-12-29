//! # Hardware - A module to simulate hardware of LC-3 (Little Computer - 3).
//! The `hardware` module is created to simulate hardware components of the LC-3 (Little Computer - 3) CPU.
//! Main components of the hardware are Memory, Registers and OpCodes.
pub mod instructions;
pub mod memory;
pub mod register;

use self::instructions::opcode;
use self::memory::Memory;
use self::register::Registers;

pub fn execute_program(mem: Memory) {
    //initialize Registers
    let mut memory = mem.clone();
    let mut registers = Registers::new();
    while registers.r_pc < std::u16::MAX {
        //read instruction
        let instruction = memory.read(registers.r_pc);

        //increment program counter
        registers.r_pc += 1;

        //extract op_code and execute operation...
        opcode::execute_instruction(instruction, &mut registers, &mut memory);
    }
}
