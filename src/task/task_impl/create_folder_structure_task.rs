use std::fs;
use std::path::Path;
use crate::config::config::{get_config, get_project_dir};
use crate::env::Env;
use crate::task::message_type::MessageType;
use crate::task::task::{Message, Success, Task};
use crate::task::task_type::TaskType;
use crate::url_build;

pub struct CreateFolderStructure {}

impl Task for CreateFolderStructure {
    fn run(self: &Self, _env: &mut Env) -> Result<Success, Message> {

        sudo::escalate_if_needed().expect("Super user permissions are required");

        let config = get_config();
        if let Err(error) = config {
            return Err(error);
        }

        let project_dir = get_project_dir();

        let workspace_home = url_build(vec![project_dir.as_str(), config.as_ref().unwrap().workspace.workspace_folder.as_str()], false);
        let create_folder_result = fs::create_dir(&workspace_home);

        if let Err(error) = create_folder_result {
            return Err(Message {
                code: 0,
                message: "Error creating folder".to_string(),
                kind: MessageType::Error,
                task: "".to_string(),
                stack: vec![error.to_string()],
            });
        }

        let folders = &config.as_ref().unwrap().workspace.folders;

        for folder in folders {
            let create_folder_result = fs::create_dir(url_build(vec![&workspace_home.as_str(), folder.as_str()], false));

            if let Err(error) = create_folder_result {
                return Err(Message {
                    code: 0,
                    message: "Error creating folder".to_string(),
                    kind: MessageType::Error,
                    task: "".to_string(),
                    stack: vec![error.to_string()],
                });
            }
        }
        Ok(Success{})
    }

    fn check(self: &Self, _env: &mut Env) -> Result<Success, Message> {
        let config = get_config();
        if let Err(error) = config {
            return Err(error);
        }

        let error = Message {
            code: 0,
            message: "Error creating folder structures".to_string(),
            kind: MessageType::Error,
            task: "".to_string(),
            stack: vec![],
        };

        let project_dir = get_project_dir();

        let workspace_home = url_build(vec![project_dir.as_str(), &config.as_ref().unwrap().workspace.workspace_folder.as_str()], false);

        if !Path::new(&workspace_home).is_dir() { return Err(error.clone()); }

        let folders = &config.as_ref().unwrap().workspace.folders;

        for folder in folders {
            let dir = url_build(vec![&workspace_home.as_str(), folder.as_str()], false);
            if !Path::new(dir.as_str()).is_dir() { return Err(error.clone()); }
        }

        Ok(Success {})
    }

    fn get_type(self: &Self) -> TaskType {
        TaskType::CreateFolderStructure
    }
}