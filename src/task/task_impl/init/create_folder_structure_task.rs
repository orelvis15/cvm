#![allow(dead_code, unused_variables)]

use std::fs;
use std::path::Path;
use crate::config::config::Config;
use crate::env::Env;
use crate::error::error::{Message, Error};
use crate::utils::folders::Folder;
use crate::task::task::{Success, Task};
use crate::task::task_type::TaskType;
use crate::{Term, url_build};

pub struct CreateFolderStructure {}

impl Task for CreateFolderStructure {
    fn run(self: &Self, _env: &mut Env, config: &Config, term: &mut Term) -> Result<Success, Message> {

        let workspace_home = Folder::get_path(Folder::ROOT, &config);

        if !Path::new(&workspace_home).exists() {
            fs::create_dir(&workspace_home)?;
        }

        let folders = &config.structure_folder_item;

        for folder in folders {
            let path = url_build(vec![&workspace_home, &folder.name], false);
            if !Path::new(&path).exists() {
                fs::create_dir(path)?;
            }
        }
        Ok(Success {})
    }

    fn check(self: &Self, _env: &mut Env, config: &Config, term: &mut Term) -> Result<Success, Message> {
        let error = Message::CreateFolderStructure(Error {
            message: "Error creating folder structure".to_string(),
            task: self.get_type(),
            stack: vec![],
        });

        let workspace_home = Folder::get_path(Folder::ROOT, &config);
        if !Path::new(&workspace_home).is_dir() { return Err(error.clone()); }

        let folders = &config.structure_folder_item;

        for folder in folders {
            let dir = url_build(vec![&workspace_home, &folder.name], false);
            if !Path::new(dir.as_str()).is_dir() { return Err(error.clone()); }
        }

        Ok(Success {})
    }

    fn get_type(self: &Self) -> TaskType {
        TaskType::CreateFolderStructure
    }
}