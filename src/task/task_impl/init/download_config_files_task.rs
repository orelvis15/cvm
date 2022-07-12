#![allow(dead_code, unused_variables)]

extern crate strfmt;

use std::collections::HashMap;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::str::FromStr;
use crate::context::context::Context;
use strfmt::strfmt;
use crate::config::remote_config::{RemoteConfig, ConfigFileItem};
use crate::task::task::{Success, Task};
use crate::task::task_type::TaskType;
use crate::url_build;
use crate::config::state_config::{add_init_file, get_task_complete, set_task_complete};
use crate::message::message::Message;
use crate::task::task_impl::commons::file_manager_task::{FileManagerAction, FileManagerTask};
use crate::utils::folders::Folder;
use crate::task::task_impl::commons::run_command_task::{Cmd, RunCommandInputData, RunCommandTask};
use crate::task_manager::task_manager::TaskManager;
use crate::term::log_level::LogLevel::L2;
use crate::utils::download_manager::download_in_path;

#[derive(Default)]
pub struct DownloadConfigFilesTask {
    pub network: String,
}

const NETWORK: &str = "network";

impl Task for DownloadConfigFilesTask {
    fn prepare(self: &mut Self, context: &mut Context, config: &RemoteConfig) -> Result<bool, Message> {
        if get_task_complete(&self.get_type()) {
            return Ok(false);
        };
        Ok(true)
    }

    fn run(self: &Self, context: &mut Context, config: &RemoteConfig) -> Result<Success, Message> {
        download_config_files(&self.network, &config.config_file_item, &config, context)
    }

    fn check(self: &Self, context: &mut Context, config: &RemoteConfig) -> Result<Success, Message> {
        let mut paths = vec![];

        for item in &config.config_file_item {
            paths.push(Folder::get_path(Folder::from_str(item.folder_key.as_str()).unwrap(), config));
        }

        let result = TaskManager {}.start(vec![
            Box::new(FileManagerTask { input_data: FileManagerAction::Check(paths) }),
        ], config, L2, context);

        set_task_complete(&self.get_type());

        result
    }

    fn get_type(self: &Self) -> TaskType {
        TaskType::DownloadConfigFiles
    }

    fn get_id(self: &Self) -> String {
        "".to_string()
    }
}

fn download_config_files(network: &String, items: &Vec<ConfigFileItem>, config: &RemoteConfig, context: &mut Context) -> Result<Success, Message> {
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
            apply_pattern_sed(url_build(vec![&folder_path, &item.name], false), &item.pattern_sed, config, context)?;
        }

        if item.folder_key == Folder::SCRIPTS.to_string() {
            fs::set_permissions(&file_path, fs::Permissions::from_mode(0o755))?;
        }

        add_init_file(&file_path)?;
    }
    Ok(Success::default())
}

fn apply_pattern_sed(file_path: String, pattern: &String, config: &RemoteConfig, context: &mut Context) -> Result<Success, Message> {
    let args = vec!["-i".to_string(), pattern.to_string(), file_path.to_string()];
    TaskManager::default().start(vec![
        Box::new(RunCommandTask {
            input_data: RunCommandInputData { command: Cmd::Sed.as_string(), args, ..Default::default() },
            command_description: "".to_string(),
        }),
    ], config, L2, context)
}