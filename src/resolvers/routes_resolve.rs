#![allow(dead_code, unused_variables)]

use serde::Deserialize;
use crate::Context;
use crate::resolvers::folders::custom_folders::CustomFolders;
use crate::resolvers::folders::system_folders::SystemFolder;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RoutesResolve {
    Folder(FolderType),
    FolderVec(Vec<FolderType>),
    File(FileResolve),
    Url(UrlResolve),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FolderType {
    SYSTEM(FolderSystemResolve),
    CUSTOM(FolderCustomResolve),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FolderSystemResolve {
    pub key: SystemFolder,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FolderCustomResolve {
    pub key: CustomFolders,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct FolderCustom {
    pub key: String,
    pub name: String,
    pub parent: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileResolve {
    pub key: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UrlResolve {
    pub key: String,
}

impl RoutesResolve {
    pub fn resolve(&self, context: Context) -> Vec<String> {
        return match &self {
            RoutesResolve::Folder(folder_type) => {
                vec![Self::simple_folder_resolve(&context, folder_type)]
            }
            RoutesResolve::FolderVec(folder_type_vec) => {
                let mut result = vec![];
                for folder in folder_type_vec {
                    result.push(Self::simple_folder_resolve(&context, folder))
                }
                result
            }
            RoutesResolve::File(file) => {
                vec![]
            }
            RoutesResolve::Url(url) => {
                vec![]
            }
        };
    }

    fn simple_folder_resolve(context: &Context, folder_type: &FolderType) -> String {
        match folder_type {
            FolderType::SYSTEM(folder_resolve) => {
                SystemFolder::get_path_string(&folder_resolve.key)
            }
            FolderType::CUSTOM(folder_resolve) => {
                CustomFolders::get_path_string(&folder_resolve.key, &context.remote_config)
            }
        }
    }
}