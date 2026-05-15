use crate::emu::cpu::{
    helper::*, i_type::*, j_type::*, operand::Operands, r_type::*, reg::CpuRegs,
};
use crate::emu::memory::dram::Memory;
use colored::Colorize;

/// CPU 执行状态。
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum CpuState {
    Stop,
    Running,
    End,
}

/// MIPS 模拟器核心，聚合 CPU、内存与执行状态。
pub struct Emulator {
    pub cpu: CpuRegs,
    pub memory: Memory,
    pub state: CpuState,
    pub pc_updated: bool,
    pub assembly: String,
    pub asm_buf: String,
    pub debug: bool,
    pub instr: u32,
    pub ops: Operands,
}

impl Emulator {
    pub fn new() -> Self {
        Self {
            cpu: CpuRegs::new(),
            memory: Memory::new(),
            state: CpuState::Stop,
            pc_updated: false,
            assembly: String::new(),
            asm_buf: String::new(),
            debug: false,
            instr: 0,
            ops: Operands::new(),
        }
    }

    pub fn instr_fetch(&mut self, addr: u32, len: usize) -> u32 {
        self.memory.read(addr, len)
    }

    pub fn cpu_exec(&mut self, n: u32) {
        if self.state == CpuState::End {
            println!(
                "{}",
                "Program execution has ended. To restart the program, exit TEMU and run again.\n"
                    .blue()
            );
            return;
        }
        self.state = CpuState::Running;

        for _ in 0..n {
            if self.state == CpuState::End {
                break;
            }
            let pc = self.cpu.pc;
            let masked_pc = pc & 0x1F_FF_FF_FF;
            self.assembly.clear();
            self.pc_updated = false;
            self.exec(masked_pc);
            if !self.pc_updated {
                self.cpu.pc += 4;
            }

            self.print_bin_instr(masked_pc);
            self.asm_buf.push_str(" => ");
            self.asm_buf.push_str(&self.assembly);
            println!("{}", self.asm_buf.blue());

            if self.state != CpuState::Running {
                return;
            }
        }

        if self.state == CpuState::Running {
            self.state = CpuState::Stop;
        }
    }

    fn print_bin_instr(&mut self, pc: u32) {
        self.asm_buf.clear();
        self.asm_buf = format!("{:08x}:   ", pc);
        for i in (0..4).rev() {
            let byte = self.instr_fetch(pc + i, 1);
            self.asm_buf.push_str(&format!("{:02x} ", byte));
        }
    }

    pub fn exec(&mut self, pc: u32) {
        self.instr = self.instr_fetch(pc, 4);
        self.ops.opcode = self.instr >> 26;
        self.dbg_println(format!("[DEBUG] func1: {:02x}", self.ops.opcode));
        OPCODE_TABLE[self.ops.opcode as usize](self, pc);
    }

    pub fn _2byte_esc(&mut self, pc: u32) {
        self.ops.func = self.instr & FUNC_MASK;
        self.dbg_println(format!("[DEBUG] func2: {:02x}", self.ops.func));
        _2BYTE_OPCODE_TABLE[self.ops.func as usize](self, pc);
    }

    pub fn inv(&mut self, pc: u32) {
        let p = self.instr.to_be_bytes();
        println!(
            "{}",
            format!(
                "invalid opcode(pc = 0x{:08x}): {:02x} {:02x} {:02x} {:02x} ...",
                pc, p[3], p[2], p[1], p[0]
            )
            .red()
        );
        self.state = CpuState::End;
    }

    pub fn good_trap(&mut self, pc: u32) {
        println!(
            "{}",
            format!("temu: HIT GOOD TRAP at $pc = 0x{:08x}", pc).green()
        );
        self.state = CpuState::End;
    }

    pub fn bad_trap(&mut self, pc: u32) {
        println!(
            "{}",
            format!("temu: HIT BAD TRAP at $pc = 0x{:08x}", pc).red()
        );
        self.state = CpuState::End;
    }

    pub fn dbg_println(&self, msg: String) {
        if self.debug {
            println!("{}", msg.yellow());
        }
    }
}

// 将实例方法 coerced 为函数指针需要显式写出类型，
// 但 Rust 允许 `fn(&mut Emulator, u32)` 指向 `impl Emulator { fn xxx(&mut self, u32) }`。
type InstrFn = fn(&mut Emulator, u32);

