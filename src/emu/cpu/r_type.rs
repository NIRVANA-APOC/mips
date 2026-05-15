use crate::emu::cpu::exec::Emulator;
use crate::emu::cpu::helper::*;
use crate::emu::cpu::reg::REG_NAME;
use colored::Colorize;

fn decode_r_type(emu: &mut Emulator) {
    let instr = emu.instr;
    emu.ops
        .src1
        .set_reg((instr & RS_MASK) >> (RT_SIZE + IMM_SIZE));
    emu.ops.src1.val = emu.cpu.gpr.read(emu.ops.src1.get_reg());
    emu.ops
        .src2
        .set_imm((instr & RT_MASK) >> (RD_SIZE + SHAMT_SIZE + FUNC_SIZE));
    emu.ops.src2.val = emu.cpu.gpr.read(emu.ops.src2.get_reg());
    emu.ops
        .dest
        .set_reg((instr & RD_MASK) >> (SHAMT_SIZE + FUNC_SIZE));

    emu.dbg_println(format!(
        "[DEBUG] op_src1->val: 0x{:08x}, op_src2->val: 0x{:08x}",
        emu.ops.src1.val, emu.ops.src2.val
    ));
}

fn decode_shift_imm(emu: &mut Emulator) {
    let instr = emu.instr;
    emu.ops
        .src1
        .set_reg((instr & RT_MASK) >> (RD_SIZE + SHAMT_SIZE + FUNC_SIZE));
    emu.ops.src1.val = emu.cpu.gpr.read(emu.ops.src1.get_reg());
    emu.ops.src2.set_imm((instr & SHAMT_MASK) >> FUNC_SIZE);
    emu.ops.src2.val = emu.ops.src2.get_imm();
    emu.ops
        .dest
        .set_reg((instr & RD_MASK) >> (SHAMT_SIZE + FUNC_SIZE));

    emu.dbg_println(format!(
        "[DEBUG] op_src1->val: 0x{:08x}, op_src2->val: 0x{:08x}",
        emu.ops.src1.val, emu.ops.src2.val
    ));
}

fn decode_branch(emu: &mut Emulator) {
    let instr = emu.instr;
    emu.ops
        .src1
        .set_reg((instr & RS_MASK) >> (RT_SIZE + IMM_SIZE));
    emu.ops.src1.val = emu.cpu.gpr.read(emu.ops.src1.get_reg());
    emu.ops.src2.set_imm(instr & IMM_MASK);
    emu.ops.src2.val = emu.ops.src2.get_imm();

    emu.dbg_println(format!(
        "[DEBUG] op_src1->val: 0x{:08x}, op_src2->val: 0x{:08x}",
        emu.ops.src1.val, emu.ops.src2.val
    ));
}

fn branch_target(emu: &Emulator, pc: u32) -> u32 {
    let offset = emu.ops.src2.get_simm() << 2;
    (pc as i32 + 4 + offset) as u32
}

pub fn add(emu: &mut Emulator, _pc: u32) {
    decode_r_type(emu);

    let a = emu.ops.src1.val as i32;
    let b = emu.ops.src2.val as i32;
    let result = a.wrapping_add(b);
    if ((a ^ result) & (b ^ result)) < 0 {
        println!("{}", "Arithmetic overflow in add".red());
        emu.state = crate::emu::cpu::exec::CpuState::End;
        return;
    }
    emu.cpu.gpr.write(emu.ops.dest.get_reg(), result as u32);
    emu.assembly = format!(
        "add   {},   {},   {}",
        REG_NAME[emu.ops.dest.get_reg()],
        REG_NAME[emu.ops.src1.get_reg()],
        REG_NAME[emu.ops.src2.get_reg()]
    );
}

pub fn addu(emu: &mut Emulator, _pc: u32) {
    decode_r_type(emu);

    emu.cpu.gpr.write(
        emu.ops.dest.get_reg(),
        emu.ops.src1.val.wrapping_add(emu.ops.src2.val),
    );
    emu.assembly = format!(
        "addu  {},   {},   {}",
        REG_NAME[emu.ops.dest.get_reg()],
        REG_NAME[emu.ops.src1.get_reg()],
        REG_NAME[emu.ops.src2.get_reg()]
    );
}

