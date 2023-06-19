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
        monitor::{init_monitor, load_entry},
        ui::{ui_mainloop},
    },
};

fn main() {
    // load_entry();
    ui_mainloop();
}
