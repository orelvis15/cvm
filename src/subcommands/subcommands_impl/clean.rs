#![allow(dead_code, unused_variables)]

use clap::{ArgMatches};
use crate::{Command, Message, Success, Term};
use crate::config::remote_config::RemoteConfig;
use crate::task::task_impl::commons::folder_manager_task::{FolderManagerAction, FolderManagerTask};
use crate::task::task_impl::commons::run_command_task::{Cmd, RunCommandInputData, RunCommandTask};
use crate::task_manager::task_manager::TaskManager;
use crate::term::log_level::LogLevel::L1;
use crate::utils::folders::Folder;

pub struct Clean{}

impl Command for Clean{
    fn start(command: &ArgMatches, config: &RemoteConfig, term: &mut Term) -> Result<Success, Message> {

        let home_dir = Folder::get_home_dir()?;
        let mut ghcup_dir = String::from(home_dir);
        ghcup_dir.push_str(format!("/{}", &config.init.ghcup_bin_path).as_str());
        let git_folder = Folder::get_path(Folder::GIT, &config);


        TaskManager::default().start(vec![
            Box::new(RunCommandTask { input_data: build_cabal_clean_command(ghcup_dir), command_description: "Cleaning cabal cache, package and build temporal files".to_string() }),
            Box::new(FolderManagerTask { input_data: FolderManagerAction::Clean(vec![git_folder]) }),
        ], config, term, L1)
    }
}


fn build_cabal_clean_command(ghcup_dir: String) -> RunCommandInputData{
    let args = vec![Cmd::Clean.as_string()];
    RunCommandInputData { command: Cmd::Cabal.as_string(), args, current_dir: ghcup_dir }
}