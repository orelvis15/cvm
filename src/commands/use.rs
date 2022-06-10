#![allow(dead_code, unused_variables)]

use clap::{ArgMatches};
use crate::{Command, Message, Success};
use crate::commands::config::Args;
use crate::utils::version_utils::{get_last_tag, LATEST, verify_version};
use crate::config::config::Config;
use crate::error::error::Error;
use crate::task::task_impl::r#use::deploy_system_task::DeploySystemTask;
use crate::task::task_impl::r#use::service_manager_task::{ServicesAction, ServicesManagerTask};
use crate::task::task_impl::r#use::use_version_task::{UserVersionData, UserVersionTask};
use crate::task_manager::task_manager::TaskManager;
use crate::task::task_type::TaskType::EmptyTask;

pub struct Use{}

impl Command for Use{
    fn start(command: &ArgMatches, config: &Config) -> Result<Success, Message> {

        let mut version: String = LATEST.to_string();

        let version_arg: String = command.value_of_t(Args::VERSION._to_string())?;

        if !version_arg.is_empty() {
            if verify_version(version_arg.as_str()) || version == LATEST {
                version = version_arg.to_string()
            } else {
                return Err(Message::VersionBadFormed (Error{
                    message: "The version is not well formed".to_string(),
                    task: EmptyTask("Use Command".to_string()),
                    stack: vec![],
                }));
            }
        }

        if version == LATEST {
            let last_tag = get_last_tag(&config.build_cardano_node.cnode_release);
            match last_tag {
                Ok(tag) => version = tag,
                Err(error) => return Err(error)
            }
        };

        TaskManager::start(vec![
            Box::new(ServicesManagerTask { input_data: ServicesAction::STOP }),
            Box::new(UserVersionTask { input_data: UserVersionData { version }}),
            Box::new(DeploySystemTask { }),
        ], config)
    }
}