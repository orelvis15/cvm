extern crate strfmt;

use std::collections::HashMap;
use strfmt::strfmt;
use crate::config::config::{ConfigFileItem, get_config, get_project_dir};
use crate::env::Env;
use crate::task::task::{Message, Success, Task};
use crate::task::task_type::TaskType;
use crate::{MessageType, url_build};
use crate::task::folders::Folder;
use crate::utils::download_manager::download_in_path;

pub struct DownloadConfigFilesTask {
    pub network: String,
}

const NETWORK: &str = "network";

impl Task for DownloadConfigFilesTask {
    fn run(self: &Self, _env: &mut Env) -> Result<Success, Message> {
        let config = get_config();
        if let Err(error) = config {
            return Err(error);
        }
        let config = config.as_ref().unwrap();

        let project_dir = get_project_dir();

        let workspace_home = url_build(vec![project_dir.as_str(), Folder::get(Folder::ROOT, &config)], false);
        let download_result = download_config_files(&workspace_home, &self.network, &config.config_file_item);

        if let Err(error) = download_result {
            return Err(error);
        };

        Ok(Success {})
    }

    fn check(self: &Self, _env: &mut Env) -> Result<Success, Message> {
        Ok(Success {})
    }

    fn get_type(self: &Self) -> TaskType {
        TaskType::DownloadConfigFiles
    }
}

fn download_config_files(workspace_home: &String, network: &String, items: &Vec<ConfigFileItem>) -> Result<Success, Message> {
    for item in items {
        let folder_path = url_build(vec![&workspace_home.as_str(), item.folder.as_str()], false);

        let mut vars = HashMap::new();
        vars.insert(NETWORK.to_string(), network);

        let url = strfmt(item.url.as_str(), &vars);

        if let Ok(url) = url {
            let download_result = download_in_path(&url, folder_path.to_string(), item.name.as_str());
            if let Err(error) = download_result {
                return Err(show_error(error));
            };
        } else {
            let download_result = download_in_path(&item.url, folder_path.to_string(), item.name.as_str());
            if let Err(error) = download_result {
                return Err(show_error(error));
            };
        }
    }
    Ok(Success {})
}

fn show_error(error: Message) -> Message {
    return Message {
        code: 0,
        message: "Error downloading config files".to_string(),
        kind: MessageType::Error,
        task: "".to_string(),
        stack: vec![error.to_string()],
    };
}