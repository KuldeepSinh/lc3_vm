extern crate termios;

// use std::io;
use std::os::unix::io::RawFd;
use std::process;
use termios::*;

pub fn get_original_termios(fd: RawFd) -> Termios {
    //todo : use memoization
    let term: Termios = Termios::from_fd(fd).unwrap();
    term
}

pub fn handle_interrupt() {
    //restore_original_termainal();
    println!("Exiting with interrupt code.\n");
    process::exit(-2);
}

pub fn restore_original_termainal(fd: RawFd, term: &Termios) {
    tcsetattr(fd, TCSANOW, &term).unwrap();
}

pub fn disable_canonical_mode_of_terminal(fd: RawFd, term: Termios) {
    let mut new_term = term;
    new_term.c_lflag = (!ICANON) & (!ECHO);
    tcsetattr(fd, TCSANOW, &new_term).unwrap();
}
