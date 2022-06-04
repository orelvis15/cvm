#![allow(dead_code, unused_variables)]

use clap::{ArgMatches};
use crate::{CvmError, Success};
use crate::commands::config::Args;
use crate::utils::version_utils::{get_last_tag, LATEST, verify_version};
use crate::config::config::Config;
use crate::task::cvm_error::Error;
use crate::task::task_impl::use_version_task::{UserVersionData, UserVersionTask};
use crate::task::task_manager::TaskManager;
use crate::task::task_type::TaskType::EmptyTask;

pub fn start(command: &ArgMatches, config: &Config) -> Result<Success, CvmError> {

    let mut version: String = LATEST.to_string();

    let version_arg: String = command.value_of_t(Args::VERSION._to_string())?;

    if !version_arg.is_empty() {
        if verify_version(version_arg.as_str()) || version == LATEST {
            version = version_arg.to_string()
        } else {
            return Err(CvmError::VersionBadFormed (Error{
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
        Box::new(UserVersionTask { input_data: UserVersionData { version },  }),
    ], config)
}