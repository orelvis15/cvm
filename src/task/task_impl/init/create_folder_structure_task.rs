#![allow(dead_code, unused_variables)]

use std::str::FromStr;
use crate::context::context::Context;
use crate::config::remote_config::RemoteConfig;
use crate::message::message::Message;
use crate::resolvers::folders::custom_folders::CustomFolders;
use crate::task::task::{Success, Task};
use crate::task::task_type::TaskType;
use crate::url_build;
use crate::config::state_config::{get_task_complete, set_task_complete};
use crate::resolvers::folders::system_folders::SystemFolder;
use crate::task::task_impl::commons::folder_manager::folder_manager_io_data::FolderManagerAction;
use crate::task::task_impl::commons::folder_manager::folder_manager_task::FolderManagerTask;
use crate::task::task_impl::task_input_data::TaskInputData;
use crate::task_manager::task_manager::TaskManager;
use crate::term::log_level::LogLevel::L2;

#[derive(Default)]
pub struct CreateFolderStructure {}

impl Task for CreateFolderStructure {
    fn prepare(self: &mut Self, context: &mut Context, config: &RemoteConfig) -> Result<bool, Message> {
        if get_task_complete(&self.get_type()) {
            return Ok(false);
        };
        Ok(true)
    }

    fn run(self: &Self, context: &mut Context, config: &RemoteConfig) -> Result<Success, Message> {
        let mut folders = vec![];

        folders.push((SystemFolder::get_path_string(&SystemFolder::UnixOpt), CustomFolders::get_folder_item(&CustomFolders::ROOT, config).name.to_string()));

        for folder in &config.folder_custom {
            folders.push((CustomFolders::get_path_string(&CustomFolders::from_str(folder.parent.as_str()).unwrap(), config), folder.name.to_string()));
        }

        TaskManager::default().start(vec![
            Box::new(FolderManagerTask { input_data: TaskInputData::FolderManager(FolderManagerAction::Create(folders)), ..Default::default() }),
        ], config, L2, context)
    }

    fn check(self: &Self, context: &mut Context, config: &RemoteConfig) -> Result<Success, Message> {
        let mut folders = vec![];

        for folder in &config.folder_custom {
            folders.push(url_build(vec![&CustomFolders::get_path_string(&CustomFolders::from_str(folder.parent.as_str()).unwrap(), config), &folder.name.to_string()], false));
        }

        let result = TaskManager::default().start(vec![
            Box::new(FolderManagerTask { input_data: TaskInputData::FolderManager(FolderManagerAction::Exits(folders)), ..Default::default() }),
        ], config, L2, context);

        set_task_complete(&self.get_type());

        result
    }

    fn get_type(self: &Self) -> TaskType {
        TaskType::CreateFolderStructure
    }

    fn get_id(self: &Self) -> String {
        "".to_string()
    }
}