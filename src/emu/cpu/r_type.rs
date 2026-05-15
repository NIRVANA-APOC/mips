use std::ptr;

use super::super::memory::dram::{clear_dram, init_ddr3, DRAM};
use super::super::monitor::cpu_exec::{ASSEMBLY, CPU, CPU_STATE, CpuState, PC_UPDATED};
use super::exec::{INSTR, OPS_DECODED};
use super::helper::*;
use super::operand::OPType;
use super::reg::REG_NAME;
use crate::emu::monitor::ui::dbg_println;
use colored::Colorize;

fn decode_r_type() {
    unsafe {
        let instr = INSTR;
        OPS_DECODED.src1.ty = OPType::REG;
        OPS_DECODED
            .src1
            .set_reg((instr & RS_MASK) >> (RT_SIZE + IMM_SIZE));
        OPS_DECODED.src1.val = CPU.gpr.reg_w(OPS_DECODED.src1.get_reg());

        OPS_DECODED.src2.ty = OPType::REG;
        OPS_DECODED
            .src2
            .set_imm((instr & RT_MASK) >> (RD_SIZE + SHAMT_SIZE + FUNC_SIZE));
        OPS_DECODED.src2.val = CPU.gpr.reg_w(OPS_DECODED.src2.get_reg());

        OPS_DECODED.dest.ty = OPType::REG;
        OPS_DECODED
            .dest
            .set_reg((instr & RD_MASK) >> (SHAMT_SIZE + FUNC_SIZE));

        dbg_println(format!(
            "[DEBUG] op_src1->val: 0x{:08x}, op_src2->val: 0x{:08x}",
            OPS_DECODED.src1.val, OPS_DECODED.src2.val
        ));
    }
}

fn decode_shift_imm() {
    unsafe {
        let instr = INSTR;
        OPS_DECODED.src1.ty = OPType::REG;
        OPS_DECODED
            .src1
            .set_reg((instr & RT_MASK) >> (RD_SIZE + SHAMT_SIZE + FUNC_SIZE));
        OPS_DECODED.src1.val = CPU.gpr.reg_w(OPS_DECODED.src1.get_reg());

        OPS_DECODED.src2.ty = OPType::IMM;
        OPS_DECODED.src2.set_imm((instr & SHAMT_MASK) >> FUNC_SIZE);
        OPS_DECODED.src2.val = OPS_DECODED.src2.get_imm();

        OPS_DECODED.dest.ty = OPType::REG;
        OPS_DECODED
            .dest
            .set_reg((instr & RD_MASK) >> (SHAMT_SIZE + FUNC_SIZE));

        dbg_println(format!(
            "[DEBUG] op_src1->val: 0x{:08x}, op_src2->val: 0x{:08x}",
            OPS_DECODED.src1.val, OPS_DECODED.src2.val
        ));
    }
}

fn decode_branch() {
    unsafe {
        let instr = INSTR;
        OPS_DECODED.src1.ty = OPType::REG;
        OPS_DECODED
            .src1
            .set_reg((instr & RS_MASK) >> (RT_SIZE + IMM_SIZE));
        OPS_DECODED.src1.val = CPU.gpr.reg_w(OPS_DECODED.src1.get_reg());

        OPS_DECODED.src2.ty = OPType::IMM;
        OPS_DECODED.src2.set_imm(instr & IMM_MASK);
        OPS_DECODED.src2.val = OPS_DECODED.src2.get_imm();

        dbg_println(format!(
            "[DEBUG] op_src1->val: 0x{:08x}, op_src2->val: 0x{:08x}",
            OPS_DECODED.src1.val, OPS_DECODED.src2.val
        ));
    }
}

fn branch_target(pc: u32) -> u32 {
    unsafe {
        let offset = OPS_DECODED.src2.get_simm() << 2;
        (pc as i32 + 4 + offset) as u32
    }
}

