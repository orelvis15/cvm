#![allow(dead_code, unused_variables)]

use clap::ArgMatches;
use crate::{CvmError, Success};
use crate::config::config::{Config, get_config};
use crate::task::task_impl::check_update_task::{CheckUpdateData, CheckUpdateTask};
use crate::task::task_manager;

pub fn start(_command: &ArgMatches, current_version: String, config: &Config) -> Result<Success, CvmError> {

    let config = get_config();
    if let Err(error) = config {
        return Err(error);
    }

    task_manager::start(vec![
        Box::new(CheckUpdateTask { input_data: CheckUpdateData { version: current_version } }),
    ])
}