#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod r#mod;
use r#mod::{
    cpu::{

    },
    memory::{

    },
    monitor::{
        monitor::restart,
        ui::{ui_mainloop},
    },
};

fn main() {
    restart();
    ui_mainloop();
}
