//! Our VM will simulate a fictional computer called the `LC-3`.
//! The LC-3 is popular for teaching university students how to program in assembly language.
//! It has a simplified instruction set compared to x86, but contains all the main ideas used in modern CPUs.

pub mod file;
pub mod hardware;
pub mod sys;

use self::hardware::memory::Memory;
use std::env::Args;

/// `handle_args` fn processes commandline arguments.
/// If appropriate file path is found in the args, it reads the file into the memory.
pub fn handle_args(mut args: Args) -> Result<Memory, &'static str> {
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
