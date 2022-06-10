extern crate core;

use owo_colors::OwoColorize;
use crate::commands::config::CommandsConfig;
use crate::task::cvm_error::{CvmError, Error};
use crate::task::task::Success;
use crate::task::task_type::TaskType::EmptyTask;
use crate::utils::url_build::url_build;

mod task;
mod config;
mod env;
mod commands;
mod utils;

fn main() {
    let config = config::config::get_config();
    let current_version = commands::config::get_version();

    if let Err(error) = &config {
        error.print();
        return;
    }
    let config = config.unwrap();

    show_update_alert(&config.update.last_cvm_version, &current_version);

    let args = commands::config::command_config();
    let result = match args.subcommand() {
        Some(("init", matches)) => {
            match CommandsConfig::INIT.is_enable(&config.commands_item) {
                Ok(_) => { commands::init::start(matches, &config) }
                Err(error) => { Err(error) }
            }
        }
        Some(("install", matches)) => {
            match CommandsConfig::INSTALL.is_enable(&config.commands_item) {
                Ok(_) => { commands::install::start(matches, &config) }
                Err(error) => { Err(error) }
            }
        }
        Some(("use", matches)) => {
            match CommandsConfig::USE.is_enable(&config.commands_item) {
                Ok(_) => { commands::r#use::start(matches, &config) }
                Err(error) => { Err(error) }
            }
        }
        Some(("list", matches)) => {
            match CommandsConfig::LIST.is_enable(&config.commands_item) {
                Ok(_) => { commands::list::start(matches, &config) }
                Err(error) => { Err(error) }
            }
        }
        Some(("update", matches)) => {
            match CommandsConfig::UPDATE.is_enable(&config.commands_item) {
                Ok(_) => { commands::update::start(matches, current_version, &config) }
                Err(error) => { Err(error) }
            }
        }
        Some(("start", matches)) => {
            match CommandsConfig::START.is_enable(&config.commands_item) {
                Ok(_) => { commands::start::start(matches, &config) }
                Err(error) => { Err(error) }
            }
        }
        Some(("stop", matches)) => {
            match CommandsConfig::STOP.is_enable(&config.commands_item) {
                Ok(_) => { commands::stop::start(matches, &config) }
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

fn error_not_found() -> Result<Success, CvmError> {
    return Err(CvmError::CommandNotFound(Error {
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