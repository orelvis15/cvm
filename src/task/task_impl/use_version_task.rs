#![allow(dead_code, unused_variables)]

use std::fs;
use std::path::Path;
use crate::env::Env;
use crate::{Success, url_build};
use crate::config::config::Config;
use crate::task::cvm_error::{CvmError, Error};
use crate::task::folders::Folder;
use crate::task::task::Task;
use crate::task::task_type::TaskType;
use crate::utils::version_utils::write_version;

pub struct UserVersionTask {
    pub input_data: UserVersionData,
}

#[derive(Debug, Clone)]
pub struct UserVersionData {
    pub version: String,
}

impl Task for UserVersionTask {
    fn run(self: &Self, _env: &mut Env, config: &Config) -> Result<Success, CvmError> {

        sudo::escalate_if_needed().expect("Super user permissions are required");

        let bin_folder = Folder::get_path(Folder::BIN, &config);
        let version_folder = url_build(vec![bin_folder.as_str(), &self.input_data.version.as_str()], false);
        let version_folder_path = Path::new(version_folder.as_str());
        let current_folder = Folder::get_path(Folder::CURRENT, &config);
        let current_folder_path = Path::new(current_folder.as_str());

        if !version_folder_path.exists() {
            return Err(CvmError::VersionInstaller(Error {
                message: format!("The version {version} is not installed yet, please install it using the command: cvm install {version}", version = &self.input_data.version),
                task: self.get_type(),
                stack: vec![],
            }));
        };

        if !current_folder_path.exists() {
            let folder_result = fs::create_dir_all(current_folder.clone());

            if let Err(error) = folder_result {
                return Err(CvmError::CreateFolderStructure(Error {
                    message: "Error creating folder structure".to_string(),
                    task: self.get_type(),
                    stack: vec![error.to_string()],
                }));
            }
        };

        copy_file_version(&version_folder, &current_folder, &config.binaries.files, self)?;
        write_version(&current_folder, &self.input_data.version);
        Ok(Success {})
    }

    fn check(self: &Self, _env: &mut Env, config: &Config) -> Result<Success, CvmError> {
        Ok(Success {})
    }

    fn get_type(self: &Self) -> TaskType {
        TaskType::UseVersion(self.input_data.clone())
    }
}

fn copy_file_version(version_folder: &String, current_folder: &String, files_names: &Vec<String>, _self: &UserVersionTask) -> Result<Success, CvmError> {
    for name in files_names {
        let file = url_build(vec![version_folder.as_str(), name.as_str()], false);
        let file_out = url_build(vec![current_folder.as_str(), name.as_str()], false);
        let copy_result = fs::copy(file, file_out);

        match copy_result {
            Ok(_) => continue,
            Err(error) => {
                return Err(CvmError::Copy(Error {
                    message: format!("Error copying file {}", name),
                    task: _self.get_type(),
                    stack: vec![error.to_string()],
                }));
            }
        }
    }
    Ok(Success {})
}