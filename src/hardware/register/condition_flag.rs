use crate::hardware::register::Registers;

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
    pub fn get_flag_value(flag: ConditionFlag) -> u16 {
        match flag {
            ConditionFlag::FlPos => 1 << 0, // Positive
            ConditionFlag::FlZro => 1 << 1, // Zero
            ConditionFlag::FlNeg => 1 << 2, // Negative
        }
    }
}

pub fn update_flags(r: u16, registers: &mut Registers) {
    if registers.get(r) == 0 {
        registers.update(9, ConditionFlag::get_flag_value(ConditionFlag::FlZro));
    } else if (registers.get(r) >> 15) != 0 {
        // a 1 in the left-most bit indicates negative
        registers.update(9, ConditionFlag::get_flag_value(ConditionFlag::FlNeg));
    } else {
        registers.update(9, ConditionFlag::get_flag_value(ConditionFlag::FlPos));
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
