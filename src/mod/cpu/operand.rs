enum OPType {
    REG, IMM, JUMP,
}

const OP_STR_SIZE: usize = 40;

struct Operand{
    ty: u32,
    reg_or_imm: u32,
    val: u32,
}

impl Operand {
    pub fn new() -> Self{
        Self { ty: 0, reg_or_imm: 0, val: 0 }
    }

    pub fn reg(&self) -> u32{
        self.reg_or_imm as u32
    }

    pub fn imm(&self) -> u32{
        self.reg_or_imm as u32
    }

    pub fn simm(&self) -> i32{
        self.reg_or_imm as i32
    }

    pub fn instr_index(&self) -> i32{
        self.reg_or_imm as i32
    }
}

pub struct Operands{
    pub opcode: u32,
    pub func: u32,
    pub src1: u32,
    pub src2: u32,
    pub dest: u32,
}

impl Operands {
    pub const fn new() -> Self{
        Self { opcode: 0, func: 0, src1: 0, src2: 0, dest: 0 }
    }
}