#![allow(dead_code, unused_variables)]

use std::str::FromStr;
use crate::config::config::Config;
use crate::env::Env;
use crate::error::message::Message;
use crate::utils::folders::Folder;
use crate::task::task::{Success, Task};
use crate::task::task_type::TaskType;
use crate::Term;
use crate::task::task_impl::commons::folder_manager_task::{FolderManagerAction, FolderManagerTask};
use crate::task_manager::task_manager::TaskManager;
use crate::term::log_level::LogLevel::L2;

pub struct CreateFolderStructure {}

impl Task for CreateFolderStructure {
    fn run(self: &Self, _env: &mut Env, config: &Config, term: &mut Term) -> Result<Success, Message> {

        let mut folders = vec![];

        folders.push((Folder::project_folder().to_string(), Folder::get_folder_item(&Folder::ROOT, config).name.to_string()));

        for folder in &config.structure_folder_item {
            folders.push((Folder::get_path(Folder::from_str(folder.parent.as_str()).unwrap(), config), folder.name.to_string()));
        }

        TaskManager{}.start(vec![
            Box::new(FolderManagerTask { input_data: FolderManagerAction::Create(folders) }),
        ], config, term, L2)
    }

    fn check(self: &Self, _env: &mut Env, config: &Config, term: &mut Term) -> Result<Success, Message> {
        Ok(Success {})
    }

    fn get_type(self: &Self) -> TaskType {
        TaskType::CreateFolderStructure
    }
}