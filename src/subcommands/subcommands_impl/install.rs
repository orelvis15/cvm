#![allow(dead_code, unused_variables)]

use clap::ArgMatches;
use crate::subcommands::config::{Args};
use crate::task::task::Success;
use crate::utils::version_utils::{get_last_tag, LATEST, verify_version};
use crate::config::remote_config::RemoteConfig;
use crate::{Message, Command, Term, Error};
use crate::config::state_config::get_state;
use crate::task::task_impl::install::build_cardano_node_task::BuildCardanoNodeTask;
use crate::task::task_type::TaskType;
use crate::task_manager::task_manager::TaskManager;
use crate::term::log_level::LogLevel::L1;

pub struct Install{}

impl Command for Install {
    fn start(command: &ArgMatches, config: &RemoteConfig, term: &mut Term) -> Result<Success, Message> {

        let version_arg = command.get_one::<String>(Args::VERSION._to_string()).unwrap();
        let mut version = verify_version(version_arg.as_str())?.to_string();

        if !get_state()?.init.success {
            return Err(Message::ProjectNotInit(
                Error {
                    message: "The project is still not initialized, please execute the [cvm init] command".to_string(),
                    task: TaskType::EmptyTask("".to_string()),
                    stack: vec![],
                }
            ));
        }

        if version == LATEST {
            let last_tag = get_last_tag(&config.build_cardano_node.cnode_release);
            match last_tag {
                Ok(tag) => version = tag,
                Err(error) => return Err(error)
            }
        }

        TaskManager{}.start(vec![
            Box::new(BuildCardanoNodeTask { version: version.to_string() }),
        ], config, term, L1)
    }
}