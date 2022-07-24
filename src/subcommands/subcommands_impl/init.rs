#![allow(dead_code, unused_variables)]

use std::io::stdout;
use clap::ArgMatches;
use crate::subcommands::subcommand_strategy::CommandStrategy;
use crate::subcommands::commands_config::Args;
use crate::config::state_config::{reset_init, set_init_network, set_init_success};
use crate::message::message::Message;
use crate::task::task::{Success, Task};
use crate::task::task_impl::commons::permission_task::{PermissionAction, PermissionTask};
use crate::task::task_impl::init::create_folder_structure_task::CreateFolderStructure;
use crate::task::task_impl::init::download_config_files_task::DownloadConfigFilesTask;
use crate::task::task_impl::init::install_dependences_task::InstallDependenciesTask;
use crate::task::task_impl::init::install_ghcup_task::InstallHanskellGhcTask;
use crate::task::task_impl::init::install_libsecp256k1_task::Installlibsecp256k1Task;
use crate::task_manager::task_manager::TaskManager;
use crate::{config, Term};
use crate::term::log_level::LogLevel::L1;
use crate::utils::folders::Folder;

const MAINNET: &str = "mainnet";
const TESTNET: &str = "testnet";

pub struct Init {}

impl CommandStrategy for Init {
    fn start(command: &ArgMatches) -> Result<Success, Message> {
        let config = config::remote_config::get_remote_config()?;
        let mut term = Term { stdout: stdout() };

        let mut network = MAINNET;

        if let Some(value) = command.get_one::<String>(Args::NETWORK._to_string()) {
            if value == TESTNET {
                network = TESTNET
            }
        }

        if command.contains_id(Args::FORCE._to_string()) {
            reset_init()?;
        };

        let mut tasks: Vec<Box<dyn Task>> = vec![
            Box::new(PermissionTask { input_data: PermissionAction::CheckWrite(vec![Folder::get_workspaces_dir().to_string()]) }),
            Box::new(InstallDependenciesTask::default()),
        ];

        if command.contains_id(Args::BUILD._to_string()) {
            tasks.append(&mut vec![
                Box::new(InstallHanskellGhcTask::default()),
                Box::new(Installlibsecp256k1Task::default()),
            ]);
        } else {
            tasks.append(&mut vec![
                Box::new(CreateFolderStructure::default()),
                Box::new(DownloadConfigFilesTask { network: network.to_string() }),
            ]);
        }

        TaskManager::default().start(tasks, &config, &mut term, L1)?;

        set_init_network(network.to_string())?;
        set_init_success(true)
    }
}