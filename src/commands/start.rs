#![allow(dead_code, unused_variables)]

use clap::{ArgMatches};
use crate::{CvmError, Success};
use crate::config::config::Config;
use crate::task::task_impl::service_manager_task::{ServicesAction, ServicesManagerTask};
use crate::task::task_manager::TaskManager;

pub fn start(command: &ArgMatches, config: &Config) -> Result<Success, CvmError> {
    TaskManager::start(vec![
        Box::new(ServicesManagerTask { input_data: ServicesAction::START }),
    ], config)
}