#![allow(dead_code, unused_variables)]

use std::fs;
use std::path::Path;
use crate::context::context::Context;
use walkdir::WalkDir;
use crate::{Success, url_build};
use crate::config::remote_config::RemoteConfig;
use crate::message::message::Message;
use crate::utils::folders::Folder;
use crate::task::task::Task;
use crate::task::task_impl::commons::file_manager::file_manager_io_data::FileManagerAction;
use crate::task::task_impl::commons::file_manager::file_manager_task::FileManagerTask;
use crate::task::task_impl::commons::folder_manager::folder_manager_io_data::FolderManagerAction;
use crate::task::task_impl::commons::folder_manager::folder_manager_task::FolderManagerTask;
use crate::task::task_impl::task_input_data::TaskInputData;
use crate::task::task_type::TaskType;
use crate::task_manager::task_manager::TaskManager;
use crate::term::log_level::LogLevel::L2;

pub struct CopyBinTask {
    pub input_data: CopyBinInputData,
}

#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct CopyBinInputData {
    pub files_names: Vec<String>,
    pub origin_path: String,
    pub version: String,
    pub bin_folder: String,
    pub version_folder: String,
}

impl Task for CopyBinTask {
    fn prepare(self: &mut Self, context: &mut Context, config: &RemoteConfig) -> Result<bool, Message> {
        let version_folder_path = Path::new(&self.input_data.version_folder);
        if !version_folder_path.exists() {
            return Ok(false);
        }
        Ok(true)
    }

    fn run(self: &Self, context: &mut Context, config: &RemoteConfig) -> Result<Success, Message> {
        for entry in WalkDir::new(&self.input_data.origin_path) {
            let entry = entry.unwrap();
            for file_name in &self.input_data.files_names {
                if entry.file_name().to_str().unwrap() == file_name && entry.path().is_file() {
                    fs::copy(entry.path(), format!("{}/{}", &self.input_data.version_folder, file_name))?;
                }
            }
        }
        Ok(Success::default())
    }

    fn check(self: &Self, context: &mut Context, config: &RemoteConfig) -> Result<Success, Message> {
        let bin_folder = Folder::get_path(Folder::BIN, &config);
        let version_folder = url_build(vec![&bin_folder, &self.input_data.version], false);

        let mut files_paths = vec![];

        for required_file in &config.binaries.required_files {
            files_paths.push(url_build(vec![&version_folder.to_string(), required_file], false))
        }

        TaskManager::default().start(vec![
            Box::new(FolderManagerTask { input_data: TaskInputData::FolderManager(FolderManagerAction::Exits(vec![version_folder])), ..Default::default() }),
            Box::new(FileManagerTask { input_data: TaskInputData::FileManager(FileManagerAction::Check(files_paths)), ..Default::default() }),
        ], config, L2, context)
    }

    fn get_type(self: &Self) -> TaskType {
        TaskType::CopyBinFiles(self.input_data.clone())
    }

    fn get_id(self: &Self) -> String {
        "".to_string()
    }
}