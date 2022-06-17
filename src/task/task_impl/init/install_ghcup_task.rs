#![allow(dead_code, unused_variables)]

use crate::config::config::{Config, get_home_dir, Init};
use crate::env::Env;
use crate::error::message::Message;
use crate::task::task::{Success, Task};
use crate::task::task_impl::commons::run_command_task::{Cmd, RunCommandInputData, RunCommandTask};
use crate::task_manager::task_manager::TaskManager;
use crate::task::task_type::TaskType;
use crate::Term;
use crate::term::log_level::LogLevel::L2;
use crate::utils::download_manager::download;

pub struct InstallHanskellGhcTask {}

#[derive(Debug, Clone)]
pub struct InstallHanskellGhcOutputData {
    pub ghcup_path: String,
}

impl Task for InstallHanskellGhcTask {
    fn run(self: &Self, env: &mut Env, config: &Config, term: &mut Term) -> Result<Success, Message> {
        let home_dir = get_home_dir()?;
        let uri = download_install_ghc_file(&config.init)?;

        let mut ghcup_dir = String::from(home_dir);
        ghcup_dir.push_str(format!("/{}", &config.init.ghcup_bin_path).as_str());

        TaskManager{}.start(vec![
            Box::new(RunCommandTask { input_data: build_sed_install_file_command(&uri, &config.init.ghcup_pattern_sed), command_description: "Editing ghcup installation file".to_string() }),
            Box::new(RunCommandTask { input_data: build_install_command(uri), command_description: "Installing ghcup".to_string() }),
            Box::new(RunCommandTask { input_data: build_install_ghc_version_command(ghcup_dir.clone(), &config.init.haskell_ghc_version), command_description: "Installing ghc".to_string() }),
            Box::new(RunCommandTask { input_data: build_set_ghc_version_command(ghcup_dir.clone(), &config.init.haskell_ghc_version), command_description: "Changing to the corresponding version of ghc".to_string() }),
            Box::new(RunCommandTask { input_data: build_install_cabal_version_command(ghcup_dir.clone(), &config.init.haskell_cabal_version), command_description: "Installing cabal".to_string() }),
            Box::new(RunCommandTask { input_data: build_set_cabal_version_command(ghcup_dir.clone(), &config.init.haskell_cabal_version), command_description: "Changing to the corresponding version of cabal".to_string() }),
        ], config, term, L2)
    }

    fn check(self: &Self, env: &mut Env, config: &Config, term: &mut Term) -> Result<Success, Message> {
        Ok(Success{})
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

fn build_install_command(uri: String) -> RunCommandInputData {
    let args = vec![uri, "&>/dev/null".to_string()];
    RunCommandInputData { command: Cmd::Bash.as_string(), args, ..Default::default() }
}

fn build_install_ghc_version_command(ghcup_dir: String, version: &String) -> RunCommandInputData {
    let args = vec![Cmd::Install.as_string(), Cmd::Ghc.as_string(), version.to_string()];
    RunCommandInputData { command: Cmd::Ghcup.as_string(), args, current_dir: ghcup_dir }
}

fn build_set_ghc_version_command(ghcup_dir: String, version: &String) -> RunCommandInputData {
    let args = vec![Cmd::Set.as_string(), Cmd::Ghc.as_string(), version.to_string()];
    RunCommandInputData { command: Cmd::Ghcup.as_string(), args, current_dir: ghcup_dir }
}

fn build_install_cabal_version_command(ghcup_dir: String, version: &String) -> RunCommandInputData {
    let args = vec![Cmd::Install.as_string(), Cmd::Cabal.as_string(), version.to_string()];
    RunCommandInputData { command: Cmd::Ghcup.as_string(), args, current_dir: ghcup_dir }
}

fn build_set_cabal_version_command(ghcup_dir: String, version: &String) -> RunCommandInputData {
    let args = vec![Cmd::Set.as_string(), Cmd::Cabal.as_string(), version.to_string()];
    RunCommandInputData { command: Cmd::Ghcup.as_string(), args, current_dir: ghcup_dir }
}