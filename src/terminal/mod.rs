extern crate termios;

use std::process;
use termios::*;

pub fn handle_interrupt() {
    restore_original_termainal();
    println!("Exiting with interrupt code.\n");
    process::exit(-2);
}

pub fn restore_original_termainal() {
    //0 passed for from_fd is equal to "STDIN_FILENO" (its a libc const)
    let mut term: Termios = Termios::from_fd(0).unwrap();
    //turn on canonical mode and echo mode
    term.c_lflag |= ICANON | ECHO;
    tcsetattr(0, TCSANOW, &term).unwrap();
}

pub fn change_terminal() {
    //0 passed for from_fd is equal to "STDIN_FILENO" (its a libc const)
    let mut term: Termios = Termios::from_fd(0).unwrap();
    //turn off canonical mode and echo mode
    term.c_lflag &= !(ICANON | ECHO);
    tcsetattr(0, TCSANOW, &term).unwrap();
}
