use clap::ArgMatches;
use futures::future::err;
use owo_colors::AnsiColors::Default;
use owo_colors::OwoColorize;
use serde::{Serialize, Deserialize};
use crate::commands::config::{Args};
use crate::task::task_impl::create_folder_structure_task::CreateFolderStructure;
use crate::task::task_impl::download_config_files_task::DownloadConfigFilesTask;
use crate::task::task_impl::install_dependences_task::InstallDependencesTask;
use crate::task::task_impl::install_haskell_ghc_task::InstallHanskellGhcTask;
use crate::task::task_manager;
use crate::task::task_type::TaskType;
use regex::Regex;
use crate::task::task::{Message, Success};
use crate::task::task_impl::build_cardano_node_task::BuildCardanoNodeTask;
use reqwest::header;
use crate::utils::version_utils::{get_last_tag, LATEST, verify_version};
use crate::config::config::get_config;
use crate::task::message_type::MessageType;

pub fn start(command: &ArgMatches) -> Result<Success, Message> {
    let mut version: String = LATEST.to_string();

    let config = get_config();
    if let Err(error) = config {
        return Result::Err(error);
    }

    match command.value_of(Args::VERSION.to_string()) {
        Some(value) => {
            if !value.is_empty() {
                if verify_version(value) || version == LATEST {
                    version = value.to_string()
                } else {
                    return Err(Message {
                        code: 0,
                        message: "The version is not well formed".to_string(),
                        kind: MessageType::Error,
                        task: "".to_string(),
                        stack: vec![],
                    });
                }
            }
        }
        None => {}
    };

    if version == LATEST {
        let last_tag = get_last_tag(config.unwrap().build_cardano_node.cnode_release);
        match last_tag {
            Ok(tag) => version = tag,
            Err(error) => return Result::Err(error)
        }
    }

    task_manager::start(vec![
        Box::new(BuildCardanoNodeTask { version: version.to_string() }),
    ])
}