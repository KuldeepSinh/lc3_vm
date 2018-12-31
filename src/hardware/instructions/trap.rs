//! `trap` :
//! The LC-3 provides a few predefined routines for performing common tasks and interacting with I/O devices.
//! For example, there are routines for getting input from the keyboard and for displaying strings to the console.
//! These are called trap routines which you can think of as the operating system or API for the LC-3.
//! Each trap routine is assigned a trap code which identifies it (similar to an opcode).
//! To execute one, the TRAP instruction is called with the trap code of the desired routine.
use crate::hardware::register::Registers;
use crate::hardware::Memory;
use crate::sys::getchar;
use crate::sys::terminal;
use std::io;
use std::io::Write;
use std::process;

// TRAP Codes
pub enum TrapCode {
    /// get character from keyboard
    Getc = 0x20, /* get character from keyboard */
    /// output a character
    Out = 0x21, /* output a character */
    /// output a word string
    Puts = 0x22, /* output a word string */
    /// input a string
    In = 0x23, /* input a string */
    /// output a byte string
    Putsp = 0x24, /* output a byte string */
    /// halt the program
    Halt = 0x25, /* halt the program */
}

/// `trap` fn allows interacting with I/O devices.
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
            terminal::restore_terminal_settings();
            process::exit(1);
        }
        _ => {
            terminal::restore_terminal_settings();
            process::exit(1);
        }
    }
    terminal::restore_terminal_settings();
}
