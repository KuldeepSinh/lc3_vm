//set initial value of the program counter = 0x3000
pub const PC_START: u16 = 0x3000;
#[derive(Debug)]
pub struct Registers {
    pub r_01: u16,   // general purpose register
    pub r_02: u16,   // general purpose register
    pub r_03: u16,   // general purpose register
    pub r_04: u16,   // general purpose register
    pub r_05: u16,   // general purpose register
    pub r_06: u16,   // general purpose register
    pub r_07: u16,   // general purpose register
    pub r_pc: u16,   // program counter
    pub r_cond: u16, // condition flag
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            r_01: 0,        // general purpose register
            r_02: 0,        // general purpose register
            r_03: 0,        // general purpose register
            r_04: 0,        // general purpose register
            r_05: 0,        // general purpose register
            r_06: 0,        // general purpose register
            r_07: 0,        // general purpose register
            r_pc: PC_START, // program counter
            r_cond: 0,      // condition flag
        }
    }
}

// The R_COND register stores condition flags which provide information
// about the most recently executed calculation.
// This allows programs to check logical conditions such as if (x > 0) { ... }.
// The LC-3 uses only 3 condition flags
// which indicate the sign of the previous calculation.
pub enum ConditionFlag {
    FlPos, // = 1, Positive
    FlZro, // = 2, Zero
    FlNeg, // = 4, Negative
}

pub fn get_flag_value(flag: ConditionFlag) -> u8 {
    match flag {
        ConditionFlag::FlPos => 1 << 0, // Positive
        ConditionFlag::FlZro => 1 << 1, // Zero
        ConditionFlag::FlNeg => 1 << 2, // Negative
    }
}

#[cfg(test)]
mod registers_test {
    use super::*;
    #[test]
    fn value_of_r_pc_in_a_new_register_should_be_0x3000() {
        let registers = Registers::new();
        assert_eq!(0x3000, registers.r_pc);
    }
}

#[cfg(test)]
mod condition_flag_test {
    use super::*;
    #[test]
    fn value_of_flpos_should_be_1() {
        assert_eq!(1, get_flag_value(ConditionFlag::FlPos));
    }
    #[test]
    fn value_of_flzro_should_be_2() {
        assert_eq!(2, get_flag_value(ConditionFlag::FlZro));
    }

    #[test]
    fn value_of_flneg_should_be_4() {
        assert_eq!(4, get_flag_value(ConditionFlag::FlNeg));
    }
}
