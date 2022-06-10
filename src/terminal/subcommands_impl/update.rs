#![allow(dead_code, unused_variables)]

use clap::ArgMatches;
use crate::{Command, Message, Success};
use crate::config::config::{Config};
use crate::task::task_impl::update::check_update_task::{CheckUpdateData, CheckUpdateTask};
use crate::task_manager::task_manager::TaskManager;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Update{}

impl Command for Update{
    fn start(command: &ArgMatches, config: &Config) -> Result<Success, Message> {
        TaskManager::start(vec![
            Box::new(CheckUpdateTask { input_data: CheckUpdateData { version: VERSION.to_string() } }),
        ], config)
    }
}