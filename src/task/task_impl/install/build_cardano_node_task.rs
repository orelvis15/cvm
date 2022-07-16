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
use crate::task::task_impl::commons::permission::permission_task::PermissionTask;
use crate::task::task_impl::commons::command::run_command_task::{Cmd, RunCommandTask};
use crate::task::task_impl::commons::file_manager::file_manager_io_data::FileManagerAction;
use crate::task::task_impl::commons::folder_manager::folder_manager_io_data::FolderManagerAction;
use crate::task::task_impl::commons::permission::permission_io_data::PermissionAction;
use crate::task::task_impl::task_input_data::TaskInputData;
use crate::task_manager::task_manager::TaskManager;
use crate::task::task_type::TaskType;
use crate::term::log_level::LogLevel::L2;

#[derive(Default)]
pub struct BuildCardanoNodeTask {
    pub version: String,
    cardano_folder: String,
    ghcup_folder: String,
    libsodium_ported_file: String,
    git_folder: String,
}

impl Task for BuildCardanoNodeTask {
    fn prepare(self: &mut Self, context: &mut Context, config: &RemoteConfig) -> Result<bool, Message> {
        self.cardano_folder = url_build(vec![&Folder::get_path(Folder::GIT, &config), &config.build_cardano_node.cnode_repository_name], false);
        self.ghcup_folder = url_build(vec![&Folder::get_home_dir()?, &config.init.ghcup_bin_path], false);
        self.libsodium_ported_file = url_build(vec![&self.cardano_folder, &config.build_cardano_node.cnode_ported_libsodium_file_name], false);
        self.git_folder = Folder::get_path(Folder::GIT, &config);
        Ok(true)
    }

    fn run(self: &Self, context: &mut Context, config: &RemoteConfig) -> Result<Success, Message> {
        TaskManager::default().start(vec![
            Box::new(PermissionTask { input_data: TaskInputData::Permission(PermissionAction::CheckWrite(vec![self.git_folder.clone().to_string()])), ..Default::default() }),
            Box::new(FolderManagerTask { input_data: TaskInputData::FolderManager(FolderManagerAction::Remove(vec![self.cardano_folder.clone()])), ..Default::default() }),
            Box::new(RunCommandTask { input_data: build_clone_repo_command(&config.build_cardano_node.cnode_repository, &self.git_folder.to_string()), ..Default::default() }),
            Box::new(FileManagerTask { input_data: TaskInputData::FileManager(FileManagerAction::CreateFileString((self.libsodium_ported_file.to_string(), config.build_cardano_node.cnode_ported_libsodium_data.clone().to_string()))), ..Default::default() }),
            Box::new(RunCommandTask { input_data: build_fetch_all_command(&self.cardano_folder.clone()), ..Default::default() }),
            Box::new(RunCommandTask { input_data: build_checkout_version_command(&self.version, &self.cardano_folder), ..Default::default() }),
            Box::new(RunCommandTask { input_data: build_cabal_update_command(&self.ghcup_folder), ..Default::default() }),
            Box::new(RunCommandTask { input_data: build_run_cabal_command(&self.ghcup_folder, &self.cardano_folder, &config.binaries.required_files), ..Default::default() }),
        ], config, L2, context)
    }

    fn check(self: &Self, context: &mut Context, config: &RemoteConfig) -> Result<Success, Message> {
        Ok(Success::default())
    }

    fn get_type(self: &Self) -> TaskType {
        TaskType::BuildCardanoNode
    }

    fn get_id(self: &Self) -> String {
        "".to_string()
    }
}

fn build_clone_repo_command(repo: &String, path: &String) -> RunCommandInputData {
    let args = vec![Cmd::Clone.as_string(), repo.clone()];
    RunCommandInputData {
        command: TaskInputData::String(Cmd::Git.as_string()),
        args: TaskInputData::VecString(args),
        current_dir: TaskInputData::String(path.to_string()),
        description: TaskInputData::String("Cloning cardano node repository".to_string()),
    }
}

fn build_fetch_all_command(path: &String) -> RunCommandInputData {
    let args = vec![Cmd::Fetch.as_string(), "--all".to_string(), "--recurse-submodules".to_string(), "--tags".to_string()];
    RunCommandInputData {
        command: TaskInputData::String(Cmd::Git.as_string()),
        args: TaskInputData::VecString(args),
        current_dir: TaskInputData::String(path.to_string()),
        description: TaskInputData::String("Fetch cardano node repository".to_string()),
    }
}

fn build_checkout_version_command(version: &String, path: &String) -> RunCommandInputData {
    let arg_version = version.to_string();
    let args = vec![Cmd::Checkout.as_string(), arg_version];
    RunCommandInputData {
        command: TaskInputData::String(Cmd::Git.as_string()),
        args: TaskInputData::VecString(args),
        current_dir: TaskInputData::String(path.to_string()),
        description: TaskInputData::String(format!("changing to the version {}", version)),
    }
}

fn build_run_cabal_command(cabal_path: &String, folder_path: &String, binaries: &Vec<String>) -> RunCommandInputData {
    let mut args: Vec<String> = vec![Cmd::Build.as_string()];
    for binary in binaries {
        args.push(binary.to_string());
    }
    RunCommandInputData {
        command: TaskInputData::String(url_build(vec![&cabal_path, &Cmd::Cabal.as_string()], false)),
        args: TaskInputData::VecString(args),
        current_dir: TaskInputData::String(folder_path.to_string()),
        description: TaskInputData::String("Updating cabal packages".to_string()),
    }
}

fn build_cabal_update_command(cabal_path: &String) -> RunCommandInputData {
    let args: Vec<String> = vec![url_build(vec![cabal_path, &Cmd::Cabal.as_string()], false), Cmd::Update.as_string()];
    RunCommandInputData {
        command: TaskInputData::String(Cmd::Sudo.as_string()),
        args: TaskInputData::VecString(args),
        description: TaskInputData::String("Building cardano node".to_string()),
        ..Default::default()
    }
}