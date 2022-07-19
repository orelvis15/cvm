#![allow(dead_code, unused_variables)]

use clap::{ArgMatches};
use crate::{CommandStrategy, config, Message, Success};
use crate::context::context::Context;
use crate::task::task_impl::commons::command::run_command_io_data::RunCommandInputData;
use crate::task::task_impl::commons::folder_manager::folder_manager_task::FolderManagerTask;
use crate::task::task_impl::commons::command::run_command_task::{Cmd, RunCommandTask};
use crate::task::task_impl::commons::folder_manager::folder_manager_io_data::FolderManagerAction;
use crate::task::task_impl::task_input_data::TaskInputData;
use crate::task_manager::task_manager::TaskManager;
use crate::term::log_level::LogLevel::L1;
use crate::resolvers::folders::custom_folders::CustomFolders;

pub struct Clean {}

impl CommandStrategy for Clean {
    fn start(command: &ArgMatches, context: &mut Context) -> Result<Success, Message> {
        let config = config::remote_config::get_remote_config(context)?;

        let home_dir = CustomFolders::get_home_dir()?;
        let mut ghcup_dir = String::from(home_dir);
        ghcup_dir.push_str(format!("/{}", &config.init.ghcup_bin_path).as_str());
        let git_folder = CustomFolders::get_path_string(&CustomFolders::GIT, &config);


        TaskManager::default().start(vec![
            Box::new(RunCommandTask { input_data: build_cabal_clean_command(ghcup_dir), ..Default::default() }),
            Box::new(FolderManagerTask { input_data: TaskInputData::FolderManager(FolderManagerAction::Clean(vec![git_folder])), ..Default::default() }),
        ], &config, L1, context)
    }
}


fn build_cabal_clean_command(ghcup_dir: String) -> RunCommandInputData {
    let args = vec![Cmd::Clean.as_string()];
    RunCommandInputData {
        command: TaskInputData::String(Cmd::Cabal.as_string()),
        args: TaskInputData::VecString(args),
        current_dir: TaskInputData::String(ghcup_dir),
        description: TaskInputData::String("Cleaning cabal cache, package and build temporal files".to_string()),
    }
}