pub fn add(pc: u32) {
    decode_r_type();
    unsafe {
        let a = OPS_DECODED.src1.val as i32;
        let b = OPS_DECODED.src2.val as i32;
        let result = a.wrapping_add(b);
        if ((a ^ result) & (b ^ result)) < 0 {
            println!("{}", "Arithmetic overflow in add".red());
            CPU_STATE = CpuState::END;
            return;
        }
        CPU.gpr.set_w(OPS_DECODED.dest.get_reg(), result as u32);
        ASSEMBLY = format!(
            "add   {},   {},   {}",
            REG_NAME[OPS_DECODED.dest.get_reg()],
            REG_NAME[OPS_DECODED.src1.get_reg()],
            REG_NAME[OPS_DECODED.src2.get_reg()]
        );
    }
}

pub fn addu(pc: u32) {
    decode_r_type();
    unsafe {
        CPU.gpr.set_w(
            OPS_DECODED.dest.get_reg(),
            OPS_DECODED.src1.val.wrapping_add(OPS_DECODED.src2.val),
        );
        ASSEMBLY = format!(
            "addu  {},   {},   {}",
            REG_NAME[OPS_DECODED.dest.get_reg()],
            REG_NAME[OPS_DECODED.src1.get_reg()],
            REG_NAME[OPS_DECODED.src2.get_reg()]
        );
    }
}

pub fn sub(pc: u32) {
    decode_r_type();
    unsafe {
        let a = OPS_DECODED.src1.val as i32;
        let b = OPS_DECODED.src2.val as i32;
        let result = a.wrapping_sub(b);
        if ((a ^ b) & (a ^ result)) < 0 {
            println!("{}", "Arithmetic overflow in sub".red());
            CPU_STATE = CpuState::END;
            return;
        }
        CPU.gpr.set_w(OPS_DECODED.dest.get_reg(), result as u32);
        ASSEMBLY = format!(
            "sub   {},   {},   {}",
            REG_NAME[OPS_DECODED.dest.get_reg()],
            REG_NAME[OPS_DECODED.src1.get_reg()],
            REG_NAME[OPS_DECODED.src2.get_reg()]
        );
    }
}

pub fn subu(pc: u32) {
    decode_r_type();
    unsafe {
        CPU.gpr.set_w(
            OPS_DECODED.dest.get_reg(),
            OPS_DECODED.src1.val.wrapping_sub(OPS_DECODED.src2.val),
        );
        ASSEMBLY = format!(
            "subu  {},   {},   {}",
            REG_NAME[OPS_DECODED.dest.get_reg()],
            REG_NAME[OPS_DECODED.src1.get_reg()],
            REG_NAME[OPS_DECODED.src2.get_reg()]
        );
    }
}

pub fn slt(pc: u32) {
    decode_r_type();
    unsafe {
        let result = if (OPS_DECODED.src1.val as i32) < (OPS_DECODED.src2.val as i32) {
            1
        } else {
            0
        };
        CPU.gpr.set_w(OPS_DECODED.dest.get_reg(), result);
        ASSEMBLY = format!(
            "slt   {},   {},   {}",
            REG_NAME[OPS_DECODED.dest.get_reg()],
            REG_NAME[OPS_DECODED.src1.get_reg()],
            REG_NAME[OPS_DECODED.src2.get_reg()]
        );
    }
}

pub fn sltu(pc: u32) {
    decode_r_type();
    unsafe {
        let result = if OPS_DECODED.src1.val < OPS_DECODED.src2.val {
            1
        } else {
            0
        };
        CPU.gpr.set_w(OPS_DECODED.dest.get_reg(), result);
        ASSEMBLY = format!(
            "sltu  {},   {},   {}",
            REG_NAME[OPS_DECODED.dest.get_reg()],
            REG_NAME[OPS_DECODED.src1.get_reg()],
            REG_NAME[OPS_DECODED.src2.get_reg()]
        );
    }
}

pub fn div(pc: u32) {
    decode_r_type();
    unsafe {
        let rs = OPS_DECODED.src1.val as i32;
        let rt = OPS_DECODED.src2.val as i32;
        if rt == 0 {
            println!("{}", "Divide by zero in div".red());
            CPU_STATE = CpuState::END;
            return;
        }
        CPU.lo = (rs / rt) as u32;
        CPU.hi = (rs % rt) as u32;
        ASSEMBLY = format!(
            "div   {},   {}",
            REG_NAME[OPS_DECODED.src1.get_reg()],
            REG_NAME[OPS_DECODED.src2.get_reg()]
        );
    }
}

