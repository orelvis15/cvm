#![allow(dead_code, unused_variables)]

use crate::env::Env;
use crate::{Success, Term, url_build};
use crate::config::remote_config::RemoteConfig;
use crate::config::state_config::{get_task_complete, set_task_complete};
use crate::message::message::Message;
use crate::utils::folders::Folder;
use crate::task::task::Task;
use crate::task::task_impl::commons::folder_manager_task::{FolderManagerAction, FolderManagerTask};
use crate::task::task_impl::commons::run_command_task::{Cmd, RunCommandInputData, RunCommandTask};
use crate::task_manager::task_manager::TaskManager;
use crate::task::task_type::TaskType;
use crate::term::log_level::LogLevel::L2;

#[derive(Default)]
pub struct Installlibsecp256k1Task {
    libsecp256k1_repo:String,
    git_folder: String,
    libsecp256k1_folder: String
}

impl Task for Installlibsecp256k1Task {

    fn prepare(self: &mut Self, env: &mut Env, config: &RemoteConfig, term: &mut Term) -> Result<bool, Message> {
        if get_task_complete(&self.get_type()) {
            return Ok(false);
        };
        self.libsecp256k1_repo = config.clone().init.libsecp256k1_repository;
        self.git_folder = Folder::get_path(Folder::GIT, &config);
        self.libsecp256k1_folder = url_build(vec![&self.git_folder, &config.init.libsecp256k1_folder], false);
        Ok(true)
    }

    fn run(self: &Self, _env: &mut Env, config: &RemoteConfig, term: &mut Term) -> Result<Success, Message> {
        TaskManager {}.start(vec![
            Box::new(FolderManagerTask { input_data: FolderManagerAction::Remove(vec![self.libsecp256k1_folder.clone()]) }),
            Box::new(RunCommandTask { input_data: build_clone_repo_command(&self.libsecp256k1_repo.clone(), &self.git_folder), command_description: "Cloning libsecp256k1 repository".to_string() }),
            Box::new(RunCommandTask { input_data: build_checkout_repo_command(&self.libsecp256k1_folder.clone(), &config.init.libsecp256k1_commit.clone()), command_description: "Switching to the specified commit".to_string() }),
            Box::new(RunCommandTask { input_data: build_autogen_repo_command(&self.libsecp256k1_folder.clone(), &config.init.libsecp256k1_autogen_file.clone()), command_description: "Running autogen executable".to_string() }),
            Box::new(RunCommandTask { input_data: build_configure_repo_command(&self.libsecp256k1_folder.clone(), &config.init.libsecp256k1_configure_file.clone()), command_description: "Configuring the installation".to_string() }),
            Box::new(RunCommandTask { input_data: build_make_repo_command(&self.libsecp256k1_folder.clone()), command_description: "Compiling libsecp256k1".to_string() }),
            Box::new(RunCommandTask { input_data: build_make_install_repo_command(&self.libsecp256k1_folder.clone()), command_description: "Installing libsecp256k1".to_string() }),
        ], config, term, L2)
    }

    fn check(self: &Self, _env: &mut Env, config: &RemoteConfig, term: &mut Term) -> Result<Success, Message> {
        set_task_complete(&self.get_type());
        Ok(Success {})
    }

    fn get_type(self: &Self) -> TaskType {
        TaskType::Libsecp256k1
    }
}

fn build_clone_repo_command(repo: &String, path: &String) -> RunCommandInputData {
    let args = vec![Cmd::Clone.as_string(), repo.to_string()];
    RunCommandInputData { command: Cmd::Git.as_string(), args, current_dir: path.to_string() }
}

fn build_checkout_repo_command(path: &String, commit: &String) -> RunCommandInputData {
    let args = vec![Cmd::Checkout.as_string(), commit.to_string()];
    RunCommandInputData { command: Cmd::Git.as_string(), args, current_dir: path.to_string() }
}

fn build_autogen_repo_command(path: &String, autogen_file: &String) -> RunCommandInputData {
    let args = vec![autogen_file.to_string()];
    RunCommandInputData { command: Cmd::Sh.as_string(), args, current_dir: path.to_string() }
}

fn build_configure_repo_command(path: &String, config_file: &String) -> RunCommandInputData {
    let args = vec![config_file.to_string(), "--prefix=/usr".to_string(), "--enable-module-schnorrsig".to_string(), "--enable-experimental".to_string()];
    RunCommandInputData { command: Cmd::Sh.as_string(), args, current_dir: path.to_string() }
}

fn build_make_repo_command(path: &String) -> RunCommandInputData {
    RunCommandInputData { command: Cmd::Make.as_string(), args: vec![], current_dir: path.to_string() }
}

fn build_make_install_repo_command(path: &String) -> RunCommandInputData {
    let args = vec![Cmd::Make.as_string(), Cmd::Install.as_string()];
    RunCommandInputData { command: Cmd::Sudo.as_string(), args, current_dir: path.to_string() }
}