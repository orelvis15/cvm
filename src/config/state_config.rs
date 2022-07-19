#![allow(dead_code, unused_variables)]

use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use crate::{Message, Success, url_build};
use crate::task::task_type::TaskType;
use crate::resolvers::folders::custom_folders::CustomFolders;

const FILE_NAME: &str = "state.tom";
const PROJECT_FOLDER: &str = ".cvm";

pub fn get_state() -> Result<State, Message> {
    let home_dir = CustomFolders::get_home_dir()?;
    let file_path = url_build(vec![&home_dir, &PROJECT_FOLDER.to_string(), &FILE_NAME.to_string()], false);

    if !Path::new(file_path.as_str()).exists() {
        create_state_file()?;
    };

    let file = fs::read_to_string(file_path)?;
    let parse_file = toml::from_str(&file)?;
    Ok(parse_file)
}

pub fn reset_init() -> Result<Success, Message>{
    let mut state = get_state()?;
    state.init = Init::default();
    set_state(state)
}

pub fn add_init_file(file_uri: &String) -> Result<Success, Message> {
    let mut state = get_state()?;

    let file_path = Path::new(&file_uri);
    let file_name = file_path.file_name().unwrap().to_str().unwrap().to_string();

    let hash = sha256::digest_file(file_path)?;
    state.init.files_item.push(ConfigFiles { name: file_name.clone(), hash });

    set_state(state)
}

pub fn update_init_files(file_uri: &String) -> Result<Success, Message> {
    let mut state = get_state()?;
    let file_path = Path::new(&file_uri);
    let file_name = file_path.file_name().unwrap().to_str().unwrap().to_string();

    state.init.files_item.iter()
        .position(|item| item.name == file_name)
        .map(|index| state.init.files_item.remove(index));

    let hash = sha256::digest_file(file_path)?;
    state.init.files_item.push(ConfigFiles { name: file_name.clone(), hash });

    set_state(state)
}

pub fn set_init_success(value: bool) -> Result<Success, Message> {
    let mut state = get_state()?;
    state.init.success = value;
    set_state(state)
}

pub fn set_init_network(value: String) -> Result<Success, Message> {
    let mut state = get_state()?;
    state.init.network = value;
    set_state(state)
}

pub fn set_version_use(version: String) -> Result<Success, Message> {
    let mut state = get_state()?;
    state.r#use = Use { version };
    set_state(state)
}

fn set_state(state: State) -> Result<Success, Message> {
    let home_dir = CustomFolders::get_home_dir()?;
    let file_path = url_build(vec![&home_dir, &PROJECT_FOLDER.to_string(), &FILE_NAME.to_string()], false);
    let mut file = File::options().truncate(true).write(true).open(file_path)?;
    let toml_str = toml::to_string(&state).unwrap();
    file.write_all(toml_str.as_bytes())?;
    Ok(Success::default())
}

pub fn set_task_complete(task: &TaskType) {
    if let Ok(mut state) = get_state() {
        match task {
            TaskType::InstallDependences => { state.init.install_dependences_task = true }
            TaskType::InstallGhcup => { state.init.install_hanskell_ghc_task = true }
            TaskType::CreateFolderStructure => { state.init.create_folder_structure = true }
            TaskType::DownloadConfigFiles => { state.init.download_config_files_task = true }
            TaskType::Libsecp256k1 => { state.init.install_libsecp256k1_task = true }
            _ => {}
        }
        let _ = set_state(state);
    }
}

pub fn get_task_complete(task: &TaskType) -> bool {
    if let Ok(state) = get_state() {
        return match task {
            TaskType::InstallDependences => { state.init.install_dependences_task }
            TaskType::InstallGhcup => { state.init.install_hanskell_ghc_task }
            TaskType::CreateFolderStructure => { state.init.create_folder_structure }
            TaskType::DownloadConfigFiles => { state.init.download_config_files_task }
            TaskType::Libsecp256k1 => { state.init.install_libsecp256k1_task }
            _ => { false }
        };
    }
    false
}

fn create_state_file() -> Result<Success, Message> {
    let home_dir = CustomFolders::get_home_dir()?;
    let file_path = url_build(vec![&home_dir, &PROJECT_FOLDER.to_string(), &FILE_NAME.to_string()], false);
    let state = State::default();
    let toml_str = toml::to_string(&state).unwrap();
    let mut file = File::create(Path::new(file_path.as_str()))?;
    file.write_all(toml_str.as_bytes())?;
    Ok(Success::default())
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct State {
    pub init: Init,
    pub r#use: Use,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Use {
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Init {
    pub network: String,
    pub success: bool,
    pub install_dependences_task: bool,
    pub install_hanskell_ghc_task: bool,
    pub create_folder_structure: bool,
    pub install_libsecp256k1_task: bool,
    pub download_config_files_task: bool,
    pub files_item: Vec<ConfigFiles>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ConfigFiles {
    pub name: String,
    pub hash: String,
}