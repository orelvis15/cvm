#![allow(dead_code, unused_variables)]

use clap::{ArgMatches};
use crate::{Command, Message, Success};
use crate::config::config::Config;
use crate::task::task_impl::r#use::service_manager_task::{ServicesAction, ServicesManagerTask};
use crate::task_manager::task_manager::TaskManager;

pub struct Start{}

impl Command for Start{
    fn start(command: &ArgMatches, config: &Config) -> Result<Success, Message> {
        TaskManager::start(vec![
            Box::new(ServicesManagerTask { input_data: ServicesAction::START }),
        ], config)
    }
}