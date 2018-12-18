pub mod hardware;
// use crate::hardware::Memory;
// use crate::hardware::OpCode;
// use crate::hardware::PC_START;
//
// fn run(memory: Memory) {
//     //loop through memory from PC_START to end of the memory.
//     let mut pc_pointer = PC_START as usize;
//     loop {
//         //get instruction from the current position
//         let instruction = memory.instructions[pc_pointer];
//         //extract op_code and run operation...
//         match_and_perform_op(&instruction);
//         //increment pointer
//         pc_pointer += 1;
//     }
// }

// fn match_and_perform_op(instruction: &u16) {
//     //extract op_code from the instruction
//     let op_code = extract_op_code(&instruction);
//     //match and perform operation
//     match op_code {
//         Some(OpCode::op_add) => {}
//         // Some(OpCode::op_and) => {}
//         // Some(OpCode::op_not) => {}
//         // Some(OpCode::op_br) => {}
//         // Some(OpCode::op_jmp) => {}
//         // Some(OpCode::op_jsr) => {}
//         // Some(OpCode::op_ld) => {}
//         // Some(OpCode::op_ldi) => {}
//         // Some(OpCode::op_ldr) => {}
//         // Some(OpCode::op_lea) => {}
//         // Some(OpCode::op_st) => {}
//         // Some(OpCode::op_sti) => {}
//         // Some(OpCode::op_str) => {}
//         Some(OpCode::op_trap) => {}
//         _ => {}
//     }
// }
//
// fn extract_op_code(instruction: &u16) -> Option<OpCode> {
//     OpCode::get_op_code(instruction >> 12)
// }
//
// #[cfg(test)]
// mod extract_op_code_test {
//     use super::*;
//     #[test]
//     fn extract_test() {
//         let four = 16384;
//         assert_eq!(Some(OpCode::op_jsr), extract_op_code(&four));
//     }
// }
