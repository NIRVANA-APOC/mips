use crate::emu::cpu::exec::Emulator;
use crate::emu::cpu::helper::*;
use crate::emu::cpu::reg::REG_NAME;
use colored::Colorize;

fn imm_extend(src: &mut crate::emu::cpu::operand::Operand) {
    if src.val & 0x00008000 != 0 {
        src.val |= 0xFFFF0000;
    }
}

fn decode_imm_type(emu: &mut Emulator) {
    let instr = emu.instr;
    emu.ops
        .src1
        .set_reg((instr & RS_MASK) >> (RT_SIZE + IMM_SIZE));
    emu.ops.src1.val = emu.cpu.gpr.read(emu.ops.src1.get_reg());
    emu.ops.src2.set_imm(instr & IMM_MASK);
    emu.ops.src2.val = emu.ops.src2.get_imm();
    emu.ops.dest.set_reg((instr & RT_MASK) >> IMM_SIZE);

    emu.dbg_println(format!(
        "[DEBUG] op_src1->val: 0x{:08x}, op_src2->val: 0x{:08x}",
        emu.ops.src1.val, emu.ops.src2.val
    ));
}

pub fn addi(emu: &mut Emulator, pc: u32) {
    decode_imm_type(emu);

    let a = emu.ops.src1.val as i32;
    let b = emu.ops.src2.get_simm();
    let result = a.wrapping_add(b);
    if ((a ^ result) & (b ^ result)) < 0 {
        println!("{}", "Arithmetic overflow in addi".red());
        emu.inv(pc);
        return;
    }
    emu.cpu.gpr.write(emu.ops.dest.get_reg(), result as u32);
    emu.assembly = format!(
        "addi  {},   {},   0x{:04x}",
        REG_NAME[emu.ops.dest.get_reg()],
        REG_NAME[emu.ops.src1.get_reg()],
        emu.ops.src2.get_imm()
    );
}

pub fn addiu(emu: &mut Emulator, _pc: u32) {
    decode_imm_type(emu);

    imm_extend(&mut emu.ops.src2);
    emu.cpu.gpr.write(
        emu.ops.dest.get_reg(),
        emu.ops.src1.val.wrapping_add(emu.ops.src2.val),
    );
    emu.assembly = format!(
        "addiu {},   {},   0x{:04x}",
        REG_NAME[emu.ops.dest.get_reg()],
        REG_NAME[emu.ops.src1.get_reg()],
        emu.ops.src2.get_imm()
    );
}

pub fn slti(emu: &mut Emulator, _pc: u32) {
    decode_imm_type(emu);

    imm_extend(&mut emu.ops.src2);
    let result = if (emu.ops.src1.val as i32) < (emu.ops.src2.val as i32) {
        1
    } else {
        0
    };
    emu.cpu.gpr.write(emu.ops.dest.get_reg(), result);
    emu.assembly = format!(
        "slti  {},   {},   0x{:04x}",
        REG_NAME[emu.ops.dest.get_reg()],
        REG_NAME[emu.ops.src1.get_reg()],
        emu.ops.src2.get_imm()
    );
}

pub fn sltiu(emu: &mut Emulator, _pc: u32) {
    decode_imm_type(emu);

    let result = if emu.ops.src1.val < emu.ops.src2.val {
        1
    } else {
        0
    };
    emu.cpu.gpr.write(emu.ops.dest.get_reg(), result);
    emu.assembly = format!(
        "sltiu {},   {},   0x{:04x}",
        REG_NAME[emu.ops.dest.get_reg()],
        REG_NAME[emu.ops.src1.get_reg()],
        emu.ops.src2.get_imm()
    );
}

pub fn andi(emu: &mut Emulator, _pc: u32) {
    decode_imm_type(emu);

    emu.cpu
        .gpr
        .write(emu.ops.dest.get_reg(), emu.ops.src1.val & emu.ops.src2.val);
    emu.assembly = format!(
        "andi  {},   {},   0x{:04x}",
        REG_NAME[emu.ops.dest.get_reg()],
        REG_NAME[emu.ops.src1.get_reg()],
        emu.ops.src2.get_imm()
    );
}

pub fn lui(emu: &mut Emulator, _pc: u32) {
    decode_imm_type(emu);

    emu.cpu
        .gpr
        .write(emu.ops.dest.get_reg(), emu.ops.src2.val << 16);
    emu.assembly = format!(
        "lui   {},   0x{:04x}",
        REG_NAME[emu.ops.dest.get_reg()],
        emu.ops.src2.get_imm()
    );
}

