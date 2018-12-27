use crate::hardware::register::Registers;
use crate::hardware::Memory;
use crate::sys::getchar::get_char;
use crate::sys::terminal::restore_terminal_settings;
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
    match instr & 0xFF {
        0x20 => {
            registers.update(0, get_char() as u16);
        }
        0x21 => {
            print!("{}", registers.r_00);
            io::stdout().flush().expect("Flushed.");
        }
        0x22 => {
            // /* one char per word */
            let mut index = registers.r_00 as usize;
            let mut c = memory.cells[index] as u8;
            while c as char != '\0' {
                print!("{}", c);
                index += 1;
                c = memory.cells[index] as u8
            }
            // {
            //  putc((char)*c, stdout);
            //  ++c;
            // }
            // fflush(stdout);
            // }
        }
        0x23 => {
            println!("Enter a character : ");
            registers.update(0, get_char() as u16);
        }
        0x24 => {
            unimplemented!("putc still needs to be implemented");
            // /* TRAP PUTSP */
            // {
            // /* one char per byte (two bytes per word)
            // here we need to swap back to
            // big endian format */
            // uint16_t* c = memory + reg[R_R0];
            // while (*c)
            // {
            //      char char1 = (*c) & 0xFF;
            //      putc(char1, stdout);
            //      char char2 = (*c) >> 8;
            //      if (char2) putc(char2, stdout);
            //         ++c;
            //  }
            //  fflush(stdout);
            //  }
        }
        0x25 => {
            /* TRAP HALT */
            print!("HALT");
            io::stdout().flush().expect("Flushed.");
            restore_terminal_settings();
            process::exit(1);
        }
        _ => {
            process::exit(1);
        }
    }
}
