use std::{ptr, array};
use std::io::Read;
use std::os::windows::prelude::FileExt;
use colored::Colorize;
use crate::r#mod::memory::memory::mem_write;

use super::cpu_exec::{CpuState, CPU, CPU_STATE};
use super::super::memory::dram::{DRAM, init_ddr3, clear_dram, ddr3_write};

const ENTRY_START: u32 = 0xBF_C0_00_00;
static mut HW_MEM: *mut u8 = 0 as *mut u8;


pub fn init_monitor(){
    println!("{}", "Hello World.".green());
}

// pub fn vec2arr<T, const N: usize>(v: Vec<T>) -> [T; N]{
//     v.try_into().unwrap()
//     // v.try_into().unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
// }

pub fn memcpy(src: *const u8, dst: *mut u8, len: isize){
    unsafe{
        for i in 0..len {
            println!("{:02x} <- {:02x}", *dst.offset(i), *src.offset(i));
            *dst.offset(i) = *src.offset(i) as u8;
        }
    }
}

pub fn load_entry(){
    let mut buf: Vec<u8> = Vec::new();

    let inst_file = "./bin/inst.bin";
    let mut f = std::fs::File::open(inst_file).expect(format!("Can not open '{}'", inst_file).as_str());
    f.read_to_end(&mut buf).unwrap();
    unsafe{
        ptr::copy_nonoverlapping(buf.as_ptr(), (DRAM.as_mut_ptr() as *mut u8).offset((ENTRY_START & 0x1F_FF_FF_FF) as isize), buf.len());
        // ddr3_write(0x1fc00000, buf.as_ptr(), 0_u32.to_ne_bytes().as_ptr());
    }
        println!("load {}", inst_file.green());

        let data_file = "./bin/data.bin";
        let mut f = std::fs::File::open(data_file).expect(format!("Can not open '{}'", data_file).as_str());
        f.read_to_end(&mut buf).unwrap();
    unsafe{
        ptr::copy_nonoverlapping(buf.as_ptr(), DRAM.as_mut_ptr() as *mut u8, buf.len());
    }
    println!("load {}", data_file.green());
}

pub fn restart(){
    init_monitor();

    clear_dram();

    load_entry();

    unsafe{
        CPU.pc = ENTRY_START;
        CPU_STATE = CpuState::STOP;
    }

    init_ddr3();
}


mod test{
    use crate::r#mod::memory::memory::mem_read;

    use super::*;

    #[test]
    fn write(){
        unsafe{
            clear_dram();
            init_ddr3();
            let buf: Vec<u8> = vec![0x11, 0x45, 0x14];
            println!("{:p}", DRAM.as_mut_ptr());
            memcpy(buf.as_ptr(), (DRAM.as_mut_ptr() as *mut u8).offset(0x1fc00000), buf.len() as isize);
            println!("{:02x}", DRAM[0][0][0][0]);
            println!("{:02x}", DRAM[0][0][0][1]);
            println!("{:02x}", DRAM[0][0][0][2]);
            println!("{:02x}", DRAM[0][0][0][3]);
            println!("{:08x}", *((DRAM.as_ptr() as *const u8).offset(0x1fc00000) as *const u32));
            println!("{:08x}", mem_read(0x1fc00000, 4));

        }
    }
}