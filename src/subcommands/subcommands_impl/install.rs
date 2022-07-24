#![allow(dead_code, unused_variables)]

use std::io::stdout;
use std::path::Path;
use clap::ArgMatches;
use crate::subcommands::commands_config::{Args};
use crate::task::task::Success;
use crate::utils::version_utils::{get_last_tag, LATEST, verify_version};
use crate::{Message, CommandStrategy, Term, MessageData, url_build, config};
use crate::config::state_config::get_state;
use crate::message::message::MessageKind;
use crate::task::task_impl::commons::folder_manager_task::{FolderManagerAction, FolderManagerTask};
use crate::task::task_impl::install::build::build_cardano_node_task::BuildCardanoNodeTask;
use crate::task::task_impl::install::build::copy_bin_task::{CopyBinInputData, CopyBinTask};
use crate::task::task_impl::install::download::download_install_task::DownloadInstallTask;
use crate::task_manager::task_manager::TaskManager;
use crate::term::log_level::LogLevel::L1;
use crate::utils::folders::Folder;

pub struct Install {}

impl CommandStrategy for Install {
    fn start(command: &ArgMatches) -> Result<Success, Message> {
        let config = config::remote_config::get_remote_config()?;
        let mut term = Term { stdout: stdout() };

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
        let version_folder_path = Path::new(&version_folder);
        let cardano_folder = url_build(vec![&Folder::get_path(Folder::GIT, &config), &config.build_cardano_node.cnode_repository_name], false);

        if version_folder_path.exists() {
            return Err(Message::VersionExist(MessageData {
                message: format!("the version {ver} is already installed to reinstall it firts remove it with the command [cvm remove {ver}]", ver = version),
                kind: MessageKind::Info,
                ..Default::default()
            }));
        }

        let mut build_cardano_task = BuildCardanoNodeTask::default();
        build_cardano_task.version = version.to_string();

        if command.contains_id(Args::BUILD._to_string()) {
            TaskManager::default().start(vec![
                Box::new(build_cardano_task),
                Box::new(FolderManagerTask { input_data: FolderManagerAction::Create(vec![(bin_folder.clone(), version.clone())]) }),
                Box::new(CopyBinTask {
                    input_data: CopyBinInputData {
                        files_names: config.binaries.required_files.clone(),
                        origin_path: cardano_folder.clone(),
                        version: version.clone(),
                        bin_folder: bin_folder.clone(),
                        version_folder: version_folder.clone(),
                    }
                }),
            ], &config, &mut term, L1)
        } else {
            TaskManager::default().start(vec![
                Box::new(FolderManagerTask { input_data: FolderManagerAction::Create(vec![(bin_folder.clone(), version.clone())]) }),
                Box::new(DownloadInstallTask { version }),
            ], &config, &mut term, L1)
        }
    }
}