#![allow(dead_code, unused_variables)]

use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use crate::{Message, Success, url_build};
use crate::config::remote_config::get_home_dir;

const FILE_NAME: &str = "state.tom";
const PROJECT_FOLDER: &str = ".cvm";

pub fn get_state() -> Result<State, Message> {
    let home_dir = get_home_dir()?;
    let file_path = url_build(vec![&home_dir, &PROJECT_FOLDER.to_string(), &FILE_NAME.to_string()], false);

    if !Path::new(file_path.as_str()).exists() {
        create_state_file()?;
    };

    let file = fs::read_to_string(file_path)?;
    let parse_file = toml::from_str(&file)?;
    Ok(parse_file)
}

pub fn set_state(state: State) -> Result<Success, Message> {
    let home_dir = get_home_dir()?;
    let file_path = url_build(vec![&home_dir, &PROJECT_FOLDER.to_string(), &FILE_NAME.to_string()], false);
    let mut file = File::options().truncate(true).write(true).open(file_path)?;
    let toml_str = toml::to_string(&state).unwrap();
    file.write_all(toml_str.as_bytes())?;
    Ok(Success {})
}

pub fn add_init_file(path: String) -> Result<Success, Message> {
    let mut state = get_state()?;

    let file_path = Path::new(&path);
    let hash = sha256::digest_file(file_path)?;
    state.init.files_item.push(ConfigFiles{ name: file_path.file_name().unwrap().to_str().unwrap().to_string(), hash });

    set_state(state)?;
    Ok(Success{})
}

pub fn set_init_success(value: bool) -> Result<Success, Message> {
    let mut state = get_state()?;
    state.init.success = value;
    set_state(state)?;
    Ok(Success{})
}

pub fn set_version_use(version: String) -> Result<Success, Message> {
    let mut state = get_state()?;
    state.r#use = Use{ version };
    set_state(state)?;
    Ok(Success{})
}

fn create_state_file() -> Result<Success, Message> {
    let home_dir = get_home_dir()?;
    let file_path = url_build(vec![&home_dir, &PROJECT_FOLDER.to_string(), &FILE_NAME.to_string()], false);
    let state = State { init: Init { success: false, files_item: vec![] }, r#use: Use { version: "".to_string() } };
    let toml_str = toml::to_string(&state).unwrap();
    let mut file = File::create(Path::new(file_path.as_str()))?;
    file.write_all(toml_str.as_bytes())?;
    Ok(Success {})
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct State {
    pub init: Init,
    pub r#use: Use,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Use {
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Init {
    pub success: bool,
    pub files_item: Vec<ConfigFiles>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigFiles {
    pub name: String,
    pub hash: String,
}