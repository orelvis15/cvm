#![allow(dead_code, unused_variables)]

use serde::Deserialize;
use crate::Context;
use crate::resolvers::folders::custom_folders::CustomFolders;
use crate::resolvers::folders::system_folders::SystemFolder;

pub enum RoutersResolve {
    Folder(FolderType),
    File(FileResolve),
    Url(UrlResolve),
}

pub enum FolderType {
    SYSTEM(FolderSystemResolve),
    CUSTOM(FolderCustomResolve),
}

pub struct FolderSystemResolve {
    pub key: SystemFolder,
}

pub struct FolderCustomResolve {
    pub key: CustomFolders,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct FolderCustom {
    pub key: String,
    pub name: String,
    pub parent: String,
}

pub struct FileResolve {
    pub key: String,
}

pub struct UrlResolve {
    pub key: String,
}

impl RoutersResolve {
    pub fn get_path_string(&self, context: Context) -> String {
        return match &self {
            RoutersResolve::Folder(folder_type) => {
                match folder_type {
                    FolderType::SYSTEM(folder_resolve) => {
                        SystemFolder::get_path_string(&folder_resolve.key)
                    }
                    FolderType::CUSTOM(folder_resolve) => {
                        CustomFolders::get_path_string(&folder_resolve.key, &context.remote_config)
                    }
                }
            }
            RoutersResolve::File(file) => {
                "".to_string()
            }
            RoutersResolve::Url(url) => {
                "".to_string()
            }
        }
    }
}