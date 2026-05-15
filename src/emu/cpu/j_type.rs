use crate::emu::cpu::exec::Emulator;
use crate::emu::cpu::helper::INDEX_MASK;

pub fn j(emu: &mut Emulator, pc: u32) {
    let target = (pc & 0xF000_0000) | ((emu.instr & INDEX_MASK) << 2);
    emu.cpu.pc = target;
    emu.pc_updated = true;
    emu.assembly = format!("j     0x{:08x}", target);
}

pub fn jal(emu: &mut Emulator, pc: u32) {
    let target = (pc & 0xF000_0000) | ((emu.instr & INDEX_MASK) << 2);
    emu.cpu.gpr.write(31, pc + 4);
    emu.cpu.pc = target;
    emu.pc_updated = true;
    emu.assembly = format!("jal   0x{:08x}", target);
}