pub fn divu(pc: u32) {
    decode_r_type();
    unsafe {
        let rs = OPS_DECODED.src1.val;
        let rt = OPS_DECODED.src2.val;
        if rt == 0 {
            println!("{}", "Divide by zero in divu".red());
            CPU_STATE = CpuState::END;
            return;
        }
        CPU.lo = rs / rt;
        CPU.hi = rs % rt;
        ASSEMBLY = format!(
            "divu  {},   {}",
            REG_NAME[OPS_DECODED.src1.get_reg()],
            REG_NAME[OPS_DECODED.src2.get_reg()]
        );
    }
}

pub fn mult(pc: u32) {
    decode_r_type();
    unsafe {
        let result = (OPS_DECODED.src1.val as i64) * (OPS_DECODED.src2.val as i64);
        CPU.lo = result as u32;
        CPU.hi = (result >> 32) as u32;
        ASSEMBLY = format!(
            "mult  {},   {}",
            REG_NAME[OPS_DECODED.src1.get_reg()],
            REG_NAME[OPS_DECODED.src2.get_reg()]
        );
    }
}

pub fn multu(pc: u32) {
    decode_r_type();
    unsafe {
        let result = (OPS_DECODED.src1.val as u64) * (OPS_DECODED.src2.val as u64);
        CPU.lo = result as u32;
        CPU.hi = (result >> 32) as u32;
        ASSEMBLY = format!(
            "multu {},   {}",
            REG_NAME[OPS_DECODED.src1.get_reg()],
            REG_NAME[OPS_DECODED.src2.get_reg()]
        );
    }
}

pub fn and(pc: u32) {
    decode_r_type();
    unsafe {
        CPU.gpr.set_w(
            OPS_DECODED.dest.get_reg(),
            OPS_DECODED.src1.val & OPS_DECODED.src2.val,
        );
        ASSEMBLY = format!(
            "and   {},   {},   {}",
            REG_NAME[OPS_DECODED.dest.get_reg()],
            REG_NAME[OPS_DECODED.src1.get_reg()],
            REG_NAME[OPS_DECODED.src2.get_reg()]
        );
    }
}

pub fn nor(pc: u32) {
    decode_r_type();
    unsafe {
        CPU.gpr.set_w(
            OPS_DECODED.dest.get_reg(),
            !(OPS_DECODED.src1.val | OPS_DECODED.src2.val),
        );
        ASSEMBLY = format!(
            "nor   {},   {},   {}",
            REG_NAME[OPS_DECODED.dest.get_reg()],
            REG_NAME[OPS_DECODED.src1.get_reg()],
            REG_NAME[OPS_DECODED.src2.get_reg()]
        );
    }
}

pub fn or(pc: u32) {
    decode_r_type();
    unsafe {
        CPU.gpr.set_w(
            OPS_DECODED.dest.get_reg(),
            OPS_DECODED.src1.val | OPS_DECODED.src2.val,
        );
        ASSEMBLY = format!(
            "or    {},   {},   {}",
            REG_NAME[OPS_DECODED.dest.get_reg()],
            REG_NAME[OPS_DECODED.src1.get_reg()],
            REG_NAME[OPS_DECODED.src2.get_reg()]
        );
    }
}

pub fn xor(pc: u32) {
    decode_r_type();
    unsafe {
        CPU.gpr.set_w(
            OPS_DECODED.dest.get_reg(),
            OPS_DECODED.src1.val ^ OPS_DECODED.src2.val,
        );
        ASSEMBLY = format!(
            "xor   {},   {},   {}",
            REG_NAME[OPS_DECODED.dest.get_reg()],
            REG_NAME[OPS_DECODED.src1.get_reg()],
            REG_NAME[OPS_DECODED.src2.get_reg()]
        );
    }
}

