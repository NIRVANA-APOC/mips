use super::{operand::Operands, i_type::*, j_type::*, r_type::*};
use super::super::monitor::cpu_exec::{CpuState, CPU_STATE};
use super::super::memory::memory::mem_read;
use colored::Colorize;

pub const FUNC_MASK: u32 = 0x0000003F;
pub const RS_MASK: u32 = 0x03E00000;
pub const RT_MASK: u32 = 0x001F0000;
pub const RD_MASK: u32 = 0x0000F800;
pub const SHAMT_MASK: u32 = 0x000007C0;
pub const IMM_MASK:u32 = 0x0000FFFF;
pub const INDEX_MASK:u32 = 0x03FFFFFF;

pub const OPCODE_SIZE: u32 = 6;
pub const FUNC_SIZE: u32 = 6;
pub const RS_SIZE: u32 = 5;
pub const RT_SIZE: u32 = 5;
pub const RD_SIZE: u32 = 5;
pub const SHAMT_SIZE: u32 = 5;
pub const IMM_SIZE: u32 = 16;
pub const INDEX_SIZE: u32 = 26;

pub static mut INSTR: u32 = 0;
pub static mut OPS_DECODED: Operands = Operands::new();

pub fn inv(pc: u32){
    // red
    let p = instr_fetch(pc, 4).to_be_bytes();
    println!("{}", format!("invalid opcode(pc = 0x{:08x}): {:02x} {:02x} {:02x} {:02x} ...",
    pc, p[3], p[2], p[1], p[0]).red());
    unsafe{
        CPU_STATE = CpuState::END;
    }
}

pub fn good_trap(pc: u32){
    // green
    println!("{}", format!("temu: HIT GOOD TRAP at $pc = 0x{:08x}", pc).green());
    unsafe{
        CPU_STATE = CpuState::END;
    }
}

pub fn bad_trap(pc: u32){
    // red
    println!("{}", format!("temu: HIT BAD TRAP at $pc = 0x{:08x}", pc).red());
    unsafe{
        CPU_STATE = CpuState::END;
    }
}

const OPCODE_TABLE: [fn(u32); 64] = [
/* 0x00 */	_2byte_esc, bz, j, jal,
/* 0x04 */	beq, bne, blez, bgtz,
/* 0x08 */	addi, addiu, slti, sltiu,
/* 0x0c */	andi, ori, xori, lui,
/* 0x10 */	eret, inv, good_trap, bad_trap,
/* 0x14 */	inv, inv, inv, inv,
/* 0x18 */	inv, inv, inv, inv,
/* 0x1c */	inv, inv, inv, inv,
/* 0x20 */	lb, lh, inv, lw,
/* 0x24 */	lbu, lhu, inv, inv,
/* 0x28 */	sb, sh, inv, sw,
/* 0x2c */	inv, inv, inv, inv,
/* 0x30 */	inv, inv, inv, inv,
/* 0x34 */	inv, inv, inv, inv,
/* 0x38 */	inv, inv, inv, inv,
/* 0x3c */	inv, inv, inv, inv
];

const _2BYTE_OPCODE_TABLE: [fn(u32); 64] = [
/* 0x00 */	sll, inv, srl, sra, 
/* 0x04 */	sllv, inv, srlv, srav, 
/* 0x08 */	jr, jalr, inv, inv, 
/* 0x0c */	syscall, _break, inv, inv, 
/* 0x10 */	mfhi, mthi, mflo, mtlo, 
/* 0x14 */	inv, inv, inv, inv, 
/* 0x18 */	mult, multu, div, divu, 
/* 0x1c */	inv, inv, inv, inv, 
/* 0x20 */	add, addu, sub, subu, 
/* 0x24 */	and, or, xor, nor,
/* 0x28 */	inv, inv, slt, sltu, 
/* 0x2c */	inv, inv, inv, inv, 
/* 0x30 */	inv, inv, inv, inv, 
/* 0x34 */	inv, inv, inv, inv,
/* 0x38 */	inv, inv, inv, inv, 
/* 0x3c */	inv, inv, inv, inv
];

pub fn instr_fetch(addr: u32, len: usize) -> u32{
    mem_read(addr, len)
}

pub fn exec(pc: u32){
    unsafe{
        INSTR = instr_fetch(pc, 4);
        OPS_DECODED.opcode = INSTR >> 26;
        println!("func1: {:02x}", OPS_DECODED.opcode);
        OPCODE_TABLE[OPS_DECODED.opcode as usize](pc);
    }
}

pub fn _2byte_esc(pc: u32){
    unsafe{
        OPS_DECODED.func = INSTR & FUNC_MASK;
        println!("func2: {:02x}", OPS_DECODED.func);
        _2BYTE_OPCODE_TABLE[OPS_DECODED.func as usize](pc);
    }
}

mod test{
    use super::*;
    use super::super::super::monitor::monitor::load_entry;
    #[test]
    fn inv_trap(){
        let pc = 0xbfc00000 & 0x1F_FF_FF_FF;
        load_entry();
        println!("0x{:08x}: {:08x}", pc, mem_read(pc, 4));
        inv(pc);
        good_trap(pc);
        bad_trap(pc);
    }
}