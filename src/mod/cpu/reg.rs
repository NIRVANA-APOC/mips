enum REG {
    RZERO, AT, RV0, RV1, RA0, RA1, RA2, RA3, 
    RT0, RT1, RT2, RT3, RT4, RT5, RT6, RT7, 
    RS0, RS1, RS2, RS3, RS4, RS5, RS6, RS7, 
    RT8, RT9, RK0, RK1, RGP, RSP, RFP, RRA,
}

const REG_FILE: [&str; 32] = [
    "$zero", "$at", "$v0", "$v1", "$a0", "$a1", "$a2", "$a3", 
    "$t0", "$t1", "$t2", "$t3", "$t4", "$t5", "$t6", "$t7", 
    "$s0", "$s1", "$s2", "$s3", "$s4", "$s5", "$s6", "$s7", 
    "$t8", "$t9", "$k0", "$k1", "$gp", "$sp", "$fp", "$ra"
    ];

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct GPR{
    gpr: [u32; 32],
}

pub struct CPU{
    pub gpr: GPR,
    pub pc: u32,
    pub hi: u32,
    pub lo: u32,
}

impl CPU {
    pub const fn new() -> Self{
        Self { gpr: GPR::new(), pc: 0, hi: 0, lo: 0 }
    }
}

impl GPR {
    pub const fn new() -> Self{
        Self { gpr: [0; 32] }
    }

    fn check_index(idx: usize) -> usize{
        assert!(idx <= 31, "reg index out of bound");
        idx
    }

    pub fn reg_w(&self, reg: usize) -> u32{
        self.gpr[GPR::check_index(reg)]
    }

    pub fn reg_h(&self, reg: usize) -> u16{
        self.gpr[GPR::check_index(reg)] as u16
    }

    pub fn reg_b(&self, reg: usize) -> u8{
        self.gpr[GPR::check_index(reg)] as u8
    }
}


mod test{
    use super::*;

    #[test]
    fn gpr_init(){
        let gpr = GPR::new();
        assert_eq!(gpr.gpr, [0; 32]);
    }

    #[test]
    fn reg_test(){
        let mut gpr = GPR::new();
        let test_num = 0x12345678;
        let reg_id = 3;
        gpr.gpr[reg_id] = test_num;
        
        assert_eq!(gpr.reg_b(reg_id), test_num as u8);
        assert_eq!(gpr.reg_h(reg_id), test_num as u16);
        assert_eq!(gpr.reg_w(reg_id), test_num as u32);

        println!("{:#x}", gpr.reg_b(reg_id));
        println!("{:#x}", gpr.reg_h(reg_id));
        println!("{:#x}", gpr.reg_w(reg_id));
    }

}