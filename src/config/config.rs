
use serde::{Deserialize};
use std::{fs};
use std::path::Path;
use directories::{BaseDirs, ProjectDirs};

use crate::task::task::{Error, Success};
use crate::utils::download_manager::download;

const CONFIG_URL: &str = "https://raw.githubusercontent.com/orelvis15/cvm_config/master/config.toml";
const QUALIFIER: &str = "com";
const ORGANIZATION: &str = "orelvis15";
const APPLICATION: &str = "cvm";
const FILE_NAME: &str = "config.tom";

pub fn get_config() -> Result<Config, Error> {

        let project_dir = get_project_dir();
        if let Err(error) = project_dir{
            return Result::Err(error)
        }

        let config_dir = project_dir.unwrap().config_dir().to_str().unwrap().to_string();

        // obtenemos el path del directorio como string y lo concatenamos con  el archivo
        let mut path_str: String = config_dir.clone();
        path_str.push_str("/");
        path_str.push_str(FILE_NAME);

        // en caso de no existir los descargamos
        let download_config = download_config(config_dir.clone()
                                              , format!("/{}", FILE_NAME.to_string()));
        if let Err(error) = download_config {
            return Result::Err(error);
        }

        // leemos el archivo de configuracion
        let config_file = fs::read_to_string(
            format!("{}/{}", config_dir, FILE_NAME)
        );

        // si fue correcta la lectura se devuelve el objeto Config parseado del archivo
        match config_file {
            Ok(file) => {

                match toml::from_str(&file) {
                    Ok(config) => {return Result::Ok(config);},
                    Err(error) => {
                        return Result::Err(Error {
                            code: 0,
                            message: "Error try parsing config file".to_string(),
                            task: "".to_string(),
                            stack: vec![error.to_string()],
                        });
                    }
                }
            }
            Err(_) => return Result::Err(Error {
                code: 0,
                message: "Error try reading config file".to_string(),
                task: "".to_string(),
                stack: vec![],
            }),
        };
}

pub fn get_home_dir() -> Result<String, Error> {
    let error = Result::Err(Error {
        code: 0,
        message: "An error occurred reading the home directory".to_string(),
        task: "".to_string(),
        stack: vec![],
    });

    if let Some(dir) = BaseDirs::new() {
        if let Some(path) = dir.home_dir().to_str() {
            return Result::Ok(String::from(path));
        } else {
            return error.clone();
        }
    }
    error
}

pub fn download_config(config_folder: String, file_name: String) -> Result<Success, Error> {
    let download_path = download(CONFIG_URL.to_string(), &file_name);

    if let Err(error) = download_path {
        return Result::Err(error);
    }

    fs::create_dir_all(&config_folder);
    let result = fs::copy(download_path.unwrap(), format!("{}/{}", &config_folder, file_name));

    if result.is_err() {
        return Result::Err(Error {
            code: 0,
            message: "Error download configurations files".to_string(),
            task: "".to_string(),
            stack: vec![],
        });
    }
    Result::Ok(Success {})
}

pub fn get_project_dir() -> Result<ProjectDirs, Error> {

    if let Some(proj_dirs) = ProjectDirs::from(
        QUALIFIER,
        ORGANIZATION,
        APPLICATION,
    ) {
        let config_dir = proj_dirs.config_dir();

        if !config_dir.exists() {
            fs::create_dir_all(config_dir);
        }

        Result::Ok(proj_dirs)
    } else {
        Result::Err(Error {
            code: 0,
            message: "Not found config directory".to_string(),
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
    pub build_cardano_node: BuildCardanoNode
}

#[derive(Deserialize, Debug, Clone)]
pub struct General {
    pub version: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Init {
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
    pub cnode_release: String
}