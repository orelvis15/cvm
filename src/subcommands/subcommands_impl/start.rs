#![allow(dead_code, unused_variables)]

use std::io::stdout;
use clap::ArgMatches;
use crate::{CommandStrategy, config, Message, MessageData, Success, Term};
use crate::config::state_config::get_state;
use crate::message::message::MessageKind;
use crate::task::task_impl::r#use::service_manager_task::{ServicesAction, ServicesManagerTask};
use crate::task_manager::task_manager::TaskManager;
use crate::term::log_level::LogLevel::L1;

pub struct Start {}

impl CommandStrategy for Start {
    fn start(command: &ArgMatches) -> Result<Success, Message> {
        let config = config::remote_config::get_remote_config()?;
        let mut term = Term { stdout: stdout() };

        if get_state()?.r#use == "" {
            return Err(Message::UseVersion(
                MessageData {
                    message: "There is no version in use yet, run the command [cvm use x.x.x]".to_string(),
                    kind: MessageKind::Info,
                    ..Default::default()
                }
            ));
        }

        sudo::escalate_if_needed().expect("Super user permissions are required");
        TaskManager::default().start(vec![
            Box::new(ServicesManagerTask { input_data: ServicesAction::START }),
        ], &config, &mut term, L1)
    }
}