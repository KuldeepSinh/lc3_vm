//! A VM is a program that acts like a computer.
//! It simulates a CPU along with a few other hardware components,
//! allowing it to perform arithmetic, read and write to memory, and interact with I/O devices, just like a physical computer.
//! Most importantly, it can understand a machine language which you can you can use to program it.
//! `LC-3 (Little Computer - 3) VM` simulates a fictional computer called the LC-3.
//! It has a simplified instruction set compared to x86, but contains all the main ideas used in modern CPUs.
//!
//! - Run `lc3_vm` using cargo : cargo run resources/2048.obj

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
