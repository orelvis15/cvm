#![allow(dead_code, unused_variables)]

use std::path::Path;
use clap::{ArgMatches};
use crate::{Command, Message, Success, Term, url_build};
use crate::subcommands::commands_config::Args;
use crate::utils::version_utils::{get_last_tag, LATEST, read_version, verify_version};
use crate::config::remote_config::RemoteConfig;
use crate::task::task::Task;
use crate::task::task_impl::commons::folder_manager_task::{FolderManagerAction, FolderManagerTask};
use crate::task::task_impl::r#use::service_manager_task::{ServicesAction, ServicesManagerTask};
use crate::task_manager::task_manager::TaskManager;
use crate::term::log_level::LogLevel::L1;
use crate::utils::folders::Folder;

pub struct Remove {}

impl Command for Remove {
    fn start(command: &ArgMatches, config: &RemoteConfig, term: &mut Term) -> Result<Success, Message> {

        let version_arg = command.get_one::<String>(Args::VERSION._to_string()).unwrap();
        let mut version = verify_version(version_arg.as_str())?.to_string();

        if version == LATEST {
            let last_tag = get_last_tag(&config.build_cardano_node.cnode_release);
            match last_tag {
                Ok(tag) => version = tag,
                Err(error) => return Err(error)
            }
        };

        let current_folder = Folder::get_path(Folder::CURRENT, &config);
        let current_version = read_version(&current_folder);
        let bin_folder = Folder::get_path(Folder::BIN, &config);
        let version_folder = url_build(vec![&bin_folder, &version], false);

        let mut task_queue:Vec<Box<dyn Task>> = vec![];

        if current_version == version {
            task_queue.push(Box::new(ServicesManagerTask { input_data: ServicesAction::STOP }));
            task_queue.push(Box::new(FolderManagerTask { input_data: FolderManagerAction::Clean(vec![current_folder]) }));
        }

        if Path::new(&version_folder).exists() {
            task_queue.push(Box::new(FolderManagerTask { input_data: FolderManagerAction::Remove(vec![version_folder]) }));
        }

        task_queue.reverse();
        TaskManager::default().start(task_queue, config, term, L1)
    }
}