pub fn sub(emu: &mut Emulator, _pc: u32) {
    decode_r_type(emu);

    let a = emu.ops.src1.val as i32;
    let b = emu.ops.src2.val as i32;
    let result = a.wrapping_sub(b);
    if ((a ^ b) & (a ^ result)) < 0 {
        println!("{}", "Arithmetic overflow in sub".red());
        emu.state = crate::emu::cpu::exec::CpuState::End;
        return;
    }
    emu.cpu.gpr.write(emu.ops.dest.get_reg(), result as u32);
    emu.assembly = format!(
        "sub   {},   {},   {}",
        REG_NAME[emu.ops.dest.get_reg()],
        REG_NAME[emu.ops.src1.get_reg()],
        REG_NAME[emu.ops.src2.get_reg()]
    );
}

pub fn subu(emu: &mut Emulator, _pc: u32) {
    decode_r_type(emu);

    emu.cpu.gpr.write(
        emu.ops.dest.get_reg(),
        emu.ops.src1.val.wrapping_sub(emu.ops.src2.val),
    );
    emu.assembly = format!(
        "subu  {},   {},   {}",
        REG_NAME[emu.ops.dest.get_reg()],
        REG_NAME[emu.ops.src1.get_reg()],
        REG_NAME[emu.ops.src2.get_reg()]
    );
}

pub fn slt(emu: &mut Emulator, _pc: u32) {
    decode_r_type(emu);

    let result = if (emu.ops.src1.val as i32) < (emu.ops.src2.val as i32) {
        1
    } else {
        0
    };
    emu.cpu.gpr.write(emu.ops.dest.get_reg(), result);
    emu.assembly = format!(
        "slt   {},   {},   {}",
        REG_NAME[emu.ops.dest.get_reg()],
        REG_NAME[emu.ops.src1.get_reg()],
        REG_NAME[emu.ops.src2.get_reg()]
    );
}

pub fn sltu(emu: &mut Emulator, _pc: u32) {
    decode_r_type(emu);

    let result = if emu.ops.src1.val < emu.ops.src2.val {
        1
    } else {
        0
    };
    emu.cpu.gpr.write(emu.ops.dest.get_reg(), result);
    emu.assembly = format!(
        "sltu  {},   {},   {}",
        REG_NAME[emu.ops.dest.get_reg()],
        REG_NAME[emu.ops.src1.get_reg()],
        REG_NAME[emu.ops.src2.get_reg()]
    );
}

pub fn div(emu: &mut Emulator, _pc: u32) {
    decode_r_type(emu);

    let rs = emu.ops.src1.val as i32;
    let rt = emu.ops.src2.val as i32;
    if rt == 0 {
        println!("{}", "Divide by zero in div".red());
        emu.state = crate::emu::cpu::exec::CpuState::End;
        return;
    }
    emu.cpu.lo = (rs / rt) as u32;
    emu.cpu.hi = (rs % rt) as u32;
    emu.assembly = format!(
        "div   {},   {}",
        REG_NAME[emu.ops.src1.get_reg()],
        REG_NAME[emu.ops.src2.get_reg()]
    );
}

pub fn divu(emu: &mut Emulator, _pc: u32) {
    decode_r_type(emu);

    let rs = emu.ops.src1.val;
    let rt = emu.ops.src2.val;
    if rt == 0 {
        println!("{}", "Divide by zero in divu".red());
        emu.state = crate::emu::cpu::exec::CpuState::End;
        return;
    }
    emu.cpu.lo = rs / rt;
    emu.cpu.hi = rs % rt;
    emu.assembly = format!(
        "divu  {},   {}",
        REG_NAME[emu.ops.src1.get_reg()],
        REG_NAME[emu.ops.src2.get_reg()]
    );
}

pub fn mult(emu: &mut Emulator, _pc: u32) {
    decode_r_type(emu);

    let result = (emu.ops.src1.val as i64) * (emu.ops.src2.val as i64);
    emu.cpu.lo = result as u32;
    emu.cpu.hi = (result >> 32) as u32;
    emu.assembly = format!(
        "mult  {},   {}",
        REG_NAME[emu.ops.src1.get_reg()],
        REG_NAME[emu.ops.src2.get_reg()]
    );
}

