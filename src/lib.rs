//! Our VM will simulate a fictional computer called the `LC-3`.
//! The LC-3 is popular for teaching university students how to program in assembly language.
//! It has a simplified instruction set compared to x86, but contains all the main ideas used in modern CPUs.

pub mod hardware;
use self::hardware::memory::Memory;
use self::hardware::opcode::OpCode;
use self::hardware::register::PC_START;

pub fn run(memory: Memory) {
    //loop through memory from PC_START to end of the memory.
    let mut pc_pointer = PC_START as usize;
    loop {
        //get instruction from the current position
        let instruction = memory.cells[pc_pointer];
        //extract op_code and run operation...
        match_and_perform_op(&instruction);
        //increment pointer
        pc_pointer += 1;
    }
}

fn match_and_perform_op(instruction: &u16) {
    //extract op_code from the instruction
    let op_code = extract_op_code(&instruction);
    //match and perform operation
    match op_code {
        Some(OpCode::Add) => {}
        // Some(OpCode::op_and) => {}
        // Some(OpCode::op_not) => {}
        // Some(OpCode::op_br) => {}
        // Some(OpCode::op_jmp) => {}
        // Some(OpCode::op_jsr) => {}
        // Some(OpCode::op_ld) => {}
        // Some(OpCode::op_ldi) => {}
        // Some(OpCode::op_ldr) => {}
        // Some(OpCode::op_lea) => {}
        // Some(OpCode::op_st) => {}
        // Some(OpCode::op_sti) => {}
        // Some(OpCode::op_str) => {}
        //Some(OpCode::OpTrap) => {}
        _ => {}
    }
}

//Each instruction is 16 bits long, with the left 4 bits storing the opcode.
//The rest of the bits are used to store the parameters.
//To extract left 4 bits out of the instruction, we'll use ">>" shift-right
//operator and shift first 4 bits 12 positions towards right
fn extract_op_code(instruction: &u16) -> Option<OpCode> {
    OpCode::get_op_code(instruction >> 12)
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
