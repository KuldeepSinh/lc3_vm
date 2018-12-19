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
    /// `OpBr` is an `OpCode` for branch.
    OpBr,
    /// `OpAdd` is an `OpCode` for add.
    OpAdd,
    /// `OpLd` is an `OpCode` for load.
    OpLd,
    /// `OpSt` is an `OpCode` for store.
    OpSt,
    /// `OpJsr` is an `OpCode` for jump register.
    OpJsr,
    /// `OpAnd` is an `OpCode` for and.
    OpAnd,
    /// `OpLdr` is an `OpCode` for load register.
    OpLdr,
    /// `OpStr` is an `OpCode` for store register.
    OpStr,
    /// `OpRti` is an unused `OpCode`.
    OpRti,
    /// `OpNot` is an `OpCode` for bitwise not.
    OpNot,
    /// `OpLdi` is an `OpCode` for load indirect.
    OpLdi,
    /// `OpSti` is an `OpCode` for store indirect.
    OpSti,
    /// `OpJmp` is an `OpCode` for jump.
    OpJmp,
    /// `OpRes` is a reserved (unused) `OpCode`.
    OpRes,
    /// `OpLea` is an `OpCode` for load effective address.
    OpLea,
    /// `OpTrap` is an `OpCode` for execute trap.
    OpTrap,
}
impl OpCode {
    /// `get_op_code` returns Some(OpCode), when a valid value (between 0 to 15) is passed,
    /// otherwise it returns None.
    pub fn get_op_code(op_code: u16) -> Option<OpCode> {
        match op_code {
            0 => Some(OpCode::OpBr),
            1 => Some(OpCode::OpAdd),
            2 => Some(OpCode::OpLd),
            3 => Some(OpCode::OpSt),
            4 => Some(OpCode::OpJsr),
            5 => Some(OpCode::OpAnd),
            6 => Some(OpCode::OpLdr),
            7 => Some(OpCode::OpStr),
            8 => Some(OpCode::OpRti),
            9 => Some(OpCode::OpNot),
            10 => Some(OpCode::OpLdi),
            11 => Some(OpCode::OpSti),
            12 => Some(OpCode::OpJmp),
            13 => Some(OpCode::OpRes),
            14 => Some(OpCode::OpLea),
            15 => Some(OpCode::OpTrap),
            _ => None,
        }
    }
}

#[cfg(test)]
mod op_code_test {
    use super::*;
    #[test]
    fn op_codes_initial_values() {
        assert_eq!(Some(OpCode::OpBr), OpCode::get_op_code(0));
        assert_eq!(Some(OpCode::OpAdd), OpCode::get_op_code(1));
        assert_eq!(Some(OpCode::OpLd), OpCode::get_op_code(2));
        assert_eq!(Some(OpCode::OpSt), OpCode::get_op_code(3));
        assert_eq!(Some(OpCode::OpJsr), OpCode::get_op_code(4));
        assert_eq!(Some(OpCode::OpAnd), OpCode::get_op_code(5));
        assert_eq!(Some(OpCode::OpLdr), OpCode::get_op_code(6));
        assert_eq!(Some(OpCode::OpStr), OpCode::get_op_code(7));
        assert_eq!(Some(OpCode::OpRti), OpCode::get_op_code(8));
        assert_eq!(Some(OpCode::OpNot), OpCode::get_op_code(9));
        assert_eq!(Some(OpCode::OpLdi), OpCode::get_op_code(10));
        assert_eq!(Some(OpCode::OpSti), OpCode::get_op_code(11));
        assert_eq!(Some(OpCode::OpJmp), OpCode::get_op_code(12));
        assert_eq!(Some(OpCode::OpRes), OpCode::get_op_code(13));
        assert_eq!(Some(OpCode::OpLea), OpCode::get_op_code(14));
        assert_eq!(Some(OpCode::OpTrap), OpCode::get_op_code(15));
    }
}
