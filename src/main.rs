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
    let current_version = commands::config::get_version();

    if let Err(error) = config {
        println!("{}", error.to_string());
        return;
    }

    show_update_alert(&config.as_ref().unwrap().update.last_cvm_version, &current_version);

    let args = commands::config::command_config();
    let result = match args.subcommand_name() {
        Some("init") => {
            let subcommand = args.subcommand_matches(Commands::INIT.to_string());
            commands::init::start(subcommand.unwrap())
        }
        Some("install") => {
            let subcommand = args.subcommand_matches(Commands::INSTALL.to_string());
            commands::install::start(subcommand.unwrap())
        }
        Some("use") => {
            let subcommand = args.subcommand_matches(Commands::USE.to_string());
            commands::r#use::start(subcommand.unwrap())
        }
        Some("list") => {
            let subcommand = args.subcommand_matches(Commands::LIST.to_string());
            commands::list::start(subcommand.unwrap())
        }
        Some("update") => {
            let subcommand = args.subcommand_matches(Commands::UPDATE.to_string());
            commands::update::start(subcommand.unwrap(), current_version)
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

fn show_update_alert(last_version: &String, current_version: &String) {
    if &last_version > &current_version {
        println!("{} {} => {}", "New update available".blue(), &current_version.blue(), &last_version.green());
    };
}