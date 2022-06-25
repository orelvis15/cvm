#![allow(dead_code, unused_variables)]

use std::fs;
use std::path::Path;
use walkdir::WalkDir;
use crate::env::Env;
use crate::{Success, Term, url_build};
use crate::config::remote_config::Config;
use crate::error::message::Message;
use crate::utils::folders::Folder;
use crate::task::task::Task;
use crate::task::task_impl::commons::file_manager_task::{FileManagerAction, FileManagerTask};
use crate::task::task_impl::commons::folder_manager_task::{FolderManagerAction, FolderManagerTask};
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
}

impl Task for CopyBinTask {
    fn run(self: &Self, _env: &mut Env, config: &Config, term: &mut Term) -> Result<Success, Message> {
        let bin_folder = Folder::get_path(Folder::BIN, &config);
        let version_folder = url_build(vec![&bin_folder, &self.input_data.version], false);
        let version_folder_path = Path::new(version_folder.as_str());

        if !version_folder_path.exists() {
            fs::create_dir_all(version_folder_path)?;
        };

        build_copy_program_to_bin_folder_command(&self.input_data.files_names, &version_folder.to_string(), &self.input_data.origin_path, &self)
    }

    fn check(self: &Self, _env: &mut Env, config: &Config, term: &mut Term) -> Result<Success, Message> {
        let bin_folder = Folder::get_path(Folder::BIN, &config);
        let version_folder = url_build(vec![&bin_folder, &self.input_data.version], false);

        let mut files_paths = vec![];

        for required_file in &config.binaries.required_files {
            files_paths.push(url_build(vec![&version_folder.to_string(), required_file], false))
        }

        TaskManager {}.start(vec![
            Box::new(FolderManagerTask { input_data: FolderManagerAction::Exits(vec![version_folder]) }),
            Box::new(FileManagerTask { input_data: FileManagerAction::Check(files_paths) }),
        ], config, term, L2)
    }

    fn get_type(self: &Self) -> TaskType {
        TaskType::CopyBinFiles(self.input_data.clone())
    }
}

fn build_copy_program_to_bin_folder_command(file_names: &Vec<String>, destination_path: &String, origin_path: &String, _self: &CopyBinTask) -> Result<Success, Message> {
    for entry in WalkDir::new(origin_path) {
        let entry = entry.unwrap();
        for file_name in file_names {
            if entry.file_name().to_str().unwrap() == file_name && entry.path().is_file() {
                fs::copy(entry.path(), format!("{}/{}", destination_path, file_name))?;
            }
        }
    }
    Ok(Success {})
}