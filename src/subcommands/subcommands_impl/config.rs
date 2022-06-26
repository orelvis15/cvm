#![allow(dead_code, unused_variables)]

use clap::ArgMatches;
use crate::subcommands::subcommand::Command;
use crate::subcommands::config::Args;
use crate::config::remote_config::RemoteConfig;
use crate::error::message::Message;
use crate::task::task::Success;
use crate::task_manager::task_manager::TaskManager;
use crate::Term;
use crate::task::task_impl::config::update_config_files_task::UpdateConfigFilesTask;
use crate::term::log_level::LogLevel::L1;


pub struct Config {}

impl Command for Config {
    fn start<'a>(command: &'a ArgMatches, config: &RemoteConfig, term: &mut Term) -> Result<Success, Message> {
        let default_value = "".to_string();

        match command.subcommand() {
            Some(("update", matches)) => {
                let force_arg = matches.contains_id(Args::FORCE._to_string());
                update_config(force_arg, config, term)
            }
            _ => { Ok(Success {}) }
        }
    }
}

fn update_config(force: bool, config: &RemoteConfig, term: &mut Term) -> Result<Success, Message> {
    TaskManager {}.start(vec![
        Box::new(UpdateConfigFilesTask { force }),
    ], config, term, L1)
}