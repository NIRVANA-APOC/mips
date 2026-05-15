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

pub fn encode_r_type(rs: u32, rt: u32, rd: u32, shamt: u32, func: u32) -> u32 {
    (rs << 21) | (rt << 16) | (rd << 11) | (shamt << 6) | (func & FUNC_MASK)
}

pub fn encode_i_type(opcode: u32, rs: u32, rt: u32, imm: u16) -> u32 {
    (opcode << 26) | (rs << 21) | (rt << 16) | (imm as u32)
}

pub fn encode_j_type(opcode: u32, target: u32) -> u32 {
    (opcode << 26) | (target & INDEX_MASK)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_encode_r_type() {
        // add $v0, $zero, $at  =>  rs=0, rt=1, rd=2, shamt=0, func=0x20
        assert_eq!(encode_r_type(0, 1, 2, 0, 0x20), 0x00011020);
    }

    #[test]
    fn test_encode_i_type() {
        // addi $t0, $zero, 5  =>  opcode=0x08, rs=0, rt=8, imm=5
        assert_eq!(encode_i_type(0x08, 0, 8, 5), 0x20080005);
    }

    #[test]
    fn test_encode_j_type() {
        // j 0x00400000  =>  opcode=0x02, target=0x00400000
        assert_eq!(encode_j_type(0x02, 0x00400000), 0x08000000 | 0x00400000);
    }
}
