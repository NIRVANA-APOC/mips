pub const FUNC_MASK: u32 = 0x0000003F;
pub const RS_MASK: u32 = 0x03E00000;
pub const RT_MASK: u32 = 0x001F0000;
pub const RD_MASK: u32 = 0x0000F800;
pub const SHAMT_MASK: u32 = 0x000007C0;
pub const IMM_MASK: u32 = 0x0000FFFF;
pub const INDEX_MASK: u32 = 0x03FFFFFF;

pub const OPCODE_SIZE: u32 = 6;
pub const FUNC_SIZE: u32 = 6;
pub const RS_SIZE: u32 = 5;
pub const RT_SIZE: u32 = 5;
pub const RD_SIZE: u32 = 5;
pub const SHAMT_SIZE: u32 = 5;
pub const IMM_SIZE: u32 = 16;
pub const INDEX_SIZE: u32 = 26;