use std::io::Read;

use crate::emu::cpu::exec::{CpuState, Emulator};
use colored::Colorize;

const ENTRY_START: u32 = 0xBF_C0_00_00;

pub fn init_monitor() {
    println!("{}", "Hello World.".green());
}

pub fn load_entry(emu: &mut Emulator) {
    let inst_file = "./bin/inst.bin";
    let mut buf = Vec::new();
    let mut f =
        std::fs::File::open(inst_file).unwrap_or_else(|_| panic!("Can not open '{}'", inst_file));
    f.read_to_end(&mut buf).unwrap();
    emu.memory.load_binary(&buf, ENTRY_START & 0x1F_FF_FF_FF);
    println!("load {}", inst_file.green());

    buf.clear();
    let data_file = "./bin/data.bin";
    let mut f =
        std::fs::File::open(data_file).unwrap_or_else(|_| panic!("Can not open '{}'", data_file));
    f.read_to_end(&mut buf).unwrap();
    emu.memory.load_binary(&buf, 0);
    println!("load {}", data_file.green());
}

pub fn restart(emu: &mut Emulator) {
    init_monitor();
    *emu = Emulator::new();
    load_entry(emu);
    emu.cpu.pc = ENTRY_START;
    emu.state = CpuState::Stop;
}
