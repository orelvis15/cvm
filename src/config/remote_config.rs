use serde::{Deserialize};
use std::fs;
use crate::{Message, url_build};
use crate::utils::download_manager::download_in_path;
use crate::utils::folders::Folder;

const CONFIG_URL: &str = "https://raw.githubusercontent.com/orelvis15/cvm/master/config/config_remote.toml";
const FILE_NAME: &str = "config_remote.tom";
const PROJECT_FOLDER: &str = ".cvm";

pub fn get_remote_config() -> Result<RemoteConfig, Message> {
    let home_dir = Folder::get_home_dir()?;
    let project_folder = url_build(vec![&home_dir, &PROJECT_FOLDER.to_string()], false);
    let file_path = download_in_path(&CONFIG_URL.to_string(), project_folder, FILE_NAME.to_string())?;
    let file = fs::read_to_string(format!("{}/{}", file_path, FILE_NAME))?;
    let parse_file = toml::from_str(&file)?;
    Ok(parse_file)
}

#[derive(Deserialize, Debug, Clone)]
pub struct RemoteConfig {
    pub general: General,
    pub update: Update,
    pub init: Init,
    pub dependencies: Dependencies,
    pub config_file_item: Vec<ConfigFileItem>,
    pub build_cardano_node: BuildCardanoNode,
    pub structure_folder_item: Vec<StructureFolderItem>,
    pub binaries: Binaries,
    pub services_item: Vec<Services>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Binaries {
    pub required_files: Vec<String>,
    pub others_files: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct General {
    pub version: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Init {
    pub git_assets: String,
    pub ghcup_url: String,
    pub install_ghc_file: String,
    pub ghcup_bin_path: String,
    pub ghcup_pattern_sed: String,
    pub libsodium_repository: String,
    pub libsodium_commit: String,
    pub libsodium_folder: String,
    pub libsodium_autogen_file: String,
    pub libsodium_config_file: String,
    pub libsecp256k1_repository: String,
    pub libsecp256k1_commit: String,
    pub libsecp256k1_autogen_file: String,
    pub libsecp256k1_configure_file: String,
    pub libsecp256k1_folder: String,
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
    pub cnode_ported_libsodium_file_name: String,
    pub cnode_ported_libsodium_data: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StructureFolderItem {
    pub key: String,
    pub name: String,
    pub parent: String,
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