use super::super::memory::memory::{mem_read, mem_write};
use super::super::monitor::cpu_exec::{ASSEMBLY, CPU};
use super::exec::{INSTR, OPS_DECODED};
use super::helper::*;
use super::operand::{OPType, Operand};
use super::reg::REG_NAME;
use crate::emu::monitor::ui::dbg_println;
use colored::Colorize;

fn imm_extend(src: &mut Operand) {
    if src.val & 0x00008000 != 0 {
        src.val |= 0xFFFF0000;
    }
}

fn decode_imm_type() {
    unsafe {
        let instr = INSTR;
        OPS_DECODED.src1.ty = OPType::IMM;
        OPS_DECODED
            .src1
            .set_reg((instr & RS_MASK) >> (RT_SIZE + IMM_SIZE));
        OPS_DECODED.src1.val = CPU.gpr.reg_w(OPS_DECODED.src1.get_reg());

        OPS_DECODED.src2.ty = OPType::IMM;
        OPS_DECODED.src2.set_imm(instr & IMM_MASK);
        OPS_DECODED.src2.val = OPS_DECODED.src2.get_imm();

        OPS_DECODED.dest.ty = OPType::REG;
        OPS_DECODED.dest.set_reg((instr & RT_MASK) >> IMM_SIZE);

        dbg_println(format!(
            "[DEBUG] op_src1->val: 0x{:08x}, op_src2->val: 0x{:08x}",
            OPS_DECODED.src1.val, OPS_DECODED.src2.val
        ));
    }
}

pub fn addi(pc: u32) {
    decode_imm_type();
    unsafe {
        let a = OPS_DECODED.src1.val as i32;
        let b = OPS_DECODED.src2.get_simm();
        let result = a.wrapping_add(b);
        if ((a ^ result) & (b ^ result)) < 0 {
            println!("{}", "Arithmetic overflow in addi".red());
            super::exec::inv(pc);
            return;
        }
        CPU.gpr.set_w(OPS_DECODED.dest.get_reg(), result as u32);
        ASSEMBLY = format!(
            "addi  {},   {},   0x{:04x}",
            REG_NAME[OPS_DECODED.dest.get_reg()],
            REG_NAME[OPS_DECODED.src1.get_reg()],
            OPS_DECODED.src2.get_imm()
        );
    }
}

pub fn addiu(pc: u32) {
    decode_imm_type();
    unsafe {
        imm_extend(&mut OPS_DECODED.src2);
        CPU.gpr.set_w(
            OPS_DECODED.dest.get_reg(),
            OPS_DECODED.src1.val.wrapping_add(OPS_DECODED.src2.val),
        );
        ASSEMBLY = format!(
            "addiu {},   {},   0x{:04x}",
            REG_NAME[OPS_DECODED.dest.get_reg()],
            REG_NAME[OPS_DECODED.src1.get_reg()],
            OPS_DECODED.src2.get_imm()
        );
    }
}

pub fn slti(pc: u32) {
    decode_imm_type();
    unsafe {
        imm_extend(&mut OPS_DECODED.src2);
        let result = if (OPS_DECODED.src1.val as i32) < (OPS_DECODED.src2.val as i32) {
            1
        } else {
            0
        };
        CPU.gpr.set_w(OPS_DECODED.dest.get_reg(), result);
        ASSEMBLY = format!(
            "slti  {},   {},   0x{:04x}",
            REG_NAME[OPS_DECODED.dest.get_reg()],
            REG_NAME[OPS_DECODED.src1.get_reg()],
            OPS_DECODED.src2.get_imm()
        );
    }
}

pub fn sltiu(pc: u32) {
    decode_imm_type();
    unsafe {
        let result = if OPS_DECODED.src1.val < OPS_DECODED.src2.val {
            1
        } else {
            0
        };
        CPU.gpr.set_w(OPS_DECODED.dest.get_reg(), result);
        ASSEMBLY = format!(
            "sltiu {},   {},   0x{:04x}",
            REG_NAME[OPS_DECODED.dest.get_reg()],
            REG_NAME[OPS_DECODED.src1.get_reg()],
            OPS_DECODED.src2.get_imm()
        );
    }
}

pub fn andi(pc: u32) {
    decode_imm_type();
    unsafe {
        CPU.gpr.set_w(
            OPS_DECODED.dest.get_reg(),
            OPS_DECODED.src1.val & OPS_DECODED.src2.val,
        );
        ASSEMBLY = format!(
            "andi  {},   {},   0x{:04x}",
            REG_NAME[OPS_DECODED.dest.get_reg()],
            REG_NAME[OPS_DECODED.src1.get_reg()],
            OPS_DECODED.src2.get_imm()
        );
    }
}

