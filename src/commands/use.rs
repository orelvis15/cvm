#![allow(dead_code, unused_variables)]

use clap::ArgMatches;
use crate::{CvmError, Success};
use crate::commands::config::Args;
use crate::utils::version_utils::{get_last_tag, LATEST, verify_version};
use crate::config::config::Config;
use crate::task::cvm_error::Error;
use crate::task::task_impl::use_version_task::{UserVersionData, UserVersionTask};
use crate::task::task_manager;
use crate::task::task_type::TaskType::EmptyTask;

pub fn start(command: &ArgMatches, config: &Config) -> Result<Success, CvmError> {

    let mut version: String = LATEST.to_string();

    match command.value_of(Args::VERSION.to_string()) {
        Some(value) => {
            if !value.is_empty() {
                if verify_version(value) || version == LATEST {
                    version = value.to_string()
                } else {
                    return Err(CvmError::VersionBadFormed (Error{
                        message: "The version is not well formed".to_string(),
                        task: EmptyTask("Use Command".to_string()),
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
    };

    task_manager::start(vec![
        Box::new(UserVersionTask { input_data: UserVersionData { version },  }),
    ])
}