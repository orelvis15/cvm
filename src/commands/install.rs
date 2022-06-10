#![allow(dead_code, unused_variables)]

use clap::ArgMatches;
use crate::commands::config::{Args};
use crate::task::task::Success;
use crate::utils::version_utils::{get_last_tag, LATEST, verify_version};
use crate::config::config::Config;
use crate::{Message, Error, Command};
use crate::task::task_impl::install::build_cardano_node_task::BuildCardanoNodeTask;
use crate::task_manager::task_manager::TaskManager;
use crate::task::task_type::TaskType::EmptyTask;

pub struct Install{}

impl Command for Install {
    fn start(command: &ArgMatches, config: &Config) -> Result<Success, Message> {
        let mut version: String = LATEST.to_string();

        match command.value_of(Args::VERSION.to_string()) {
            Some(value) => {
                if !value.is_empty() {
                    if verify_version(value) || version == LATEST {
                        version = value.to_string()
                    } else {
                        return Err(Message::VersionBadFormed (Error{
                            message: "The version is not well formed".to_string(),
                            task: EmptyTask("Install command".to_string()),
                            stack: vec![],
                        }));
                    }
                }
            }
            None => {}
        };

        if version == LATEST {
            let last_tag = get_last_tag(&config.build_cardano_node.cnode_release);
            match last_tag {
                Ok(tag) => version = tag,
                Err(error) => return Err(error)
            }
        }

        TaskManager::start(vec![
            Box::new(BuildCardanoNodeTask { version: version.to_string() }),
        ], config)
    }
}