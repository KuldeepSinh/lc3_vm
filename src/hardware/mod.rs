//! # Hardware - A module to simulate hardware of LC-3 (Little Computer - 3).
//! The `hardware` module is created to simulate hardware components of the LC-3 (Little Computer - 3) CPU.
//! Main components of the hardware are Memory, Registers and OpCodes.
pub mod instructions;
pub mod memory;
pub mod opcode;
pub mod register;

use self::memory::Memory;
use self::opcode::OpCode;
use self::register::Registers;

pub fn execute_program(mem: Memory) {
    //initialize Registers
    let mut memory = mem.clone();
    let mut registers = Registers::new();
    loop {
        //read instruction
        let instruction = memory.read(registers.r_pc);
        //increment program counter
        registers.r_pc += 1;
        //extract op_code and execute operation...
        execute_instruction(instruction, &mut registers, &mut memory);
    }
}

fn execute_instruction(instr: u16, mut registers: &mut Registers, mut memory: &mut Memory) {
    //extract op_code from the instruction
    let op_code = extract_op_code(&instr);
    //match op_code and execute instruction
    match op_code {
        Some(OpCode::Add) => instructions::add::add(instr, &mut registers),
        Some(OpCode::And) => instructions::and::and(instr, &mut registers),
        Some(OpCode::Not) => instructions::not::not(instr, &mut registers),
        Some(OpCode::Br) => instructions::br::br(instr, &mut registers),
        Some(OpCode::Jmp) => instructions::jmp::jmp(instr, &mut registers),
        Some(OpCode::Jsr) => instructions::jsr::jsr(instr, &mut registers),
        Some(OpCode::Ld) => instructions::ld::ld(instr, &mut registers, &mut memory),
        Some(OpCode::Ldi) => instructions::ldi::ldi(instr, &mut registers, &mut memory),
        Some(OpCode::Ldr) => instructions::ldr::ldr(instr, &mut registers, &mut memory),
        Some(OpCode::Lea) => instructions::lea::lea(instr, &mut registers),
        Some(OpCode::St) => instructions::st::st(instr, &mut registers, &mut memory),
        Some(OpCode::Sti) => instructions::sti::sti(instr, &mut registers, &mut memory),
        Some(OpCode::Str) => instructions::str::str(instr, &mut registers, &mut memory),
        Some(OpCode::Trap) => instructions::trap::trap(instr, &mut registers, &mut memory),
        _ => {}
    }
}

//Each instruction is 16 bits long, with the left 4 bits storing the opcode.
//The rest of the bits are used to store the parameters.
//To extract left 4 bits out of the instruction, we'll use ">>" shift-right
//operator and shift first 4 bits 12 positions towards right
fn extract_op_code(instruction: &u16) -> Option<OpCode> {
    OpCode::get(instruction >> 12)
}

#[cfg(test)]
mod extract_op_code_test {
    use super::*;
    #[test]
    fn extract_test() {
        let four = 16384;
        assert_eq!(Some(OpCode::Jsr), extract_op_code(&four));
    }
}
