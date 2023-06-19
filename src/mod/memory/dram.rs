use std::{ptr, ops::BitAnd, usize, env::consts};
use colored::Colorize;

use crate::r#mod::cpu::r_type::add;
const BURST_LEN: usize = 8;
const BURST_MASK: usize = BURST_LEN - 1;

pub fn memcpy_with_mask(src: *const u8, dest: *mut u8, mask: *const u8, len: isize){
    unsafe{
            for i in 0..len{
            if mask.offset(i) as u8 != 0 {
                *dest.offset(i) = *src.offset(i) as u8;
            }
        }
    }
}

const COL_WIDTH: usize = 10;
const ROW_WIDTH: usize = 10;
const BANK_WIDTH: usize = 3;
const RANK_WIDTH: usize = 29 - COL_WIDTH - ROW_WIDTH - BANK_WIDTH;

struct DramAddr{
    addr: u32,
}

impl DramAddr {
    pub const fn new() -> Self{
        Self { addr: 0 }
    }

    pub fn col(&self) -> u32{
        self.addr & (NR_COL - 1) as u32
    }

    pub fn row(&self) -> u32{
        self.addr & (NR_ROW - 1) as u32
    }

    pub fn bank(&self) -> u32{
        self.addr & (NR_BANK - 1) as u32
    }

    pub fn rank(&self) -> u32{
        self.addr & (NR_RANK - 1) as u32
    }
}

const NR_COL: usize = 1 << COL_WIDTH;
const NR_ROW: usize = 1 << ROW_WIDTH;
const NR_BANK: usize = 1 << BANK_WIDTH;
const NR_RANK: usize = 1 << RANK_WIDTH;

const HW_MEM_SIZE: usize = 1 << (COL_WIDTH + ROW_WIDTH + BANK_WIDTH + RANK_WIDTH);

pub static mut DRAM: [[[[u8; NR_COL]; NR_ROW]; NR_BANK]; NR_RANK] = [[[[0; NR_COL]; NR_ROW]; NR_BANK]; NR_RANK];

#[derive(Clone, Copy)]
struct RB{
    buf: [u8; NR_COL],
    row_idx: i32,
    valid: bool,
}

impl RB {
    pub const fn new() -> Self{
        Self { buf: [0; NR_COL], row_idx: 0, valid: false }
    }
}

static mut ROW_BUFS: [[RB; NR_BANK]; NR_RANK] = [[RB::new(); NR_BANK]; NR_RANK];

pub fn init_ddr3(){
    for i in 0..NR_RANK{
        for j in 0..NR_BANK{
            unsafe{
                ROW_BUFS[i][j].valid = false;
            }
        }
    }
}

pub fn ddr3_read(addr: u32, data: *mut u8){
    assert!(addr < HW_MEM_SIZE as u32, "{}", format!("physical address {:08x} is outside of the physical memory!", addr).red());

    let mut temp = DramAddr::new();
    temp.addr = addr & !(BURST_MASK as u32);
    let rank = temp.rank() as usize;
    let bank = temp.bank() as usize;
    let row = temp.row() as usize;
    let col = temp.col() as usize;

    unsafe{
        let row_bufs = &ROW_BUFS[rank][bank];
        if !(row_bufs.valid && row_bufs.row_idx == row as i32){
            /* read a row into row buffer */
            ptr::copy_nonoverlapping(DRAM[rank][bank][row].as_ptr(), ROW_BUFS[rank][bank].buf.as_mut_ptr(), NR_COL);
            ROW_BUFS[rank][bank].row_idx = row as i32;
            ROW_BUFS[rank][bank].valid = true;
        }

        /* burst read */
        ptr::copy_nonoverlapping(ROW_BUFS[rank][bank].buf.as_ptr().offset(col as isize), data, BURST_LEN);
    }
}

pub fn ddr3_write(addr: u32, data: *const u8, mask: *const u8){
    assert!(addr < HW_MEM_SIZE as u32, "{}", format!("physical address {:08x} is outside of the physical memory!", addr).red());

    let mut temp = DramAddr::new();
    temp.addr = addr & !(BURST_MASK as u32);
    let rank = temp.rank() as usize;
    let bank = temp.bank() as usize;
    let row = temp.row() as usize;
    let col = temp.col() as usize;

    unsafe{
        let row_bufs = &ROW_BUFS[rank][bank];
        if !(row_bufs.valid && row_bufs.row_idx == row as i32){
            /* read a row into row buffer */
            ptr::copy_nonoverlapping(DRAM[rank][bank][row].as_ptr(), ROW_BUFS[rank][bank].buf.as_mut_ptr(), NR_COL);
            ROW_BUFS[rank][bank].row_idx = row as i32;
            ROW_BUFS[rank][bank].valid = true;
        }

        /* burst write */
        memcpy_with_mask(data, ROW_BUFS[rank][bank].buf.as_mut_ptr().offset(col as isize), mask, BURST_LEN as isize);

        /* write back to dram */
        ptr::copy_nonoverlapping(ROW_BUFS[rank][bank].buf.as_ptr(), DRAM[rank][bank][row].as_mut_ptr(), NR_COL)
    }
}

