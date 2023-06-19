use std::io::Write;

use colored::Colorize;
use super::cpu_exec::cpu_exec;

#[derive(PartialEq, Eq)]
pub enum UiState {
    OK, EXCEPTION, ERR, EXIT,
}

struct CMD{
    name: &'static str,
    description: &'static str,
    handler: fn(&String) -> UiState,
}

impl CMD {
    pub const fn from(name: &'static str, description: &'static str, handler: fn(&String)->UiState) -> Self{
        Self { name: name, description: description, handler: handler }
    }
}

const CMD_TABLE: [CMD; 3] = [
    CMD::from("help", "Display informations about all supported commands", cmd_help),
    CMD::from("c", "Continue the execution of the program", cmd_c),
    CMD::from("q", "Exit CPU", cmd_q),
];

fn cmd_help(args: &String) -> UiState{
    let args: Vec<&str> = args.trim().split(" ").collect();
    if args.len() <= 1 {
        for cmd in CMD_TABLE {
            println!("{}", format!("{} - {}", cmd.name, cmd.description).yellow());
        }
        return UiState::OK;
    }
    else{
        for cmd in CMD_TABLE {
            if cmd.name == args[1] {
                println!("{}", format!("{} - {}", cmd.name, cmd.description).yellow());
                return UiState::OK;
            }
        }
        return UiState::EXCEPTION;
    }
}

fn cmd_c(args: &String) -> UiState{
    unsafe{cpu_exec(u32::MAX);}
    UiState::OK
}

fn cmd_q(args: &String) -> UiState{
    println!("{}", "mips exit successfully ...".green());
    UiState::EXIT
}

pub fn ui_mainloop(){
    let mut state = UiState::OK;
    while state != UiState::EXIT {
        print!("{}", format!("(Azathoth)>>> ").green());
        std::io::stdout().flush().unwrap();
        let mut oper = String::new();
        std::io::stdin().read_line(&mut oper).unwrap();
        let oper = oper.trim();
        let arg: Vec<&str> = oper.split(" ").collect();
        let mut _runover = false;
        for cmd in CMD_TABLE{
            if arg[0] == cmd.name{
                state = (cmd.handler)(&String::from(oper));
                _runover = true;
                break;
            }
        }
        if !_runover {
            println!("{}", format!("Unknown Command '{}'", arg[0]).red());
        }
    }
}