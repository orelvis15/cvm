extern crate strfmt;

use std::collections::HashMap;
use strfmt::strfmt;
use crate::config::config::{ConfigFileItem, get_config, get_home_dir};
use crate::env::Env;
use crate::task::task::{Message, Success, Task};
use crate::task::task_type::TaskType;
use crate::url_build;
use crate::utils::download_manager::download_in_path;

pub struct DownloadConfigFilesTask {
    pub network: String,
}

const NETWORK: &str = "network";

impl Task for DownloadConfigFilesTask {
    fn run(self: &Self, _env: &mut Env) -> Result<Success, Message> {
        let config = get_config();
        if let Err(error) = config {
            return Result::Err(error);
        }

        let home_dir = get_home_dir();
        if let Err(error) = home_dir {
            return Result::Err(error);
        }

        let workspace_home = url_build(vec![home_dir.as_ref().unwrap().as_str(), &config.as_ref().unwrap().workspace.workspace_folder.as_str()], false);
        download_config_files(&workspace_home, &self.network, &config.as_ref().unwrap().config_file_item);

        Result::Ok(Success {})
    }

    fn check(self: &Self, _env: &mut Env) -> Result<Success, Message> {
        Result::Ok(Success {})
    }

    fn get_type(self: &Self) -> TaskType {
        TaskType::DownloadConfigFiles
    }
}

fn download_config_files(workspace_home: &String, network: &String, items: &Vec<ConfigFileItem>) {
    for item in items {
        let folder_path = url_build(vec![&workspace_home.as_str(), item.folder.as_str()], false);

        let mut vars = HashMap::new();
        vars.insert(NETWORK.to_string(), network);

        let url = strfmt(item.url.as_str(), &vars);

        if let Ok(url) = url {
            download_in_path(&url, folder_path.to_string(), item.name.as_str());
        } else {
            download_in_path(&item.url, folder_path.to_string(), item.name.as_str());
        }
    }
}