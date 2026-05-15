use super::super::memory::memory::mem_read;
use super::super::monitor::cpu_exec::{CpuState, CPU_STATE};
use super::helper::*;
use super::{i_type::*, j_type::*, operand::Operands, r_type::*};
use crate::emu::monitor::ui::dbg_println;
use colored::Colorize;

pub static mut INSTR: u32 = 0;
pub static mut OPS_DECODED: Operands = Operands::new();

pub fn inv(pc: u32) {
    // red
    let p = instr_fetch(pc, 4).to_be_bytes();
    println!(
        "{}",
        format!(
            "invalid opcode(pc = 0x{:08x}): {:02x} {:02x} {:02x} {:02x} ...",
            pc, p[3], p[2], p[1], p[0]
        )
        .red()
    );
    unsafe {
        CPU_STATE = CpuState::END;
    }
}

pub fn good_trap(pc: u32) {
    // green
    println!(
        "{}",
        format!("temu: HIT GOOD TRAP at $pc = 0x{:08x}", pc).green()
    );
    unsafe {
        CPU_STATE = CpuState::END;
    }
}

pub fn bad_trap(pc: u32) {
    // red
    println!(
        "{}",
        format!("temu: HIT BAD TRAP at $pc = 0x{:08x}", pc).red()
    );
    unsafe {
        CPU_STATE = CpuState::END;
    }
}

const OPCODE_TABLE: [fn(u32); 64] = [
    /* 0x00 */ _2byte_esc, bz, j, jal, /* 0x04 */ beq, bne, blez, bgtz,
    /* 0x08 */ addi, addiu, slti, sltiu, /* 0x0c */ andi, ori, xori, lui,
    /* 0x10 */ eret, inv, good_trap, bad_trap, /* 0x14 */ inv, inv, inv, inv,
    /* 0x18 */ inv, inv, inv, inv, /* 0x1c */ inv, inv, inv, inv, /* 0x20 */ lb, lh,
    inv, lw, /* 0x24 */ lbu, lhu, inv, inv, /* 0x28 */ sb, sh, inv, sw,
    /* 0x2c */ inv, inv, inv, inv, /* 0x30 */ inv, inv, inv, inv, /* 0x34 */ inv,
    inv, inv, inv, /* 0x38 */ inv, inv, inv, inv, /* 0x3c */ inv, inv, inv, inv,
];

const _2BYTE_OPCODE_TABLE: [fn(u32); 64] = [
    /* 0x00 */ sll, inv, srl, sra, /* 0x04 */ sllv, inv, srlv, srav, /* 0x08 */ jr,
    jalr, inv, inv, /* 0x0c */ syscall, _break, inv, inv, /* 0x10 */ mfhi, mthi, mflo,
    mtlo, /* 0x14 */ inv, inv, inv, inv, /* 0x18 */ mult, multu, div, divu,
    /* 0x1c */ inv, inv, inv, inv, /* 0x20 */ add, addu, sub, subu, /* 0x24 */ and,
    or, xor, nor, /* 0x28 */ inv, inv, slt, sltu, /* 0x2c */ inv, inv, inv, inv,
    /* 0x30 */ inv, inv, inv, inv, /* 0x34 */ inv, inv, inv, inv, /* 0x38 */ inv,
    inv, inv, inv, /* 0x3c */ inv, inv, inv, inv,
];

pub fn instr_fetch(addr: u32, len: usize) -> u32 {
    mem_read(addr, len)
}

pub fn exec(pc: u32) {
    unsafe {
        INSTR = instr_fetch(pc, 4);
        OPS_DECODED.opcode = INSTR >> 26;
        dbg_println(format!("[DEBUG] func1: {:02x}", OPS_DECODED.opcode));
        OPCODE_TABLE[OPS_DECODED.opcode as usize](pc);
    }
}

pub fn _2byte_esc(pc: u32) {
    unsafe {
        OPS_DECODED.func = INSTR & FUNC_MASK;
        dbg_println(format!("[DEBUG] func2: {:02x}", OPS_DECODED.func));
        _2BYTE_OPCODE_TABLE[OPS_DECODED.func as usize](pc);
    }
}

mod test {
    use std::ptr;

    use super::super::super::cpu::reg::CPU;
    use super::super::super::memory::dram::{clear_dram, init_ddr3, DRAM};
    use super::super::super::memory::memory::mem_write;
    use super::super::super::monitor::cpu_exec::{cpu_exec, CPU as CPU_GLOBAL, CPU_STATE, CpuState};
    use super::super::super::monitor::monitor::load_entry;
    use super::*;

    pub unsafe fn load_instructions(instructions: &[u32]) {
        clear_dram();
        init_ddr3();
        CPU_GLOBAL = CPU::new();
        CPU_GLOBAL.hi = 0;
        CPU_GLOBAL.lo = 0;
        let entry = 0xBFC00000;
        let dram_offset = (entry & 0x1F_FF_FF_FF) as isize;
        ptr::copy_nonoverlapping(
            instructions.as_ptr() as *const u8,
            (DRAM.as_mut_ptr() as *mut u8).offset(dram_offset),
            instructions.len() * 4,
        );
        CPU_GLOBAL.pc = entry;
        CPU_STATE = CpuState::STOP;
    }

    #[test]
    fn inv_trap() {
        let pc = 0xbfc00000 & 0x1F_FF_FF_FF;
        load_entry();
        println!("0x{:08x}: {:08x}", pc, mem_read(pc, 4));
        inv(pc);
        good_trap(pc);
        bad_trap(pc);
    }
}
