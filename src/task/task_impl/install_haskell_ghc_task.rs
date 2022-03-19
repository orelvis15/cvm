
use std::path::Path;
use crate::config::config::{get_config, get_home_dir, Init};
use crate::config::enviroment::{Enviroment, set_env};
use crate::env::Env;
use crate::task::task::{Error, Success, Task};
use crate::task::task_impl::run_command_task::{Cmd, RunCommandInputData, RunCommandTask};

use crate::task::task_manager;
use crate::task::task_type::TaskType;
use crate::utils::download_manager::download;

pub struct InstallHanskellGhcTask {}

#[derive(Debug, Clone)]
pub struct InstallHanskellGhcOutputData {
    pub ghcup_path: String,
}

const SUBST: &str = "s#read.*#answer=Y;next_answer=P;hls_answer=N#";
const GHCUP_BIN_PATH: &str = "/.ghcup/bin";

impl Task for InstallHanskellGhcTask {
    fn run(self: &Self, env: &mut Env) -> Result<Success, Error> {

        let config = get_config();
        if let Err(error) = config {
            return Result::Err(error);
        }

        let home_dir = get_home_dir();
        if let Err(error) = home_dir {
            return Result::Err(error);
        }

        let uri = download_install_ghc_file(config.clone().unwrap().init);
        if let Err(error) = uri {
            return Result::Err(error);
        }

        let mut ghcup_dir = String::from(home_dir.as_ref().unwrap());
        ghcup_dir.push_str(GHCUP_BIN_PATH);

        let result = task_manager::start(vec![
            Box::new(RunCommandTask { input_data: build_sed_install_file_command(uri.as_ref().unwrap()) }),
            Box::new(RunCommandTask { input_data: build_install_command(uri.unwrap()) }),
            Box::new(RunCommandTask { input_data: build_install_ghc_version_command(ghcup_dir.clone(), &config.as_ref().unwrap().init.haskell_ghc_version) }),
            Box::new(RunCommandTask { input_data: build_set_ghc_version_command(ghcup_dir.clone(), &config.as_ref().unwrap().init.haskell_ghc_version) }),
            Box::new(RunCommandTask { input_data: build_install_cabal_version_command(ghcup_dir.clone(), &config.as_ref().unwrap().init.haskell_cabal_version) }),
            Box::new(RunCommandTask { input_data: build_set_cabal_version_command(ghcup_dir.clone(), &config.as_ref().unwrap().init.haskell_cabal_version) }),
        ]);

        set_env(Enviroment {
            ghcup_home: ghcup_dir.clone(),
            cabal_home: ghcup_dir.clone(),
            ghc_home: ghcup_dir.clone(),
            ..Default::default()
        });
        *env = Env::InstallHaskellGhc(InstallHanskellGhcOutputData { ghcup_path: ghcup_dir.clone() });
        result
    }

    fn check(self: &Self, env: &mut Env) -> Result<Success, Error> {
        match env {
            Env::InstallHaskellGhc(output) => {
                let path = Path::new(output.ghcup_path.as_str());
                if path.is_dir() {
                    Result::Ok(Success {})
                } else {
                    Result::Err(Error { code: 0, message: format!("Not found directory: {}", output.ghcup_path), task: "".to_string(), stack: vec![] })
                }
            }
            _ => Result::Err(Error { code: 0, message: format!("task type {} is expected", self.get_type()), task: "".to_string(), stack: vec![] })
        }
    }

    fn get_type(self: &Self) -> TaskType {
        TaskType::InstallHaskellGsh
    }
}

fn download_install_ghc_file(init: Init) -> Result<String, Error> {
    download(init.ghcup_path, format!("/{}", init.install_ghc_file).as_str())
}

fn build_sed_install_file_command(uri: &String) -> RunCommandInputData {
    let args = vec!["-i".to_string(), "-e".to_string(), SUBST.to_string(), uri.to_string()];
    RunCommandInputData { command: Cmd::Sed.as_str(), args, ..Default::default() }
}

fn build_install_command(uri: String) -> RunCommandInputData {
    let args = vec![uri];
    RunCommandInputData { command: Cmd::Bash.as_str(), args, ..Default::default() }
}

fn build_install_ghc_version_command(ghcup_dir: String, version: &String) -> RunCommandInputData {
    let args = vec![Cmd::Install.as_str(), Cmd::Ghc.as_str(), version.to_string()];
    RunCommandInputData { command: Cmd::Ghcup.as_str(), args, current_dir: ghcup_dir }
}

fn build_set_ghc_version_command(ghcup_dir: String, version: &String) -> RunCommandInputData {
    let args = vec![Cmd::Set.as_str(), Cmd::Ghc.as_str(), version.to_string()];
    RunCommandInputData { command: Cmd::Ghcup.as_str(), args, current_dir: ghcup_dir }
}

fn build_install_cabal_version_command(ghcup_dir: String, version: &String) -> RunCommandInputData {
    let args = vec![Cmd::Install.as_str(), Cmd::Cabal.as_str(), version.to_string()];
    RunCommandInputData { command: Cmd::Ghcup.as_str(), args, current_dir: ghcup_dir }
}

fn build_set_cabal_version_command(ghcup_dir: String, version: &String) -> RunCommandInputData {
    let args = vec![Cmd::Set.as_str(), Cmd::Cabal.as_str(), version.to_string()];
    RunCommandInputData { command: Cmd::Ghcup.as_str(), args, current_dir: ghcup_dir }
}