pub fn sllv(pc: u32) {
    decode_r_type();
    unsafe {
        let shamt = OPS_DECODED.src1.val & 0x1F;
        CPU.gpr.set_w(
            OPS_DECODED.dest.get_reg(),
            OPS_DECODED.src2.val << shamt,
        );
        ASSEMBLY = format!(
            "sllv  {},   {},   {}",
            REG_NAME[OPS_DECODED.dest.get_reg()],
            REG_NAME[OPS_DECODED.src2.get_reg()],
            REG_NAME[OPS_DECODED.src1.get_reg()]
        );
    }
}

pub fn sll(pc: u32) {
    decode_shift_imm();
    unsafe {
        CPU.gpr.set_w(
            OPS_DECODED.dest.get_reg(),
            OPS_DECODED.src1.val << OPS_DECODED.src2.val,
        );
        ASSEMBLY = format!(
            "sll   {},   {},   {}",
            REG_NAME[OPS_DECODED.dest.get_reg()],
            REG_NAME[OPS_DECODED.src1.get_reg()],
            OPS_DECODED.src2.val
        );
    }
}

pub fn srav(pc: u32) {
    decode_r_type();
    unsafe {
        let shamt = OPS_DECODED.src1.val & 0x1F;
        CPU.gpr.set_w(
            OPS_DECODED.dest.get_reg(),
            (OPS_DECODED.src2.val as i32 >> shamt) as u32,
        );
        ASSEMBLY = format!(
            "srav  {},   {},   {}",
            REG_NAME[OPS_DECODED.dest.get_reg()],
            REG_NAME[OPS_DECODED.src2.get_reg()],
            REG_NAME[OPS_DECODED.src1.get_reg()]
        );
    }
}

pub fn sra(pc: u32) {
    decode_shift_imm();
    unsafe {
        CPU.gpr.set_w(
            OPS_DECODED.dest.get_reg(),
            (OPS_DECODED.src1.val as i32 >> OPS_DECODED.src2.val) as u32,
        );
        ASSEMBLY = format!(
            "sra   {},   {},   {}",
            REG_NAME[OPS_DECODED.dest.get_reg()],
            REG_NAME[OPS_DECODED.src1.get_reg()],
            OPS_DECODED.src2.val
        );
    }
}

pub fn srlv(pc: u32) {
    decode_r_type();
    unsafe {
        let shamt = OPS_DECODED.src1.val & 0x1F;
        CPU.gpr.set_w(
            OPS_DECODED.dest.get_reg(),
            OPS_DECODED.src2.val >> shamt,
        );
        ASSEMBLY = format!(
            "srlv  {},   {},   {}",
            REG_NAME[OPS_DECODED.dest.get_reg()],
            REG_NAME[OPS_DECODED.src2.get_reg()],
            REG_NAME[OPS_DECODED.src1.get_reg()]
        );
    }
}

pub fn srl(pc: u32) {
    decode_shift_imm();
    unsafe {
        CPU.gpr.set_w(
            OPS_DECODED.dest.get_reg(),
            OPS_DECODED.src1.val >> OPS_DECODED.src2.val,
        );
        ASSEMBLY = format!(
            "srl   {},   {},   {}",
            REG_NAME[OPS_DECODED.dest.get_reg()],
            REG_NAME[OPS_DECODED.src1.get_reg()],
            OPS_DECODED.src2.val
        );
    }
}

pub fn bz(pc: u32) {
    unsafe {
        let rt = (INSTR & RT_MASK) >> (RD_SIZE + SHAMT_SIZE + FUNC_SIZE);
        match rt {
            0x00 => bltz(pc),
            0x01 => bgez(pc),
            0x10 => bltzal(pc),
            0x11 => bgezal(pc),
            _ => {
                println!("{}", format!("Unknown bz/rt = {}", rt).red());
                CPU_STATE = CpuState::END;
            }
        }
    }
}

