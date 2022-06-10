#![allow(dead_code, unused_variables)]

extern crate strfmt;

use std::collections::HashMap;
use std::str::FromStr;
use strfmt::strfmt;
use crate::config::config::{Config, ConfigFileItem};
use crate::env::Env;
use crate::task::task::{Success, Task};
use crate::task::task_type::TaskType;
use crate::{url_build};
use crate::task::cvm_error::CvmError;
use crate::task::folders::Folder;
use crate::task::task_impl::run_command_task::{Cmd, RunCommandInputData, RunCommandTask};
use crate::task::task_manager::TaskManager;
use crate::utils::download_manager::download_in_path;

pub struct DownloadConfigFilesTask {
    pub network: String,
}

const NETWORK: &str = "network";

impl Task for DownloadConfigFilesTask {
    fn run(self: &Self, _env: &mut Env, config: &Config) -> Result<Success, CvmError> {
        download_config_files(&self.network, &config.config_file_item, &config)?;
        Ok(Success {})
    }

    fn check(self: &Self, _env: &mut Env, config: &Config) -> Result<Success, CvmError> {
        Ok(Success {})
    }

    fn get_type(self: &Self) -> TaskType {
        TaskType::DownloadConfigFiles
    }
}

fn download_config_files(network: &String, items: &Vec<ConfigFileItem>, config: &Config) -> Result<Success, CvmError> {
    for item in items {
        let folder_path = Folder::get_path(Folder::from_str(item.folder_key.as_str()).unwrap(), config);

        let mut vars = HashMap::new();
        vars.insert(NETWORK.to_string(), network);

        let url = strfmt(item.url.as_str(), &vars);

        if let Ok(url) = url {
            download_in_path(&url, folder_path.to_string(), item.name.as_str())?;
        } else {
            download_in_path(&item.url, folder_path.to_string(), item.name.as_str())?;
        }

        if item.pattern_sed != "" {
            apply_pattern_sed(url_build(vec![folder_path.as_str(), item.name.as_str()], false), &item.pattern_sed, config)?;
        }
    }
    Ok(Success {})
}

fn apply_pattern_sed(file_path: String, pattern: &String, config: &Config) -> Result<Success, CvmError> {
    let args = vec!["-i".to_string(), pattern.to_string(), file_path.to_string()];
    TaskManager::start(vec![
        Box::new(RunCommandTask { input_data: RunCommandInputData { command: Cmd::Sed.as_string(), args, ..Default::default() } }),
    ], config)
}