pub fn multu(emu: &mut Emulator, _pc: u32) {
    decode_r_type(emu);

    let result = (emu.ops.src1.val as u64) * (emu.ops.src2.val as u64);
    emu.cpu.lo = result as u32;
    emu.cpu.hi = (result >> 32) as u32;
    emu.assembly = format!(
        "multu {},   {}",
        REG_NAME[emu.ops.src1.get_reg()],
        REG_NAME[emu.ops.src2.get_reg()]
    );
}

pub fn and(emu: &mut Emulator, _pc: u32) {
    decode_r_type(emu);

    emu.cpu
        .gpr
        .write(emu.ops.dest.get_reg(), emu.ops.src1.val & emu.ops.src2.val);
    emu.assembly = format!(
        "and   {},   {},   {}",
        REG_NAME[emu.ops.dest.get_reg()],
        REG_NAME[emu.ops.src1.get_reg()],
        REG_NAME[emu.ops.src2.get_reg()]
    );
}

pub fn nor(emu: &mut Emulator, _pc: u32) {
    decode_r_type(emu);

    emu.cpu.gpr.write(
        emu.ops.dest.get_reg(),
        !(emu.ops.src1.val | emu.ops.src2.val),
    );
    emu.assembly = format!(
        "nor   {},   {},   {}",
        REG_NAME[emu.ops.dest.get_reg()],
        REG_NAME[emu.ops.src1.get_reg()],
        REG_NAME[emu.ops.src2.get_reg()]
    );
}

pub fn or(emu: &mut Emulator, _pc: u32) {
    decode_r_type(emu);

    emu.cpu
        .gpr
        .write(emu.ops.dest.get_reg(), emu.ops.src1.val | emu.ops.src2.val);
    emu.assembly = format!(
        "or    {},   {},   {}",
        REG_NAME[emu.ops.dest.get_reg()],
        REG_NAME[emu.ops.src1.get_reg()],
        REG_NAME[emu.ops.src2.get_reg()]
    );
}

pub fn xor(emu: &mut Emulator, _pc: u32) {
    decode_r_type(emu);

    emu.cpu
        .gpr
        .write(emu.ops.dest.get_reg(), emu.ops.src1.val ^ emu.ops.src2.val);
    emu.assembly = format!(
        "xor   {},   {},   {}",
        REG_NAME[emu.ops.dest.get_reg()],
        REG_NAME[emu.ops.src1.get_reg()],
        REG_NAME[emu.ops.src2.get_reg()]
    );
}

pub fn sllv(emu: &mut Emulator, _pc: u32) {
    decode_r_type(emu);

    let shamt = emu.ops.src1.val & 0x1F;
    emu.cpu
        .gpr
        .write(emu.ops.dest.get_reg(), emu.ops.src2.val << shamt);
    emu.assembly = format!(
        "sllv  {},   {},   {}",
        REG_NAME[emu.ops.dest.get_reg()],
        REG_NAME[emu.ops.src2.get_reg()],
        REG_NAME[emu.ops.src1.get_reg()]
    );
}

pub fn sll(emu: &mut Emulator, _pc: u32) {
    decode_shift_imm(emu);

    emu.cpu
        .gpr
        .write(emu.ops.dest.get_reg(), emu.ops.src1.val << emu.ops.src2.val);
    emu.assembly = format!(
        "sll   {},   {},   {}",
        REG_NAME[emu.ops.dest.get_reg()],
        REG_NAME[emu.ops.src1.get_reg()],
        emu.ops.src2.val
    );
}

pub fn srav(emu: &mut Emulator, _pc: u32) {
    decode_r_type(emu);

    let shamt = emu.ops.src1.val & 0x1F;
    emu.cpu.gpr.write(
        emu.ops.dest.get_reg(),
        (emu.ops.src2.val as i32 >> shamt) as u32,
    );
    emu.assembly = format!(
        "srav  {},   {},   {}",
        REG_NAME[emu.ops.dest.get_reg()],
        REG_NAME[emu.ops.src2.get_reg()],
        REG_NAME[emu.ops.src1.get_reg()]
    );
}

