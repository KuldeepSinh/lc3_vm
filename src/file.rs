//! `file` reading :
//! The first 16 bits of the program file specify the address in memory where the program should start.
//! This address is called the origin. It must be read first,
//! after which the rest of the data can be read from the file into memory starting at the origin address.

extern crate byteorder;

use crate::hardware::memory::Memory;
use byteorder::{BigEndian, ReadBytesExt};
use std::{
    fs::File,
    io::{self, BufReader, Read},
};
/// `read_file` fn reads a file into the LC-3 Memory.
/// LC-3 programs are big-endian, but most of the modern computers we use are little endian.
// As a result, we are reading instruction as BigEndian (with the help of `exter crate byteorder`).
pub fn read_file(name: String) -> io::Result<Memory> {
    let f = File::open(name)?;
    let f = BufReader::new(f);
    read_file_into_lc3_memory(f)
}

fn read_file_into_lc3_memory<R>(mut rdr: R) -> io::Result<Memory>
where
    R: Read,
{
    //find the base address
    let base_address = rdr.read_u16::<BigEndian>()?;
    let mut address = base_address as usize;
    let mut mem = Memory::new();
    loop {
        match rdr.read_u16::<BigEndian>() {
            Ok(instruction) => {
                mem.write(address, instruction);
                address += 1;
            }
            Err(e) => {
                return if e.kind() == std::io::ErrorKind::UnexpectedEof {
                    Ok(mem)
                } else {
                    Err(e)
                }
            }
        }
    }
}
