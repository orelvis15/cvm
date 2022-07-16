#![allow(dead_code, unused_variables)]

use crate::context::context::Context;
use crate::{Success, url_build};
use crate::config::remote_config::RemoteConfig;
use crate::message::message::Message;
use crate::utils::folders::Folder;
use crate::task::task::Task;
use crate::task::task_impl::commons::command::run_command_io_data::RunCommandInputData;
use crate::task::task_impl::commons::file_manager::file_manager_task::FileManagerTask;
use crate::task::task_impl::commons::folder_manager::folder_manager_task::FolderManagerTask;
use crate::task::task_impl::commons::command::run_command_task::{Cmd, RunCommandTask};
use crate::task::task_impl::commons::file_manager::file_manager_io_data::FileManagerAction;
use crate::task::task_impl::commons::folder_manager::folder_manager_io_data::FolderManagerAction;
use crate::task::task_impl::task_input_data::TaskInputData;
use crate::task_manager::task_manager::TaskManager;
use crate::task::task_type::TaskType;
use crate::term::log_level::LogLevel::L2;

pub struct InstallLibsodiumTask {}

impl Task for InstallLibsodiumTask {
    fn prepare(self: &mut Self, context: &mut Context, config: &RemoteConfig) -> Result<bool, Message> {
        Ok(true)
    }

    fn run(self: &Self, context: &mut Context, config: &RemoteConfig) -> Result<Success, Message> {
        let libsodium_repo = &config.init.libsodium_repository;
        let git_folder = Folder::get_path(Folder::GIT, &config);
        let libsodium_folder = url_build(vec![&git_folder, &config.init.libsodium_folder], false);

        TaskManager::default().start(vec![
            Box::new(FolderManagerTask { input_data: TaskInputData::FolderManager(FolderManagerAction::Remove(vec![libsodium_folder.to_string()])), ..Default::default() }),
            Box::new(RunCommandTask { input_data: build_clone_repo_command(libsodium_repo.clone(), git_folder), ..Default::default() }),
            Box::new(RunCommandTask { input_data: build_checkout_repo_command(libsodium_folder.clone(), config.init.libsodium_commit.clone()), ..Default::default() }),
            Box::new(RunCommandTask { input_data: build_autogen_repo_command(libsodium_folder.clone(), config.init.libsodium_autogen_file.clone()), ..Default::default() }),
            Box::new(RunCommandTask { input_data: build_configure_repo_command(libsodium_folder.clone(), config.init.libsodium_config_file.clone()), ..Default::default() }),
            Box::new(RunCommandTask { input_data: build_make_repo_command(libsodium_folder.clone()), ..Default::default() }),
            Box::new(RunCommandTask { input_data: build_make_install_repo_command(libsodium_folder.clone()), ..Default::default() }),
        ], config, L2, context)
    }

    fn check(self: &Self, context: &mut Context, config: &RemoteConfig) -> Result<Success, Message> {
        let l_lib_a = "/usr/local/lib/libsodium.a".to_string();
        let l_lib_la = "/usr/local/lib/libsodium.la".to_string();
        let l_lib_so = "/usr/local/lib/libsodium.so".to_string();

        TaskManager::default().start(vec![
            Box::new(FileManagerTask { input_data: TaskInputData::FileManager(FileManagerAction::Check(vec![l_lib_a, l_lib_la, l_lib_so])), ..Default::default() }),
        ], config, L2, context)
    }

    fn get_type(self: &Self) -> TaskType {
        TaskType::InstallLibsodium
    }

    fn get_id(self: &Self) -> String {
        "".to_string()
    }
}

fn build_clone_repo_command(repo: String, path: String) -> RunCommandInputData {
    let args = vec![Cmd::Clone.as_string(), repo];
    RunCommandInputData {
        command: TaskInputData::String(Cmd::Git.as_string()),
        args: TaskInputData::VecString(args),
        current_dir: TaskInputData::String(path),
        description: TaskInputData::String("Cloning Libsodium repository".to_string()),
    }
}

fn build_checkout_repo_command(path: String, commit: String) -> RunCommandInputData {
    let args = vec![Cmd::Checkout.as_string(), commit];
    RunCommandInputData {
        command: TaskInputData::String(Cmd::Git.as_string()),
        args: TaskInputData::VecString(args),
        current_dir: TaskInputData::String(path),
        description: TaskInputData::String("Switching to the specified commit".to_string()),
    }
}

fn build_autogen_repo_command(path: String, autogen_file: String) -> RunCommandInputData {
    let args = vec![autogen_file];
    RunCommandInputData {
        command: TaskInputData::String(Cmd::Sh.as_string()),
        args: TaskInputData::VecString(args),
        current_dir: TaskInputData::String(path),
        description: TaskInputData::String("Running autogen executable".to_string()),
    }
}

fn build_configure_repo_command(path: String, config_file: String) -> RunCommandInputData {
    let args = vec![config_file];
    RunCommandInputData {
        command: TaskInputData::String(Cmd::Sh.as_string()),
        args: TaskInputData::VecString(args),
        current_dir: TaskInputData::String(path),
        description: TaskInputData::String("Configuring the installation".to_string()),
    }
}

fn build_make_repo_command(path: String) -> RunCommandInputData {
    RunCommandInputData {
        command: TaskInputData::String(Cmd::Make.as_string()),
        current_dir: TaskInputData::String(path),
        description: TaskInputData::String("Compiling Libsodium".to_string()),
        ..Default::default()
    }
}

fn build_make_install_repo_command(path: String) -> RunCommandInputData {
    let args = vec![Cmd::Install.as_string()];
    RunCommandInputData {
        command: TaskInputData::String(Cmd::Make.as_string()),
        args: TaskInputData::VecString(args),
        current_dir: TaskInputData::String(path),
        description: TaskInputData::String("Installing Libsodium".to_string()),
    }
}