pub fn sra(emu: &mut Emulator, _pc: u32) {
    decode_shift_imm(emu);

    emu.cpu.gpr.write(
        emu.ops.dest.get_reg(),
        (emu.ops.src1.val as i32 >> emu.ops.src2.val) as u32,
    );
    emu.assembly = format!(
        "sra   {},   {},   {}",
        REG_NAME[emu.ops.dest.get_reg()],
        REG_NAME[emu.ops.src1.get_reg()],
        emu.ops.src2.val
    );
}

pub fn srlv(emu: &mut Emulator, _pc: u32) {
    decode_r_type(emu);

    let shamt = emu.ops.src1.val & 0x1F;
    emu.cpu
        .gpr
        .write(emu.ops.dest.get_reg(), emu.ops.src2.val >> shamt);
    emu.assembly = format!(
        "srlv  {},   {},   {}",
        REG_NAME[emu.ops.dest.get_reg()],
        REG_NAME[emu.ops.src2.get_reg()],
        REG_NAME[emu.ops.src1.get_reg()]
    );
}

pub fn srl(emu: &mut Emulator, _pc: u32) {
    decode_shift_imm(emu);

    emu.cpu
        .gpr
        .write(emu.ops.dest.get_reg(), emu.ops.src1.val >> emu.ops.src2.val);
    emu.assembly = format!(
        "srl   {},   {},   {}",
        REG_NAME[emu.ops.dest.get_reg()],
        REG_NAME[emu.ops.src1.get_reg()],
        emu.ops.src2.val
    );
}

pub fn bz(emu: &mut Emulator, pc: u32) {
    let rt = (emu.instr & RT_MASK) >> (RD_SIZE + SHAMT_SIZE + FUNC_SIZE);
    match rt {
        0x00 => bltz(emu, pc),
        0x01 => bgez(emu, pc),
        0x10 => bltzal(emu, pc),
        0x11 => bgezal(emu, pc),
        _ => {
            println!("{}", format!("Unknown bz/rt = {}", rt).red());
            emu.state = crate::emu::cpu::exec::CpuState::End;
        }
    }
}

pub fn beq(emu: &mut Emulator, pc: u32) {
    decode_branch(emu);

    if emu.ops.src1.val == emu.cpu.gpr.read(emu.ops.src2.get_reg()) {
        emu.cpu.pc = branch_target(emu, pc);
        emu.pc_updated = true;
    }
    emu.assembly = format!(
        "beq   {},   {},   0x{:08x}",
        REG_NAME[emu.ops.src1.get_reg()],
        REG_NAME[emu.ops.src2.get_reg()],
        branch_target(emu, pc)
    );
}

pub fn bne(emu: &mut Emulator, pc: u32) {
    decode_branch(emu);

    if emu.ops.src1.val != emu.cpu.gpr.read(emu.ops.src2.get_reg()) {
        emu.cpu.pc = branch_target(emu, pc);
        emu.pc_updated = true;
    }
    emu.assembly = format!(
        "bne   {},   {},   0x{:08x}",
        REG_NAME[emu.ops.src1.get_reg()],
        REG_NAME[emu.ops.src2.get_reg()],
        branch_target(emu, pc)
    );
}

pub fn bgez(emu: &mut Emulator, pc: u32) {
    decode_branch(emu);

    if (emu.ops.src1.val as i32) >= 0 {
        emu.cpu.pc = branch_target(emu, pc);
        emu.pc_updated = true;
    }
    emu.assembly = format!(
        "bgez  {},   0x{:08x}",
        REG_NAME[emu.ops.src1.get_reg()],
        branch_target(emu, pc)
    );
}

pub fn bgtz(emu: &mut Emulator, pc: u32) {
    decode_branch(emu);

    if (emu.ops.src1.val as i32) > 0 {
        emu.cpu.pc = branch_target(emu, pc);
        emu.pc_updated = true;
    }
    emu.assembly = format!(
        "bgtz  {},   0x{:08x}",
        REG_NAME[emu.ops.src1.get_reg()],
        branch_target(emu, pc)
    );
}

#[allow(dead_code)]
pub fn gbtz(_emu: &mut Emulator, _pc: u32) {}

