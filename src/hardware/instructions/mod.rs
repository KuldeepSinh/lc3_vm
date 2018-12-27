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

use crate::hardware::memory::Memory;
use crate::hardware::register::ConditionFlag;
use crate::hardware::register::Registers;
use std::process;

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

pub fn process_instructions(mem: Memory) {
    let mut registers = Registers::new();
    let mut memory = mem.clone();
    loop {
        let instruction = memory.read(registers.r_pc);
        registers.r_pc += 1;
        match instruction >> 12 {
            0 => br::br(instruction, &mut registers),
            1 => add::add(instruction, &mut registers),
            2 => ld::ld(instruction, &mut registers, &mut memory),
            3 => st::st(instruction, &mut registers, &mut memory),
            4 => jsr::jsr(instruction, &mut registers),
            5 => and::and(instruction, &mut registers),
            6 => ldr::ldr(instruction, &mut registers, &mut memory),
            7 => str::str(instruction, &mut registers, &mut memory),
            //8 => {} //rti
            9 => not::not(instruction, &mut registers),
            10 => ldi::ldi(instruction, &mut registers, &mut memory),
            11 => sti::sti(instruction, &mut registers, &mut memory),
            12 => jmp::jmp(instruction, &mut registers),
            //13 => {} //res
            14 => lea::lea(instruction, &mut registers),
            15 => trap::trap(instruction, &mut registers, &mut memory),
            _ => process::exit(1),
        }
    }
}
