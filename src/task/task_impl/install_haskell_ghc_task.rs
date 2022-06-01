use std::path::Path;
use crate::config::config::{get_config, get_home_dir, Init};
use crate::env::Env;
use crate::task::message_type::MessageType;
use crate::task::task::{Message, Success, Task};
use crate::task::task_impl::run_command_task::{Cmd, RunCommandInputData, RunCommandTask};
use crate::task::task_manager;
use crate::task::task_type::TaskType;
use crate::utils::download_manager::download;

pub struct InstallHanskellGhcTask {}

#[derive(Debug, Clone)]
pub struct InstallHanskellGhcOutputData {
    pub ghcup_path: String,
}

const SUBST: &str = "s/read /#/g";
const GHCUP_BIN_PATH: &str = "/.ghcup/bin";

impl Task for InstallHanskellGhcTask {
    fn run(self: &Self, env: &mut Env) -> Result<Success, Message> {
        let config = get_config();
        if let Err(error) = config {
            return Err(error);
        }
        let config = config.as_ref().unwrap();

        let home_dir = get_home_dir();
        if let Err(error) = home_dir {
            return Err(error);
        }

        let uri = download_install_ghc_file(&config.init);
        if let Err(error) = uri {
            return Err(error);
        }

        let mut ghcup_dir = String::from(home_dir.as_ref().unwrap());
        ghcup_dir.push_str(GHCUP_BIN_PATH);

        let result = task_manager::start(vec![
            Box::new(RunCommandTask { input_data: build_sed_install_file_command(uri.as_ref().unwrap()) }),
            Box::new(RunCommandTask { input_data: build_install_command(uri.unwrap()) }),
            Box::new(RunCommandTask { input_data: build_install_ghc_version_command(ghcup_dir.clone(), &config.init.haskell_ghc_version) }),
            Box::new(RunCommandTask { input_data: build_set_ghc_version_command(ghcup_dir.clone(), &config.init.haskell_ghc_version) }),
            Box::new(RunCommandTask { input_data: build_install_cabal_version_command(ghcup_dir.clone(), &config.init.haskell_cabal_version) }),
            Box::new(RunCommandTask { input_data: build_set_cabal_version_command(ghcup_dir.clone(), &config.init.haskell_cabal_version) }),
        ]);

        *env = Env::InstallHaskellGhc(InstallHanskellGhcOutputData { ghcup_path: ghcup_dir.clone() });
        result
    }

    fn check(self: &Self, env: &mut Env) -> Result<Success, Message> {
        match env {
            Env::InstallHaskellGhc(output) => {
                let path = Path::new(output.ghcup_path.as_str());
                if path.is_dir() {
                    Ok(Success {})
                } else {
                    Err(Message { code: 0, message: format!("Not found directory: {}", output.ghcup_path), kind: MessageType::Error, task: "".to_string(), stack: vec![] })
                }
            }
            _ => Err(Message { code: 0, message: format!("task type {} is expected", self.get_type()), kind: MessageType::Error, task: "".to_string(), stack: vec![] })
        }
    }

    fn get_type(self: &Self) -> TaskType {
        TaskType::InstallHaskellGsh
    }
}

fn download_install_ghc_file(init: &Init) -> Result<String, Message> {
    download(&init.ghcup_path, format!("/{}", init.install_ghc_file).as_str())
}

fn build_sed_install_file_command(uri: &String) -> RunCommandInputData {
    let args = vec!["-i".to_string(), "-e".to_string(), SUBST.to_string(), uri.to_string()];
    RunCommandInputData { command: Cmd::Sed.as_string(), args, ..Default::default() }
}

fn build_install_command(uri: String) -> RunCommandInputData {
    let args = vec![uri];
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