#![allow(dead_code, unused_variables)]

use clap::ArgMatches;
use crate::commands::config::Args;
use crate::config::config::Config;
use crate::task::cvm_error::CvmError;
use crate::task::task::{Success};
use crate::task::task_impl::create_folder_structure_task::CreateFolderStructure;
use crate::task::task_impl::download_config_files_task::DownloadConfigFilesTask;
use crate::task::task_impl::install_dependences_task::InstallDependencesTask;
use crate::task::task_impl::install_haskell_ghc_task::InstallHanskellGhcTask;
use crate::task::task_impl::install_libsodium_task::InstallLibsodiumTask;
use crate::task::task_manager::TaskManager;

const MAINNET: &str = "mainnet";
const TESTNET: &str = "testnet";

pub fn start(command: &ArgMatches, config: &Config) -> Result<Success, CvmError> {
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