pub fn ori(emu: &mut Emulator, _pc: u32) {
    decode_imm_type(emu);

    emu.cpu
        .gpr
        .write(emu.ops.dest.get_reg(), emu.ops.src1.val | emu.ops.src2.val);
    emu.assembly = format!(
        "ori   {},   {},   0x{:04x}",
        REG_NAME[emu.ops.dest.get_reg()],
        REG_NAME[emu.ops.src1.get_reg()],
        emu.ops.src2.get_imm()
    );
}

pub fn xori(emu: &mut Emulator, _pc: u32) {
    decode_imm_type(emu);

    emu.cpu
        .gpr
        .write(emu.ops.dest.get_reg(), emu.ops.src1.val ^ emu.ops.src2.val);
    emu.assembly = format!(
        "xori  {},   {},   0x{:04x}",
        REG_NAME[emu.ops.dest.get_reg()],
        REG_NAME[emu.ops.src1.get_reg()],
        emu.ops.src2.get_imm()
    );
}

fn mem_addr(emu: &Emulator) -> u32 {
    let base = emu.ops.src1.val;
    let offset = emu.ops.src2.get_simm() as u32;
    base.wrapping_add(offset)
}

pub fn lb(emu: &mut Emulator, _pc: u32) {
    decode_imm_type(emu);

    let addr = mem_addr(emu);
    let val = emu.memory.read(addr, 1);
    let extended = if val & 0x80 != 0 {
        val | 0xFFFF_FF00
    } else {
        val
    };
    emu.cpu.gpr.write(emu.ops.dest.get_reg(), extended);
    emu.assembly = format!(
        "lb    {},   {}({})",
        REG_NAME[emu.ops.dest.get_reg()],
        emu.ops.src2.get_simm(),
        REG_NAME[emu.ops.src1.get_reg()]
    );
}

pub fn lbu(emu: &mut Emulator, _pc: u32) {
    decode_imm_type(emu);

    let addr = mem_addr(emu);
    let val = emu.memory.read(addr, 1);
    emu.cpu.gpr.write(emu.ops.dest.get_reg(), val);
    emu.assembly = format!(
        "lbu   {},   {}({})",
        REG_NAME[emu.ops.dest.get_reg()],
        emu.ops.src2.get_simm(),
        REG_NAME[emu.ops.src1.get_reg()]
    );
}

pub fn lh(emu: &mut Emulator, pc: u32) {
    decode_imm_type(emu);

    let addr = mem_addr(emu);
    if !addr.is_multiple_of(2) {
        emu.inv(pc);
        return;
    }
    let val = emu.memory.read(addr, 2);
    let extended = if val & 0x8000 != 0 {
        val | 0xFFFF_0000
    } else {
        val
    };
    emu.cpu.gpr.write(emu.ops.dest.get_reg(), extended);
    emu.assembly = format!(
        "lh    {},   {}({})",
        REG_NAME[emu.ops.dest.get_reg()],
        emu.ops.src2.get_simm(),
        REG_NAME[emu.ops.src1.get_reg()]
    );
}

pub fn lhu(emu: &mut Emulator, pc: u32) {
    decode_imm_type(emu);

    let addr = mem_addr(emu);
    if !addr.is_multiple_of(2) {
        emu.inv(pc);
        return;
    }
    let val = emu.memory.read(addr, 2);
    emu.cpu.gpr.write(emu.ops.dest.get_reg(), val);
    emu.assembly = format!(
        "lhu   {},   {}({})",
        REG_NAME[emu.ops.dest.get_reg()],
        emu.ops.src2.get_simm(),
        REG_NAME[emu.ops.src1.get_reg()]
    );
}

pub fn lw(emu: &mut Emulator, pc: u32) {
    decode_imm_type(emu);

    let addr = mem_addr(emu);
    if !addr.is_multiple_of(4) {
        emu.inv(pc);
        return;
    }
    let val = emu.memory.read(addr, 4);
    emu.cpu.gpr.write(emu.ops.dest.get_reg(), val);
    emu.assembly = format!(
        "lw    {},   {}({})",
        REG_NAME[emu.ops.dest.get_reg()],
        emu.ops.src2.get_simm(),
        REG_NAME[emu.ops.src1.get_reg()]
    );
}

pub fn sb(emu: &mut Emulator, _pc: u32) {
    decode_imm_type(emu);

    let addr = mem_addr(emu);
    let val = emu.cpu.gpr.read(emu.ops.dest.get_reg());
    emu.memory.write(addr, 1, val);
    emu.assembly = format!(
        "sb    {},   {}({})",
        REG_NAME[emu.ops.dest.get_reg()],
        emu.ops.src2.get_simm(),
        REG_NAME[emu.ops.src1.get_reg()]
    );
}

