#![allow(dead_code, unused_variables)]

use std::fs;
use std::path::Path;
use crate::env::Env;
use crate::{Success, url_build};
use crate::config::config::{get_config, get_home_dir, get_project_dir};
use crate::task::cvm_error::CvmError;
use crate::task::folders::Folder;
use crate::task::task::Task;
use crate::task::task_impl::copy_bin_task::{CopyBinInputData, CopyBinTask};
use crate::task::task_impl::run_command_task::{Cmd, RunCommandInputData, RunCommandTask};
use crate::task::task_manager;
use crate::task::task_type::TaskType;

pub struct BuildCardanoNodeTask {
    pub version: String,
}

impl Task for BuildCardanoNodeTask {
    fn run(self: &Self, _env: &mut Env) -> Result<Success, CvmError> {

        sudo::escalate_if_needed().expect("Super user permissions are required");

        let config = get_config();
        if let Err(error) = config {
            return Err(error);
        }
        let config = config.as_ref().unwrap();

        let home_dir = get_home_dir();
        if let Err(error) = home_dir {
            return Err(error);
        }

        let project_dir = get_project_dir();

        let repo = &config.build_cardano_node.cnode_repository;
        let git_folder = url_build(vec![project_dir.as_str(), Folder::get(Folder::ROOT, &config), Folder::get(Folder::GIT, &config)], false);
        let cardano_folder = url_build(vec![git_folder.as_str(), &config.build_cardano_node.cnode_repository_name], false);
        let cardano_folder_path = Path::new(cardano_folder.as_str());
        let cabal_route = url_build(vec![home_dir.as_ref().unwrap().as_str(), &config.init.ghcup_bin_path], false);

        if cardano_folder_path.exists() {
            fs::remove_dir_all(cardano_folder_path)?;
        };

        let cardano_node_file_name = config.binaries.cardano_node.to_string();
        let cardano_cli_file_name = config.binaries.cardano_cli.to_string();

        task_manager::start(vec![
            Box::new(RunCommandTask { input_data: build_clone_repo_command(repo.clone(), git_folder) }),
            Box::new(RunCommandTask { input_data: build_fetch_all_command(cardano_folder.clone()) }),
            Box::new(RunCommandTask { input_data: build_checkout_version_command(self.version.clone(), cardano_folder.clone()) }),
            Box::new(RunCommandTask { input_data: build_run_cabal_command(cabal_route, cardano_folder.clone(), cardano_node_file_name.clone(), cardano_cli_file_name.clone()) }),
            Box::new(CopyBinTask { input_data: CopyBinInputData { file_name: cardano_node_file_name, origin_path: cardano_folder.clone(), version: self.version.clone() } }),
            Box::new(CopyBinTask { input_data: CopyBinInputData { file_name: cardano_cli_file_name, origin_path: cardano_folder.clone(), version: self.version.clone() } }),
        ])
    }

    fn check(self: &Self, _env: &mut Env) -> Result<Success, CvmError> {
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

fn build_run_cabal_command(cabal_path: String, folder_path: String, cardano_node_file_name: String, cardano_cli_file_name: String) -> RunCommandInputData {
    let args = vec![Cmd::Build.as_string(), cardano_node_file_name, cardano_cli_file_name];
    RunCommandInputData { command: url_build(vec![cabal_path.as_str(), Cmd::Cabal.as_string().as_str()], false), args, current_dir: folder_path }
}