use serde::{Deserialize};
use std::{fs};

use directories::{BaseDirs, ProjectDirs};
use crate::task::message_type::MessageType;

use crate::task::task::{Message, Success};
use crate::utils::download_manager::download;

const CONFIG_URL: &str = "https://raw.githubusercontent.com/orelvis15/cvm_config/master/config.toml";
const QUALIFIER: &str = "com";
const ORGANIZATION: &str = "orelvis15";
const APPLICATION: &str = "cvm";
const FILE_NAME: &str = "config.tom";

pub fn get_config() -> Result<Config, Message> {
    let project_dir = get_config_dir();
    if let Err(error) = project_dir {
        return Err(error);
    }

    let config_dir = project_dir.unwrap().config_dir().to_str().unwrap().to_string();

    let mut path_str: String = config_dir.clone();
    path_str.push_str("/");
    path_str.push_str(FILE_NAME);

    let download_config = download_config(config_dir.clone()
                                          , format!("/{}", FILE_NAME.to_string()));
    if let Err(error) = download_config {
        return Err(error);
    }

    let config_file = fs::read_to_string(
        format!("{}/{}", config_dir, FILE_NAME)
    );

    return match config_file {
        Ok(file) => {
            match toml::from_str(&file) {
                Ok(config) => { Ok(config) }
                Err(error) => {
                    Err(Message {
                        code: 0,
                        message: "Error try parsing config file".to_string(),
                        kind: MessageType::Error,
                        task: "".to_string(),
                        stack: vec![error.to_string()],
                    })
                }
            }
        }
        Err(_) => Err(Message {
            code: 0,
            message: "Error try reading config file".to_string(),
            kind: MessageType::Error,
            task: "".to_string(),
            stack: vec![],
        }),
    };
}

pub fn get_home_dir() -> Result<String, Message> {
    let error = Err(Message {
        code: 0,
        message: "An error occurred reading the home directory".to_string(),
        kind: MessageType::Error,
        task: "".to_string(),
        stack: vec![],
    });

    if let Some(dir) = BaseDirs::new() {
        return if let Some(path) = dir.home_dir().to_str() {
            Ok(String::from(path))
        } else {
            error.clone()
        };
    }
    error
}

pub fn get_project_dir() -> String {
    sudo::escalate_if_needed().expect("Error running sudo");
    String::from("/opt")
}

pub fn download_config(config_folder: String, file_name: String) -> Result<Success, Message> {
    let download_path = download(CONFIG_URL.to_string(), &file_name);

    if let Err(error) = download_path {
        return Err(error);
    }

    let folder_result = fs::create_dir_all(&config_folder);

    if let Err(error) = folder_result {
        return Err(Message {
            code: 0,
            message: "Error creating folder structure".to_string(),
            kind: MessageType::Error,
            task: "".to_string(),
            stack: vec![error.to_string()],
        });
    }

    let result = fs::copy(download_path.unwrap(), format!("{}/{}", &config_folder, file_name));

    if result.is_err() {
        return Err(Message {
            code: 0,
            message: "Error download configurations files".to_string(),
            kind: MessageType::Error,
            task: "".to_string(),
            stack: vec![result.err().unwrap().to_string()],
        });
    }
    Ok(Success {})
}

pub fn get_config_dir() -> Result<ProjectDirs, Message> {
    if let Some(proj_dirs) = ProjectDirs::from(
        QUALIFIER,
        ORGANIZATION,
        APPLICATION,
    ) {
        let config_dir = proj_dirs.config_dir();

        if !config_dir.exists() {
            let folder_result = fs::create_dir_all(config_dir);

            if let Err(error) = folder_result {
                return Err(Message {
                    code: 0,
                    message: "Error creating folder structure".to_string(),
                    kind: MessageType::Error,
                    task: "".to_string(),
                    stack: vec![error.to_string()],
                });
            }
        }

        Ok(proj_dirs)
    } else {
        Err(Message {
            code: 0,
            message: "Not found config directory".to_string(),
            kind: MessageType::Error,
            task: "".to_string(),
            stack: vec![],
        })
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub general: General,
    pub init: Init,
    pub workspace: Workspace,
    pub dependencies: Dependencies,
    pub config_file_item: Vec<ConfigFileItem>,
    pub build_cardano_node: BuildCardanoNode,
}

#[derive(Deserialize, Debug, Clone)]
pub struct General {
    pub version: String,
    pub last_cvm_version: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Init {
    pub git_assets: String,
    pub config_path: String,
    pub ghcup_path: String,
    pub install_ghc_file: String,
    pub libsodium_repository: String,
    pub libsodium_commit: String,
    pub haskell_ghc_version: String,
    pub haskell_cabal_version: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Workspace {
    pub workspace_folder: String,
    pub folders: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ConfigFileItem {
    pub url: String,
    pub name: String,
    pub folder: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Dependencies {
    pub macos: Vec<String>,
    pub debian: Vec<String>,
    pub ubuntu: Vec<String>,
    pub centos: Vec<String>,
    pub centos_7: Vec<String>,
    pub centos_8: Vec<String>,
    pub fedora: Vec<String>,
    pub rhel: Vec<String>,
    pub rhel_7: Vec<String>,
    pub rhel_8: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BuildCardanoNode {
    pub cnode_repository: String,
    pub cnode_release: String,
}