pub fn beq(pc: u32) {
    decode_branch();
    unsafe {
        if OPS_DECODED.src1.val == CPU.gpr.reg_w(OPS_DECODED.src2.get_reg()) {
            CPU.pc = branch_target(pc);
            PC_UPDATED = true;
        }
        ASSEMBLY = format!(
            "beq   {},   {},   0x{:08x}",
            REG_NAME[OPS_DECODED.src1.get_reg()],
            REG_NAME[OPS_DECODED.src2.get_reg()],
            branch_target(pc)
        );
    }
}

pub fn bne(pc: u32) {
    decode_branch();
    unsafe {
        if OPS_DECODED.src1.val != CPU.gpr.reg_w(OPS_DECODED.src2.get_reg()) {
            CPU.pc = branch_target(pc);
            PC_UPDATED = true;
        }
        ASSEMBLY = format!(
            "bne   {},   {},   0x{:08x}",
            REG_NAME[OPS_DECODED.src1.get_reg()],
            REG_NAME[OPS_DECODED.src2.get_reg()],
            branch_target(pc)
        );
    }
}

pub fn bgez(pc: u32) {
    decode_branch();
    unsafe {
        if (OPS_DECODED.src1.val as i32) >= 0 {
            CPU.pc = branch_target(pc);
            PC_UPDATED = true;
        }
        ASSEMBLY = format!(
            "bgez  {},   0x{:08x}",
            REG_NAME[OPS_DECODED.src1.get_reg()],
            branch_target(pc)
        );
    }
}

pub fn bgtz(pc: u32) {
    decode_branch();
    unsafe {
        if (OPS_DECODED.src1.val as i32) > 0 {
            CPU.pc = branch_target(pc);
            PC_UPDATED = true;
        }
        ASSEMBLY = format!(
            "bgtz  {},   0x{:08x}",
            REG_NAME[OPS_DECODED.src1.get_reg()],
            branch_target(pc)
        );
    }
}

pub fn gbtz(pc: u32) {}

pub fn blez(pc: u32) {
    decode_branch();
    unsafe {
        if (OPS_DECODED.src1.val as i32) <= 0 {
            CPU.pc = branch_target(pc);
            PC_UPDATED = true;
        }
        ASSEMBLY = format!(
            "blez  {},   0x{:08x}",
            REG_NAME[OPS_DECODED.src1.get_reg()],
            branch_target(pc)
        );
    }
}

pub fn bltz(pc: u32) {
    decode_branch();
    unsafe {
        if (OPS_DECODED.src1.val as i32) < 0 {
            CPU.pc = branch_target(pc);
            PC_UPDATED = true;
        }
        ASSEMBLY = format!(
            "bltz  {},   0x{:08x}",
            REG_NAME[OPS_DECODED.src1.get_reg()],
            branch_target(pc)
        );
    }
}

pub fn bgezal(pc: u32) {
    decode_branch();
    unsafe {
        CPU.gpr.set_w(31, pc + 4);
        if (OPS_DECODED.src1.val as i32) >= 0 {
            CPU.pc = branch_target(pc);
            PC_UPDATED = true;
        }
        ASSEMBLY = format!(
            "bgezal {},   0x{:08x}",
            REG_NAME[OPS_DECODED.src1.get_reg()],
            branch_target(pc)
        );
    }
}

pub fn bltzal(pc: u32) {
    decode_branch();
    unsafe {
        CPU.gpr.set_w(31, pc + 4);
        if (OPS_DECODED.src1.val as i32) < 0 {
            CPU.pc = branch_target(pc);
            PC_UPDATED = true;
        }
        ASSEMBLY = format!(
            "bltzal {},   0x{:08x}",
            REG_NAME[OPS_DECODED.src1.get_reg()],
            branch_target(pc)
        );
    }
}

pub fn jr(pc: u32) {
    decode_r_type();
    unsafe {
        CPU.pc = OPS_DECODED.src1.val;
        PC_UPDATED = true;
        ASSEMBLY = format!("jr    {}", REG_NAME[OPS_DECODED.src1.get_reg()]);
    }
}

