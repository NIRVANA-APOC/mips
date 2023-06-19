use super::dram::{dram_read, dram_write};

pub fn mem_read(addr: u32, len: usize) -> u32{
    dram_read(addr, len) & (!(0 as u32) >> ((4 - len) << 3))
}

pub fn mem_write(addr: u32, len: usize, data: u32){
    dram_write(addr, len, data)
}