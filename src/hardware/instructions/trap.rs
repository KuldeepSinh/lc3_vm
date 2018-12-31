use crate::hardware::register::Registers;
use crate::hardware::Memory;
use crate::sys::getchar;
use crate::sys::terminal;
use std::io;
use std::io::Write;
use std::process;

/* TRAP Codes */
pub enum TrapCode {
    Getc = 0x20,  /* get character from keyboard */
    Out = 0x21,   /* output a character */
    Puts = 0x22,  /* output a word string */
    In = 0x23,    /* input a string */
    Putsp = 0x24, /* output a byte string */
    Halt = 0x25,  /* halt the program */
}

pub fn trap(instr: u16, registers: &mut Registers, memory: &mut Memory) {
    terminal::turn_off_canonical_and_echo_modes();
    match instr & 0xFF {
        0x20 => {
            registers.update(0, getchar::get_char() as u16);
        }
        0x21 => {
            print!("{}", (registers.r_00 as u8) as char);
            io::stdout().flush().expect("Flushed.");
        }
        0x22 => {
            // /* one char per word */
            let mut index = registers.r_00 as usize;
            let mut c = memory.cells[index];
            while c != 0x0000 {
                print!("{}", (c as u8) as char);
                index += 1;
                c = memory.cells[index];
            }
            io::stdout().flush().expect("Flushed.");
        }
        0x23 => {
            print!("Enter a character : ");
            io::stdout().flush().expect("Flushed.");
            registers.update(0, getchar::get_char() as u16);
        }
        0x24 => {
            let mut index = registers.r_00 as usize;
            let mut c = memory.cells[index];
            while c != 0x0000 {
                let c1 = ((c & 0xFF) as u8) as char;
                print!("{}", c1);
                let c2 = ((c >> 8) as u8) as char;
                if c2 != '\0' {
                    print!("{}", c2);
                }
                index += 1;
                c = memory.cells[index];
            }
            io::stdout().flush().expect("Flushed.");
        }
        0x25 => {
            /* TRAP HALT */
            print!("HALT");
            io::stdout().flush().expect("Flushed.");
            process::exit(1);
        }
        _ => {
            process::exit(1);
        }
    }
    terminal::restore_terminal_settings();
}
