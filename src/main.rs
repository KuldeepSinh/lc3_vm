extern crate signal_hook;

use signal_hook::{iterator::Signals, SIGINT};
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
    //change terminal as follows...
    //1. turn off canonical mode
    //2. turn off echo mod
    lc3_vm::terminal::change_terminal();
    println!("Hello, world!");
    Ok(())
}
