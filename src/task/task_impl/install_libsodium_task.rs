#![allow(dead_code, unused_variables)]

use std::fs;
use std::path::Path;
use crate::env::Env;
use crate::{Success, url_build};
use crate::config::config::{Config, get_home_dir, get_project_dir};
use crate::task::cvm_error::{CvmError, Error};
use crate::task::folders::Folder;
use crate::task::task::Task;
use crate::task::task_impl::run_command_task::{Cmd, RunCommandInputData, RunCommandTask};
use crate::task::task_manager::TaskManager;
use crate::task::task_type::TaskType;

pub struct InstallLibsodiumTask {}

impl Task for InstallLibsodiumTask {
    fn run(self: &Self, _env: &mut Env, config: &Config) -> Result<Success, CvmError> {

        let home_dir = get_home_dir();
        if let Err(error) = home_dir {
            return Err(error);
        }

        let project_dir = get_project_dir();

        let repo = &config.init.libsodium_repository;
        let folder = url_build(vec![project_dir.as_str(), Folder::get(Folder::ROOT, &config), Folder::get(Folder::GIT, &config)], false);
        let libsodium_folder = url_build(vec![folder.as_str(), &config.init.libsodium_folder], false);
        let path = Path::new(libsodium_folder.as_str());

        if path.exists() {
            let remove_result = fs::remove_dir_all(path);
            if let Err(error) = remove_result {
                return Err(CvmError::DeletingFolder(Error {
                    message: "Error deleting folders".to_string(),
                    task: self.get_type(),
                    stack: vec![error.to_string()],
                }));
            }
        };

        TaskManager::start(vec![
            Box::new(RunCommandTask { input_data: build_clone_repo_command(repo.clone(), folder) }),
            Box::new(RunCommandTask { input_data: build_checkout_repo_command(libsodium_folder.clone(), config.init.libsodium_commit.clone()) }),
            Box::new(RunCommandTask { input_data: build_autogen_repo_command(libsodium_folder.clone(), config.init.libsodium_autogen_file.clone()) }),
            Box::new(RunCommandTask { input_data: build_configure_repo_command(libsodium_folder.clone(), config.init.libsodium_config_file.clone()) }),
            Box::new(RunCommandTask { input_data: build_make_repo_command(libsodium_folder.clone()) }),
            Box::new(RunCommandTask { input_data: build_make_install_repo_command(libsodium_folder.clone()) }),
        ], config)
    }

    fn check(self: &Self, _env: &mut Env, config: &Config) -> Result<Success, CvmError> {
        Ok(Success {})
    }

    fn get_type(self: &Self) -> TaskType {
        TaskType::InstallLibsodium
    }
}

fn build_clone_repo_command(repo: String, path: String) -> RunCommandInputData {
    let args = vec![Cmd::Clone.as_string(), repo];
    RunCommandInputData { command: Cmd::Git.as_string(), args, current_dir: path }
}

fn build_checkout_repo_command(path: String, commit: String) -> RunCommandInputData {
    let args = vec![Cmd::Checkout.as_string(), commit];
    RunCommandInputData { command: Cmd::Git.as_string(), args, current_dir: path }
}

fn build_autogen_repo_command(path: String, autogen_file: String) -> RunCommandInputData {
    let args = vec![autogen_file];
    RunCommandInputData { command: Cmd::Sh.as_string(), args, current_dir: path }
}

fn build_configure_repo_command(path: String, config_file: String) -> RunCommandInputData {
    let args = vec![config_file];
    RunCommandInputData { command: Cmd::Sh.as_string(), args, current_dir: path }
}

fn build_make_repo_command(path: String) -> RunCommandInputData {
    RunCommandInputData { command: Cmd::Make.as_string(), args: vec![], current_dir: path }
}

fn build_make_install_repo_command(path: String) -> RunCommandInputData {
    let args = vec![Cmd::Make.as_string(), Cmd::Install.as_string()];
    RunCommandInputData { command: Cmd::Sudo.as_string(), args, current_dir: path }
}