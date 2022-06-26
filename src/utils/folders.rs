#![allow(dead_code, unused_variables)]

use std::str::FromStr;
use crate::config::remote_config::{RemoteConfig, StructureFolderItem};
use crate::utils::folders::Folder::*;
use crate::url_build;

#[derive(Debug, Clone, PartialEq)]
pub enum Folder {
    ROOT,
    SCRIPTS,
    FILES,
    DB,
    GUILDDB,
    SOCKETS,
    PRIV,
    TMP,
    LOGS,
    BIN,
    GIT,
    CURRENT,
}

impl Folder {
    pub fn get_path(key: Folder, config: &RemoteConfig) -> String {
        let path = vec![];
        let mut path_result = Folder::find_folder_path(&key, config, path);
        path_result.reverse();
        let path_str: Vec<&String> = path_result.iter().map(|s| s as &String).collect();
        return url_build(path_str, false);
    }

    pub fn get_folder_root(config: &RemoteConfig) -> String {
        let root_folder = Folder::get_folder_item(&ROOT, &config);
        return url_build(vec![&Folder::project_folder().to_string(), &root_folder.name], false);
    }

    pub fn find_folder_path<'a>(item: &'a Folder, config: &'a RemoteConfig, mut path: Vec<String>) -> Vec<String> {
        let item_struct = Folder::get_folder_item(item, config);
        if item_struct.parent == "." {
            let root_path = Folder::get_folder_root(&config);
            path.push(root_path);
            return path;
        } else {
            let parent = Folder::from_str(item_struct.parent.as_str()).unwrap_or(ROOT);
            path.push(item_struct.name.to_string());
            return Folder::find_folder_path(&parent, &config, path);
        }
    }

    pub fn get_folder_item<'a>(item: &'a Folder, config: &'a RemoteConfig) -> &'a StructureFolderItem {
        for folder in &config.structure_folder_item {
            if Folder::from_str(folder.key.as_str()).unwrap().eq(&item) {
                return folder;
            }
        }
        &config.structure_folder_item.get(0).unwrap()
    }

    pub fn project_folder() -> &'static str {
        "/opt"
    }

    pub fn to_string(&self) -> String {
        match &self {
            SCRIPTS => {"SCRIPTS".to_string()}
            _ => {"".to_string()}
        }
    }
}

impl FromStr for Folder {
    type Err = ();

    fn from_str(input: &str) -> Result<Folder, Self::Err> {
        match input.to_uppercase().as_str() {
            "ROOT" => Ok(ROOT),
            "SCRIPTS" => Ok(SCRIPTS),
            "FILES" => Ok(FILES),
            "DB" => Ok(DB),
            "GUILDDB" => Ok(GUILDDB),
            "SOCKETS" => Ok(SOCKETS),
            "PRIV" => Ok(PRIV),
            "TMP" => Ok(TMP),
            "LOGS" => Ok(LOGS),
            "BIN" => Ok(BIN),
            "GIT" => Ok(GIT),
            "CURRENT" => Ok(CURRENT),
            _ => Ok(ROOT) // this case never execute
        }
    }
}