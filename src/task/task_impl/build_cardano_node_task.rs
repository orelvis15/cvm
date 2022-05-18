use std::fs;
use std::fs::DirEntry;
use std::path::Path;
use walkdir::WalkDir;
use crate::env::Env;
use crate::{Message, Success, url_build};
use crate::config::config::{get_config, get_home_dir};
use crate::task::task::Task;
use crate::task::task_impl::copy_bin_task::{CopyBinInputData, CopyBinTask};
use crate::task::task_impl::run_command_task::{Cmd, RunCommandInputData, RunCommandTask};
use crate::task::task_manager;
use crate::task::task_type::TaskType;
use crate::task::task_type::TaskType::CopyBinFiles;

pub struct BuildCardanoNodeTask {
    pub version: String,
}

const CARDANO_REPOSITORY_FOLDER: &str = "cardano-node";
const GIT_FOLDER: &str = "git";
const CARDANO_NODE_FILE_NAME: &str = "cardano-node";
const CARDANO_CLI_FILE_NAME: &str = "cardano-cli";

impl Task for BuildCardanoNodeTask {
    fn run(self: &Self, env: &mut Env) -> Result<Success, Message> {
        let config = get_config();
        if let Err(error) = config {
            return Result::Err(error);
        }

        let home_dir = get_home_dir();
        if let Err(error) = home_dir {
            return Result::Err(error);
        }

        let repo = &config.as_ref().unwrap().build_cardano_node.cnode_repository;
        let git_folder = url_build(vec![home_dir.clone().unwrap().as_str(), &config.as_ref().unwrap().workspace.workspace_folder.as_str(), GIT_FOLDER], false);
        let cardano_folder = url_build(vec![git_folder.as_str(), CARDANO_REPOSITORY_FOLDER], false);
        let cardano_folder_path = Path::new(cardano_folder.as_str());

        if cardano_folder_path.exists() {
            fs::remove_dir_all(cardano_folder_path);
        };

        task_manager::start(vec![
            Box::new(RunCommandTask { input_data: build_clone_repo_command(repo.clone(), git_folder) }),
            Box::new(RunCommandTask { input_data: build_fetch_all_command(cardano_folder.clone()) }),
            Box::new(RunCommandTask { input_data: build_checkout_version_command(self.version.clone(), cardano_folder.clone()) }),
            Box::new(RunCommandTask { input_data: build_run_cabal_command(cardano_folder.clone()) }),
            Box::new(CopyBinTask{ input_data: CopyBinInputData { file_name: CARDANO_NODE_FILE_NAME.to_string(), origin_path: cardano_folder.clone(), version: self.version.clone() } }),
            Box::new(CopyBinTask{ input_data: CopyBinInputData { file_name: CARDANO_CLI_FILE_NAME.to_string(), origin_path: cardano_folder.clone(), version: self.version.clone() } }),
        ])
    }

    fn check(self: &Self, env: &mut Env) -> Result<Success, Message> {
        Result::Ok(Success {})
    }

    fn get_type(self: &Self) -> TaskType {
        TaskType::BuildCardanoNode
    }
}

fn build_clone_repo_command(repo: String, path: String) -> RunCommandInputData {
    let args = vec![Cmd::Clone.as_str(), repo];
    RunCommandInputData { command: Cmd::Git.as_str(), args, current_dir: path }
}

fn build_fetch_all_command(path: String) -> RunCommandInputData {
    let args = vec![Cmd::Fetch.as_str(), "--all".to_string(), "--recurse-submodules".to_string(), "--tags".to_string()];
    RunCommandInputData { command: Cmd::Git.as_str(), args, current_dir: path }
}

fn build_checkout_version_command(version: String, path: String) -> RunCommandInputData {
    let mut arg_version = version.to_string();
    let args = vec![Cmd::Checkout.as_str(), arg_version];
    RunCommandInputData { command: Cmd::Git.as_str(), args, current_dir: path }
}

fn build_run_cabal_command(path: String) -> RunCommandInputData {
    let args = vec![Cmd::Build.as_str(), CARDANO_NODE_FILE_NAME.to_string(), CARDANO_CLI_FILE_NAME.to_string()];
    RunCommandInputData { command: Cmd::Cabal.as_str(), args, current_dir: path }
}