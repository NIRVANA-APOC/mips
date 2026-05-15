pub enum OPType {
    REG,
    IMM,
    JUMP,
}

const OP_STR_SIZE: usize = 40;

pub struct Operand {
    pub ty: OPType,
    reg_or_imm: u32,
    pub val: u32,
}

impl Operand {
    pub const fn new() -> Self {
        Self {
            ty: OPType::IMM,
            reg_or_imm: 0,
            val: 0,
        }
    }

    pub fn reg(&mut self) -> u32{
        self.reg_or_imm
    }

    pub fn imm(&mut self) -> u32{
        self.reg_or_imm
    }

    pub fn simm(&mut self) -> i32{
        self.reg_or_imm as i32
    }

    pub fn instr_index(&mut self) -> i32{
        self.reg_or_imm as i32
    }

    pub fn get_reg(&self) -> usize {
        self.reg_or_imm as usize
    }

    pub fn get_imm(&self) -> u32 {
        self.reg_or_imm as u32
    }

    pub fn get_simm(&self) -> i32 {
        self.reg_or_imm as i32
    }

    pub fn get_instr_index(&self) -> i32 {
        self.reg_or_imm as i32
    }

    pub fn set_reg(&mut self, reg: u32) {
        self.reg_or_imm = reg;
    }

    pub fn set_imm(&mut self, imm: u32) {
        self.reg_or_imm = imm;
    }

    pub fn set_simm(&mut self, simm: u32) {
        self.reg_or_imm = simm;
    }

    pub fn set_instr_index(&mut self, instr_index: u32) {
        self.reg_or_imm = instr_index;
    }
}

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
