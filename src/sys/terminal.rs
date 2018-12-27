extern crate termios;

pub use libc::STDIN_FILENO;
use std::process;
use termios::*;

use signal_hook::{iterator::Signals, SIGINT};
use std::{error::Error, thread};

pub fn handle_interrupt() {
    turn_on_canonical_and_echo_modes();
    println!("Exiting with interrupt code.\n");
    process::exit(-2);
}

pub fn turn_on_canonical_and_echo_modes() {
    let mut term: Termios = Termios::from_fd(STDIN_FILENO).unwrap();
    //turn on canonical mode and echo mode
    term.c_lflag |= ICANON | ECHO;
    tcsetattr(0, TCSANOW, &term).unwrap();
}

pub fn turn_off_canonical_and_echo_modes() {
    let mut term: Termios = Termios::from_fd(STDIN_FILENO).unwrap();
    //turn off canonical mode and echo mode
    term.c_lflag &= !(ICANON | ECHO);
    tcsetattr(0, TCSANOW, &term).unwrap();
}

pub fn spawn_thread_for_signal_processing() -> Result<(), Box<Error>> {
    //setup for interrupt handling.
    let signals = Signals::new(&[SIGINT])?;
    thread::spawn(move || {
        for _sig in signals.forever() {
            //Interrupt (Ctrl + C) is handled as follows...
            //Terminal is restored to its original configuration
            //Process is exited with (-2)
            handle_interrupt();
        }
    });
    Ok(())
}
