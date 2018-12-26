extern crate termios;

pub use libc::STDIN_FILENO;
use std::process;
use termios::*;

pub fn handle_interrupt() {
    restore_original_termainal();
    println!("Exiting with interrupt code.\n");
    process::exit(-2);
}

pub fn restore_original_termainal() {
    let mut term: Termios = Termios::from_fd(STDIN_FILENO).unwrap();
    //turn on canonical mode and echo mode
    term.c_lflag |= ICANON | ECHO;
    tcsetattr(0, TCSANOW, &term).unwrap();
}

pub fn change_terminal() {
    let mut term: Termios = Termios::from_fd(STDIN_FILENO).unwrap();
    //turn off canonical mode and echo mode
    term.c_lflag &= !(ICANON | ECHO);
    tcsetattr(0, TCSANOW, &term).unwrap();
}
