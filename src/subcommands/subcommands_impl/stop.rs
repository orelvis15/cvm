#![allow(dead_code, unused_variables)]

use clap::ArgMatches;
use crate::{CommandStrategy, config, Message, Success};
use crate::context::context::Context;
use crate::task::task_impl::r#use::service_manager_task::{ServicesAction, ServicesManagerTask};
use crate::task_manager::task_manager::TaskManager;
use crate::term::log_level::LogLevel::L1;

pub struct Stop {}

impl CommandStrategy for Stop {
    fn start(command: &ArgMatches, context: &mut Context) -> Result<Success, Message> {
        let config = config::remote_config::get_remote_config()?;

        sudo::escalate_if_needed().expect("Super user permissions are required");

        TaskManager::default().start(vec![
            Box::new(ServicesManagerTask { input_data: ServicesAction::STOP }),
        ], &config, L1, context)
    }
}