pub fn jalr(pc: u32) {
    decode_r_type();
    unsafe {
        let target = OPS_DECODED.src1.val;
        let rd = OPS_DECODED.dest.get_reg();
        CPU.gpr.set_w(rd, pc + 4);
        CPU.pc = target;
        PC_UPDATED = true;
        ASSEMBLY = format!(
            "jalr  {},   {}",
            REG_NAME[OPS_DECODED.dest.get_reg()],
            REG_NAME[OPS_DECODED.src1.get_reg()]
        );
    }
}

pub fn mfhi(pc: u32) {
    decode_r_type();
    unsafe {
        CPU.gpr.set_w(OPS_DECODED.dest.get_reg(), CPU.hi);
        ASSEMBLY = format!("mfhi  {}", REG_NAME[OPS_DECODED.dest.get_reg()]);
    }
}

pub fn mflo(pc: u32) {
    decode_r_type();
    unsafe {
        CPU.gpr.set_w(OPS_DECODED.dest.get_reg(), CPU.lo);
        ASSEMBLY = format!("mflo  {}", REG_NAME[OPS_DECODED.dest.get_reg()]);
    }
}

pub fn mthi(pc: u32) {
    decode_r_type();
    unsafe {
        CPU.hi = OPS_DECODED.src1.val;
        ASSEMBLY = format!("mthi  {}", REG_NAME[OPS_DECODED.src1.get_reg()]);
    }
}

pub fn mtlo(pc: u32) {
    decode_r_type();
    unsafe {
        CPU.lo = OPS_DECODED.src1.val;
        ASSEMBLY = format!("mtlo  {}", REG_NAME[OPS_DECODED.src1.get_reg()]);
    }
}

pub fn _break(pc: u32) {
    unsafe {
        println!("{}", format!("Breakpoint at $pc = 0x{:08x}", pc).red());
        CPU_STATE = CpuState::END;
        ASSEMBLY = "break".to_string();
    }
}

pub fn syscall(pc: u32) {
    unsafe {
        println!("{}", format!("Syscall at $pc = 0x{:08x}", pc).yellow());
        CPU_STATE = CpuState::END;
        ASSEMBLY = "syscall".to_string();
    }
}

pub fn eret(pc: u32) {
    unsafe {
        println!("{}", format!("eret at $pc = 0x{:08x}", pc).yellow());
        CPU_STATE = CpuState::END;
        ASSEMBLY = "eret".to_string();
    }
}

pub fn mfc0(pc: u32) {}