pub fn sh(emu: &mut Emulator, pc: u32) {
    decode_imm_type(emu);

    let addr = mem_addr(emu);
    if !addr.is_multiple_of(2) {
        emu.inv(pc);
        return;
    }
    let val = emu.cpu.gpr.read(emu.ops.dest.get_reg());
    emu.memory.write(addr, 2, val);
    emu.assembly = format!(
        "sh    {},   {}({})",
        REG_NAME[emu.ops.dest.get_reg()],
        emu.ops.src2.get_simm(),
        REG_NAME[emu.ops.src1.get_reg()]
    );
}

pub fn sw(emu: &mut Emulator, pc: u32) {
    decode_imm_type(emu);

    let addr = mem_addr(emu);
    if !addr.is_multiple_of(4) {
        emu.inv(pc);
        return;
    }
    let val = emu.cpu.gpr.read(emu.ops.dest.get_reg());
    emu.memory.write(addr, 4, val);
    emu.assembly = format!(
        "sw    {},   {}({})",
        REG_NAME[emu.ops.dest.get_reg()],
        emu.ops.src2.get_simm(),
        REG_NAME[emu.ops.src1.get_reg()]
    );
}

#[cfg(test)]
mod test {
    use crate::emu::cpu::exec::Emulator;
    use crate::emu::cpu::helper::encode_i_type;
    use crate::emu::cpu::reg::CpuRegs;

    fn load_instructions(emu: &mut Emulator, instructions: &[u32]) {
        emu.cpu = CpuRegs::new();
        let entry = 0xBFC00000;
        let data: Vec<u8> = instructions.iter().flat_map(|&x| x.to_ne_bytes()).collect();
        emu.memory.load_binary(&data, entry & 0x1F_FF_FF_FF);
        emu.cpu.pc = entry;
        emu.state = crate::emu::cpu::exec::CpuState::Stop;
    }

    fn set_reg(emu: &mut Emulator, reg: usize, val: u32) {
        emu.cpu.gpr.write(reg, val);
    }

    #[test]
    fn test_addi() {
        let mut emu = Emulator::new();

        load_instructions(
            &mut emu,
            &[
                encode_i_type(0x08, 0, 8, 5), // addi $t0, $zero, 5
            ],
        );
        emu.cpu_exec(1);
        assert_eq!(emu.cpu.gpr.read(8), 5);
    }

    #[test]
    fn test_addi_overflow() {
        let mut emu = Emulator::new();

        load_instructions(
            &mut emu,
            &[
                encode_i_type(0x08, 8, 9, 1), // addi $t0, $zero, 1
            ],
        );
        set_reg(&mut emu, 8, 0x7FFFFFFF);
        emu.cpu_exec(1);
        assert_eq!(emu.state, crate::emu::cpu::exec::CpuState::End);
    }

    #[test]
    fn test_addiu() {
        let mut emu = Emulator::new();

        load_instructions(
            &mut emu,
            &[
                encode_i_type(0x09, 8, 9, 1), // addiu $t0, $zero, 1
            ],
        );
        set_reg(&mut emu, 8, 0xFFFFFFFF);
        emu.cpu_exec(1);
        assert_eq!(emu.cpu.gpr.read(9), 0);
    }

    #[test]
    fn test_slti() {
        let mut emu = Emulator::new();

        load_instructions(
            &mut emu,
            &[
                encode_i_type(0x0A, 0, 8, 5), // slti $t0, $zero, 5
            ],
        );
        emu.cpu_exec(1);
        assert_eq!(emu.cpu.gpr.read(8), 1);
    }

    #[test]
    fn test_sltiu() {
        let mut emu = Emulator::new();

        load_instructions(
            &mut emu,
            &[
                encode_i_type(0x0B, 8, 9, 1), // sltiu $t0, $zero, 1
            ],
        );
        set_reg(&mut emu, 8, 0xFFFFFFFF);
        emu.cpu_exec(1);
        assert_eq!(emu.cpu.gpr.read(9), 0);
    }

    #[test]
    fn test_andi() {
        let mut emu = Emulator::new();

        load_instructions(
            &mut emu,
            &[
                encode_i_type(0x0C, 8, 9, 0xFF), // andi $t0, $zero, 0xFF
            ],
        );
        set_reg(&mut emu, 8, 0xFFFF);
        emu.cpu_exec(1);
        assert_eq!(emu.cpu.gpr.read(9), 0xFF);
    }

