use std::ptr;
use std::io::Read;
use std::os::windows::prelude::FileExt;
use colored::Colorize;
use super::cpu_exec::{CpuState, CPU, CPU_STATE};
use super::super::memory::dram::{DRAM, init_ddr3, clear_dram};

const ENTRY_START: u32 = 0xbfc00000;
static mut HW_MEM: *mut u8 = 0 as *mut u8;


pub fn init_monitor(){
    println!("{}", "Hello World.".green());
}

pub fn load_entry(){
    let mut buf: Vec<u8> = Vec::new();
    unsafe{
        let inst_file = "inst.bin";
        let mut f = std::fs::File::open(inst_file).expect(format!("Can not open '{}'", inst_file).as_str());
        f.read_to_end(&mut buf).unwrap();
        ptr::copy_nonoverlapping(buf.as_ptr(), (DRAM[0][0][0][0] as *mut u8).offset((ENTRY_START & 0x1F_FF_FF_FF) as isize), buf.len());

        let data_file = "data.bin";
        let mut f = std::fs::File::open(data_file).expect(format!("Can not open '{}'", data_file).as_str());
        f.read_to_end(&mut buf).unwrap();
        ptr::copy_nonoverlapping(buf.as_ptr(), DRAM[0][0][0][0] as *mut u8, buf.len());
    }
}

pub unsafe fn restart(){
    init_monitor();

    clear_dram();

    load_entry();

    CPU.pc = ENTRY_START;
    CPU_STATE = CpuState::STOP;

    init_ddr3();
}