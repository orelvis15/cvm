#![allow(dead_code, unused_variables)]

use crate::env::Env;
use crate::{Success, Term, url_build};
use crate::config::remote_config::RemoteConfig;
use crate::message::message::Message;
use crate::utils::folders::Folder;
use crate::task::task::Task;
use crate::task::task_impl::commons::folder_manager_task::{FolderManagerAction, FolderManagerTask};
use crate::task::task_impl::commons::run_command_task::{Cmd, RunCommandInputData, RunCommandTask};
use crate::task_manager::task_manager::TaskManager;
use crate::task::task_type::TaskType;
use crate::term::log_level::LogLevel::L2;

pub struct Installlibsecp256k1Task {}

impl Task for Installlibsecp256k1Task {
    fn run(self: &Self, _env: &mut Env, config: &RemoteConfig, term: &mut Term) -> Result<Success, Message> {
        let libsecp256k1_repo = &config.init.libsecp256k1_repository;
        let git_folder = Folder::get_path(Folder::GIT, &config);
        let libsecp256k1_folder = url_build(vec![&git_folder, &config.init.libsecp256k1_folder], false);

        TaskManager {}.start(vec![
            Box::new(FolderManagerTask { input_data: FolderManagerAction::Remove(vec![libsecp256k1_folder.to_string()]) }),
            Box::new(RunCommandTask { input_data: build_clone_repo_command(libsecp256k1_repo.clone(), git_folder), command_description: "Cloning libsecp256k1 repository".to_string() }),
            Box::new(RunCommandTask { input_data: build_checkout_repo_command(libsecp256k1_folder.clone(), config.init.libsecp256k1_commit.clone()), command_description: "Switching to the specified commit".to_string() }),
            Box::new(RunCommandTask { input_data: build_autogen_repo_command(libsecp256k1_folder.clone(), config.init.libsecp256k1_autogen_file.clone()), command_description: "Running autogen executable".to_string() }),
            Box::new(RunCommandTask { input_data: build_configure_repo_command(libsecp256k1_folder.clone(), config.init.libsecp256k1_configure_file.clone()), command_description: "Configuring the installation".to_string() }),
            Box::new(RunCommandTask { input_data: build_make_repo_command(libsecp256k1_folder.clone()), command_description: "Compiling libsecp256k1".to_string() }),
            Box::new(RunCommandTask { input_data: build_make_install_repo_command(libsecp256k1_folder.clone()), command_description: "Installing libsecp256k1".to_string() }),
        ], config, term, L2)
    }

    fn check(self: &Self, _env: &mut Env, config: &RemoteConfig, term: &mut Term) -> Result<Success, Message> {
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
    let args = vec![config_file, "--prefix=/usr".to_string(), "--enable-module-schnorrsig".to_string(), "--enable-experimental".to_string()];
    RunCommandInputData { command: Cmd::Sh.as_string(), args, current_dir: path }
}

fn build_make_repo_command(path: String) -> RunCommandInputData {
    RunCommandInputData { command: Cmd::Make.as_string(), args: vec![], current_dir: path }
}

fn build_make_install_repo_command(path: String) -> RunCommandInputData {
    let args = vec![Cmd::Make.as_string(), Cmd::Install.as_string()];
    RunCommandInputData { command: Cmd::Sudo.as_string(), args, current_dir: path }
}