use std::fs;
use std::path::Path;
use crate::env::Env;
use crate::{Message, Success, url_build};
use crate::config::config::{get_config, get_project_dir};
use crate::task::folders::Folder;
use crate::task::message_type::MessageType;
use crate::task::task::Task;
use crate::task::task_type::TaskType;

pub struct UserVersionTask {
    pub version: String,
}

const CURRENT_FOLDER_NAME: &str = "current";
const CARDANO_NODE_FILE_NAME: &str = "cardano-node";
const CARDANO_CLI_FILE_NAME: &str = "cardano-cli";

impl Task for UserVersionTask {
    fn run(self: &Self, _env: &mut Env) -> Result<Success, Message> {

        sudo::escalate_if_needed().expect("Super user permissions are required");

        let config = get_config();
        if let Err(error) = config {
            return Err(error);
        }
        let config = config.as_ref().unwrap();

        let project_dir = get_project_dir();

        let bin_folder = url_build(vec![project_dir.as_str(), Folder::get(Folder::ROOT, &config), Folder::get(Folder::BIN, &config)], false);
        let version_folder = url_build(vec![bin_folder.as_str(), &self.version.as_str()], false);
        let version_folder_path = Path::new(version_folder.as_str());
        let current_folder = url_build(vec![bin_folder.as_str(), CURRENT_FOLDER_NAME], false);
        let current_folder_path = Path::new(current_folder.as_str());

        if !version_folder_path.exists() {
            return Err(Message {
                code: 0,
                message: format!("The version {version} is not installed yet, please install it using the command: cvm install {version}", version = &self.version),
                kind: MessageType::Error,
                task: "".to_string(),
                stack: vec![],
            });
        };

        if !current_folder_path.exists() {
            let folder_result = fs::create_dir_all(current_folder.clone());

            if let Err(error) = folder_result {
                return Err(Message {
                    code: 0,
                    message: "Error creating folder structure".to_string(),
                    kind: MessageType::Error,
                    task: "".to_string(),
                    stack: vec![error.to_string()],
                });
            }
        };

        let copy_result = copy_file_version(version_folder, current_folder.clone(), vec![CARDANO_NODE_FILE_NAME, CARDANO_CLI_FILE_NAME]);
        if let Err(error) = copy_result {
            return Err(error);
        };

        Ok(Success{})
    }

    fn check(self: &Self, _env: &mut Env) -> Result<Success, Message> {
        Ok(Success {})
    }

    fn get_type(self: &Self) -> TaskType {
        TaskType::UseVersion
    }
}

fn copy_file_version(version_folder: String, current_folder: String, file_names: Vec<&str>) -> Result<Success, Message> {
    for name in file_names {
        let file = url_build(vec![version_folder.as_str(), name], false);
        let file_out = url_build(vec![current_folder.as_str(), name], false);
        let copy_result = fs::copy(file, file_out);

        match copy_result {
            Ok(_) => continue,
            Err(error) => {
                return Err(Message {
                    code: 0,
                    message: "Error copying the new files".to_string(),
                    kind: MessageType::Error,
                    task: TaskType::UseVersion.to_string(),
                    stack: vec![error.to_string()],
                });
            }
        }
    }
    Ok(Success {})
}