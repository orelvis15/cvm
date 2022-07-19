#![allow(dead_code, unused_variables)]

use std::str::FromStr;
use crate::config::remote_config::RemoteConfig;
use crate::resolvers::folders::custom_folders::CustomFolders::*;
use crate::{Message, url_build};
use crate::resolvers::folders::system_folders::SystemFolder;
use crate::resolvers::routes_resolve::FolderCustom;
use crate::utils::user::get_current_user;

#[derive(Debug, Clone, PartialEq)]
pub enum CustomFolders {
    ROOT,
    Scripts,
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

impl CustomFolders {
    pub fn get_path_string(key: &CustomFolders, config: &RemoteConfig) -> String {
        let path = vec![];
        let mut path_result = CustomFolders::find_folder_path(&key, config, path);
        path_result.reverse();
        let path_str: Vec<&String> = path_result.iter().map(|s| s as &String).collect();
        return url_build(path_str, false);
    }

    fn get_folder_root(config: &RemoteConfig) -> String {
        let root_folder = CustomFolders::get_folder_item(&ROOT, &config);
        return url_build(vec![&SystemFolder::get_path_string(&SystemFolder::UnixOpt), &root_folder.name], false);
    }

    fn find_folder_path<'a>(item: &'a CustomFolders, config: &'a RemoteConfig, mut path: Vec<String>) -> Vec<String> {
        let item_struct = CustomFolders::get_folder_item(item, config);
        return if item_struct.parent == "." {
            let root_path = CustomFolders::get_folder_root(&config);
            path.push(root_path);
            path
        } else {
            let parent = CustomFolders::from_str(item_struct.parent.as_str()).unwrap_or(ROOT);
            path.push(item_struct.name.to_string());
            CustomFolders::find_folder_path(&parent, &config, path)
        }
    }

    pub fn get_folder_item<'a>(item: &'a CustomFolders, config: &'a RemoteConfig) -> &'a FolderCustom {
        for folder in &config.folder_custom {
            if CustomFolders::from_str(folder.key.as_str()).unwrap().eq(&item) {
                return folder;
            }
        }
        &config.folder_custom.get(0).unwrap()
    }

    pub fn get_home_dir() -> Result<String, Message> {
        let user = get_current_user()?;
        Ok(String::from(format!("/home/{}", user)))
    }

    pub fn to_string(&self) -> String {
        match &self {
            Scripts => { "SCRIPTS".to_string() }
            _ => { "".to_string() }
        }
    }
}

impl FromStr for CustomFolders {
    type Err = ();

    fn from_str(input: &str) -> Result<CustomFolders, Self::Err> {
        match input.to_uppercase().as_str() {
            "ROOT" => Ok(ROOT),
            "SCRIPTS" => Ok(Scripts),
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