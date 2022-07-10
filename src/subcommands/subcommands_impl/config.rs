#![allow(dead_code, unused_variables)]

use clap::ArgMatches;
use crate::subcommands::subcommand_strategy::CommandStrategy;
use crate::subcommands::commands_config::Args;
use crate::config::remote_config::RemoteConfig;
use crate::message::message::Message;
use crate::task::task::Success;
use crate::task_manager::task_manager::TaskManager;
use crate::config;
use crate::context::context::Context;
use crate::task::task_impl::config::update_config_files_task::UpdateConfigFilesTask;
use crate::term::log_level::LogLevel::L1;

pub struct Config {}

impl CommandStrategy for Config {
    fn start(command: &ArgMatches, context: &mut Context) -> Result<Success, Message> {
        let config = config::remote_config::get_remote_config()?;

        let default_value = "".to_string();

        match command.subcommand() {
            Some(("update", matches)) => {
                let force_arg = matches.contains_id(Args::FORCE._to_string());
                update_config(force_arg, &config, context)
            }
            _ => { Ok(Success {}) }
        }
    }
}

fn update_config(force: bool, config: &RemoteConfig, context: &mut Context) -> Result<Success, Message> {
    TaskManager {}.start(vec![
        Box::new(UpdateConfigFilesTask { force }),
    ], config, L1, context)
}