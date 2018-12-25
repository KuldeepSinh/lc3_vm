extern crate byteorder;

use crate::hardware::memory::Memory;
use byteorder::{BigEndian, ReadBytesExt};
use std::{
    fs::File,
    io::{self, BufReader, Read},
};

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
