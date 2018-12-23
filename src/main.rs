extern crate signal_hook;

use signal_hook::{iterator::Signals, SIGINT};
use std::{error::Error, thread};

fn main() -> Result<(), Box<Error>> {
    //setup for interrupt handling.
    let signals = Signals::new(&[SIGINT])?;
    thread::spawn(move || {
        for _sig in signals.forever() {
            lc3_vm::terminal::handle_interrupt();
        }
    });
    println!("Hello, world!");
    Ok(())
}