pub fn blez(emu: &mut Emulator, pc: u32) {
    decode_branch(emu);

    if (emu.ops.src1.val as i32) <= 0 {
        emu.cpu.pc = branch_target(emu, pc);
        emu.pc_updated = true;
    }
    emu.assembly = format!(
        "blez  {},   0x{:08x}",
        REG_NAME[emu.ops.src1.get_reg()],
        branch_target(emu, pc)
    );
}

pub fn bltz(emu: &mut Emulator, pc: u32) {
    decode_branch(emu);

    if (emu.ops.src1.val as i32) < 0 {
        emu.cpu.pc = branch_target(emu, pc);
        emu.pc_updated = true;
    }
    emu.assembly = format!(
        "bltz  {},   0x{:08x}",
        REG_NAME[emu.ops.src1.get_reg()],
        branch_target(emu, pc)
    );
}

pub fn bgezal(emu: &mut Emulator, pc: u32) {
    decode_branch(emu);

    emu.cpu.gpr.write(31, pc + 4);
    if (emu.ops.src1.val as i32) >= 0 {
        emu.cpu.pc = branch_target(emu, pc);
        emu.pc_updated = true;
    }
    emu.assembly = format!(
        "bgezal {},   0x{:08x}",
        REG_NAME[emu.ops.src1.get_reg()],
        branch_target(emu, pc)
    );
}

pub fn bltzal(emu: &mut Emulator, pc: u32) {
    decode_branch(emu);

    emu.cpu.gpr.write(31, pc + 4);
    if (emu.ops.src1.val as i32) < 0 {
        emu.cpu.pc = branch_target(emu, pc);
        emu.pc_updated = true;
    }
    emu.assembly = format!(
        "bltzal {},   0x{:08x}",
        REG_NAME[emu.ops.src1.get_reg()],
        branch_target(emu, pc)
    );
}

pub fn jr(emu: &mut Emulator, _pc: u32) {
    decode_r_type(emu);

    emu.cpu.pc = emu.ops.src1.val;
    emu.pc_updated = true;
    emu.assembly = format!("jr    {}", REG_NAME[emu.ops.src1.get_reg()]);
}

pub fn jalr(emu: &mut Emulator, pc: u32) {
    decode_r_type(emu);

    let target = emu.ops.src1.val;
    let rd = emu.ops.dest.get_reg();
    emu.cpu.gpr.write(rd, pc + 4);
    emu.cpu.pc = target;
    emu.pc_updated = true;
    emu.assembly = format!(
        "jalr  {},   {}",
        REG_NAME[emu.ops.dest.get_reg()],
        REG_NAME[emu.ops.src1.get_reg()]
    );
}

pub fn mfhi(emu: &mut Emulator, _pc: u32) {
    decode_r_type(emu);

    emu.cpu.gpr.write(emu.ops.dest.get_reg(), emu.cpu.hi);
    emu.assembly = format!("mfhi  {}", REG_NAME[emu.ops.dest.get_reg()]);
}

pub fn mflo(emu: &mut Emulator, _pc: u32) {
    decode_r_type(emu);

    emu.cpu.gpr.write(emu.ops.dest.get_reg(), emu.cpu.lo);
    emu.assembly = format!("mflo  {}", REG_NAME[emu.ops.dest.get_reg()]);
}

pub fn mthi(emu: &mut Emulator, _pc: u32) {
    decode_r_type(emu);

    emu.cpu.hi = emu.ops.src1.val;
    emu.assembly = format!("mthi  {}", REG_NAME[emu.ops.src1.get_reg()]);
}

pub fn mtlo(emu: &mut Emulator, _pc: u32) {
    decode_r_type(emu);

    emu.cpu.lo = emu.ops.src1.val;
    emu.assembly = format!("mtlo  {}", REG_NAME[emu.ops.src1.get_reg()]);
}

pub fn _break(emu: &mut Emulator, pc: u32) {
    println!("{}", format!("Breakpoint at $pc = 0x{:08x}", pc).red());
    emu.state = crate::emu::cpu::exec::CpuState::End;
    emu.assembly = "break".to_string();
}

