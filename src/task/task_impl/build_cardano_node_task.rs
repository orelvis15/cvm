#![allow(dead_code, unused_variables)]

use std::fs;
use std::path::Path;
use crate::env::Env;
use crate::{Success, url_build};
use crate::config::config::{Config, get_home_dir};
use crate::task::cvm_error::CvmError;
use crate::task::folders::Folder;
use crate::task::task::Task;
use crate::task::task_impl::copy_bin_task::{CopyBinInputData, CopyBinTask};
use crate::task::task_impl::run_command_task::{Cmd, RunCommandInputData, RunCommandTask};
use crate::task::task_manager::TaskManager;
use crate::task::task_type::TaskType;

pub struct BuildCardanoNodeTask {
    pub version: String,
}

impl Task for BuildCardanoNodeTask {
    fn run(self: &Self, _env: &mut Env, config: &Config) -> Result<Success, CvmError> {

        sudo::escalate_if_needed().expect("Super user permissions are required");

        let home_dir = get_home_dir()?;

        let repo = &config.build_cardano_node.cnode_repository;
        let git_folder = Folder::get_path(Folder::GIT, &config);
        let cardano_folder = url_build(vec![git_folder.as_str(), &config.build_cardano_node.cnode_repository_name], false);
        let cardano_folder_path = Path::new(cardano_folder.as_str());
        let cabal_route = url_build(vec![home_dir.as_str(), &config.init.ghcup_bin_path], false);

        if cardano_folder_path.exists() {
            fs::remove_dir_all(cardano_folder_path)?;
        };

        TaskManager::start(vec![
            Box::new(RunCommandTask { input_data: build_clone_repo_command(repo.clone(), git_folder) }),
            Box::new(RunCommandTask { input_data: build_fetch_all_command(cardano_folder.clone()) }),
            Box::new(RunCommandTask { input_data: build_checkout_version_command(self.version.clone(), cardano_folder.clone()) }),
            Box::new(RunCommandTask { input_data: build_run_cabal_command(cabal_route, cardano_folder.clone(), &config.binaries.files) }),
            Box::new(CopyBinTask { input_data: CopyBinInputData { files_names: config.binaries.files.clone(), origin_path: cardano_folder.clone(), version: self.version.clone() } }),
        ], config)
    }

    fn check(self: &Self, _env: &mut Env, config: &Config) -> Result<Success, CvmError> {
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

fn build_run_cabal_command(cabal_path: String, folder_path: String, files: &Vec<String>) -> RunCommandInputData {
    let mut args:Vec<String> = vec![Cmd::Build.as_string()];
    for file in files {
        args.push(file.to_string());
    }
    RunCommandInputData { command: url_build(vec![cabal_path.as_str(), Cmd::Cabal.as_string().as_str()], false), args, current_dir: folder_path }
}