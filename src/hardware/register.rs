//! A `register` is a slot for storing a single value on the CPU.
//! Registers are like the "workbench" of the CPU.
//! For the CPU to work with a piece of data, it has to be in one of the registers.
//! However, since there are just a few registers, only a minimal amount of data can be loaded at any given time.
//! Programs work around this by loading values from memory into registers, calculating values into other registers,
//! and then storing the final results back in memory.
/// `PC_START` sets initial value of the program counter (r_pc) = 0x3000.
pub const PC_START: u16 = 0x3000;

/// The LC-3 has 10 total registers, each of which is 16 bits. Most of them are general purpose, but a few have designated roles.
///
/// - 8 general purpose registers (R0-R7)
/// - 1 program counter (PC) register
/// - 1 condition flags (COND) register
/// The general purpose registers can be used to perform any program calculations.
/// The program counter is an unsigned integer which is the address of the next instruction in memory to execute.
/// The condition flags tell us information about the previous calculation.
#[derive(Debug)]
pub struct Registers {
    /// `r_00` is a general purpose register.
    pub r_00: u16, // general purpose register
    /// `r_01` is a general purpose register.
    pub r_01: u16, // general purpose register
    /// `r_02` is a general purpose register.
    pub r_02: u16, // general purpose register
    /// `r_03` is a general purpose register.
    pub r_03: u16, // general purpose register
    /// `r_04` is a general purpose register.
    pub r_04: u16, // general purpose register
    /// `r_05` is a general purpose register.
    pub r_05: u16, // general purpose register
    /// `r_06` is a general purpose register.
    pub r_06: u16, // general purpose register
    /// `r_07` is a general purpose register.
    pub r_07: u16, // general purpose register
    /// `r_pc` is a register for program counter.
    pub r_pc: u16, // program counter
    /// `r_cond` is a register to store cinformation about the previous calculation.
    pub r_cond: u16, // condition flag
}

impl Registers {
    /// `new()` is an associated method of `Registers`.
    /// It is used to initialize all the registers to zero, except
    /// `r_pc`, which represents the program counter. It's initialized with value = `0x3000`.
    pub fn new() -> Registers {
        Registers {
            r_00: 0,        // general purpose register
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

/// The R_COND register stores condition flags which provide information
/// about the most recently executed calculation.
/// This allows programs to check logical conditions such as if (x > 0) { ... }.
/// The LC-3 uses only 3 condition flags
/// which indicate the sign of the previous calculation.
/// - `FlPos` represents `Positive` value.
/// - `FlZro` represents `Zero` value.
/// - `FlNeg` represents `Negative` value.
pub enum ConditionFlag {
    /// `ConditionFlag::FlPos` represents `Positive`.
    FlPos, // = 1, Positive
    /// `ConditionFlag::FlZro` represents `Zero`.
    FlZro, // = 2, Zero
    /// `ConditionFlag::FlNeg` represents `Negative`.
    FlNeg, // = 4, Negative
}

impl ConditionFlag {
    /// `get_flag_value` function returns `u8` value for the `ConditionFlag` passed to its argument.
    /// - `ConditionFlag::FlPos` = 1, Positive
    /// - `ConditionFlag::FlZro` = 2, Zero
    /// - `ConditionFlag::FlNeg` = 4, Negative
    pub fn get_flag_value(flag: ConditionFlag) -> u8 {
        match flag {
            ConditionFlag::FlPos => 1 << 0, // Positive
            ConditionFlag::FlZro => 1 << 1, // Zero
            ConditionFlag::FlNeg => 1 << 2, // Negative
        }
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
        assert_eq!(1, ConditionFlag::get_flag_value(ConditionFlag::FlPos));
    }
    #[test]
    fn value_of_flzro_should_be_2() {
        assert_eq!(2, ConditionFlag::get_flag_value(ConditionFlag::FlZro));
    }

    #[test]
    fn value_of_flneg_should_be_4() {
        assert_eq!(4, ConditionFlag::get_flag_value(ConditionFlag::FlNeg));
    }
}
