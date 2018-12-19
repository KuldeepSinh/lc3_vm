//! An `instruction` is a command which tells the CPU to do some fundamental task,
//! such as add two numbers.
//! Instructions have both an `opcode` which indicates the kind of task to perform
//! and a set of `parameters` which provide inputs to the task being performed.
//! Each `opcode` represents one task that the CPU "knows" how to do.
//! There are just 16 opcodes in LC-3. Everything the computer can calculate is some
//! sequence of these simple instructions. Each instruction is 16 bits long,
//! with the left 4 bits storing the opcode.
//! The rest of the bits are used to store the parameters.

/// Each `opcode` represents one task that the CPU "knows" how to do.
/// There are just 16 opcodes in LC-3. Everything the computer can calculate is some
/// sequence of these simple instructions. Each instruction is 16 bits long,
/// with the left 4 bits storing the opcode.
/// The rest of the bits are used to store the parameters.
#[derive(PartialEq, Debug)]
pub enum OpCode {
    /// `Br` is an `OpCode` for branch.
    Br,
    /// `Add` is an `OpCode` for add.
    Add,
    /// `Ld` is an `OpCode` for load.
    Ld,
    /// `St` is an `OpCode` for store.
    St,
    /// `Jsr` is an `OpCode` for jump register.
    Jsr,
    /// `And` is an `OpCode` for and.
    And,
    /// `Ldr` is an `OpCode` for load register.
    Ldr,
    /// `Str` is an `OpCode` for store register.
    Str,
    /// `Rti` is an unused `OpCode`.
    Rti,
    /// `Not` is an `OpCode` for bitwise not.
    Not,
    /// `Ldi` is an `OpCode` for load indirect.
    Ldi,
    /// `Sti` is an `OpCode` for store indirect.
    Sti,
    /// `Jmp` is an `OpCode` for jump.
    Jmp,
    /// `Res` is a reserved (unused) `OpCode`.
    Res,
    /// `Lea` is an `OpCode` for load effective address.
    Lea,
    /// `Trap` is an `OpCode` for execute trap.
    Trap,
}
impl OpCode {
    /// `get_op_code` returns Some(OpCode), when a valid value (between 0 to 15) is passed,
    /// otherwise it returns None.
    pub fn get_op_code(op_code: u16) -> Option<OpCode> {
        match op_code {
            0 => Some(OpCode::Br),
            1 => Some(OpCode::Add),
            2 => Some(OpCode::Ld),
            3 => Some(OpCode::St),
            4 => Some(OpCode::Jsr),
            5 => Some(OpCode::And),
            6 => Some(OpCode::Ldr),
            7 => Some(OpCode::Str),
            8 => Some(OpCode::Rti),
            9 => Some(OpCode::Not),
            10 => Some(OpCode::Ldi),
            11 => Some(OpCode::Sti),
            12 => Some(OpCode::Jmp),
            13 => Some(OpCode::Res),
            14 => Some(OpCode::Lea),
            15 => Some(OpCode::Trap),
            _ => None,
        }
    }
}

#[cfg(test)]
mod op_code_test {
    use super::*;
    #[test]
    fn op_codes_initial_values() {
        assert_eq!(Some(OpCode::Br), OpCode::get_op_code(0));
        assert_eq!(Some(OpCode::Add), OpCode::get_op_code(1));
        assert_eq!(Some(OpCode::Ld), OpCode::get_op_code(2));
        assert_eq!(Some(OpCode::St), OpCode::get_op_code(3));
        assert_eq!(Some(OpCode::Jsr), OpCode::get_op_code(4));
        assert_eq!(Some(OpCode::And), OpCode::get_op_code(5));
        assert_eq!(Some(OpCode::Ldr), OpCode::get_op_code(6));
        assert_eq!(Some(OpCode::Str), OpCode::get_op_code(7));
        assert_eq!(Some(OpCode::Rti), OpCode::get_op_code(8));
        assert_eq!(Some(OpCode::Not), OpCode::get_op_code(9));
        assert_eq!(Some(OpCode::Ldi), OpCode::get_op_code(10));
        assert_eq!(Some(OpCode::Sti), OpCode::get_op_code(11));
        assert_eq!(Some(OpCode::Jmp), OpCode::get_op_code(12));
        assert_eq!(Some(OpCode::Res), OpCode::get_op_code(13));
        assert_eq!(Some(OpCode::Lea), OpCode::get_op_code(14));
        assert_eq!(Some(OpCode::Trap), OpCode::get_op_code(15));
    }
}
