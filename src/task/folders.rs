#![allow(dead_code, unused_variables)]

use std::collections::HashMap;
use std::str::FromStr;
use crate::config::config::{Config, StructureFolderItem};
use crate::task::folders::Folder::*;
use strfmt::strfmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Folder {
    ROOT,
    SCRIPT,
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
    pub fn get_path(key: Folder, config: &Config) -> String {
        let path = vec![];
        let mut path_result = Folder::find_folder_path(&key, config, path);
        path_result.reverse();
        return Folder::build_url(path_result, false);
    }

    pub fn get_folder_root(config: &Config) -> String {
        let root_folder = Folder::get_folder_item(&ROOT, &config);
        return Folder::build_url(vec![Folder::project_folder().to_string(), root_folder.name.clone()], false);
    }

    pub fn find_folder_path<'a>(item: &'a Folder, config: &'a Config, mut path: Vec<String>) -> Vec<String> {
        let item_struct = Folder::get_folder_item(item, config);
        if item_struct.path == "." {
            let root_path = Folder::get_folder_root(&config);
            path.push(root_path);
            return path;
        } else {
            let parent = Folder::from_str(item_struct.path.as_str()).unwrap_or(ROOT);
            path.push(item_struct.name.to_string());
            return Folder::find_folder_path(&parent, &config, path);
        }
    }

    pub fn get_folder_item<'a>(item: &'a Folder, config: &'a Config) -> &'a StructureFolderItem {
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

    pub fn build_url(args: Vec<String>, last_slash: bool) -> String {
        let mut patter = String::new();
        let mut args_map: HashMap<String, String> = HashMap::new();

        for (i, arg) in args.iter().enumerate() {
            let _ = &args_map.insert(arg.clone().to_string(), arg.clone().to_string());

            let _ = &patter.push_str("{");
            let _ = &patter.push_str(&arg);
            let _ = &patter.push_str("}");

            if i != args.len() - 1 || last_slash {
                let _ = &patter.push_str("/");
            }
        }
        strfmt(&patter.as_str(), &args_map).unwrap()
    }
}

impl FromStr for Folder {
    type Err = ();

    fn from_str(input: &str) -> Result<Folder, Self::Err> {
        match input.to_uppercase().as_str() {
            "ROOT" => Ok(ROOT),
            "SCRIPTS" => Ok(SCRIPT),
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