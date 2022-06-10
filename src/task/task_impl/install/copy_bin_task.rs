#![allow(dead_code, unused_variables)]

use std::fs;
use std::path::Path;
use walkdir::WalkDir;
use crate::env::Env;
use crate::{Success, url_build};
use crate::config::config::Config;
use crate::error::error::{Message, Error};
use crate::utils::folders::Folder;
use crate::task::task::Task;
use crate::task::task_type::TaskType;

pub struct CopyBinTask {
    pub input_data: CopyBinInputData,
}

#[derive(Default, Clone, Debug)]
pub struct CopyBinInputData {
    pub files_names: Vec<String>,
    pub origin_path: String,
    pub version: String,
}

impl Task for CopyBinTask {
    fn run(self: &Self, _env: &mut Env, config: &Config) -> Result<Success, Message> {

        let bin_folder = Folder::get_path(Folder::BIN, &config);
        let version_folder = url_build(vec![&bin_folder, &self.input_data.version], false);
        let version_folder_path = Path::new(version_folder.as_str());
        if !version_folder_path.exists() {
            fs::create_dir_all(version_folder_path)?;
        };

        build_copy_program_to_bin_folder_command(&self.input_data.files_names, &version_folder.to_string(), &self.input_data.origin_path, &self)
    }

    fn check(self: &Self, _env: &mut Env, config: &Config) -> Result<Success, Message> {
        Ok(Success {})
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
                return Ok(Success {});
            }
        }
    }
    return Err(Message::BinNotFound(Error {
        message: "Cardano executable not found".to_string(),
        task: _self.get_type(),
        stack: vec![],
    }));
}