pub unsafe fn unalign_rw(addr: *const u8, len: usize) -> u32{
    // 输入一个u8的指针，将其转换为u32类型并按照len返回有效部分
    let addr = *(addr as *const u32);
    match len {
        1 => addr & 0x00_00_00_FF,
        2 => addr & 0x00_00_FF_FF,
        3 => addr & 0x00_FF_FF_FF,
        4 => addr & 0xFF_FF_FF_FF,
        _ => 0,
    }
}

pub fn dram_read(addr: u32, len: usize) -> u32{
    let offset = addr & BURST_MASK as u32;
    let mut temp: [u8; 2 * BURST_LEN] = [0; 2 * BURST_LEN];

    ddr3_read(addr, temp.as_mut_ptr());

    if offset as usize + len > BURST_LEN {
        unsafe{
            ddr3_read(addr + BURST_LEN as u32, temp.as_mut_ptr().offset(BURST_LEN as isize));
        }
    }
    unsafe{
        unalign_rw(temp.as_ptr().offset(offset as isize), len)
    }
}

pub fn dram_write(addr: u32, len: usize, data: u32){
    let offset = addr & BURST_MASK as u32;
    let mut temp: [u8; 2 * BURST_LEN] = [0; 2 * BURST_LEN];
    let mut mask: [u8; 2 * BURST_LEN] = [0; 2 * BURST_LEN];

    unsafe{
        *(temp.as_mut_ptr().offset(offset as isize) as *mut u32) = data;
        ptr::write_bytes(mask.as_mut_ptr().offset(offset as isize), 1, len);

        ddr3_write(addr, temp.as_ptr(), mask.as_ptr());

        if offset as usize + len > BURST_LEN {
            ddr3_write(addr + BURST_LEN as u32, temp.as_ptr().offset(BURST_LEN as isize), mask.as_ptr().offset(BURST_LEN as isize));
        }
    }
}

pub fn clear_dram(){
    unsafe{
        ptr::write_bytes(DRAM.as_mut_ptr(), 0, DRAM.len());
        ptr::write_bytes(ROW_BUFS.as_mut_ptr(), 0, ROW_BUFS.len());
        init_ddr3();
    }
}


mod test{
    use super::*;

    #[test]
    fn ptr_test(){
        let a = 114514_u32;
        let a_ptr = &a.to_ne_bytes().as_mut_ptr();

        unsafe{
            println!("1@ {}", *(*a_ptr as *const u32));
            unsafe fn modify(addr: *mut u8){
                // *(addr as *mut u32) = 1919810;
                let b = 1919810_u32;
                let b_ptr = b.to_ne_bytes().as_ptr();
                ptr::copy_nonoverlapping(b_ptr, addr, 4);
            }
            modify(*a_ptr);
            println!("2@ {}", *(*a_ptr as *const u32));
            println!("3@ {}", a);
        }
    }

    #[test]
    fn ddr3(){
        let addr = 0x1fc00000;
        let data: u32 = 0x12345678;
        let mask: u32 = 0x10101010;

        let data_ptr = data.to_ne_bytes().as_ptr();
        let mask_ptr = mask.to_ne_bytes().as_ptr();

        ddr3_write(addr, data_ptr, mask_ptr);
        
        let ret: u32 = 114514;
        let ret_ptr = ret.to_ne_bytes().as_mut_ptr();
        unsafe{
            println!("val is: {}", *(ret_ptr as *const u32));
        }
        ddr3_read(addr, ret_ptr);
        unsafe{
            println!("{:08x}", *(ret_ptr as *const u32));

            assert_eq!(*(ret_ptr as *const u32), data);
        }
    }

    #[test]
    fn dram(){
        let addr = 0x1fc00000;
        let data: u32 = 0x12345678;
        let mask: u32 = 0x10101010;

        let data_ptr = data.to_ne_bytes().as_ptr();
        let mask_ptr = mask.to_ne_bytes().as_ptr();

        // dram_write(addr, 1, data);
        dram_write(0x1fc00002, 1, data);
        
        let read_1 = dram_read(addr, 1);
        let read_2 = dram_read(addr, 2);
        let read_3 = dram_read(addr, 3);
        let read_4 = dram_read(addr, 4);

        println!("{:08x}", read_1);
        println!("{:08x}", read_2);
        println!("{:08x}", read_3);
        println!("{:08x}", read_4);
    }
}