pub fn mtc0(pc: u32) {}

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
    fn test_add() {
        unsafe {
            load_instructions(&[
                encode_r_type(8, 9, 10, 0, 0x20), // add $t2, $t0, $t1
                encode_r_type(0, 0, 0, 0, 0),     // nop (sll $zero, $zero, 0)
            ]);
            set_reg(8, 5);
            set_reg(9, 3);
            cpu_exec(1);
            assert_eq!(CPU_GLOBAL.gpr.reg_w(10), 8);
        }
    }

    #[test]
    fn test_add_overflow() {
        unsafe {
            load_instructions(&[
                encode_r_type(8, 9, 10, 0, 0x20), // add $t2, $t0, $t1
            ]);
            set_reg(8, 0x7FFFFFFF);
            set_reg(9, 1);
            cpu_exec(1);
            assert_eq!(CPU_STATE, CpuState::END);
        }
    }

    #[test]
    fn test_addu() {
        unsafe {
            load_instructions(&[
                encode_r_type(8, 9, 10, 0, 0x21), // addu $t2, $t0, $t1
            ]);
            set_reg(8, 0xFFFFFFFF);
            set_reg(9, 1);
            cpu_exec(1);
            assert_eq!(CPU_GLOBAL.gpr.reg_w(10), 0);
        }
    }

    #[test]
    fn test_sub() {
        unsafe {
            load_instructions(&[
                encode_r_type(8, 9, 10, 0, 0x22), // sub $t2, $t0, $t1
            ]);
            set_reg(8, 8);
            set_reg(9, 3);
            cpu_exec(1);
            assert_eq!(CPU_GLOBAL.gpr.reg_w(10), 5);
        }
    }

    #[test]
    fn test_subu() {
        unsafe {
            load_instructions(&[
                encode_r_type(8, 9, 10, 0, 0x23), // subu $t2, $t0, $t1
            ]);
            set_reg(8, 0);
            set_reg(9, 1);
            cpu_exec(1);
            assert_eq!(CPU_GLOBAL.gpr.reg_w(10), 0xFFFFFFFF);
        }
    }

    #[test]
    fn test_and() {
        unsafe {
            load_instructions(&[
                encode_r_type(8, 9, 10, 0, 0x24), // and $t2, $t0, $t1
            ]);
            set_reg(8, 0xFF00);
            set_reg(9, 0x0FF0);
            cpu_exec(1);
            assert_eq!(CPU_GLOBAL.gpr.reg_w(10), 0x0F00);
        }
    }

    #[test]
    fn test_or() {
        unsafe {
            load_instructions(&[
                encode_r_type(8, 9, 10, 0, 0x25), // or $t2, $t0, $t1
            ]);
            set_reg(8, 0xFF00);
            set_reg(9, 0x00FF);
            cpu_exec(1);
            assert_eq!(CPU_GLOBAL.gpr.reg_w(10), 0xFFFF);
        }
    }

    #[test]
    fn test_xor() {
        unsafe {
            load_instructions(&[
                encode_r_type(8, 9, 10, 0, 0x26), // xor $t2, $t0, $t1
            ]);
            set_reg(8, 0xFFFF);
            set_reg(9, 0x0F0F);
            cpu_exec(1);
            assert_eq!(CPU_GLOBAL.gpr.reg_w(10), 0xF0F0);
        }
    }

    #[test]
    fn test_nor() {
        unsafe {
            load_instructions(&[
                encode_r_type(8, 9, 10, 0, 0x27), // nor $t2, $t0, $t1
            ]);
            set_reg(8, 0xFF00);
            set_reg(9, 0x00FF);
            cpu_exec(1);
            assert_eq!(CPU_GLOBAL.gpr.reg_w(10), 0xFFFF0000);
        }
    }

    #[test]
    fn test_slt() {
        unsafe {
            load_instructions(&[
                encode_r_type(8, 9, 10, 0, 0x2A), // slt $t2, $t0, $t1
            ]);
            set_reg(8, 3);
            set_reg(9, 5);
            cpu_exec(1);
            assert_eq!(CPU_GLOBAL.gpr.reg_w(10), 1);

            load_instructions(&[
                encode_r_type(8, 9, 10, 0, 0x2A), // slt $t2, $t0, $t1
            ]);
            set_reg(8, 5);
            set_reg(9, 3);
            cpu_exec(1);
            assert_eq!(CPU_GLOBAL.gpr.reg_w(10), 0);
        }
    }

    #[test]
    fn test_sltu() {
        unsafe {
            load_instructions(&[
                encode_r_type(8, 9, 10, 0, 0x2B), // sltu $t2, $t0, $t1
            ]);
            set_reg(8, 0xFFFFFFFF);
            set_reg(9, 1);
            cpu_exec(1);
            assert_eq!(CPU_GLOBAL.gpr.reg_w(10), 0);
        }
    }

    #[test]
    fn test_sll() {
        unsafe {
            load_instructions(&[
                encode_r_type(0, 8, 10, 4, 0x00), // sll $t2, $t0, 4
            ]);
            set_reg(8, 0x01);
            cpu_exec(1);
            assert_eq!(CPU_GLOBAL.gpr.reg_w(10), 0x10);
        }
    }

    #[test]
    fn test_srl() {
        unsafe {
            load_instructions(&[
                encode_r_type(0, 8, 10, 4, 0x02), // srl $t2, $t0, 4
            ]);
            set_reg(8, 0x10);
            cpu_exec(1);
            assert_eq!(CPU_GLOBAL.gpr.reg_w(10), 0x01);
        }
    }

    #[test]
    fn test_sra() {
        unsafe {
            load_instructions(&[
                encode_r_type(0, 8, 10, 4, 0x03), // sra $t2, $t0, 4
            ]);
            set_reg(8, 0x80000000);
            cpu_exec(1);
            assert_eq!(CPU_GLOBAL.gpr.reg_w(10), 0xF8000000);
        }
    }

    #[test]
    fn test_sllv() {
        unsafe {
            load_instructions(&[
                encode_r_type(9, 8, 10, 0, 0x04), // sllv $t2, $t0, $t1
            ]);
            set_reg(8, 0x01);
            set_reg(9, 4);
            cpu_exec(1);
            assert_eq!(CPU_GLOBAL.gpr.reg_w(10), 0x10);
        }
    }

    #[test]
    fn test_srlv() {
        unsafe {
            load_instructions(&[
                encode_r_type(9, 8, 10, 0, 0x06), // srlv $t2, $t0, $t1
            ]);
            set_reg(8, 0x10);
            set_reg(9, 4);
            cpu_exec(1);
            assert_eq!(CPU_GLOBAL.gpr.reg_w(10), 0x01);
        }
    }

    #[test]
    fn test_srav() {
        unsafe {
            load_instructions(&[
                encode_r_type(9, 8, 10, 0, 0x07), // srav $t2, $t0, $t1
            ]);
            set_reg(8, 0x80000000);
            set_reg(9, 4);
            cpu_exec(1);
            assert_eq!(CPU_GLOBAL.gpr.reg_w(10), 0xF8000000);
        }
    }

    #[test]
    fn test_mult() {
        unsafe {
            load_instructions(&[
                encode_r_type(8, 9, 0, 0, 0x18), // mult $t0, $t1
            ]);
            set_reg(8, 3);
            set_reg(9, 5);
            cpu_exec(1);
            assert_eq!(CPU_GLOBAL.lo, 15);
            assert_eq!(CPU_GLOBAL.hi, 0);
        }
    }

    #[test]
    fn test_div() {
        unsafe {
            load_instructions(&[
                encode_r_type(8, 9, 0, 0, 0x1A), // div $t0, $t1
            ]);
            set_reg(8, 7);
            set_reg(9, 2);
            cpu_exec(1);
            assert_eq!(CPU_GLOBAL.lo, 3);
            assert_eq!(CPU_GLOBAL.hi, 1);
        }
    }

    #[test]
    fn test_div_by_zero() {
        unsafe {
            load_instructions(&[
                encode_r_type(8, 9, 0, 0, 0x1A), // div $t0, $t1
            ]);
            set_reg(8, 7);
            set_reg(9, 0);
            cpu_exec(1);
            assert_eq!(CPU_STATE, CpuState::END);
        }
    }

    #[test]
    fn test_mfhi_mflo() {
        unsafe {
            load_instructions(&[
                encode_r_type(8, 9, 0, 0, 0x18), // mult $t0, $t1
                encode_r_type(0, 0, 10, 0, 0x10), // mfhi $t2
                encode_r_type(0, 0, 11, 0, 0x12), // mflo $t3
            ]);
            set_reg(8, 0x10000);
            set_reg(9, 0x10000);
            cpu_exec(3);
            assert_eq!(CPU_GLOBAL.gpr.reg_w(10), 0x00000001);
            assert_eq!(CPU_GLOBAL.gpr.reg_w(11), 0x00000000);
        }
    }

    #[test]
    fn test_jr() {
        unsafe {
            load_instructions(&[
                encode_r_type(8, 0, 0, 0, 0x08), // jr $t0
            ]);
            set_reg(8, 0xBFC00010);
            cpu_exec(1);
            assert_eq!(CPU_GLOBAL.pc, 0xBFC00010);
        }
    }

    #[test]
    fn test_jalr() {
        unsafe {
            load_instructions(&[
                encode_r_type(8, 0, 10, 0, 0x09), // jalr $t2, $t0
            ]);
            set_reg(8, 0xBFC00010);
            cpu_exec(1);
            assert_eq!(CPU_GLOBAL.pc, 0xBFC00010);
            assert_eq!(CPU_GLOBAL.gpr.reg_w(10), 0x1FC00004);
        }
    }
}
