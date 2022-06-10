extern crate core;

use owo_colors::OwoColorize;
use crate::terminal::config::CommandsConfig;
use error::error::{Error, Message};
use terminal::subcommands_impl;
use crate::terminal::subcommand::Command;
use terminal::subcommands_impl::init::Init;
use terminal::subcommands_impl::install::Install;
use terminal::subcommands_impl::list::List;
use terminal::subcommands_impl::start::Start;
use terminal::subcommands_impl::stop::Stop;
use crate::subcommands_impl::r#use::Use;
use crate::subcommands_impl::update::Update;
use crate::task::task::Success;
use crate::task::task_type::TaskType::EmptyTask;
use crate::utils::url_build::url_build;

mod task;
mod config;
mod env;
mod terminal;
mod utils;
mod task_manager;
mod error;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    println!("{}", VERSION);
    let config = config::config::get_config();

    if let Err(error) = &config {
        error.print();
        return;
    }
    let config = config.unwrap();

    show_update_alert(&config.update.last_cvm_version, &VERSION.to_string());

    let args = terminal::config::command_config();
    let result = match args.subcommand() {
        Some(("init", matches)) => {
            match CommandsConfig::INIT.is_enable(&config.commands_item) {
                Ok(_) => { Init::start(matches, &config) }
                Err(error) => { Err(error) }
            }
        }
        Some(("install", matches)) => {
            match CommandsConfig::INSTALL.is_enable(&config.commands_item) {
                Ok(_) => { Install::start(matches, &config) }
                Err(error) => { Err(error) }
            }
        }
        Some(("use", matches)) => {
            match CommandsConfig::USE.is_enable(&config.commands_item) {
                Ok(_) => { Use::start(matches, &config) }
                Err(error) => { Err(error) }
            }
        }
        Some(("list", matches)) => {
            match CommandsConfig::LIST.is_enable(&config.commands_item) {
                Ok(_) => { List::start(matches, &config) }
                Err(error) => { Err(error) }
            }
        }
        Some(("update", matches)) => {
            match CommandsConfig::UPDATE.is_enable(&config.commands_item) {
                Ok(_) => { Update::start(matches, &config) }
                Err(error) => { Err(error) }
            }
        }
        Some(("start", matches)) => {
            match CommandsConfig::START.is_enable(&config.commands_item) {
                Ok(_) => { Start::start(matches, &config) }
                Err(error) => { Err(error) }
            }
        }
        Some(("stop", matches)) => {
            match CommandsConfig::STOP.is_enable(&config.commands_item) {
                Ok(_) => { Stop::start(matches, &config) }
                Err(error) => { Err(error) }
            }
        }
        _ => { error_not_found() }
    };

    match result {
        Ok(_) => {
            println!("{}", "Task completed successfully".black().on_green());
        }
        Err(error) => {
            error.print();
        }
    }
}

fn error_not_found() -> Result<Success, Message> {
    return Err(Message::CommandNotFound(Error {
        message: "Command not found".to_string(),
        task: EmptyTask("".to_string()),
        stack: vec![],
    }));
}

fn show_update_alert(last_version: &String, current_version: &String) {
    if &last_version > &current_version {
        println!("{} {} => {}", "New update available".blue(), &current_version.blue(), &last_version.green());
    };
}