use colored::Colorize;

/// MIPS 通用寄存器名称表（$zero ~ $ra）。
pub const REG_NAME: [&str; 32] = [
    "$zero", "$at", "$v0", "$v1", "$a0", "$a1", "$a2", "$a3", "$t0", "$t1", "$t2", "$t3", "$t4",
    "$t5", "$t6", "$t7", "$s0", "$s1", "$s2", "$s3", "$s4", "$s5", "$s6", "$s7", "$t8", "$t9",
    "$k0", "$k1", "$gp", "$sp", "$fp", "$ra",
];

/// 将寄存器名称解析为索引，未找到时返回 32。
pub fn get_id(reg: &str) -> usize {
    for (id, name) in REG_NAME.iter().enumerate() {
        if *name == reg {
            return id;
        }
    }
    32
}

/// MIPS 通用寄存器组（32 × 32-bit）。
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Gpr {
    regs: [u32; 32],
}

impl Gpr {
    pub const fn new() -> Self {
        Self { regs: [0; 32] }
    }

    fn check_index(idx: usize) -> usize {
        if idx >= 32 {
            eprintln!("{}", "reg index out of bound".red());
            0
        } else {
            idx
        }
    }

    pub fn read(&self, reg: usize) -> u32 {
        self.regs[Self::check_index(reg)]
    }

    pub fn write(&mut self, reg: usize, val: u32) {
        let idx = Self::check_index(reg);
        // $zero 恒为 0，忽略写入
        if idx != 0 {
            self.regs[idx] = val;
        }
    }
}

/// MIPS CPU 寄存器状态（含 PC、HI/LO）。
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CpuRegs {
    pub gpr: Gpr,
    pub pc: u32,
    pub hi: u32,
    pub lo: u32,
}

impl CpuRegs {
    pub const fn new() -> Self {
        Self {
            gpr: Gpr::new(),
            pc: 0,
            hi: 0,
            lo: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gpr_init() {
        let gpr = Gpr::new();
        assert_eq!(gpr.regs, [0; 32]);
    }

    #[test]
    fn reg_test() {
        let mut gpr = Gpr::new();
        let test_num = 0x1234_5678;
        let reg_id = 3;
        gpr.regs[reg_id] = test_num;

        assert_eq!(gpr.read(reg_id), test_num);
    }

    #[test]
    fn zero_reg_is_immutable() {
        let mut gpr = Gpr::new();
        gpr.write(0, 0xDEAD_BEEF);
        assert_eq!(gpr.read(0), 0);
    }
}
