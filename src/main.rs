extern crate signal_hook;

use lc3_vm::hardware;
use lc3_vm::sys::terminal;
use std::env;
use std::error::Error;
use std::process;

fn main() -> Result<(), Box<Error>> {
    //setup for interrupt handling.
    terminal::spawn_control_c_handler()?;
    //handle command line arguments and process instructions
    match lc3_vm::handle_args(env::args()) {
        Ok(mem) => {
            //execute program
            hardware::execute_program(mem);
            //restore terminal settings
            terminal::restore_terminal_settings();
            //return
            Ok(())
        }
        Err(_) => {
            //restore terminal settings
            terminal::restore_terminal_settings();
            //exit
            process::exit(1)
        }
    }
}
