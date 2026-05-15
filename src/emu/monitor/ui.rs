use std::io::Write;

use crate::emu::cpu::exec::Emulator;
use crate::emu::cpu::reg::{get_id, REG_NAME};
use colored::Colorize;

#[derive(PartialEq, Eq)]
pub enum UiState {
    Ok,
    Exception,

    Exit,
    Restart,
}

pub fn dbg_option(emu: &mut Emulator) {
    if emu.debug {
        println!("{}", "DEBUG OFF".bright_blue());
    } else {
        println!("{}", "DEBUG ON".bright_blue());
    }
    emu.debug = !emu.debug;
}

struct Cmd {
    name: [&'static str; 2],
    description: &'static str,
    handler: fn(&mut Emulator, &[&str]) -> UiState,
}

impl Cmd {
    pub const fn from(
        name: [&'static str; 2],
        description: &'static str,
        handler: fn(&mut Emulator, &[&str]) -> UiState,
    ) -> Self {
        Self {
            name,
            description,
            handler,
        }
    }

    pub fn check_name(&self, name: &str) -> bool {
        self.name.contains(&name)
    }
}

const CMD_TABLE: [Cmd; 7] = [
    Cmd::from(
        ["help", "h"],
        "Display informations about all supported commands",
        cmd_help,
    ),
    Cmd::from(
        ["continue", "c"],
        "Continue the execution of the program",
        cmd_c,
    ),
    Cmd::from(["quit", "q"], "Exit CPU", cmd_q),
    Cmd::from(["single", "s"], "single steps", cmd_si),
    Cmd::from(["reg", "r"], "check reg", cmd_r),
    Cmd::from(["debug", "dbg"], "turn on/off debug option", cmd_dbg),
    Cmd::from(["restart", "re"], "restart cpu", cmd_re),
];

fn cmd_help(_emu: &mut Emulator, args: &[&str]) -> UiState {
    if args.len() <= 1 {
        for cmd in CMD_TABLE {
            println!(
                "{}",
                format!("{} [{}] - {}", cmd.name[0], cmd.name[1], cmd.description).yellow()
            );
        }
        UiState::Ok
    } else {
        for cmd in CMD_TABLE {
            if cmd.check_name(args[1]) {
                println!(
                    "{}",
                    format!("{} [{}] - {}", cmd.name[0], cmd.name[1], cmd.description).yellow()
                );
                return UiState::Ok;
            }
        }
        UiState::Exception
    }
}

fn cmd_c(emu: &mut Emulator, _args: &[&str]) -> UiState {
    emu.cpu_exec(u32::MAX);
    UiState::Ok
}

fn cmd_q(_emu: &mut Emulator, _args: &[&str]) -> UiState {
    println!("{}", "mips exit successfully ...".green());
    UiState::Exit
}

fn cmd_si(emu: &mut Emulator, _args: &[&str]) -> UiState {
    emu.cpu_exec(1);
    UiState::Ok
}

fn cmd_r(emu: &mut Emulator, args: &[&str]) -> UiState {
    if args.len() <= 1 {
        for (idx, reg) in REG_NAME.iter().enumerate() {
            println!("{}: 0x{:08x}", reg, emu.cpu.gpr.read(idx));
        }
    } else if args[1].starts_with('$') {
        println!("{}: 0x{:08x}", args[1], emu.cpu.gpr.read(get_id(args[1])));
    } else {
        let reg = format!("${}", args[1]);
        println!("{}: 0x{:08x}", reg, emu.cpu.gpr.read(get_id(&reg)));
    }
    UiState::Ok
}

fn cmd_dbg(emu: &mut Emulator, _args: &[&str]) -> UiState {
    dbg_option(emu);
    UiState::Ok
}

fn cmd_re(_emu: &mut Emulator, _args: &[&str]) -> UiState {
    UiState::Restart
}

pub fn ui_mainloop(emu: &mut Emulator) -> UiState {
    let mut state = UiState::Ok;
    while state != UiState::Exit && state != UiState::Restart {
        print!("{}", "(Azathoth)>>> ".green());
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let args: Vec<&str> = input.trim().split(' ').collect();
        let mut handled = false;
        for cmd in CMD_TABLE {
            if cmd.check_name(args[0]) {
                state = (cmd.handler)(emu, &args);
                handled = true;
                break;
            }
        }
        if !handled {
            println!("{}", format!("Unknown Command '{}'", args[0]).red());
        }
    }
    state
}
