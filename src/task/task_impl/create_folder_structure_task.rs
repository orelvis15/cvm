#![allow(dead_code, unused_variables)]

use std::fs;
use std::path::Path;
use crate::config::config::{Config, get_project_dir};
use crate::env::Env;
use crate::task::cvm_error::{CvmError, Error};
use crate::task::folders::Folder;
use crate::task::task::{Success, Task};
use crate::task::task_type::TaskType;
use crate::url_build;

pub struct CreateFolderStructure {}

impl Task for CreateFolderStructure {
    fn run(self: &Self, _env: &mut Env, config: &Config) -> Result<Success, CvmError> {
        sudo::escalate_if_needed().expect("Super user permissions are required");

        let project_dir = get_project_dir();

        let workspace_home = url_build(vec![project_dir.as_str(), Folder::get(Folder::ROOT, &config)], false);
        fs::create_dir(&workspace_home)?;

        let folders = &config.structure_folder_item;

        for folder in folders {
            fs::create_dir(url_build(vec![&workspace_home.as_str(), folder.name.as_str()], false))?;
        }
        Ok(Success {})
    }

    fn check(self: &Self, _env: &mut Env, config: &Config) -> Result<Success, CvmError> {
        let error = CvmError::CreateFolderStructure(Error {
            message: "Error creating folder structure".to_string(),
            task: self.get_type(),
            stack: vec![],
        });

        let project_dir = get_project_dir();

        let workspace_home = url_build(vec![project_dir.as_str(), Folder::get(Folder::ROOT, &config)], false);
        if !Path::new(&workspace_home).is_dir() { return Err(error.clone()); }

        let folders = &config.structure_folder_item;

        for folder in folders {
            let dir = url_build(vec![&workspace_home.as_str(), folder.name.as_str()], false);
            if !Path::new(dir.as_str()).is_dir() { return Err(error.clone()); }
        }

        Ok(Success {})
    }

    fn get_type(self: &Self) -> TaskType {
        TaskType::CreateFolderStructure
    }
}