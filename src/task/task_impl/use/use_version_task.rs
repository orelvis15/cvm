#![allow(dead_code, unused_variables)]

use std::fs;
use std::fs::File;
use std::path::Path;
use crate::context::context::Context;
use file_diff::diff_files;
use crate::{Success, url_build};
use crate::config::remote_config::RemoteConfig;
use crate::config::state_config::set_version_use;
use crate::message::message::{Message, MessageData};
use crate::resolvers::folders::custom_folders::CustomFolders;
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

    fn prepare(self: &mut Self, context: &mut Context, config: &RemoteConfig) -> Result<bool, Message> {
        Ok(true)
    }

    fn run(self: &Self, context: &mut Context, config: &RemoteConfig) -> Result<Success, Message> {

        let bin_folder = CustomFolders::get_path_string(&CustomFolders::BIN, &config);
        let version_folder = url_build(vec![&bin_folder, &self.input_data.version], false);
        let version_folder_path = Path::new(version_folder.as_str());
        let current_folder = CustomFolders::get_path_string(&CustomFolders::CURRENT, &config);
        let current_folder_path = Path::new(current_folder.as_str());

        if !version_folder_path.exists() {
            return Err(Message::VersionInstaller(MessageData {
                message: format!("The version {version} is not installed yet, please install it using the command: cvm install {version}", version = &self.input_data.version),
                task: self.get_type(),
                ..Default::default()
            }));
        };

        if !current_folder_path.exists() {
            let folder_result = fs::create_dir_all(current_folder.clone());

            if let Err(error) = folder_result {
                return Err(Message::CreateFolderStructure(MessageData {
                    message: "Error creating folders structure".to_string(),
                    task: self.get_type(),
                    stack: vec![error.to_string()],
                    ..Default::default()
                }));
            }
        };

        copy_file_version(&version_folder, &current_folder, &config.binaries.required_files, self)?;
        set_version_use(self.input_data.version.clone())?;
        Ok(Success::default())
    }

    fn check(self: &Self, context: &mut Context, config: &RemoteConfig) -> Result<Success, Message> {

        let bin_folder = CustomFolders::get_path_string(&CustomFolders::BIN, &config);
        let version_folder = url_build(vec![&bin_folder, &self.input_data.version], false);
        let current_folder = CustomFolders::get_path_string(&CustomFolders::CURRENT, &config);

        for entity in fs::read_dir(Path::new(&current_folder))? {
            if entity.as_ref().unwrap().path().is_file() {
                let file_name = entity.as_ref().unwrap().file_name().to_str().unwrap().to_string();

                //file in version x.x.x folders
                let mut version_file = File::open(entity.unwrap().path())?;

                //file in current folders
                let mut current_file = File::open(Path::new(&url_build(vec![&current_folder.to_string(), &file_name], false)))?;

                if !diff_files(&mut version_file, &mut current_file) {
                    return Err(Message::UseVersion(MessageData {
                        message: "version could not be used".to_string(),
                        task: TaskType::UseVersion(UserVersionData{ version: self.input_data.version.clone() }),
                        ..Default::default()
                    }))
                }
            }
        }
        Ok(Success::default())
    }

    fn get_type(self: &Self) -> TaskType {
        TaskType::UseVersion(self.input_data.clone())
    }

    fn get_id(self: &Self) -> String {
        "".to_string()
    }
}

fn copy_file_version(version_folder: &String, current_folder: &String, files_names: &Vec<String>, _self: &UserVersionTask) -> Result<Success, Message> {
    for name in files_names {
        let file = url_build(vec![version_folder, name], false);
        let file_out = url_build(vec![current_folder, name], false);

        if !Path::new(&file).exists() { continue;}

        fs::copy(&file, &file_out)?;

    }
    Ok(Success::default())
}