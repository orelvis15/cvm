use std::fs;
use std::path::Path;
use crate::config::config::{get_config, get_home_dir};
use crate::config::enviroment::{Enviroment, set_env};
use crate::env::Env;
use crate::task::message_type::MessageType;
use crate::task::task::{Message, Success, Task};
use crate::task::task_impl::set_enviroment_variable::{SetEnvironmentVariable, SetEnvironmentVariableInput};
use crate::task::task_manager;
use crate::task::task_type::TaskType;
use crate::url_build;

pub struct CreateFolderStructure {}

const CNODE_HOME: &str = "CNODE_HOME";

impl Task for CreateFolderStructure {
    fn run(self: &Self, _env: &mut Env) -> Result<Success, Message> {
        let config = get_config();
        if let Err(error) = config {
            return Result::Err(error);
        }

        let home_dir = get_home_dir();
        if let Err(error) = home_dir {
            return Result::Err(error);
        }

        let workspace_home = url_build(vec![home_dir.unwrap().as_str(), config.as_ref().unwrap().workspace.workspace_folder.as_str()], false);
        fs::create_dir(&workspace_home);

        let folders = &config.as_ref().unwrap().workspace.folders;

        for folder in folders {
            fs::create_dir(url_build(vec![&workspace_home.as_str(), folder.as_str()], false));
        }

        task_manager::start(vec![
            Box::new(SetEnvironmentVariable { input_data: build_set_cnode_env_var_command(workspace_home) })
        ])
    }

    fn check(self: &Self, _env: &mut Env) -> Result<Success, Message> {
        let config = get_config();
        if let Err(error) = config {
            return Result::Err(error);
        }

        let error = Message {
            code: 0,
            message: "Error creating folder structures".to_string(),
            kind: MessageType::Error,
            task: "".to_string(),
            stack: vec![],
        };

        let home_dir = get_home_dir();
        if let Err(error) = home_dir {
            return Result::Err(error);
        }

        let workspace_home = url_build(vec![home_dir.unwrap().as_str(), &config.as_ref().unwrap().workspace.workspace_folder.as_str()], false);

        if !Path::new(&workspace_home).is_dir() { return Result::Err(error.clone()); }

        let folders = &config.as_ref().unwrap().workspace.folders;

        for folder in folders {
            let dir = url_build(vec![&workspace_home.as_str(), folder.as_str()], false);
            if !Path::new(dir.as_str()).is_dir() { return Result::Err(error.clone()); }
        }

        Result::Ok(Success {})
    }

    fn get_type(self: &Self) -> TaskType {
        TaskType::CreateFolderStructure
    }
}

fn build_set_cnode_env_var_command(cardano_home: String) -> SetEnvironmentVariableInput {
    set_env(Enviroment { cnode_home: cardano_home.clone(), ..Default::default() });
    SetEnvironmentVariableInput { key: CNODE_HOME.to_string(), value: cardano_home }
}