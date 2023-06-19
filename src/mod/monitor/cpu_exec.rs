use colored::Colorize;

use super::super::cpu::{exec::{exec, instr_fetch}, reg::CPU};

#[derive(PartialEq, Eq)]
pub enum CpuState{
    STOP, RUNNING, END,
}

pub static mut CPU: CPU = CPU::new();
pub static mut CPU_STATE: CpuState = CpuState::STOP;
static mut ASSEMBLY: String = String::new();
static mut ASM_BUF: String = String::new();

pub fn print_bin_instr(pc: u32) {
    unsafe{
        ASM_BUF.clear();
        ASM_BUF = format!("{:08x}:   ", pc);
        for i in 3..=0 {
            ASM_BUF += format!("{:02x} ", instr_fetch(pc + i, 1)).as_str();
        }
        // sprintf(asm_buf + l, "%*.s", 8, "");
    }
}

pub unsafe fn cpu_exec(n: u32){
    if CPU_STATE == CpuState::END {
        println!("{}", "Program execution has ended. To restart the program, exit TEMU and run again.\n".blue());
        return;
    }
    CPU_STATE = CpuState::RUNNING;

    while CPU_STATE != CpuState::END {
        let pc = CPU.pc & 0x1F_FF_FF_FF;
        exec(pc);
        CPU.pc += 4;

        print_bin_instr(pc);
        ASM_BUF += ASSEMBLY.as_str();
        
        if CPU_STATE != CpuState::RUNNING {
            return;
        }
    }

    if CPU_STATE == CpuState::RUNNING {
        CPU_STATE = CpuState::STOP;
    }
}