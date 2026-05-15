/// 单条指令操作数。
#[derive(Debug, Clone, Copy)]
pub struct Operand {
    pub val: u32,
    reg_or_imm: u32,
}

impl Operand {
    pub const fn new() -> Self {
        Self {
            val: 0,
            reg_or_imm: 0,
        }
    }

    pub fn get_reg(&self) -> usize {
        self.reg_or_imm as usize
    }

    pub fn get_imm(&self) -> u32 {
        self.reg_or_imm
    }

    pub fn get_simm(&self) -> i32 {
        self.reg_or_imm as i32
    }

    pub fn set_reg(&mut self, reg: u32) {
        self.reg_or_imm = reg;
    }

    pub fn set_imm(&mut self, imm: u32) {
        self.reg_or_imm = imm;
    }
}

/// 解码后的操作数集合。
#[derive(Debug, Clone, Copy)]
pub struct Operands {
    pub opcode: u32,
    pub func: u32,
    pub src1: Operand,
    pub src2: Operand,
    pub dest: Operand,
}

impl Operands {
    pub const fn new() -> Self {
        Self {
            opcode: 0,
            func: 0,
            src1: Operand::new(),
            src2: Operand::new(),
            dest: Operand::new(),
        }
    }
}
