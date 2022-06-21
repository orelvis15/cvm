#![allow(dead_code, unused_variables)]

use std::fs;
use std::path::Path;
use crate::env::Env;
use crate::{Success, Term, url_build};
use crate::config::config::{Config, get_home_dir};
use crate::error::message::Message;
use crate::utils::folders::Folder;
use crate::task::task::Task;
use crate::task::task_impl::commons::folder_manager_task::{FolderManagerAction, FolderManagerTask};
use crate::task::task_impl::commons::permission_task::{PermissionAction, PermissionTask};
use crate::task::task_impl::install::copy_bin_task::{CopyBinInputData, CopyBinTask};
use crate::task::task_impl::commons::run_command_task::{Cmd, RunCommandInputData, RunCommandTask};
use crate::task_manager::task_manager::TaskManager;
use crate::task::task_type::TaskType;
use crate::term::log_level::LogLevel::L2;

pub struct BuildCardanoNodeTask {
    pub version: String,
}

impl Task for BuildCardanoNodeTask {
    fn run(self: &Self, _env: &mut Env, config: &Config, term: &mut Term) -> Result<Success, Message> {
        let home_dir = get_home_dir()?;

        let repo = &config.build_cardano_node.cnode_repository;
        let git_folder = Folder::get_path(Folder::GIT, &config);
        let cardano_folder = url_build(vec![&git_folder, &config.build_cardano_node.cnode_repository_name], false);
        let cardano_folder_path = Path::new(cardano_folder.as_str());
        let cabal_route = url_build(vec![&home_dir, &config.init.ghcup_bin_path], false);

        TaskManager {}.start(vec![
            Box::new(PermissionTask { input_data: PermissionAction::CheckWrite(vec![cardano_folder.to_string()]) }),
            Box::new(FolderManagerTask { input_data: FolderManagerAction::Remove(vec![cardano_folder.clone()]) }),
            Box::new(RunCommandTask { input_data: build_clone_repo_command(repo.clone(), git_folder), command_description: "Cloning cardano node repository".to_string() }),
            Box::new(RunCommandTask { input_data: build_fetch_all_command(cardano_folder.clone()), command_description: "Fetch cardano node repository".to_string() }),
            Box::new(RunCommandTask { input_data: build_checkout_version_command(self.version.clone(), cardano_folder.clone()), command_description: format!("changing to the version {}", &self.version) }),
            Box::new(RunCommandTask { input_data: build_cabal_update_command(&cabal_route), command_description: "Updating cabal packages".to_string() }),
            Box::new(RunCommandTask { input_data: build_run_cabal_command(cabal_route, cardano_folder.clone()), command_description: "Building cardano node".to_string() }),
            Box::new(CopyBinTask { input_data: CopyBinInputData { files_names: config.binaries.files.clone(), origin_path: cardano_folder.clone(), version: self.version.clone() } }),
        ], config, term, L2)
    }

    fn check(self: &Self, _env: &mut Env, config: &Config, term: &mut Term) -> Result<Success, Message> {
        Ok(Success {})
    }

    fn get_type(self: &Self) -> TaskType {
        TaskType::BuildCardanoNode
    }
}

fn build_clone_repo_command(repo: String, path: String) -> RunCommandInputData {
    let args = vec![Cmd::Clone.as_string(), repo];
    RunCommandInputData { command: Cmd::Git.as_string(), args, current_dir: path }
}

fn build_fetch_all_command(path: String) -> RunCommandInputData {
    let args = vec![Cmd::Fetch.as_string(), "--all".to_string(), "--recurse-submodules".to_string(), "--tags".to_string()];
    RunCommandInputData { command: Cmd::Git.as_string(), args, current_dir: path }
}

fn build_checkout_version_command(version: String, path: String) -> RunCommandInputData {
    let arg_version = version.to_string();
    let args = vec![Cmd::Checkout.as_string(), arg_version];
    RunCommandInputData { command: Cmd::Git.as_string(), args, current_dir: path }
}

fn build_run_cabal_command(cabal_path: String, folder_path: String) -> RunCommandInputData {
    let args: Vec<String> = vec![Cmd::Build.as_string(), Cmd::All.as_string()];
    RunCommandInputData { command: url_build(vec![&cabal_path, &Cmd::Cabal.as_string()], false), args, current_dir: folder_path }
}

fn build_cabal_update_command(cabal_path: &String) -> RunCommandInputData {
    let args: Vec<String> = vec![Cmd::Update.as_string()];
    RunCommandInputData { command: url_build(vec![cabal_path, &Cmd::Cabal.as_string()], false), args, current_dir: "".to_string() }
}