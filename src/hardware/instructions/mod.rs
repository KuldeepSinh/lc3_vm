//! An `instr` is a command which tells the CPU to do some fundamental task,
//! instrs have both an `opcode` which indicates the kind of task to perform and
//! a set of `parameters` which provide inputs to the task being performed.
pub mod add;
pub mod and;
pub mod br;
pub mod jmp;
pub mod jsr;
pub mod ld;
pub mod ldi;
pub mod ldr;
pub mod lea;
pub mod not;
pub mod opcode;
pub mod st;
pub mod sti;
pub mod str;
pub mod trap;

/// `sign_extend` is used to convert a 5 bit number into an equivalent 16 bit number.
fn sign_extend(mut x: u16, bit_count: u8) -> u16 {
    if (x >> (bit_count - 1)) & 1 != 0 {
        x |= 0xFFFF << bit_count;
    }
    x
}