pub fn syscall(emu: &mut Emulator, pc: u32) {
    match emu.cpu.gpr.read(2) {
        // print_string: $a0 = address of null-terminated string
        4 => {
            let addr = emu.cpu.gpr.read(4);
            let mut s = String::new();
            for offset in 0..1024u32 {
                let byte = emu.memory.read(addr + offset, 1) as u8;
                if byte == 0 {
                    break;
                }
                s.push(byte as char);
            }
            print!("{}", s);
            let _ = std::io::Write::flush(&mut std::io::stdout());
        }
        // exit
        10 => {
            emu.state = crate::emu::cpu::exec::CpuState::End;
        }
        _ => {
            println!("{}", format!("Syscall at $pc = 0x{:08x}", pc).yellow());
            emu.state = crate::emu::cpu::exec::CpuState::End;
        }
    }
    emu.assembly = "syscall".to_string();
}

pub fn eret(emu: &mut Emulator, pc: u32) {
    println!("{}", format!("eret at $pc = 0x{:08x}", pc).yellow());
    emu.state = crate::emu::cpu::exec::CpuState::End;
    emu.assembly = "eret".to_string();
}

#[allow(dead_code)]
pub fn mfc0(_emu: &mut Emulator, _pc: u32) {}

#[allow(dead_code)]
pub fn mtc0(_emu: &mut Emulator, _pc: u32) {}

#[cfg(test)]
mod test {
    use crate::emu::cpu::exec::Emulator;
    use crate::emu::cpu::helper::encode_r_type;
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
    fn test_add() {
        let mut emu = Emulator::new();

        load_instructions(
            &mut emu,
            &[
                encode_r_type(8, 9, 10, 0, 0x20), // add $t2, $t0, $t1
                encode_r_type(0, 0, 0, 0, 0),     // nop (sll $zero, $zero, 0)
            ],
        );
        set_reg(&mut emu, 8, 5);
        set_reg(&mut emu, 9, 3);
        emu.cpu_exec(1);
        assert_eq!(emu.cpu.gpr.read(10), 8);
    }

    #[test]
    fn test_add_overflow() {
        let mut emu = Emulator::new();

        load_instructions(
            &mut emu,
            &[
                encode_r_type(8, 9, 10, 0, 0x20), // add $t2, $t0, $t1
            ],
        );
        set_reg(&mut emu, 8, 0x7FFFFFFF);
        set_reg(&mut emu, 9, 1);
        emu.cpu_exec(1);
        assert_eq!(emu.state, crate::emu::cpu::exec::CpuState::End);
    }

    #[test]
    fn test_addu() {
        let mut emu = Emulator::new();

        load_instructions(
            &mut emu,
            &[
                encode_r_type(8, 9, 10, 0, 0x21), // addu $t2, $t0, $t1
            ],
        );
        set_reg(&mut emu, 8, 0xFFFFFFFF);
        set_reg(&mut emu, 9, 1);
        emu.cpu_exec(1);
        assert_eq!(emu.cpu.gpr.read(10), 0);
    }

    #[test]
    fn test_sub() {
        let mut emu = Emulator::new();

        load_instructions(
            &mut emu,
            &[
                encode_r_type(8, 9, 10, 0, 0x22), // sub $t2, $t0, $t1
            ],
        );
        set_reg(&mut emu, 8, 8);
        set_reg(&mut emu, 9, 3);
        emu.cpu_exec(1);
        assert_eq!(emu.cpu.gpr.read(10), 5);
    }

    #[test]
    fn test_subu() {
        let mut emu = Emulator::new();

        load_instructions(
            &mut emu,
            &[
                encode_r_type(8, 9, 10, 0, 0x23), // subu $t2, $t0, $t1
            ],
        );
        set_reg(&mut emu, 8, 0);
        set_reg(&mut emu, 9, 1);
        emu.cpu_exec(1);
        assert_eq!(emu.cpu.gpr.read(10), 0xFFFFFFFF);
    }

    #[test]
    fn test_and() {
        let mut emu = Emulator::new();

        load_instructions(
            &mut emu,
            &[
                encode_r_type(8, 9, 10, 0, 0x24), // and $t2, $t0, $t1
            ],
        );
        set_reg(&mut emu, 8, 0xFF00);
        set_reg(&mut emu, 9, 0x0FF0);
        emu.cpu_exec(1);
        assert_eq!(emu.cpu.gpr.read(10), 0x0F00);
    }

