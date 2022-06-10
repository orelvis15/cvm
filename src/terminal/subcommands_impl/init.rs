#![allow(dead_code, unused_variables)]

use clap::ArgMatches;
use crate::terminal::subcommand::Command;
use crate::terminal::config::Args;
use crate::config::config::Config;
use crate::error::error::Message;
use crate::task::task::Success;
use crate::task::task_impl::init::create_folder_structure_task::CreateFolderStructure;
use crate::task::task_impl::init::download_config_files_task::DownloadConfigFilesTask;
use crate::task::task_impl::init::install_dependences_task::InstallDependencesTask;
use crate::task::task_impl::init::install_haskell_ghc_task::InstallHanskellGhcTask;
use crate::task::task_impl::init::install_libsodium_task::InstallLibsodiumTask;
use crate::task_manager::task_manager::TaskManager;

const MAINNET: &str = "mainnet";
const TESTNET: &str = "testnet";

pub struct Init{}

impl Command for Init {
    fn start(command: &ArgMatches, config: &Config) -> Result<Success, Message> {
        let mut network = MAINNET;

        match command.value_of(Args::NETWORK.to_string()) {
            Some(value) => {
                if value == TESTNET {
                    network = TESTNET
                }
            }
            None => {}
        }

        TaskManager::start(vec![
            Box::new(InstallDependencesTask {}),
            Box::new(InstallHanskellGhcTask {}),
            Box::new(CreateFolderStructure {}),
            Box::new(DownloadConfigFilesTask { network: network.to_string() }),
            Box::new(InstallLibsodiumTask {}),
        ], config)
    }
}