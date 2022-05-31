use std::fs;
use std::ops::Add;
use std::path::Path;
use futures::future::ok;
use crate::config::config::{get_config, get_home_dir, get_project_dir};
use crate::config::enviroment::{Enviroment, set_env};
use crate::env::Env;
use crate::task::message_type::MessageType;
use crate::task::task::{Message, Success, Task};
use crate::task::task_impl::run_command_task::{Cmd, RunCommandInputData, RunCommandTask};
use crate::task::task_impl::set_enviroment_variable::{SetEnvironmentVariable, SetEnvironmentVariableInput};
use crate::task::task_manager;
use crate::task::task_type::TaskType;
use crate::url_build;

pub struct CreateFolderStructure {}

const CNODE_HOME: &str = "CNODE_HOME";

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

fn build_set_cnode_env_var_command(cardano_home: &String) -> Result<SetEnvironmentVariableInput, Message> {
    let set_env_result = set_env(Enviroment { cnode_home: cardano_home.clone(), ..Default::default() });

    if let Err(error) = set_env_result {
        return Err(Message {
            code: 0,
            message: "Error creating environment variable".to_string(),
            kind: MessageType::Error,
            task: "".to_string(),
            stack: vec![error.to_string()],
        });
    };
    Ok(SetEnvironmentVariableInput { key: CNODE_HOME.to_string(), value: cardano_home.clone() })
}