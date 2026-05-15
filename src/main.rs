#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod emu;
use emu::monitor::{
    monitor::restart,
    ui::{ui_mainloop, UiState},
};

fn main() {
    restart();
    while ui_mainloop() == UiState::RESTART {
        restart();
    }
}
