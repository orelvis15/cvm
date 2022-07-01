extern crate core;

use std::io::stdout;
use message::message::{MessageData, Message};
use subcommands::subcommands_impl;
use crate::subcommands::subcommand::Command;
use subcommands::subcommands_impl::init::Init;
use subcommands::subcommands_impl::install::Install;
use subcommands::subcommands_impl::list::List;
use subcommands::subcommands_impl::start::Start;
use subcommands::subcommands_impl::stop::Stop;
use crate::message::message::MessageKind;
use crate::subcommands_impl::clean::Clean;
use crate::subcommands_impl::config::Config;
use crate::subcommands_impl::r#use::Use;
use crate::subcommands_impl::remove::Remove;
use crate::subcommands_impl::update::Update;
use crate::task::task::Success;
use crate::term::term::Term;
use crate::utils::url_build::url_build;

mod task;
mod config;
mod env;
mod subcommands;
mod utils;
mod task_manager;
mod message;
mod term;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let config = config::remote_config::get_remote_config();

    if let Err(error) = &config {
        error.print();
        return;
    }
    let config = config.unwrap();

    let mut term = Term { stdout: stdout() };

    let _ =show_update_alert(&config.update.last_cvm_version, &VERSION.to_string());

    let args = subcommands::commands_config::command_config();

    let result = match args.subcommand() {
        Some(("init", matches)) => {
            Init::start(matches, &config, &mut term)
        }
        Some(("install", matches)) => {
            Install::start(matches, &config, &mut term)
        }
        Some(("use", matches)) => {
            Use::start(matches, &config, &mut term)
        }
        Some(("remove", matches)) => {
            Remove::start(matches, &config, &mut term)
        }
        Some(("clean", matches)) => {
            Clean::start(matches, &config, &mut term)
        }
        Some(("list", matches)) => {
            List::start(matches, &config, &mut term)
        }
        Some(("update", matches)) => {
            Update::start(matches, &config, &mut term)
        }
        Some(("start", matches)) => {
            Start::start(matches, &config, &mut term)
        }
        Some(("stop", matches)) => {
            Stop::start(matches, &config, &mut term)
        }
        Some(("config", matches)) => {
            Config::start(matches, &config, &mut term)
        }
        _ => { error_not_found() }
    };

    match result {
        Ok(_) => {
            //println!("{}", "Task completed successfully".green());
        }
        Err(error) => {
            error.print();
        }
    }
}

fn error_not_found() -> Result<Success, Message> {
    return Err(Message::CommandNotFound(MessageData {
        message: "Command not found".to_string(),
        ..Default::default()
    }));
}

fn show_update_alert(last_version: &String, current_version: &String) -> Result<Success, Message> {
    if &last_version > &current_version {
        return Err(Message::CommandNotFound(MessageData {
            message: format!("{} {} => {}", "New update available", &current_version, &last_version),
            kind: MessageKind::Info,
            ..Default::default()
        }));
    };
    Ok(Success{})
}