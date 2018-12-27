extern crate signal_hook;

use lc3_vm::file;
use lc3_vm::hardware::instructions::*;
use lc3_vm::hardware::memory::Memory;
use signal_hook::{iterator::Signals, SIGINT};
use std::env;
use std::env::Args;
use std::process;
use std::{error::Error, thread};

fn main() -> Result<(), Box<Error>> {
    //setup for interrupt handling.
    let signals = Signals::new(&[SIGINT])?;
    thread::spawn(move || {
        for _sig in signals.forever() {
            //Interrupt (Ctrl + C) is handled as follows...
            //Terminal is restored to its original configuration
            //Process is exited with (-2)
            lc3_vm::terminal::handle_interrupt();
        }
    });
    match handle_args(env::args()) {
        Ok(mem) => process_instructions(mem),
        Err(_) => process::exit(1),
    };
    //change terminal as follows...
    //1. turn off canonical mode
    //2. turn off echo mod
    lc3_vm::terminal::change_terminal();
    println!("Hello, world!");
    Ok(())
}

fn handle_args(mut args: Args) -> Result<Memory, &'static str> {
    //skip 0th element
    args.next();
    match args.next() {
        Some(arg) => match file::read_file(arg) {
            Ok(mem) => Ok(mem),
            Err(_) => Err("Error encountered while reading into memory."),
        },
        None => Err("No more file for processing."),
    }
}
