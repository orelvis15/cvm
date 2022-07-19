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
use crate::{MessageData, url_build};
use crate::config::state_config::{get_state, update_init_files};
use crate::message::message::Message;
use crate::task::task_impl::commons::command::run_command_io_data::RunCommandInputData;
use crate::task::task_impl::commons::file_manager::file_manager_task:: FileManagerTask;
use crate::resolvers::folders::custom_folders::CustomFolders;
use crate::task::task_impl::commons::command::run_command_task::{Cmd, RunCommandTask};
use crate::task::task_impl::commons::file_manager::file_manager_io_data::FileManagerAction;
use crate::task::task_impl::task_input_data::TaskInputData;
use crate::task_manager::task_manager::TaskManager;
use crate::term::log_level::LogLevel::L2;
use crate::utils::download_manager::download;

pub struct UpdateConfigFilesTask {
    pub force: bool,
}

const NETWORK: &str = "network";

impl Task for UpdateConfigFilesTask {
    fn prepare(self: &mut Self, context: &mut Context, config: &RemoteConfig) -> Result<bool, Message> {
        Ok(true)
    }

    fn run(self: &Self, context: &mut Context, config: &RemoteConfig) -> Result<Success, Message> {
        if get_state()?.init.files_item.is_empty() {
            return Err(Message::UpdateConfigFile(MessageData {
                message: "The configuration files have not been downloaded yet".to_string(),
                task: self.get_type(),
                ..Default::default()
            }));
        }
        download_config_files(&self, &config.config_file_item, &config, context)?;
        Ok(Success::default())
    }

    fn check(self: &Self, context: &mut Context, config: &RemoteConfig) -> Result<Success, Message> {
        let mut paths = vec![];

        for item in &config.config_file_item {
            paths.push(CustomFolders::get_path_string(&CustomFolders::from_str(item.folder_key.as_str()).unwrap(), config));
        }

        TaskManager {}.start(vec![
            Box::new(FileManagerTask { input_data: TaskInputData::FileManager(FileManagerAction::Check(paths)), ..Default::default() }),
        ], config, L2, context)
    }

    fn get_type(self: &Self) -> TaskType {
        TaskType::UpdateConfigFiles
    }

    fn get_id(self: &Self) -> String {
        "".to_string()
    }
}

fn download_config_files(update_config_file_task: &UpdateConfigFilesTask, items: &Vec<ConfigFileItem>, config: &RemoteConfig, context: &mut Context) -> Result<Success, Message> {
    for item in items {
        let folder_path = CustomFolders::get_path_string(&CustomFolders::from_str(item.folder_key.as_str()).unwrap(), config);

        let local_file = url_build(vec![&folder_path, &item.name.clone()], false);
        let remote_file = download_remote_file(&item)?;

        apply_pattern_sed(&remote_file, &item.pattern_sed, config, context)?;

        if local_file_was_modify_manualy(&local_file)? && !update_config_file_task.force {
            continue;
        }

        if !is_update_necesary(&remote_file)? {
            continue;
        }

        update_file(&remote_file, &local_file)?;

        if item.folder_key == CustomFolders::Scripts.to_string() {
            fs::set_permissions(&local_file, fs::Permissions::from_mode(0o755))?;
        };
    }
    Ok(Success::default())
}

fn update_file(remote_file_uri: &String, local_file_uri: &String) -> Result<Success, Message> {
    fs::copy(remote_file_uri, local_file_uri)?;
    update_init_files(&local_file_uri)
}

fn is_update_necesary(remote_file_uri: &String) -> Result<bool, Message> {
    let state = get_state()?;
    let remote_file = Path::new(&remote_file_uri);
    let file_name = remote_file.file_name().unwrap().to_str().unwrap().to_string();

    let item_state = state.init.files_item.iter().find(|item| item.name == file_name).unwrap();
    let remote_file_hash = sha256::digest_file(remote_file)?;

    if item_state.hash == remote_file_hash {
        return Ok(false);
    }
    Ok(true)
}

/// Compare local file hash with the storage hash
/// if are diferents the file was modify manualy
fn local_file_was_modify_manualy(local_file_uri: &String) -> Result<bool, Message> {
    let state = get_state()?;
    let local_file = Path::new(&local_file_uri);
    let file_name = local_file.file_name().unwrap().to_str().unwrap().to_string();
    // if file not exist is not necesary compare
    if !local_file.exists() { return Ok(false); };

    //find hash for file
    let item = state.init.files_item.iter().find(|item| item.name == file_name).unwrap();

    let local_file_hash = sha256::digest_file(local_file)?;

    if item.hash == local_file_hash {
        return Ok(false);
    }

    Ok(true)
}

fn download_remote_file(item: &&ConfigFileItem) -> Result<String, Message> {
    let mut vars = HashMap::new();
    vars.insert(NETWORK.to_string(), get_state()?.init.network);

    let url = strfmt(item.url.as_str(), &vars);

    if let Ok(url) = url {
        download(&url, item.name.clone().as_str())
    } else {
        download(&item.url, item.name.clone().as_str())
    }
}

fn apply_pattern_sed(file_path: &String, pattern: &String, config: &RemoteConfig, context: &mut Context) -> Result<Success, Message> {
    if pattern.is_empty() { return Ok(Success::default()); }

    let args = vec!["-i".to_string(), pattern.to_string(), file_path.to_string()];
    TaskManager::default().start(vec![
        Box::new(RunCommandTask {
            input_data: RunCommandInputData {
                command: TaskInputData::String(Cmd::Sed.as_string()),
                args: TaskInputData::VecString(args),
                ..Default::default()
            },
            ..Default::default()
        })
    ], config, L2, context)
}