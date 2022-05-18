use std::fs;
use std::path::Path;
use futures::future::err;
use crate::env::Env;
use crate::{Message, Success, url_build};
use crate::config::config::{get_config, get_home_dir};
use crate::config::enviroment::{Enviroment, set_env};
use crate::task::message_type::MessageType;
use crate::task::task::Task;
use crate::task::task_impl::run_command_task::{Cmd, RunCommandInputData};
use crate::task::task_impl::set_enviroment_variable::{SetEnvironmentVariable, SetEnvironmentVariableInput};
use crate::task::task_manager;
use crate::task::task_type::TaskType;

pub struct UserVersionTask {
    pub version: String,
}

const BIN_FOLDER: &str = "bin";
const CURRENT_FOLDER_NAME: &str = "current";
const CARDANO_NODE_FILE_NAME: &str = "cardano-node";
const CARDANO_CLI_FILE_NAME: &str = "cardano-cli";
const PATH_KEY: &str = "PATH";

impl Task for UserVersionTask {
    fn run(self: &Self, env: &mut Env) -> Result<Success, Message> {
        let config = get_config();
        if let Err(error) = config {
            return Result::Err(error);
        }

        let home_dir = get_home_dir();
        if let Err(error) = home_dir {
            return Result::Err(error);
        }

        let bin_folder = url_build(vec![home_dir.clone().unwrap().as_str(), &config.as_ref().unwrap().workspace.workspace_folder.as_str(), BIN_FOLDER], false);
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
                stack: vec![]
            });
        };

        if !current_folder_path.exists() {
            fs::create_dir_all(current_folder.clone());
        };

        let copy_result = copy_file_version(version_folder, current_folder.clone(), vec![CARDANO_NODE_FILE_NAME, CARDANO_CLI_FILE_NAME]);
        if let Err(error) = copy_result {
            return Result::Err(error);
        };

        task_manager::start(vec![
            Box::new(SetEnvironmentVariable { input_data: build_add_current_dir_var_command(self.version.clone(), current_folder.clone().to_string()) })
        ])
    }

    fn check(self: &Self, env: &mut Env) -> Result<Success, Message> {
        Result::Ok(Success {})
    }

    fn get_type(self: &Self) -> TaskType {
        TaskType::UseVersion
    }
}

fn build_add_current_dir_var_command(version: String, current_dir: String) -> SetEnvironmentVariableInput {
    set_env(Enviroment { active_version: version, ..Default::default() });
    SetEnvironmentVariableInput { key: PATH_KEY.to_string(), value: current_dir }
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
    Result::Ok(Success {})
}