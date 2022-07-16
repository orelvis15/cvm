#![allow(dead_code, unused_variables)]

use crate::context::context::Context;
use crate::config::remote_config::{RemoteConfig, Init};
use crate::message::message::Message;
use crate::task::task::{Success, Task};
use crate::task::task_impl::commons::command::run_command_task::{Cmd, RunCommandTask};
use crate::task_manager::task_manager::TaskManager;
use crate::task::task_type::TaskType;
use crate::url_build;
use crate::config::state_config::{get_task_complete, set_task_complete};
use crate::task::task_impl::commons::command::run_command_io_data::RunCommandInputData;
use crate::task::task_impl::commons::file_manager::file_manager_io_data::FileManagerAction;
use crate::task::task_impl::commons::file_manager::file_manager_task::FileManagerTask;
use crate::task::task_impl::task_input_data::TaskInputData;
use crate::term::log_level::LogLevel::L2;
use crate::utils::download_manager::download;
use crate::utils::folders::Folder;

#[derive(Default)]
pub struct InstallHanskellGhcTask {
    home: String,
    install_sh_uri: String,
    ghcup_dir: String,
}

#[derive(Debug, Clone)]
pub struct InstallHanskellGhcOutputData {
    pub ghcup_path: String,
}

impl Task for InstallHanskellGhcTask {
    fn prepare(self: &mut Self, context: &mut Context, config: &RemoteConfig) -> Result<bool, Message> {
        if get_task_complete(&self.get_type()) {
            return Ok(false);
        };

        self.home = Folder::get_home_dir()?;
        self.install_sh_uri = download_install_ghc_file(&config.init)?;
        self.ghcup_dir = url_build(vec![&self.home, &config.init.ghcup_bin_path], false);
        Ok(true)
    }

    fn run(self: &Self, context: &mut Context, config: &RemoteConfig) -> Result<Success, Message> {
        TaskManager::default().start(vec![
            Box::new(RunCommandTask { input_data: build_sed_install_file_command(&self.install_sh_uri, &config.init.ghcup_pattern_sed), ..Default::default() }),
            Box::new(RunCommandTask { input_data: build_install_command(&self.install_sh_uri), ..Default::default() }),
            Box::new(RunCommandTask { input_data: build_install_ghc_version_command(&self.ghcup_dir, &config.init.haskell_ghc_version), ..Default::default() }),
            Box::new(RunCommandTask { input_data: build_set_ghc_version_command(&self.ghcup_dir, &config.init.haskell_ghc_version), ..Default::default() }),
            Box::new(RunCommandTask { input_data: build_install_cabal_version_command(&self.ghcup_dir, &config.init.haskell_cabal_version), ..Default::default() }),
            Box::new(RunCommandTask { input_data: build_set_cabal_version_command(&self.ghcup_dir, &config.init.haskell_cabal_version), ..Default::default() }),
        ], config, L2, context)
    }

    fn check(self: &Self, context: &mut Context, config: &RemoteConfig) -> Result<Success, Message> {
        let home_dir = Folder::get_home_dir()?;
        let cabal_bin_path = url_build(vec![&home_dir, &config.init.ghcup_bin_path, &"cabal".to_string()], false);
        let ghc_bin_path = url_build(vec![&home_dir, &config.init.ghcup_bin_path, &"ghc".to_string()], false);

        let result = TaskManager {}.start(vec![
            Box::new(FileManagerTask { input_data: TaskInputData::FileManager(FileManagerAction::Check(vec![cabal_bin_path, ghc_bin_path])), ..Default::default() }),
        ], config, L2, context);
        set_task_complete(&self.get_type());
        result
    }

    fn get_type(self: &Self) -> TaskType {
        TaskType::InstallGhcup
    }

    fn get_id(self: &Self) -> String {
        "".to_string()
    }
}

fn download_install_ghc_file(init: &Init) -> Result<String, Message> {
    download(&init.ghcup_url, init.install_ghc_file.as_str())
}

fn build_sed_install_file_command(uri: &String, pattern: &String) -> RunCommandInputData {
    let args = vec!["-i".to_string(), "-e".to_string(), pattern.to_string(), uri.to_string()];
    RunCommandInputData {
        command: TaskInputData::String(Cmd::Sed.as_string()),
        args: TaskInputData::VecString(args),
        description: TaskInputData::String("Editing ghcup installation file".to_string()),
        ..Default::default()
    }
}

fn build_install_command(uri: &String) -> RunCommandInputData {
    let args = vec![uri.to_string(), "&>/dev/null".to_string()];
    RunCommandInputData {
        command: TaskInputData::String(Cmd::Bash.as_string()),
        args: TaskInputData::VecString(args),
        description: TaskInputData::String("Installing ghcup".to_string()),
        ..Default::default()
    }
}

fn build_install_ghc_version_command(ghcup_dir: &String, version: &String) -> RunCommandInputData {
    let args = vec![Cmd::Ghcup.as_string(), Cmd::Install.as_string(), Cmd::Ghc.as_string(), version.to_string()];
    RunCommandInputData {
        command: TaskInputData::String(Cmd::Sudo.as_string()),
        args: TaskInputData::VecString(args),
        current_dir: TaskInputData::String(ghcup_dir.to_string()),
        description: TaskInputData::String("Installing ghc".to_string()),
    }
}

fn build_set_ghc_version_command(ghcup_dir: &String, version: &String) -> RunCommandInputData {
    let args = vec![Cmd::Ghcup.as_string(), Cmd::Set.as_string(), Cmd::Ghc.as_string(), version.to_string()];
    RunCommandInputData {
        command: TaskInputData::String(Cmd::Sudo.as_string()),
        args: TaskInputData::VecString(args),
        current_dir: TaskInputData::String(ghcup_dir.to_string()),
        description: TaskInputData::String("Changing to the corresponding version of ghc".to_string()),
    }
}

fn build_install_cabal_version_command(ghcup_dir: &String, version: &String) -> RunCommandInputData {
    let args = vec![Cmd::Ghcup.as_string(), Cmd::Install.as_string(), Cmd::Cabal.as_string(), version.to_string()];
    RunCommandInputData {
        command: TaskInputData::String(Cmd::Sudo.as_string()),
        args: TaskInputData::VecString(args),
        current_dir: TaskInputData::String(ghcup_dir.to_string()),
        description: TaskInputData::String("Installing cabal".to_string()),
    }
}

fn build_set_cabal_version_command(ghcup_dir: &String, version: &String) -> RunCommandInputData {
    let args = vec![Cmd::Ghcup.as_string(), Cmd::Set.as_string(), Cmd::Cabal.as_string(), version.to_string()];
    RunCommandInputData {
        command: TaskInputData::String(Cmd::Sudo.as_string()),
        args: TaskInputData::VecString(args),
        current_dir: TaskInputData::String(ghcup_dir.to_string()),
        description: TaskInputData::String("Changing to the corresponding version of cabal".to_string()),
    }
}