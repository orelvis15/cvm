use serde::{Deserialize};
use std::{env, fs};
use users::{get_current_uid, get_user_by_uid};
use crate::{CvmError, url_build};
use crate::utils::download_manager::download_in_path;

const CONFIG_URL: &str = "https://raw.githubusercontent.com/orelvis15/cvm_config/master/config.toml";
const FILE_NAME: &str = "config.tom";
const PROJECT_FOLDER: &str = ".cvm";

pub fn get_config() -> Result<Config, CvmError> {
    let home_dir = get_home_dir()?;
    let project_folder = url_build(vec![home_dir.as_str(), PROJECT_FOLDER], false);
    let file_path = download_in_path(&CONFIG_URL.to_string(), project_folder, FILE_NAME)?;
    let file = fs::read_to_string(format!("{}/{}", file_path, FILE_NAME))?;
    let parse_file = toml::from_str(&file)?;
    Ok(parse_file)
}

//TODO refactor
pub fn get_home_dir() -> Result<String, CvmError> {
    let user = get_user_by_uid(get_current_uid()).unwrap();
    if user.uid() != 0 {
        return Ok(String::from(format!("/home/{}", user.name().to_str().unwrap())));
    }

    //if user is root return SUDO_USER var
    if let Some(sudo_user) = env::var_os("SUDO_USER") {
        return Ok(String::from(format!("/home/{}", sudo_user.to_str().unwrap())));
    }
    Ok("".to_string())
}

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub general: General,
    pub update: Update,
    pub init: Init,
    pub dependencies: Dependencies,
    pub config_file_item: Vec<ConfigFileItem>,
    pub build_cardano_node: BuildCardanoNode,
    pub structure_folder_item: Vec<StructureFolderItem>,
    pub binaries: Binaries,
    pub commands_item: Vec<CommandItem>,
    pub services_item: Vec<Services>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Binaries {
    pub files: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct General {
    pub version: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Init {
    pub git_assets: String,
    pub config_path: String,
    pub ghcup_url: String,
    pub install_ghc_file: String,
    pub ghcup_bin_path: String,
    pub ghcup_pattern_sed: String,
    pub libsodium_repository: String,
    pub libsodium_commit: String,
    pub libsodium_folder: String,
    pub libsodium_autogen_file: String,
    pub libsodium_config_file: String,
    pub haskell_ghc_version: String,
    pub haskell_cabal_version: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ConfigFileItem {
    pub url: String,
    pub name: String,
    pub folder_key: String,
    pub pattern_sed: String,
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
    pub cnode_repository_name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StructureFolderItem {
    pub key: String,
    pub name: String,
    pub path: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CommandItem {
    pub key: String,
    pub name: String,
    pub enable: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Update {
    pub last_cvm_version: String,
    pub version_pattern: String,
    pub name_pattern: String,
    pub file_name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Services {
    pub url: String,
    pub file: String,
    pub name: String,
}