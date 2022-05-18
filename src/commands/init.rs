use clap::ArgMatches;
use owo_colors::OwoColorize;
use crate::commands::config::{Args};
use crate::task::task::{Message, Success};
use crate::task::task_impl::create_folder_structure_task::CreateFolderStructure;
use crate::task::task_impl::download_config_files_task::DownloadConfigFilesTask;
use crate::task::task_impl::install_dependences_task::InstallDependencesTask;
use crate::task::task_impl::install_haskell_ghc_task::InstallHanskellGhcTask;
use crate::task::task_impl::install_libsodium_task::InstallLibsodiumTask;
use crate::task::task_manager;
use crate::task::task_type::TaskType;

const MAINNET: &str = "mainnet";
const TESTNET: &str = "testnet";

pub fn start(command: &ArgMatches) -> Result<Success, Message> {
    let mut network = MAINNET;

    match command.value_of(Args::NETWORK.to_string()) {
        Some(value) => {
            if value == TESTNET {
                network = TESTNET
            }
        }
        None => {}
    }

    task_manager::start(vec![
        Box::new(InstallDependencesTask {}),
        Box::new(InstallHanskellGhcTask {}),
        Box::new(CreateFolderStructure {}),
        Box::new(DownloadConfigFilesTask { network: network.to_string() }),
        Box::new(InstallLibsodiumTask {}),
    ])
}