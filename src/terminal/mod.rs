extern crate termios;

use std::os::unix::io::RawFd;
use std::process;
use termios::*;

pub struct Cacher<T>
where
    T: Fn(RawFd) -> Termios,
{
    calculation: T,
    value: Option<Termios>,
}

impl<T> Cacher<T>
where
    T: Fn(RawFd) -> Termios,
{
    pub fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    pub fn value(&mut self, arg: RawFd) -> Termios {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

pub struct Term {
    pub term: Termios,
}

impl Term {
    pub fn get_term(&self) -> Term {
        Term {
            term: Termios::from_fd(0).unwrap(),
        }
    }
}

pub fn handle_interrupt() {
    restore_original_termainal();
    println!("Exiting with interrupt code.\n");
    process::exit(-2);
}

pub fn restore_original_termainal() {
    //0 passed in from_fd is equal to "STDIN_FILENO" (its a libc const)
    let mut term: Termios = Termios::from_fd(0).unwrap();
    term.c_lflag = ICANON | ECHO;
    tcsetattr(0, TCSANOW, &term).unwrap();
}

pub fn change_terminal() {
    //0 passed in from_fd is equal to "STDIN_FILENO" (its a libc const)
    let mut term: Termios = Termios::from_fd(0).unwrap();
    term.c_lflag &= (!ICANON) & (!ECHO);
    tcsetattr(0, TCSANOW, &term).unwrap();
}
