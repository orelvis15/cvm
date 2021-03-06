#![allow(dead_code, unused_variables)]

use crate::config::remote_config::{RemoteConfig, Init};
use crate::env::Env;
use crate::message::message::Message;
use crate::task::task::{Success, Task};
use crate::task::task_impl::commons::run_command_task::{Cmd, RunCommandInputData, RunCommandTask};
use crate::task_manager::task_manager::TaskManager;
use crate::task::task_type::TaskType;
use crate::{Term, url_build};
use crate::config::state_config::{get_task_complete, set_task_complete};
use crate::task::task_impl::commons::file_manager_task::{FileManagerAction, FileManagerTask};
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
    fn prepare(self: &mut Self, env: &mut Env, config: &RemoteConfig, term: &mut Term) -> Result<bool, Message> {

        if get_task_complete(&self.get_type()) {
            return Ok(false);
        };

        self.home = Folder::get_home_dir()?;
        self.install_sh_uri = download_install_ghc_file(&config.init)?;
        self.ghcup_dir = url_build(vec![&self.home, &config.init.ghcup_bin_path], false);
        Ok(true)
    }

    fn run(self: &Self, env: &mut Env, config: &RemoteConfig, term: &mut Term) -> Result<Success, Message> {
        TaskManager::default().start(vec![
            Box::new(RunCommandTask { input_data: build_sed_install_file_command(&self.install_sh_uri, &config.init.ghcup_pattern_sed), command_description: "Editing ghcup installation file".to_string() }),
            Box::new(RunCommandTask { input_data: build_install_command(&self.install_sh_uri), command_description: "Installing ghcup".to_string() }),
            Box::new(RunCommandTask { input_data: build_install_ghc_version_command(&self.ghcup_dir, &config.init.haskell_ghc_version), command_description: "Installing ghc".to_string() }),
            Box::new(RunCommandTask { input_data: build_set_ghc_version_command(&self.ghcup_dir, &config.init.haskell_ghc_version), command_description: "Changing to the corresponding version of ghc".to_string() }),
            Box::new(RunCommandTask { input_data: build_install_cabal_version_command(&self.ghcup_dir, &config.init.haskell_cabal_version), command_description: "Installing cabal".to_string() }),
            Box::new(RunCommandTask { input_data: build_set_cabal_version_command(&self.ghcup_dir, &config.init.haskell_cabal_version), command_description: "Changing to the corresponding version of cabal".to_string() }),
        ], config, term, L2)
    }

    fn check(self: &Self, env: &mut Env, config: &RemoteConfig, term: &mut Term) -> Result<Success, Message> {
        let home_dir = Folder::get_home_dir()?;
        let cabal_bin_path = url_build(vec![&home_dir, &config.init.ghcup_bin_path, &"cabal".to_string()], false);
        let ghc_bin_path = url_build(vec![&home_dir, &config.init.ghcup_bin_path, &"ghc".to_string()], false);

        let result = TaskManager {}.start(vec![
            Box::new(FileManagerTask { input_data: FileManagerAction::Check(vec![cabal_bin_path, ghc_bin_path]) }),
        ], config, term, L2);
        set_task_complete(&self.get_type());
        result
    }

    fn get_type(self: &Self) -> TaskType {
        TaskType::InstallGhcup
    }
}

fn download_install_ghc_file(init: &Init) -> Result<String, Message> {
    download(&init.ghcup_url, init.install_ghc_file.as_str())
}

fn build_sed_install_file_command(uri: &String, pattern: &String) -> RunCommandInputData {
    let args = vec!["-i".to_string(), "-e".to_string(), pattern.to_string(), uri.to_string()];
    RunCommandInputData { command: Cmd::Sed.as_string(), args, ..Default::default() }
}

fn build_install_command(uri: &String) -> RunCommandInputData {
    let args = vec![uri.to_string(), "&>/dev/null".to_string()];
    RunCommandInputData { command: Cmd::Bash.as_string(), args, ..Default::default() }
}

fn build_install_ghc_version_command(ghcup_dir: &String, version: &String) -> RunCommandInputData {
    let args = vec![Cmd::Ghcup.as_string(), Cmd::Install.as_string(), Cmd::Ghc.as_string(), version.to_string()];
    RunCommandInputData { command: Cmd::Sudo.as_string(), args, current_dir: ghcup_dir.to_string() }
}

fn build_set_ghc_version_command(ghcup_dir: &String, version: &String) -> RunCommandInputData {
    let args = vec![Cmd::Ghcup.as_string(), Cmd::Set.as_string(), Cmd::Ghc.as_string(), version.to_string()];
    RunCommandInputData { command: Cmd::Sudo.as_string(), args, current_dir: ghcup_dir.to_string() }
}

fn build_install_cabal_version_command(ghcup_dir: &String, version: &String) -> RunCommandInputData {
    let args = vec![Cmd::Ghcup.as_string(), Cmd::Install.as_string(), Cmd::Cabal.as_string(), version.to_string()];
    RunCommandInputData { command: Cmd::Sudo.as_string(), args, current_dir: ghcup_dir.to_string() }
}

fn build_set_cabal_version_command(ghcup_dir: &String, version: &String) -> RunCommandInputData {
    let args = vec![Cmd::Ghcup.as_string(), Cmd::Set.as_string(), Cmd::Cabal.as_string(), version.to_string()];
    RunCommandInputData { command: Cmd::Sudo.as_string(), args, current_dir: ghcup_dir.to_string() }
}