extern crate signal_hook;

use lc3_vm;
use lc3_vm::hardware::instructions::*;
use lc3_vm::sys::terminal::spawn_thread_for_signal_processing;
use std::env;
use std::error::Error;
use std::process;

fn main() -> Result<(), Box<Error>> {
    //setup for interrupt handling.
    spawn_thread_for_signal_processing()?;

    match lc3_vm::handle_args(env::args()) {
        Ok(mem) => process_instructions(mem),
        Err(_) => process::exit(1),
    };
    //change terminal as follows...
    //1. turn off canonical mode
    //2. turn off echo mod
    lc3_vm::sys::terminal::turn_off_canonical_and_echo_modes();
    println!("Hello, world!");
    Ok(())
}
