#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod r#mod;
use r#mod::monitor::{
    monitor::restart,
    ui::{ui_mainloop, UiState},
};

fn main() {
    restart();
    while ui_mainloop() == UiState::RESTART {
        restart();
    }
}
