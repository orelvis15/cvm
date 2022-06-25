#![allow(dead_code, unused_variables)]

use std::fs;
use std::fs::File;
use std::path::Path;
use file_diff::diff_files;
use crate::env::Env;
use crate::{Success, Term, url_build};
use crate::config::remote_config::Config;
use crate::config::state_config::set_version_use;
use crate::error::message::{Message, Error};
use crate::utils::folders::Folder;
use crate::task::task::Task;
use crate::task::task_type::TaskType;

pub struct UserVersionTask {
    pub input_data: UserVersionData,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UserVersionData {
    pub version: String,
}

impl Task for UserVersionTask {
    fn run(self: &Self, _env: &mut Env, config: &Config, term: &mut Term) -> Result<Success, Message> {

        let bin_folder = Folder::get_path(Folder::BIN, &config);
        let version_folder = url_build(vec![&bin_folder, &self.input_data.version], false);
        let version_folder_path = Path::new(version_folder.as_str());
        let current_folder = Folder::get_path(Folder::CURRENT, &config);
        let current_folder_path = Path::new(current_folder.as_str());

        if !version_folder_path.exists() {
            return Err(Message::VersionInstaller(Error {
                message: format!("The version {version} is not installed yet, please install it using the command: cvm install {version}", version = &self.input_data.version),
                task: self.get_type(),
                stack: vec![],
            }));
        };

        if !current_folder_path.exists() {
            let folder_result = fs::create_dir_all(current_folder.clone());

            if let Err(error) = folder_result {
                return Err(Message::CreateFolderStructure(Error {
                    message: "Error creating folder structure".to_string(),
                    task: self.get_type(),
                    stack: vec![error.to_string()],
                }));
            }
        };

        copy_file_version(&version_folder, &current_folder, &config.binaries.required_files, self)?;
        set_version_use(self.input_data.version.clone())?;
        Ok(Success {})
    }

    fn check(self: &Self, _env: &mut Env, config: &Config, term: &mut Term) -> Result<Success, Message> {

        let bin_folder = Folder::get_path(Folder::BIN, &config);
        let version_folder = url_build(vec![&bin_folder, &self.input_data.version], false);
        let current_folder = Folder::get_path(Folder::CURRENT, &config);

        for entity in fs::read_dir(Path::new(&current_folder))? {
            if entity.as_ref().unwrap().path().is_file() {
                let file_name = entity.as_ref().unwrap().file_name().to_str().unwrap().to_string();

                //file in version x.x.x folder
                let mut version_file = File::open(entity.unwrap().path())?;

                //file in current folder
                let mut current_file = File::open(Path::new(&url_build(vec![&current_folder.to_string(), &file_name], false)))?;

                if !diff_files(&mut version_file, &mut current_file) {
                    return Err(Message::UseVersion(Error{
                        message: "version could not be used".to_string(),
                        task: TaskType::UseVersion(UserVersionData{ version: self.input_data.version.clone() }),
                        stack: vec![]
                    }))
                }
            }
        }
        Ok(Success{})
    }

    fn get_type(self: &Self) -> TaskType {
        TaskType::UseVersion(self.input_data.clone())
    }
}

fn copy_file_version(version_folder: &String, current_folder: &String, files_names: &Vec<String>, _self: &UserVersionTask) -> Result<Success, Message> {
    for name in files_names {
        let file = url_build(vec![version_folder, name], false);
        let file_out = url_build(vec![current_folder, name], false);

        if !Path::new(&file).exists() { continue;}

        fs::copy(&file, &file_out)?;

    }
    Ok(Success {})
}