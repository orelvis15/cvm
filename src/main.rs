extern crate directories;
extern crate core;

use owo_colors::OwoColorize;
use crate::commands::config::Commands;
use crate::task::message_type::MessageType;
use crate::task::task::{Message, Success};
use crate::utils::url_build::url_build;

mod task;
mod config;
mod env;
mod commands;
mod utils;

fn main() {
    let config = config::config::get_config();

    if let Err(error) = config {
        println!("{}", error.to_string());
        return;
    }

    let args = commands::config::command_config();
    let result = match args.subcommand_name() {
        Some("init") => {
            let subcommand = args.subcommand_matches(Commands::INIT.to_string());
            commands::init::start(subcommand.unwrap())
        }
        Some("install") => {
            let subcommand = args.subcommand_matches(Commands::INSTALL.to_string());
            commands::install::start(subcommand.unwrap())
        },
        Some("use") => {
            let subcommand = args.subcommand_matches(Commands::USE.to_string());
            commands::r#use::start(subcommand.unwrap())
        },
        Some("list") => {
            let subcommand = args.subcommand_matches(Commands::LIST.to_string());
            commands::list::start(subcommand.unwrap())
        },
        Some("update") => {
            let subcommand = args.subcommand_matches(Commands::UPDATE.to_string());
            commands::update::start(subcommand.unwrap(), commands::config::get_version())
        }
        _ => { print_error() }
    };

    match result {
        Ok(_) => {
            println!("{}", "Task completed successfully".black().on_green());
        }
        Err(error) => {
            println!("{}", error.to_string());
        }
    }
}

fn print_error() -> Result<Success, Message> {
    return Err(Message {
        code: 0,
        message: "Command not found".to_string(),
        kind: MessageType::Error,
        task: "".to_string(),
        stack: vec![],
    });
}