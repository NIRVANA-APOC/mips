use super::super::monitor::cpu_exec::{ASSEMBLY, CPU, PC_UPDATED};
use super::exec::INSTR;
use super::helper::INDEX_MASK;
use super::reg::REG_NAME;

pub fn j(pc: u32) {
    unsafe {
        let target = (pc & 0xF000_0000) | ((INSTR & INDEX_MASK) << 2);
        CPU.pc = target;
        PC_UPDATED = true;
        ASSEMBLY = format!("j     0x{:08x}", target);
    }
}

pub fn jal(pc: u32) {
    unsafe {
        let target = (pc & 0xF000_0000) | ((INSTR & INDEX_MASK) << 2);
        CPU.gpr.set_w(31, pc + 4);
        CPU.pc = target;
        PC_UPDATED = true;
        ASSEMBLY = format!("jal   0x{:08x}", target);
    }
}