pub fn lui(pc: u32) {
    decode_imm_type();
    unsafe {
        CPU.gpr
            .set_w(OPS_DECODED.dest.get_reg(), OPS_DECODED.src2.val << 16);
        ASSEMBLY = format!(
            "lui   {},   0x{:04x}",
            REG_NAME[OPS_DECODED.dest.get_reg()],
            OPS_DECODED.src2.get_imm()
        );
    }
}

pub fn ori(pc: u32) {
    decode_imm_type();
    unsafe {
        CPU.gpr.set_w(
            OPS_DECODED.dest.get_reg(),
            OPS_DECODED.src1.val | OPS_DECODED.src2.val,
        );
        ASSEMBLY = format!(
            "ori   {},   {},   0x{:04x}",
            REG_NAME[OPS_DECODED.dest.get_reg()],
            REG_NAME[OPS_DECODED.src1.get_reg()],
            OPS_DECODED.src2.get_imm()
        );
    }
}

pub fn xori(pc: u32) {
    decode_imm_type();
    unsafe {
        CPU.gpr.set_w(
            OPS_DECODED.dest.get_reg(),
            OPS_DECODED.src1.val ^ OPS_DECODED.src2.val,
        );
        ASSEMBLY = format!(
            "xori  {},   {},   0x{:04x}",
            REG_NAME[OPS_DECODED.dest.get_reg()],
            REG_NAME[OPS_DECODED.src1.get_reg()],
            OPS_DECODED.src2.get_imm()
        );
    }
}

fn mem_addr() -> u32 {
    unsafe {
        let base = OPS_DECODED.src1.val;
        let offset = OPS_DECODED.src2.get_simm() as u32;
        base.wrapping_add(offset)
    }
}

pub fn lb(pc: u32) {
    decode_imm_type();
    unsafe {
        let addr = mem_addr();
        let val = mem_read(addr, 1);
        let extended = if val & 0x80 != 0 { val | 0xFFFF_FF00 } else { val };
        CPU.gpr.set_w(OPS_DECODED.dest.get_reg(), extended);
        ASSEMBLY = format!(
            "lb    {},   {}({})",
            REG_NAME[OPS_DECODED.dest.get_reg()],
            OPS_DECODED.src2.get_simm(),
            REG_NAME[OPS_DECODED.src1.get_reg()]
        );
    }
}

pub fn lbu(pc: u32) {
    decode_imm_type();
    unsafe {
        let addr = mem_addr();
        let val = mem_read(addr, 1);
        CPU.gpr.set_w(OPS_DECODED.dest.get_reg(), val);
        ASSEMBLY = format!(
            "lbu   {},   {}({})",
            REG_NAME[OPS_DECODED.dest.get_reg()],
            OPS_DECODED.src2.get_simm(),
            REG_NAME[OPS_DECODED.src1.get_reg()]
        );
    }
}

pub fn lh(pc: u32) {
    decode_imm_type();
    unsafe {
        let addr = mem_addr();
        if addr % 2 != 0 {
            super::exec::inv(pc);
            return;
        }
        let val = mem_read(addr, 2);
        let extended = if val & 0x8000 != 0 { val | 0xFFFF_0000 } else { val };
        CPU.gpr.set_w(OPS_DECODED.dest.get_reg(), extended);
        ASSEMBLY = format!(
            "lh    {},   {}({})",
            REG_NAME[OPS_DECODED.dest.get_reg()],
            OPS_DECODED.src2.get_simm(),
            REG_NAME[OPS_DECODED.src1.get_reg()]
        );
    }
}

pub fn lhu(pc: u32) {
    decode_imm_type();
    unsafe {
        let addr = mem_addr();
        if addr % 2 != 0 {
            super::exec::inv(pc);
            return;
        }
        let val = mem_read(addr, 2);
        CPU.gpr.set_w(OPS_DECODED.dest.get_reg(), val);
        ASSEMBLY = format!(
            "lhu   {},   {}({})",
            REG_NAME[OPS_DECODED.dest.get_reg()],
            OPS_DECODED.src2.get_simm(),
            REG_NAME[OPS_DECODED.src1.get_reg()]
        );
    }
}

pub fn lw(pc: u32) {
    decode_imm_type();
    unsafe {
        let addr = mem_addr();
        if addr % 4 != 0 {
            super::exec::inv(pc);
            return;
        }
        let val = mem_read(addr, 4);
        CPU.gpr.set_w(OPS_DECODED.dest.get_reg(), val);
        ASSEMBLY = format!(
            "lw    {},   {}({})",
            REG_NAME[OPS_DECODED.dest.get_reg()],
            OPS_DECODED.src2.get_simm(),
            REG_NAME[OPS_DECODED.src1.get_reg()]
        );
    }
}

