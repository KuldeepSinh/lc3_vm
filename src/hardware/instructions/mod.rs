pub mod add;
pub mod and;
pub mod br;
pub mod jmp;
pub mod jsr;
pub mod ld;
pub mod ldi;
pub mod ldr;
pub mod lea;
pub mod not;
pub mod st;
pub mod sti;
pub mod str;
pub mod trap;

use crate::hardware::register::ConditionFlag;
use crate::hardware::register::Registers;

fn sign_extend(mut x: u16, bit_count: u8) -> u16 {
    if (x >> (bit_count - 1)) & 1 != 0 {
        x |= 0xFFFF << bit_count;
    }
    x
}

fn update_flags(r: u16, registers: &mut Registers) {
    if registers.get(r) == 0 {
        registers.update(9, ConditionFlag::get_flag_value(ConditionFlag::FlZro));
    } else if (registers.get(r) >> 15) != 0 {
        // a 1 in the left-most bit indicates negative
        registers.update(9, ConditionFlag::get_flag_value(ConditionFlag::FlNeg));
    } else {
        registers.update(9, ConditionFlag::get_flag_value(ConditionFlag::FlPos));
    }
}