const OPCODE_TABLE: [InstrFn; 64] = [
    /* 0x00 */ Emulator::_2byte_esc,
    bz,
    j,
    jal,
    /* 0x04 */ beq,
    bne,
    blez,
    bgtz,
    /* 0x08 */ addi,
    addiu,
    slti,
    sltiu,
    /* 0x0c */ andi,
    ori,
    xori,
    lui,
    /* 0x10 */ eret,
    Emulator::inv,
    Emulator::good_trap,
    Emulator::bad_trap,
    /* 0x14 */ Emulator::inv,
    Emulator::inv,
    Emulator::inv,
    Emulator::inv,
    /* 0x18 */ Emulator::inv,
    Emulator::inv,
    Emulator::inv,
    Emulator::inv,
    /* 0x1c */ Emulator::inv,
    Emulator::inv,
    Emulator::inv,
    Emulator::inv,
    /* 0x20 */ lb,
    lh,
    Emulator::inv,
    lw,
    /* 0x24 */ lbu,
    lhu,
    Emulator::inv,
    Emulator::inv,
    /* 0x28 */ sb,
    sh,
    Emulator::inv,
    sw,
    /* 0x2c */ Emulator::inv,
    Emulator::inv,
    Emulator::inv,
    Emulator::inv,
    /* 0x30 */ Emulator::inv,
    Emulator::inv,
    Emulator::inv,
    Emulator::inv,
    /* 0x34 */ Emulator::inv,
    Emulator::inv,
    Emulator::inv,
    Emulator::inv,
    /* 0x38 */ Emulator::inv,
    Emulator::inv,
    Emulator::inv,
    Emulator::inv,
    /* 0x3c */ Emulator::inv,
    Emulator::inv,
    Emulator::inv,
    Emulator::inv,
];

const _2BYTE_OPCODE_TABLE: [InstrFn; 64] = [
    /* 0x00 */ sll,
    Emulator::inv,
    srl,
    sra,
    /* 0x04 */ sllv,
    Emulator::inv,
    srlv,
    srav,
    /* 0x08 */ jr,
    jalr,
    Emulator::inv,
    Emulator::inv,
    /* 0x0c */ syscall,
    _break,
    Emulator::inv,
    Emulator::inv,
    /* 0x10 */ mfhi,
    mthi,
    mflo,
    mtlo,
    /* 0x14 */ Emulator::inv,
    Emulator::inv,
    Emulator::inv,
    Emulator::inv,
    /* 0x18 */ mult,
    multu,
    div,
    divu,
    /* 0x1c */ Emulator::inv,
    Emulator::inv,
    Emulator::inv,
    Emulator::inv,
    /* 0x20 */ add,
    addu,
    sub,
    subu,
    /* 0x24 */ and,
    or,
    xor,
    nor,
    /* 0x28 */ Emulator::inv,
    Emulator::inv,
    slt,
    sltu,
    /* 0x2c */ Emulator::inv,
    Emulator::inv,
    Emulator::inv,
    Emulator::inv,
    /* 0x30 */ Emulator::inv,
    Emulator::inv,
    Emulator::inv,
    Emulator::inv,
    /* 0x34 */ Emulator::inv,
    Emulator::inv,
    Emulator::inv,
    Emulator::inv,
    /* 0x38 */ Emulator::inv,
    Emulator::inv,
    Emulator::inv,
    Emulator::inv,
    /* 0x3c */ Emulator::inv,
    Emulator::inv,
    Emulator::inv,
    Emulator::inv,
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::emu::cpu::reg::REG_NAME;
    use crate::emu::monitor::system::load_entry;

    #[test]
    fn inv_trap() {
        let mut emu = Emulator::new();
        load_entry(&mut emu);
        let pc = 0xbfc0_0000 & 0x1F_FF_FF_FF;
        println!("0x{:08x}: {:08x}", pc, emu.memory.read(pc, 4));
        emu.inv(pc);
        assert_eq!(emu.state, CpuState::End);
        emu.good_trap(pc);
        assert_eq!(emu.state, CpuState::End);
        emu.bad_trap(pc);
        assert_eq!(emu.state, CpuState::End);
    }
}