pub fn sb(pc: u32) {
    decode_imm_type();
    unsafe {
        let addr = mem_addr();
        let val = CPU.gpr.reg_w(OPS_DECODED.dest.get_reg());
        mem_write(addr, 1, val);
        ASSEMBLY = format!(
            "sb    {},   {}({})",
            REG_NAME[OPS_DECODED.dest.get_reg()],
            OPS_DECODED.src2.get_simm(),
            REG_NAME[OPS_DECODED.src1.get_reg()]
        );
    }
}

pub fn sh(pc: u32) {
    decode_imm_type();
    unsafe {
        let addr = mem_addr();
        if addr % 2 != 0 {
            super::exec::inv(pc);
            return;
        }
        let val = CPU.gpr.reg_w(OPS_DECODED.dest.get_reg());
        mem_write(addr, 2, val);
        ASSEMBLY = format!(
            "sh    {},   {}({})",
            REG_NAME[OPS_DECODED.dest.get_reg()],
            OPS_DECODED.src2.get_simm(),
            REG_NAME[OPS_DECODED.src1.get_reg()]
        );
    }
}

pub fn sw(pc: u32) {
    decode_imm_type();
    unsafe {
        let addr = mem_addr();
        if addr % 4 != 0 {
            super::exec::inv(pc);
            return;
        }
        let val = CPU.gpr.reg_w(OPS_DECODED.dest.get_reg());
        mem_write(addr, 4, val);
        ASSEMBLY = format!(
            "sw    {},   {}({})",
            REG_NAME[OPS_DECODED.dest.get_reg()],
            OPS_DECODED.src2.get_simm(),
            REG_NAME[OPS_DECODED.src1.get_reg()]
        );
    }
}

#[cfg(test)]
mod test {
    use std::ptr;

    use crate::emu::cpu::reg::CPU;
    use crate::emu::memory::dram::{clear_dram, init_ddr3, DRAM};
    use crate::emu::monitor::cpu_exec::{cpu_exec, CPU as CPU_GLOBAL, CPU_STATE, CpuState};
    use super::*;

    unsafe fn load_instructions(instructions: &[u32]) {
        clear_dram();
        init_ddr3();
        CPU_GLOBAL = CPU::new();
        CPU.hi = 0;
        CPU.lo = 0;
        let entry = 0xBFC00000;
        let dram_offset = (entry & 0x1F_FF_FF_FF) as isize;
        ptr::copy_nonoverlapping(
            instructions.as_ptr() as *const u8,
            (DRAM.as_mut_ptr() as *mut u8).offset(dram_offset),
            instructions.len() * 4,
        );
        CPU.pc = entry;
        CPU_STATE = CpuState::STOP;
    }

    unsafe fn set_reg(reg: usize, val: u32) {
        CPU_GLOBAL.gpr.set_w(reg, val);
    }

    #[test]
    fn test_addi() {
        unsafe {
            load_instructions(&[
                encode_i_type(0x08, 0, 8, 5), // addi $t0, $zero, 5
            ]);
            cpu_exec(1);
            assert_eq!(CPU_GLOBAL.gpr.reg_w(8), 5);
        }
    }

    #[test]
    fn test_addi_overflow() {
        unsafe {
            load_instructions(&[
                encode_i_type(0x08, 0, 8, 1), // addi $t0, $zero, 1
            ]);
            set_reg(0, 0x7FFFFFFF);
            cpu_exec(1);
            assert_eq!(CPU_STATE, CpuState::END);
        }
    }

    #[test]
    fn test_addiu() {
        unsafe {
            load_instructions(&[
                encode_i_type(0x09, 0, 8, 1), // addiu $t0, $zero, 1
            ]);
            set_reg(0, 0xFFFFFFFF);
            cpu_exec(1);
            assert_eq!(CPU_GLOBAL.gpr.reg_w(8), 0);
        }
    }

    #[test]
    fn test_slti() {
        unsafe {
            load_instructions(&[
                encode_i_type(0x0A, 0, 8, 5), // slti $t0, $zero, 5
            ]);
            cpu_exec(1);
            assert_eq!(CPU_GLOBAL.gpr.reg_w(8), 1);
        }
    }