    #[test]
    fn test_or() {
        let mut emu = Emulator::new();

        load_instructions(
            &mut emu,
            &[
                encode_r_type(8, 9, 10, 0, 0x25), // or $t2, $t0, $t1
            ],
        );
        set_reg(&mut emu, 8, 0xFF00);
        set_reg(&mut emu, 9, 0x00FF);
        emu.cpu_exec(1);
        assert_eq!(emu.cpu.gpr.read(10), 0xFFFF);
    }

    #[test]
    fn test_xor() {
        let mut emu = Emulator::new();

        load_instructions(
            &mut emu,
            &[
                encode_r_type(8, 9, 10, 0, 0x26), // xor $t2, $t0, $t1
            ],
        );
        set_reg(&mut emu, 8, 0xFFFF);
        set_reg(&mut emu, 9, 0x0F0F);
        emu.cpu_exec(1);
        assert_eq!(emu.cpu.gpr.read(10), 0xF0F0);
    }

    #[test]
    fn test_nor() {
        let mut emu = Emulator::new();

        load_instructions(
            &mut emu,
            &[
                encode_r_type(8, 9, 10, 0, 0x27), // nor $t2, $t0, $t1
            ],
        );
        set_reg(&mut emu, 8, 0xFF00);
        set_reg(&mut emu, 9, 0x00FF);
        emu.cpu_exec(1);
        assert_eq!(emu.cpu.gpr.read(10), 0xFFFF0000);
    }

    #[test]
    fn test_slt() {
        let mut emu = Emulator::new();

        load_instructions(
            &mut emu,
            &[
                encode_r_type(8, 9, 10, 0, 0x2A), // slt $t2, $t0, $t1
            ],
        );
        set_reg(&mut emu, 8, 3);
        set_reg(&mut emu, 9, 5);
        emu.cpu_exec(1);
        assert_eq!(emu.cpu.gpr.read(10), 1);

        load_instructions(
            &mut emu,
            &[
                encode_r_type(8, 9, 10, 0, 0x2A), // slt $t2, $t0, $t1
            ],
        );
        set_reg(&mut emu, 8, 5);
        set_reg(&mut emu, 9, 3);
        emu.cpu_exec(1);
        assert_eq!(emu.cpu.gpr.read(10), 0);
    }

    #[test]
    fn test_sltu() {
        let mut emu = Emulator::new();

        load_instructions(
            &mut emu,
            &[
                encode_r_type(8, 9, 10, 0, 0x2B), // sltu $t2, $t0, $t1
            ],
        );
        set_reg(&mut emu, 8, 0xFFFFFFFF);
        set_reg(&mut emu, 9, 1);
        emu.cpu_exec(1);
        assert_eq!(emu.cpu.gpr.read(10), 0);
    }

    #[test]
    fn test_sll() {
        let mut emu = Emulator::new();

        load_instructions(
            &mut emu,
            &[
                encode_r_type(0, 8, 10, 4, 0x00), // sll $t2, $t0, 4
            ],
        );
        set_reg(&mut emu, 8, 0x01);
        emu.cpu_exec(1);
        assert_eq!(emu.cpu.gpr.read(10), 0x10);
    }

    #[test]
    fn test_srl() {
        let mut emu = Emulator::new();

        load_instructions(
            &mut emu,
            &[
                encode_r_type(0, 8, 10, 4, 0x02), // srl $t2, $t0, 4
            ],
        );
        set_reg(&mut emu, 8, 0x10);
        emu.cpu_exec(1);
        assert_eq!(emu.cpu.gpr.read(10), 0x01);
    }

    #[test]
    fn test_sra() {
        let mut emu = Emulator::new();

        load_instructions(
            &mut emu,
            &[
                encode_r_type(0, 8, 10, 4, 0x03), // sra $t2, $t0, 4
            ],
        );
        set_reg(&mut emu, 8, 0x80000000);
        emu.cpu_exec(1);
        assert_eq!(emu.cpu.gpr.read(10), 0xF8000000);
    }

    #[test]
    fn test_sllv() {
        let mut emu = Emulator::new();

        load_instructions(
            &mut emu,
            &[
                encode_r_type(9, 8, 10, 0, 0x04), // sllv $t2, $t0, $t1
            ],
        );
        set_reg(&mut emu, 8, 0x01);
        set_reg(&mut emu, 9, 4);
        emu.cpu_exec(1);
        assert_eq!(emu.cpu.gpr.read(10), 0x10);
    }

