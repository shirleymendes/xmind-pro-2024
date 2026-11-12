//! Minimal command-line interface for the Xmind Pro 2024 helper binary.

use std::env;

pub enum Command {
    Version,
    Help,
    Install,
    Repair,
    Uninstall,
    Unknown(String),
}

pub fn parse() -> Command {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.first().map(|s| s.as_str()).unwrap_or("") {
        "--version" | "-v" => Command::Version,
        "--help"    | "-h" => Command::Help,
        "install"          => Command::Install,
        "repair"           => Command::Repair,
        "uninstall"        => Command::Uninstall,
        other              => Command::Unknown(other.to_string()),
    }
}

pub fn help_text() -> &'static str {
    "Xmind Pro 2024 v1.5.6 - Windows helper\n\
     \n\
     USAGE:\n\
         xmind-pro-2024-helper <COMMAND>\n\
     \n\
     COMMANDS:\n\
         install    Install or update Xmind Pro 2024\n\
         repair     Repair a broken installation\n\
         uninstall  Remove Xmind Pro 2024 and its config\n\
         --version  Print version and exit\n\
         --help     Show this help\n"
}