    #[test]
    fn test_ori() {
        let mut emu = Emulator::new();

        load_instructions(
            &mut emu,
            &[
                encode_i_type(0x0D, 8, 9, 0xFF), // ori $t0, $zero, 0xFF
            ],
        );
        set_reg(&mut emu, 8, 0xFF00);
        emu.cpu_exec(1);
        assert_eq!(emu.cpu.gpr.read(9), 0xFFFF);
    }

    #[test]
    fn test_xori() {
        let mut emu = Emulator::new();

        load_instructions(
            &mut emu,
            &[
                encode_i_type(0x0E, 8, 9, 0x0F), // xori $t0, $zero, 0x0F
            ],
        );
        set_reg(&mut emu, 8, 0xFFFF);
        emu.cpu_exec(1);
        assert_eq!(emu.cpu.gpr.read(9), 0xFFF0);
    }

    #[test]
    fn test_lui() {
        let mut emu = Emulator::new();

        load_instructions(
            &mut emu,
            &[
                encode_i_type(0x0F, 0, 8, 0x1234), // lui $t0, 0x1234
            ],
        );
        emu.cpu_exec(1);
        assert_eq!(emu.cpu.gpr.read(8), 0x12340000);
    }

    #[test]
    fn test_sw_lw() {
        let mut emu = Emulator::new();

        load_instructions(
            &mut emu,
            &[
                encode_i_type(0x2B, 0, 8, 0), // sw $t0, 0($zero)
                encode_i_type(0x23, 0, 9, 0), // lw $t1, 0($zero)
            ],
        );
        set_reg(&mut emu, 8, 0x12345678);
        emu.cpu_exec(2);
        assert_eq!(emu.cpu.gpr.read(9), 0x12345678);
    }

    #[test]
    fn test_sb_lb() {
        let mut emu = Emulator::new();

        load_instructions(
            &mut emu,
            &[
                encode_i_type(0x28, 0, 8, 0), // sb $t0, 0($zero)
                encode_i_type(0x20, 0, 9, 0), // lb $t1, 0($zero)
            ],
        );
        set_reg(&mut emu, 8, 0x80);
        emu.cpu_exec(2);
        assert_eq!(emu.cpu.gpr.read(9), 0xFFFFFF80);
    }

    #[test]
    fn test_sb_lbu() {
        let mut emu = Emulator::new();

        load_instructions(
            &mut emu,
            &[
                encode_i_type(0x28, 0, 8, 0), // sb $t0, 0($zero)
                encode_i_type(0x24, 0, 9, 0), // lbu $t1, 0($zero)
            ],
        );
        set_reg(&mut emu, 8, 0x80);
        emu.cpu_exec(2);
        assert_eq!(emu.cpu.gpr.read(9), 0x80);
    }

    #[test]
    fn test_sh_lh() {
        let mut emu = Emulator::new();

        load_instructions(
            &mut emu,
            &[
                encode_i_type(0x29, 0, 8, 0), // sh $t0, 0($zero)
                encode_i_type(0x21, 0, 9, 0), // lh $t1, 0($zero)
            ],
        );
        set_reg(&mut emu, 8, 0xFF00);
        emu.cpu_exec(2);
        assert_eq!(emu.cpu.gpr.read(9), 0xFFFF_FF00);
    }

    #[test]
    fn test_sh_lhu() {
        let mut emu = Emulator::new();

        load_instructions(
            &mut emu,
            &[
                encode_i_type(0x29, 0, 8, 0), // sh $t0, 0($zero)
                encode_i_type(0x25, 0, 9, 0), // lhu $t1, 0($zero)
            ],
        );
        set_reg(&mut emu, 8, 0xFF00);
        emu.cpu_exec(2);
        assert_eq!(emu.cpu.gpr.read(9), 0xFF00);
    }

    #[test]
    fn test_lw_unaligned() {
        let mut emu = Emulator::new();

        load_instructions(
            &mut emu,
            &[
                encode_i_type(0x23, 0, 8, 1), // lw $t0, 1($zero)
            ],
        );
        emu.cpu_exec(1);
        assert_eq!(emu.state, crate::emu::cpu::exec::CpuState::End);
    }

    #[test]
    fn test_lh_unaligned() {
        let mut emu = Emulator::new();

        load_instructions(
            &mut emu,
            &[
                encode_i_type(0x21, 0, 8, 1), // lh $t0, 1($zero)
            ],
        );
        emu.cpu_exec(1);
        assert_eq!(emu.state, crate::emu::cpu::exec::CpuState::End);
    }
}
