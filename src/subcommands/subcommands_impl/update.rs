#![allow(dead_code, unused_variables)]

use clap::ArgMatches;
use crate::{Command, Message, Success, Term};
use crate::config::remote_config::{RemoteConfig};
use crate::task::task_impl::update::check_update_task::{CheckUpdateData, CheckUpdateTask};
use crate::task_manager::task_manager::TaskManager;
use crate::term::log_level::LogLevel::L1;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Update{}

impl Command for Update{
    fn start(command: &ArgMatches, config: &RemoteConfig, term: &mut Term) -> Result<Success, Message> {

        TaskManager{}.start(vec![
            Box::new(CheckUpdateTask { input_data: CheckUpdateData { version: VERSION.to_string() } }),
        ], config, term, L1)
    }
}