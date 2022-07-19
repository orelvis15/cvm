#![allow(dead_code, unused_variables)]

use clap::ArgMatches;
use crate::subcommands::subcommand_strategy::CommandStrategy;
use crate::subcommands::commands_config::Args;
use crate::config::state_config::{reset_init, set_init_network, set_init_success};
use crate::message::message::Message;
use crate::task::task::Success;
use crate::task::task_impl::commons::permission::permission_task::PermissionTask;
use crate::task::task_impl::init::create_folder_structure_task::CreateFolderStructure;
use crate::task::task_impl::init::download_config_files_task::DownloadConfigFilesTask;
use crate::task::task_impl::init::install_dependences_task::InstallDependenciesTask;
use crate::task::task_impl::init::install_ghcup_task::InstallHanskellGhcTask;
use crate::task::task_impl::init::install_libsecp256k1_task::Installlibsecp256k1Task;
use crate::task_manager::task_manager::TaskManager;
use crate::config;
use crate::context::context::Context;
use crate::task::task_impl::commons::permission::permission_io_data::PermissionAction;
use crate::task::task_impl::task_input_data::TaskInputData;
use crate::term::log_level::LogLevel::L1;
use crate::resolvers::folders::system_folders::SystemFolder;

const MAINNET: &str = "mainnet";
const TESTNET: &str = "testnet";

pub struct Init {}

impl CommandStrategy for Init {
    fn start(command: &ArgMatches, context: &mut Context) -> Result<Success, Message> {
        let config = config::remote_config::get_remote_config(context)?;

        let mut network = MAINNET;

        if let Some(value) = command.get_one::<String>(Args::NETWORK._to_string()) {
            if value == TESTNET {
                network = TESTNET
            }
        }

        if command.contains_id(Args::FORCE._to_string()) {
            reset_init()?;
        };

        TaskManager::default().start(vec![
            Box::new(PermissionTask { input_data: TaskInputData::Permission(PermissionAction::CheckWrite(vec![SystemFolder::get_path_string(&SystemFolder::UnixOpt)])), ..Default::default() }),
            Box::new(InstallDependenciesTask::default()),
            Box::new(InstallHanskellGhcTask::default()),
            Box::new(CreateFolderStructure::default()),
            Box::new(Installlibsecp256k1Task::default()),
            Box::new(DownloadConfigFilesTask { network: network.to_string() }),
        ], &config, L1, context)?;

        set_init_network(network.to_string())?;
        set_init_success(true)
    }
}