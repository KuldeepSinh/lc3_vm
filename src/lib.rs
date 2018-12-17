mod hardware {
    const MEMORY_SIZE: usize = std::u16::MAX as usize;
    struct Memory {
        location: [u16; MEMORY_SIZE],
    }

    struct Registers {
        r_01: u16,   // general purpose register
        r_02: u16,   // general purpose register
        r_03: u16,   // general purpose register
        r_04: u16,   // general purpose register
        r_05: u16,   // general purpose register
        r_06: u16,   // general purpose register
        r_07: u16,   // general purpose register
        r_pc: u16,   // program counter
        r_cond: u16, // condition flag
    }

    struct OpCode {
        op_br: u8,   // branch
        op_add: u8,  // add
        op_ld: u8,   // load
        op_st: u8,   // store
        op_jsr: u8,  // jump register
        op_and: u8,  // bitwise and
        op_ldr: u8,  // load register
        op_str: u8,  // store register
        op_rti: u8,  // unused
        op_not: u8,  // bitwise not
        op_ldi: u8,  // load indirect
        op_sti: u8,  // store indirect
        op_jmp: u8,  // jump
        op_res: u8,  // reserved (unused)
        op_lea: u8,  // load effective address
        op_trap: u8, // execute trap
    }

    impl OpCode {
        fn new() -> OpCode {
            OpCode {
                op_br: 0,    // branch
                op_add: 1,   // add
                op_ld: 2,    // load
                op_st: 3,    // store
                op_jsr: 4,   // jump register
                op_and: 5,   // bitwise and
                op_ldr: 6,   // load register
                op_str: 7,   // store register
                op_rti: 8,   // unused
                op_not: 9,   // bitwise not
                op_ldi: 10,  // load indirect
                op_sti: 11,  // store indirect
                op_jmp: 12,  // jump
                op_res: 13,  // reserved (unused)
                op_lea: 14,  // load effective address
                op_trap: 15, // execute trap
            }
        }
    }

    enum ConditionFlag {
        FlPos,
        FlZro,
        FlNeg,
    }

    fn get_flag_value(flag: ConditionFlag) -> u8 {
        match flag {
            ConditionFlag::FlPos => 1 << 0, // Positive
            ConditionFlag::FlZro => 1 << 1, // Zero
            ConditionFlag::FlNeg => 1 << 2, // Negative
        }
    }

    #[cfg(test)]
    mod condition_flag_test {
        use super::*;
        #[test]
        fn value_of_FlPos_should_be_1() {
            assert_eq!(1, get_flag_value(ConditionFlag::FlPos));
        }
        #[test]
        fn value_of_FLZro_should_be_2() {
            assert_eq!(2, get_flag_value(ConditionFlag::FlZro));
        }

        #[test]
        fn value_of_FlNeg_should_be_4() {
            assert_eq!(4, get_flag_value(ConditionFlag::FlNeg));
        }

    }

    #[cfg(test)]
    mod op_code_test {
        use super::*;
        #[test]
        fn op_code_test() {
            let op_codes = OpCode::new();
            assert_eq!(0, op_codes.op_br);
            assert_eq!(1, op_codes.op_add);
            assert_eq!(2, op_codes.op_ld);
            assert_eq!(3, op_codes.op_st);
            assert_eq!(4, op_codes.op_jsr);
            assert_eq!(5, op_codes.op_and);
            assert_eq!(6, op_codes.op_ldr);
            assert_eq!(7, op_codes.op_str);
            assert_eq!(8, op_codes.op_rti);
            assert_eq!(9, op_codes.op_not);
            assert_eq!(10, op_codes.op_ldi);
            assert_eq!(11, op_codes.op_sti);
            assert_eq!(12, op_codes.op_jmp);
            assert_eq!(13, op_codes.op_res);
            assert_eq!(14, op_codes.op_lea);
            assert_eq!(15, op_codes.op_trap);
        }

    }
}
