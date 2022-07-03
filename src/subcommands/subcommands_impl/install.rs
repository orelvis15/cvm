#![allow(dead_code, unused_variables)]

use std::path::Path;
use clap::ArgMatches;
use crate::subcommands::commands_config::{Args};
use crate::task::task::Success;
use crate::utils::version_utils::{get_last_tag, LATEST, verify_version};
use crate::config::remote_config::RemoteConfig;
use crate::{Message, Command, Term, MessageData, url_build};
use crate::config::state_config::get_state;
use crate::message::message::MessageKind;
use crate::task::task_impl::install::build_cardano_node_task::BuildCardanoNodeTask;
use crate::task_manager::task_manager::TaskManager;
use crate::term::log_level::LogLevel::L1;
use crate::utils::folders::Folder;

pub struct Install{}

impl Command for Install {
    fn start(command: &ArgMatches, config: &RemoteConfig, term: &mut Term) -> Result<Success, Message> {

        let version_arg = command.get_one::<String>(Args::VERSION._to_string()).unwrap();
        let mut version = verify_version(version_arg.as_str())?.to_string();

        if !get_state()?.init.success {
            return Err(Message::ProjectNotInit(
                MessageData {
                    message: "The project is still not initialized, please execute the [cvm init] command".to_string(),
                    ..Default::default()
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

        let bin_folder = Folder::get_path(Folder::BIN, &config);
        let version_folder = url_build(vec![&bin_folder, &version], false);
        let version_folder = Path::new(&version_folder);

        if version_folder.exists() {
            return Err(Message::VersionExist(MessageData {
                message: format!("the version {ver} is already installed to reinstall it firts remove it with the command [cvm remove {ver}]", ver = version),
                kind: MessageKind::Info,
                ..Default::default()
            }))
        }

        let mut task = BuildCardanoNodeTask::default();
        task.version = version.to_string();

        TaskManager::default().start(vec![
            Box::new(task),
        ], config, term, L1)
    }
}