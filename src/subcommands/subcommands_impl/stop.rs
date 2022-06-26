#![allow(dead_code, unused_variables)]

use clap::{ArgMatches};
use crate::{Command, Message, Success, Term};
use crate::config::remote_config::RemoteConfig;
use crate::task::task_impl::r#use::service_manager_task::{ServicesAction, ServicesManagerTask};
use crate::task_manager::task_manager::TaskManager;
use crate::term::log_level::LogLevel::L1;

pub struct Stop{}

impl Command for Stop{
    fn start(command: &ArgMatches, config: &RemoteConfig, term: &mut Term) -> Result<Success, Message> {

        sudo::escalate_if_needed().expect("Super user permissions are required");

        TaskManager{}.start(vec![
            Box::new(ServicesManagerTask { input_data: ServicesAction::STOP }),
        ], config, term, L1)
    }
}