    #[test]
    fn test_srlv() {
        let mut emu = Emulator::new();

        load_instructions(
            &mut emu,
            &[
                encode_r_type(9, 8, 10, 0, 0x06), // srlv $t2, $t0, $t1
            ],
        );
        set_reg(&mut emu, 8, 0x10);
        set_reg(&mut emu, 9, 4);
        emu.cpu_exec(1);
        assert_eq!(emu.cpu.gpr.read(10), 0x01);
    }

    #[test]
    fn test_srav() {
        let mut emu = Emulator::new();

        load_instructions(
            &mut emu,
            &[
                encode_r_type(9, 8, 10, 0, 0x07), // srav $t2, $t0, $t1
            ],
        );
        set_reg(&mut emu, 8, 0x80000000);
        set_reg(&mut emu, 9, 4);
        emu.cpu_exec(1);
        assert_eq!(emu.cpu.gpr.read(10), 0xF8000000);
    }

    #[test]
    fn test_mult() {
        let mut emu = Emulator::new();

        load_instructions(
            &mut emu,
            &[
                encode_r_type(8, 9, 0, 0, 0x18), // mult $t0, $t1
            ],
        );
        set_reg(&mut emu, 8, 3);
        set_reg(&mut emu, 9, 5);
        emu.cpu_exec(1);
        assert_eq!(emu.cpu.lo, 15);
        assert_eq!(emu.cpu.hi, 0);
    }

    #[test]
    fn test_div() {
        let mut emu = Emulator::new();

        load_instructions(
            &mut emu,
            &[
                encode_r_type(8, 9, 0, 0, 0x1A), // div $t0, $t1
            ],
        );
        set_reg(&mut emu, 8, 7);
        set_reg(&mut emu, 9, 2);
        emu.cpu_exec(1);
        assert_eq!(emu.cpu.lo, 3);
        assert_eq!(emu.cpu.hi, 1);
    }

    #[test]
    fn test_div_by_zero() {
        let mut emu = Emulator::new();

        load_instructions(
            &mut emu,
            &[
                encode_r_type(8, 9, 0, 0, 0x1A), // div $t0, $t1
            ],
        );
        set_reg(&mut emu, 8, 7);
        set_reg(&mut emu, 9, 0);
        emu.cpu_exec(1);
        assert_eq!(emu.state, crate::emu::cpu::exec::CpuState::End);
    }

    #[test]
    fn test_mfhi_mflo() {
        let mut emu = Emulator::new();

        load_instructions(
            &mut emu,
            &[
                encode_r_type(8, 9, 0, 0, 0x18),  // mult $t0, $t1
                encode_r_type(0, 0, 10, 0, 0x10), // mfhi $t2
                encode_r_type(0, 0, 11, 0, 0x12), // mflo $t3
            ],
        );
        set_reg(&mut emu, 8, 0x10000);
        set_reg(&mut emu, 9, 0x10000);
        emu.cpu_exec(3);
        assert_eq!(emu.cpu.gpr.read(10), 0x00000001);
        assert_eq!(emu.cpu.gpr.read(11), 0x00000000);
    }

    #[test]
    fn test_jr() {
        let mut emu = Emulator::new();

        load_instructions(
            &mut emu,
            &[
                encode_r_type(8, 0, 0, 0, 0x08), // jr $t0
            ],
        );
        set_reg(&mut emu, 8, 0xBFC00010);
        emu.cpu_exec(1);
        assert_eq!(emu.cpu.pc, 0xBFC00010);
    }

    #[test]
    fn test_jalr() {
        let mut emu = Emulator::new();

        load_instructions(
            &mut emu,
            &[
                encode_r_type(8, 0, 10, 0, 0x09), // jalr $t2, $t0
            ],
        );
        set_reg(&mut emu, 8, 0xBFC00010);
        emu.cpu_exec(1);
        assert_eq!(emu.cpu.pc, 0xBFC00010);
        assert_eq!(emu.cpu.gpr.read(10), 0x1FC00004);
    }
}
