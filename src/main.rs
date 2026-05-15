mod emu;

use emu::cpu::exec::Emulator;
use emu::monitor::{system::restart, ui::ui_mainloop};

fn main() {
    let mut emu = Emulator::new();
    restart(&mut emu);
    while ui_mainloop(&mut emu) == emu::monitor::ui::UiState::Restart {
        restart(&mut emu);
    }
}
