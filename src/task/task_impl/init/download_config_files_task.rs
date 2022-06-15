#![allow(dead_code, unused_variables)]

extern crate strfmt;

use std::collections::HashMap;
use std::path::Path;
use std::str::FromStr;
use strfmt::strfmt;
use crate::config::config::{Config, ConfigFileItem};
use crate::env::Env;
use crate::task::task::{Success, Task};
use crate::task::task_type::TaskType;
use crate::{Term, url_build};
use crate::error::error::Message;
use crate::utils::folders::Folder;
use crate::task::task_impl::commons::run_command_task::{Cmd, RunCommandInputData, RunCommandTask};
use crate::task_manager::task_manager::TaskManager;
use crate::term::log_level::LogLevel::L2;
use crate::utils::download_manager::download_in_path;

pub struct DownloadConfigFilesTask {
    pub network: String,
}

const NETWORK: &str = "network";

impl Task for DownloadConfigFilesTask {
    fn run(self: &Self, _env: &mut Env, config: &Config, term: &mut Term) -> Result<Success, Message> {
        download_config_files(&self.network, &config.config_file_item, &config, term)?;
        Ok(Success {})
    }

    fn check(self: &Self, _env: &mut Env, config: &Config, term: &mut Term) -> Result<Success, Message> {
        Ok(Success {})
    }

    fn get_type(self: &Self) -> TaskType {
        TaskType::DownloadConfigFiles
    }
}

fn download_config_files(network: &String, items: &Vec<ConfigFileItem>, config: &Config, term: &mut Term) -> Result<Success, Message> {
    for item in items {
        let folder_path = Folder::get_path(Folder::from_str(item.folder_key.as_str()).unwrap(), config);

        let mut vars = HashMap::new();
        vars.insert(NETWORK.to_string(), network);

        let url = strfmt(item.url.as_str(), &vars);

        let file_path = url_build(vec![&folder_path, &item.name.clone()], false);
        if Path::new(&file_path).exists() {
            continue;
        }

        if let Ok(url) = url {
            download_in_path(&url, folder_path.to_string(), item.name.clone())?;
        } else {
            download_in_path(&item.url, folder_path.to_string(), item.name.clone())?;
        }

        if item.pattern_sed != "" {
            apply_pattern_sed(url_build(vec![&folder_path, &item.name], false), &item.pattern_sed, config, term)?;
        }
    }
    Ok(Success {})
}

fn apply_pattern_sed(file_path: String, pattern: &String, config: &Config, term: &mut Term) -> Result<Success, Message> {
    let args = vec!["-i".to_string(), pattern.to_string(), file_path.to_string()];
    TaskManager{}.start(vec![
        Box::new(RunCommandTask { input_data: RunCommandInputData { command: Cmd::Sed.as_string(), args, ..Default::default() },
            command_description: "Editing configuration files".to_string() }),
    ], config, term, L2)
}