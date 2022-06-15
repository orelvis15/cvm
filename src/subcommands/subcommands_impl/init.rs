#![allow(dead_code, unused_variables)]

use clap::ArgMatches;
use crate::subcommands::subcommand::Command;
use crate::subcommands::config::Args;
use crate::config::config::{Config, get_home_dir};
use crate::error::error::Message;
use crate::task::task::Success;
use crate::task::task_impl::init::create_folder_structure_task::CreateFolderStructure;
use crate::task::task_impl::init::download_config_files_task::DownloadConfigFilesTask;
use crate::task::task_impl::init::install_dependences_task::InstallDependencesTask;
use crate::task::task_impl::init::install_ghcup_task::InstallHanskellGhcTask;
use crate::task::task_impl::init::install_libsodium_task::InstallLibsodiumTask;
use crate::task_manager::task_manager::TaskManager;
use crate::Term;
use crate::term::log_level::LogLevel::L1;

const MAINNET: &str = "mainnet";
const TESTNET: &str = "testnet";

pub struct Init{}

impl Command for Init {
    fn start<'a>(command: &'a ArgMatches, config: &Config, term: &mut Term) -> Result<Success, Message> {

        let mut network = MAINNET;

        sudo::with_env(&["HOME", get_home_dir()]).expect("Super user permissions are required");
        sudo::escalate_if_needed().expect("Super user permissions are required");

        match command.get_one::<String>(Args::NETWORK._to_string()) {
            Some(value) => {
                if value == TESTNET {
                    network = TESTNET
                }
            }
            None => {}
        }
        TaskManager{}.start(vec![
            Box::new(InstallDependencesTask {}),
            Box::new(InstallHanskellGhcTask {}),
            Box::new(CreateFolderStructure {}),
            Box::new(DownloadConfigFilesTask { network: network.to_string() }),
            Box::new(InstallLibsodiumTask {}),
        ], config, term, L1)
    }
}