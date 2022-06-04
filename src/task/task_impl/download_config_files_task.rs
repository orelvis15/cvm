#![allow(dead_code, unused_variables)]

extern crate strfmt;

use std::collections::HashMap;
use std::str::FromStr;
use strfmt::strfmt;
use crate::config::config::{Config, ConfigFileItem, get_project_dir};
use crate::env::Env;
use crate::task::task::{Success, Task};
use crate::task::task_type::TaskType;
use crate::{url_build};
use crate::task::cvm_error::CvmError;
use crate::task::folders::Folder;
use crate::utils::download_manager::download_in_path;

pub struct DownloadConfigFilesTask {
    pub network: String,
}

const NETWORK: &str = "network";

impl Task for DownloadConfigFilesTask {
    fn run(self: &Self, _env: &mut Env, config: &Config) -> Result<Success, CvmError> {
        let project_dir = get_project_dir();

        let workspace_home = url_build(vec![project_dir.as_str(), Folder::get(Folder::ROOT, &config)], false);
        download_config_files(&workspace_home, &self.network, &config.config_file_item, &config)?;

        Ok(Success {})
    }

    fn check(self: &Self, _env: &mut Env, config: &Config) -> Result<Success, CvmError> {
        Ok(Success {})
    }

    fn get_type(self: &Self) -> TaskType {
        TaskType::DownloadConfigFiles
    }
}

fn download_config_files(workspace_home: &String, network: &String, items: &Vec<ConfigFileItem>, config: &Config) -> Result<Success, CvmError> {
    for item in items {
        let folder_path = url_build(vec![&workspace_home.as_str(), Folder::get(Folder::from_str(item.folder_key.as_str()).unwrap(), config)], false);

        let mut vars = HashMap::new();
        vars.insert(NETWORK.to_string(), network);

        let url = strfmt(item.url.as_str(), &vars);

        if let Ok(url) = url {
            download_in_path(&url, folder_path.to_string(), item.name.as_str())?;
        } else {
            download_in_path(&item.url, folder_path.to_string(), item.name.as_str())?;
        }
    }
    Ok(Success {})
}