use super::super::monitor::cpu_exec::{ASSEMBLY, CPU};
use super::exec::{INSTR, OPS_DECODED};
use super::helper::*;
use super::operand::OPType;
use super::reg::REG_NAME;
use crate::r#mod::monitor::ui::dbg_println;

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

pub fn add(pc: u32) {}

pub fn addu(pc: u32) {}

pub fn sub(pc: u32) {}

pub fn subu(pc: u32) {}

pub fn slt(pc: u32) {}

pub fn sltu(pc: u32) {}

pub fn div(pc: u32) {}

pub fn divu(pc: u32) {}

pub fn mult(pc: u32) {}

pub fn multu(pc: u32) {}

pub fn and(pc: u32) {
    decode_r_type();
    unsafe {
        CPU.gpr.set_w(
            OPS_DECODED.dest.get_reg(),
            OPS_DECODED.src1.val as u32 & OPS_DECODED.src2.val as u32,
        );
        ASSEMBLY = format!(
            "and   {},   {},   {}",
            REG_NAME[OPS_DECODED.dest.get_reg()],
            REG_NAME[OPS_DECODED.src1.get_reg()],
            REG_NAME[OPS_DECODED.src2.get_reg()]
        );
    }
}

pub fn nor(pc: u32) {}

pub fn or(pc: u32) {}

pub fn xor(pc: u32) {}

pub fn sllv(pc: u32) {}

pub fn sll(pc: u32) {}

pub fn srav(pc: u32) {}

pub fn sra(pc: u32) {}

pub fn srlv(pc: u32) {}

pub fn srl(pc: u32) {}

pub fn bz(pc: u32) {}

pub fn beq(pc: u32) {}

pub fn bne(pc: u32) {}

pub fn bgez(pc: u32) {}

pub fn bgtz(pc: u32) {}

pub fn gbtz(pc: u32) {}

pub fn blez(pc: u32) {}

pub fn bltz(pc: u32) {}

pub fn bgezal(pc: u32) {}

pub fn bltzal(pc: u32) {}

pub fn jr(pc: u32) {}

pub fn jalr(pc: u32) {}

pub fn mfhi(pc: u32) {}

pub fn mflo(pc: u32) {}

pub fn mthi(pc: u32) {}

pub fn mtlo(pc: u32) {}

pub fn _break(pc: u32) {}

pub fn syscall(pc: u32) {}

pub fn eret(pc: u32) {}

pub fn mfc0(pc: u32) {}

pub fn mtc0(pc: u32) {}
