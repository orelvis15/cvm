#![allow(dead_code, unused_variables)]

use clap::ArgMatches;
use crate::{Message, Success};
use crate::config::config::{Config};
use crate::task::task_impl::update::check_update_task::{CheckUpdateData, CheckUpdateTask};
use crate::task_manager::task_manager::TaskManager;


pub fn start(_command: &ArgMatches, current_version: String, config: &Config) -> Result<Success, Message> {
    TaskManager::start(vec![
        Box::new(CheckUpdateTask { input_data: CheckUpdateData { version: current_version } }),
    ], config)
}