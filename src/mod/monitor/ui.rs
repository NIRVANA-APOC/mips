use std::io::Write;

use crate::r#mod::cpu::reg::REG_NAME;
use colored::Colorize;

use super::super::cpu::reg::get_id;
use super::cpu_exec::{cpu_exec, CPU};

#[derive(PartialEq, Eq)]
pub enum UiState {
    OK,
    EXCEPTION,
    ERR,
    EXIT,
    RESTART,
}

static mut DEBUG_FLAG: bool = false;
pub fn dbg_option() {
    unsafe {
        if DEBUG_FLAG {
            println!("{}", "DEBUG OFF".bright_blue());
        } else {
            println!("{}", "DEBUG ON".bright_blue());
        }
        DEBUG_FLAG = !DEBUG_FLAG;
    }
}

pub fn dbg_println(msg: String) {
    unsafe {
        if DEBUG_FLAG {
            println!("{}", msg.yellow());
        }
    }
}

struct CMD {
    name: [&'static str; 2],
    description: &'static str,
    handler: fn(&Vec<&str>) -> UiState,
}

impl CMD {
    pub const fn from(
        name: [&'static str; 2],
        description: &'static str,
        handler: fn(&Vec<&str>) -> UiState,
    ) -> Self {
        Self {
            name: name,
            description: description,
            handler: handler,
        }
    }

    pub fn check_name(&self, name: &str) -> bool{
        for n in self.name {
            if n == name {
                return true;
            }
        }
        false
    }
}

const CMD_TABLE: [CMD; 7] = [
    CMD::from(
        ["help", "h"],
        "Display informations about all supported commands",
        cmd_help,
    ),
    CMD::from(["continue", "c"], "Continue the execution of the program", cmd_c),
    CMD::from(["quit", "q"], "Exit CPU", cmd_q),
    CMD::from(["single", "s"], "single steps", cmd_si),
    CMD::from(["reg", "r"], "check reg", cmd_r),
    CMD::from(["debug", "dbg"], "turn on/off debug option", cmd_dbg),
    CMD::from(["restart", "re"], "restart cpu", cmd_re),
];

fn cmd_help(args: &Vec<&str>) -> UiState {
    if args.len() <= 1 {
        for cmd in CMD_TABLE {
            println!("{}", format!("{} [{}] - {}", cmd.name[0], cmd.name[1], cmd.description).yellow());
        }
        return UiState::OK;
    } else {
        for cmd in CMD_TABLE {
            if cmd.check_name(args[1]) {
                println!("{}", format!("{} [{}] - {}", cmd.name[0], cmd.name[1], cmd.description).yellow());
                return UiState::OK;
            }
        }
        return UiState::EXCEPTION;
    }
}

fn cmd_c(args: &Vec<&str>) -> UiState {
    unsafe {
        cpu_exec(u32::MAX);
    }
    UiState::OK
}

fn cmd_q(args: &Vec<&str>) -> UiState {
    println!("{}", "mips exit successfully ...".green());
    UiState::EXIT
}

fn cmd_si(args: &Vec<&str>) -> UiState {
    unsafe { cpu_exec(1) };
    UiState::OK
}

fn cmd_r(args: &Vec<&str>) -> UiState {
    unsafe {
        if args.len() <= 1 {
            for (idx, reg) in REG_NAME.iter().enumerate() {
                println!("{}", format!("{}: 0x{:08x}", reg, CPU.gpr.reg_w(idx)));
            }
        } else {
            if args[1].starts_with("$") {
                println!(
                    "{}",
                    format!("{}: 0x{:08x}", args[1], CPU.gpr.reg_w(get_id(args[1])))
                );
            } else {
                let mut reg = String::from("$");
                reg += args[1];
                println!(
                    "{}",
                    format!("{}: 0x{:08x}", reg, CPU.gpr.reg_w(get_id(reg.as_str())))
                );
            }
        }
    }
    UiState::OK
}

fn cmd_x(args: &Vec<&str>) -> UiState {
    let addr = args[1];
    UiState::OK
}

fn cmd_dbg(args: &Vec<&str>) -> UiState {
    dbg_option();
    UiState::OK
}

fn cmd_re(args: &Vec<&str>) -> UiState {
    UiState::RESTART
}

pub fn ui_mainloop() -> UiState {
    let mut state = UiState::OK;
    while state != UiState::EXIT && state != UiState::RESTART {
        print!("{}", format!("(Azathoth)>>> ").green());
        std::io::stdout().flush().unwrap();
        let mut args = String::new();
        std::io::stdin().read_line(&mut args).unwrap();
        let args: Vec<&str> = args.trim().split(" ").collect();
        let mut _runover = false;
        for cmd in CMD_TABLE {
            if cmd.check_name(args[0]) {
                state = (cmd.handler)(&args);
                _runover = true;
                break;
            }
        }
        if !_runover {
            println!("{}", format!("Unknown Command '{}'", args[0]).red());
        }
    }
    state
}
