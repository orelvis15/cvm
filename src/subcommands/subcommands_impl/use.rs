#![allow(dead_code, unused_variables)]

use clap::{ArgMatches};
use crate::{Command, Message, Success, Term};
use crate::subcommands::config::Args;
use crate::utils::version_utils::{get_last_tag, LATEST, verify_version};
use crate::config::config::Config;
use crate::task::task_impl::r#use::deploy_system_task::DeploySystemTask;
use crate::task::task_impl::r#use::service_manager_task::{ServicesAction, ServicesManagerTask};
use crate::task::task_impl::r#use::use_version_task::{UserVersionData, UserVersionTask};
use crate::task_manager::task_manager::TaskManager;
use crate::term::log_level::LogLevel::L1;

pub struct Use{}

impl Command for Use{
    fn start(command: &ArgMatches, config: &Config, term: &mut Term) -> Result<Success, Message> {

        let version_arg = command.get_one::<String>(Args::VERSION._to_string()).unwrap();
        let mut version = verify_version(version_arg.as_str())?.to_string();

        if version == LATEST {
            let last_tag = get_last_tag(&config.build_cardano_node.cnode_release);
            match last_tag {
                Ok(tag) => version = tag,
                Err(error) => return Err(error)
            }
        };

        TaskManager{}.start(vec![
            Box::new(ServicesManagerTask { input_data: ServicesAction::STOP }),
            Box::new(UserVersionTask { input_data: UserVersionData { version }}),
            Box::new(DeploySystemTask { }),
        ], config, term, L1)
    }
}