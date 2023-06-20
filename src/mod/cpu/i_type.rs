use super::exec::{INSTR, OPS_DECODED, RS_MASK, RT_MASK, RT_SIZE, IMM_MASK, IMM_SIZE};
use super::operand::OPType;
use super::reg::REG_NAME;
use super::super::monitor::cpu_exec::{CPU, ASSEMBLY};

fn decode_imm_type(){
    unsafe{
        let instr = INSTR;
        OPS_DECODED.src1.ty = OPType::IMM;
        OPS_DECODED.src1.set_reg((instr & RS_MASK) >> (RT_SIZE + IMM_SIZE));
        OPS_DECODED.src1.val = CPU.gpr.reg_w(OPS_DECODED.src1.get_reg());

        OPS_DECODED.src2.ty = OPType::IMM;
        OPS_DECODED.src2.set_imm(instr & IMM_MASK);
        OPS_DECODED.src2.val = OPS_DECODED.src2.get_imm();

        OPS_DECODED.dest.ty = OPType::REG;
        OPS_DECODED.dest.set_reg((instr & RT_MASK) >> IMM_SIZE);
    }
}

pub fn addi(pc: u32){

}

pub fn addiu(pc: u32){

}

pub fn slti(pc: u32){

}

pub fn sltiu(pc: u32){

}

pub fn andi(pc: u32){

}

pub fn lui(pc: u32){
    decode_imm_type();
    unsafe{
        CPU.gpr.set_w(OPS_DECODED.dest.get_reg(), OPS_DECODED.src2.val << 16);
        ASSEMBLY = format!("lui   {},   0x{:04x}", REG_NAME[OPS_DECODED.dest.get_reg()], OPS_DECODED.src2.get_imm());
    }
}

pub fn ori(pc: u32){

}

pub fn xori(pc: u32){

}

pub fn lb(pc: u32){

}

pub fn lbu(pc: u32){

}

pub fn lh(pc: u32){

}

pub fn lhu(pc: u32){

}

pub fn lw(pc: u32){

}

pub fn sb(pc: u32){

}

pub fn sh(pc: u32){

}

pub fn sw(pc: u32){

}
