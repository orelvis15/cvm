extern crate core;

use owo_colors::OwoColorize;
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
            commands::init::start(matches, &config)
        }
        Some(("install", matches)) => {
            commands::install::start(matches, &config)
        }
        Some(("use", matches)) => {
            commands::r#use::start(matches, &config)
        }
        Some(("list", matches)) => {
            commands::list::start(matches, &config)
        }
        Some(("update", matches)) => {
            commands::update::start(matches, current_version, &config)
        }
        _ => { print_error() }
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

fn print_error() -> Result<Success, CvmError> {
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