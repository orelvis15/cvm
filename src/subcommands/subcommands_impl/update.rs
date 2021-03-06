#![allow(dead_code, unused_variables)]

use std::io::stdout;
use clap::ArgMatches;
use crate::{CommandStrategy, Message, Success, Term};
use crate::config::remote_config::RemoteConfig;
use crate::task::task_impl::update::check_update_task::{CheckUpdateData, CheckUpdateTask};
use crate::task_manager::task_manager::TaskManager;
use crate::term::log_level::LogLevel::L1;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Update{}

impl CommandStrategy for Update{
    fn start(command: &ArgMatches) -> Result<Success, Message> {

        let mut term = Term { stdout: stdout() };

        TaskManager::default().start(vec![
            Box::new(CheckUpdateTask { input_data: CheckUpdateData { old_version: VERSION.to_string(),..Default::default() } }),
        ], &RemoteConfig::default(), &mut term, L1)
    }
}