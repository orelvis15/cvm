extern crate core;

use clap::ArgMatches;
use crossterm::style::Stylize;
use message::message::{MessageData, Message};
use subcommands::subcommands_impl;
use crate::subcommands::subcommand_strategy::CommandStrategy;
use subcommands::subcommands_impl::init::Init;
use subcommands::subcommands_impl::install::Install;
use subcommands::subcommands_impl::list::List;
use subcommands::subcommands_impl::start::Start;
use subcommands::subcommands_impl::stop::Stop;
use crate::subcommands::commands_config::CommandsConfig;
use crate::subcommands_impl::clean::Clean;
use crate::subcommands_impl::config::Config;
use crate::subcommands_impl::r#use::Use;
use crate::subcommands_impl::remove::Remove;
use crate::subcommands_impl::update::Update;
use crate::task::task::Success;
use crate::term::term::Term;
use crate::utils::url_build::url_build;
use crate::utils::version_utils::get_last_cvm_version;

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
    if check_and_apply_update(VERSION.to_string()){
        return;
    }

    let args = subcommands::commands_config::command_config();
    let result = match args.subcommand() {
        Some(("init", matches)) => {
            Init::start(matches)
        }
        Some(("install", matches)) => {
            Install::start(matches)
        }
        Some(("use", matches)) => {
            Use::start(matches)
        }
        Some(("remove", matches)) => {
            Remove::start(matches)
        }
        Some(("clean", matches)) => {
            Clean::start(matches)
        }
        Some(("ls", matches)) => {
            List::start(matches)
        }
        Some(("update", matches)) => {
            Update::start(matches)
        }
        Some(("start", matches)) => {
            Start::start(matches)
        }
        Some(("stop", matches)) => {
            Stop::start(matches)
        }
        Some(("config", matches)) => {
            Config::start(matches)
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

fn check_and_apply_update(current_version: String) -> bool {
    let last_version = get_last_cvm_version().unwrap_or("".to_string());
    let last_version = last_version.replace("v", "");
    if &last_version > &current_version {
        print!("{}\n", format!("{} {} => {}", "New update available".yellow(), &current_version.blue(), &last_version.yellow()));
        let update_result = Update::start(&ArgMatches::default());
        if let Err(result) = update_result {
            result.print();
        }
        return true;
    }
    false
}