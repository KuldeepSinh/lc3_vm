#[derive(PartialEq, Debug)]
pub enum OpCode {
    OpBr,   // branch
    OpAdd,  // add
    OpLd,   // load
    OpSt,   // store
    OpJsr,  // jump register
    OpAnd,  // bitwise and
    OpLdr,  // load register
    OpStr,  // store register
    OpRti,  // unused
    OpNot,  // bitwise not
    OpLdi,  // load indirect
    OpSti,  // store indirect
    OpJmp,  // jump
    OpRes,  // reserved (unused)
    OpLea,  // load effective address
    OpTrap, // execute trap
}
impl OpCode {
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