    #[test]
    fn test_sltiu() {
        unsafe {
            load_instructions(&[
                encode_i_type(0x0B, 0, 8, 1), // sltiu $t0, $zero, 1
            ]);
            set_reg(0, 0xFFFFFFFF);
            cpu_exec(1);
            assert_eq!(CPU_GLOBAL.gpr.reg_w(8), 0);
        }
    }

    #[test]
    fn test_andi() {
        unsafe {
            load_instructions(&[
                encode_i_type(0x0C, 0, 8, 0xFF), // andi $t0, $zero, 0xFF
            ]);
            set_reg(0, 0xFFFF);
            cpu_exec(1);
            assert_eq!(CPU_GLOBAL.gpr.reg_w(8), 0xFF);
        }
    }

    #[test]
    fn test_ori() {
        unsafe {
            load_instructions(&[
                encode_i_type(0x0D, 0, 8, 0xFF), // ori $t0, $zero, 0xFF
            ]);
            set_reg(0, 0xFF00);
            cpu_exec(1);
            assert_eq!(CPU_GLOBAL.gpr.reg_w(8), 0xFFFF);
        }
    }

    #[test]
    fn test_xori() {
        unsafe {
            load_instructions(&[
                encode_i_type(0x0E, 0, 8, 0x0F), // xori $t0, $zero, 0x0F
            ]);
            set_reg(0, 0xFFFF);
            cpu_exec(1);
            assert_eq!(CPU_GLOBAL.gpr.reg_w(8), 0xFFF0);
        }
    }

    #[test]
    fn test_lui() {
        unsafe {
            load_instructions(&[
                encode_i_type(0x0F, 0, 8, 0x1234), // lui $t0, 0x1234
            ]);
            cpu_exec(1);
            assert_eq!(CPU_GLOBAL.gpr.reg_w(8), 0x12340000);
        }
    }

    #[test]
    fn test_sw_lw() {
        unsafe {
            load_instructions(&[
                encode_i_type(0x2B, 0, 8, 0), // sw $t0, 0($zero)
                encode_i_type(0x23, 0, 9, 0), // lw $t1, 0($zero)
            ]);
            set_reg(8, 0x12345678);
            cpu_exec(2);
            assert_eq!(CPU_GLOBAL.gpr.reg_w(9), 0x12345678);
        }
    }

    #[test]
    fn test_sb_lb() {
        unsafe {
            load_instructions(&[
                encode_i_type(0x28, 0, 8, 0), // sb $t0, 0($zero)
                encode_i_type(0x20, 0, 9, 0), // lb $t1, 0($zero)
            ]);
            set_reg(8, 0x80);
            cpu_exec(2);
            assert_eq!(CPU_GLOBAL.gpr.reg_w(9), 0xFFFFFF80);
        }
    }

    #[test]
    fn test_sb_lbu() {
        unsafe {
            load_instructions(&[
                encode_i_type(0x28, 0, 8, 0), // sb $t0, 0($zero)
                encode_i_type(0x24, 0, 9, 0), // lbu $t1, 0($zero)
            ]);
            set_reg(8, 0x80);
            cpu_exec(2);
            assert_eq!(CPU_GLOBAL.gpr.reg_w(9), 0x80);
        }
    }

    #[test]
    fn test_sh_lh() {
        unsafe {
            load_instructions(&[
                encode_i_type(0x29, 0, 8, 0), // sh $t0, 0($zero)
                encode_i_type(0x21, 0, 9, 0), // lh $t1, 0($zero)
            ]);
            set_reg(8, 0xFF00);
            cpu_exec(2);
            assert_eq!(CPU_GLOBAL.gpr.reg_w(9), 0xFFFF_FF00);
        }
    }

    #[test]
    fn test_sh_lhu() {
        unsafe {
            load_instructions(&[
                encode_i_type(0x29, 0, 8, 0), // sh $t0, 0($zero)
                encode_i_type(0x25, 0, 9, 0), // lhu $t1, 0($zero)
            ]);
            set_reg(8, 0xFF00);
            cpu_exec(2);
            assert_eq!(CPU_GLOBAL.gpr.reg_w(9), 0xFF00);
        }
    }

    #[test]
    fn test_lw_unaligned() {
        unsafe {
            load_instructions(&[
                encode_i_type(0x23, 0, 8, 1), // lw $t0, 1($zero)
            ]);
            cpu_exec(1);
            assert_eq!(CPU_STATE, CpuState::END);
        }
    }

    #[test]
    fn test_lh_unaligned() {
        unsafe {
            load_instructions(&[
                encode_i_type(0x21, 0, 8, 1), // lh $t0, 1($zero)
            ]);
            cpu_exec(1);
            assert_eq!(CPU_STATE, CpuState::END);
        }
    }
}
