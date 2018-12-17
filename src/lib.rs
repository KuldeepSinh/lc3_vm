mod hardware {
    struct Memory {
        location: [u16; std::u16::MAX],
    }

    struct Registers {
        R_01: u16,   // general purpose register
        R_02: u16,   // general purpose register
        R_03: u16,   // general purpose register
        R_04: u16,   // general purpose register
        R_05: u16,   // general purpose register
        R_06: u16,   // general purpose register
        R_07: u16,   // general purpose register
        R_PC: u16,   // program counter
        R_COND: u16, // condition flag
    }

    struct OpCodes {
        OP_BR: u8,   // branch
        OP_ADD: u8,  // add
        OP_LD: u8,   // load
        OP_ST: u8,   // store
        OP_JSR: u8,  // jump register
        OP_AND: u8,  // bitwise and
        OP_LDR: u8,  // load register
        OP_STR: u8,  // store register
        OP_RTI: u8,  // unused
        OP_NOT: u8,  // bitwise not
        OP_LDI: u8,  // load indirect
        OP_STI: u8,  // store indirect
        OP_JMP: u8,  // jump
        OP_RES: u8,  // reserved (unused)
        OP_LEA: u8,  // load effective address
        OP_TRAP: u8, // execute trap
    }

    impl OpCodes {
        fn new() -> OpCodes {
            OpCodes {
                OP_BR: 0,    // branch
                OP_ADD: 1,   // add
                OP_LD: 2,    // load
                OP_ST: 3,    // store
                OP_JSR: 4,   // jump register
                OP_AND: 5,   // bitwise and
                OP_LDR: 6,   // load register
                OP_STR: 7,   // store register
                OP_RTI: 8,   // unused
                OP_NOT: 9,   // bitwise not
                OP_LDI: 10,  // load indirect
                OP_STI: 11,  // store indirect
                OP_JMP: 12,  // jump
                OP_RES: 13,  // reserved (unused)
                OP_LEA: 14,  // load effective address
                OP_TRAP: 15, // execute trap
            }
        }
    }

    enum ConditionFlag {
        FL_POS,
        FL_ZRO,
        FL_NEG,
    }

    fn get_flag_value(flag: ConditionFlag) -> u8 {
        match flag {
            ConditionFlag::FL_POS => 1 << 0, // Positive
            ConditionFlag::FL_ZRO => 1 << 1, // Zero
            ConditionFlag::FL_NEG => 1 << 2, // Negative
        }
    }

}
