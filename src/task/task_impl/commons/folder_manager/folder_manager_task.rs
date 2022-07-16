#![allow(dead_code, unused_variables)]

use std::fs;
use std::path::Path;
use crate::context::context::Context;
use crate::{MessageData, Success, url_build};
use crate::config::remote_config::RemoteConfig;
use crate::context::storage::TaskOutputData;
use crate::message::message::Message;
use crate::task::task::{id_generator, Task};
use crate::task::task_impl::commons::folder_manager::folder_manager_io_data::{FolderManagerAction, FolderManagerOutputData, ResolveFolderManagerInputData};
use crate::task::task_impl::task_input_data::TaskInputData;
use crate::task::task_type::TaskType;

#[derive(Default)]
pub struct FolderManagerTask {
    pub input_data: TaskInputData,
    pub data: ResolveFolderManagerInputData
}

impl Task for FolderManagerTask {
    fn prepare(self: &mut Self, context: &mut Context, config: &RemoteConfig) -> Result<bool, Message> {
        let mut input_data = ResolveFolderManagerInputData::default();
        match &self.input_data {
            TaskInputData::FolderManager(action) => {
                input_data.action = action.to_owned();
            }
            _ => {}
        }
        self.data = input_data;
        Ok(true)
    }

    fn run(self: &Self, context: &mut Context, config: &RemoteConfig) -> Result<Success, Message> {
        match &self.data.action {
            FolderManagerAction::Create(data) => {
                create(self, data)
            }
            FolderManagerAction::Remove(data) => {
                remove(self, data)
            }
            FolderManagerAction::Clean(data) => {
                clean(self, data)
            }
            FolderManagerAction::Exits(data) => {
                exits(self, data)
            }
        }?;
        Ok(Success { value: TaskOutputData::FolderManager(FolderManagerOutputData { operation: self.data.action.to_owned() }) })
    }

    fn check(self: &Self, context: &mut Context, config: &RemoteConfig) -> Result<Success, Message> {
        match &self.data.action {
            FolderManagerAction::Create(data) => {
                check_create(self, data)
            }
            FolderManagerAction::Remove(data) => {
                check_remove(self, data)
            }
            FolderManagerAction::Clean(data) => {
                check_clean(self, data)
            }
            FolderManagerAction::Exits(data) => {
                check_exits(self, data)
            }
        }
    }

    fn get_type(self: &Self) -> TaskType {
        TaskType::FolderManager("".to_string())
    }

    fn get_id(self: &Self) -> String {
        match &self.data.action {
            FolderManagerAction::Create(data) => {
                let result: Vec<String> = data.iter()
                    .map(|(value_1, value_2)| format!("{}{}", value_1, value_2))
                    .collect();
                id_generator(&result)
            }
            FolderManagerAction::Remove(data) => { id_generator(data) }
            FolderManagerAction::Clean(data) => { id_generator(data) }
            FolderManagerAction::Exits(data) => { id_generator(data) }
        }
    }
}

fn create(task: &FolderManagerTask, data: &Vec<(String, String)>) -> Result<Success, Message> {
    for (parent_url, folder_name) in data {
        let parent_path = Path::new(parent_url);
        let folder_url = url_build(vec![parent_url, folder_name], false);
        let folder_path = Path::new(&folder_url);

        if folder_path.exists() {
            continue;
        }

        if !parent_path.exists() || folder_name.is_empty() {
            return Err(Message::CreateFolder(MessageData {
                message: format!("Trying create folder {}", folder_path.display()),
                ..Default::default()
            }));
        };

        fs::create_dir(folder_path)?;
    }

    Ok(Success::default())
}

fn check_create(task: &FolderManagerTask, data: &Vec<(String, String)>) -> Result<Success, Message> {
    for (parent_url, folder_name) in data {
        let parent_path = Path::new(parent_url);
        let folder_url = url_build(vec![parent_url, folder_name], false);
        let folder_path = Path::new(&folder_url);

        if !folder_path.exists() {
            return Err(Message::CreateFolder(MessageData {
                message: format!("Trying create folder {}", folder_path.display()),
                ..Default::default()
            }));
        };
    }

    Ok(Success::default())
}

fn remove(task: &FolderManagerTask, data: &Vec<String>) -> Result<Success, Message> {
    for folder_url in data {
        let folder_path = Path::new(folder_url);
        if !folder_path.exists() {
            continue;
        };

        fs::remove_dir_all(folder_path)?;
    }

    Ok(Success::default())
}

fn check_remove(task: &FolderManagerTask, data: &Vec<String>) -> Result<Success, Message> {
    for folder_url in data {
        let folder_path = Path::new(folder_url);
        if folder_path.exists() {
            return Err(Message::RemoveFolder(MessageData {
                message: format!("Folder {} could not be removed", folder_path.display()),
                ..Default::default()
            }));
        };
    }

    Ok(Success::default())
}

fn clean(task: &FolderManagerTask, data: &Vec<String>) -> Result<Success, Message> {
    for folder_url in data {
        let folder_path = Path::new(folder_url);
        if !folder_path.exists() {
            return Err(Message::FolderNotFound(MessageData {
                message: format!("Folder {} not exist", folder_path.display()),
                ..Default::default()
            }));
        };

        for entry in fs::read_dir(folder_path)? {
            if entry.as_ref().unwrap().path().is_dir() {
                fs::remove_dir_all(entry.as_ref().unwrap().path())?
            } else {
                fs::remove_file(entry.as_ref().unwrap().path())?;
            }
        };
    }

    Ok(Success::default())
}

fn check_clean(task: &FolderManagerTask, data: &Vec<String>) -> Result<Success, Message> {
    for folder_url in data {
        let folder_path = Path::new(folder_url);

        if !folder_path.exists() {
            return Err(Message::FolderNotFound(MessageData {
                message: format!("Folder {} not exist", folder_path.display()),
                ..Default::default()
            }));
        };

        if fs::read_dir(folder_path)?.count() == 0 {
            continue;
        } else {
            return Err(Message::FolderNotFound(MessageData {
                message: format!("Folder {} is not empty", folder_path.display()),
                ..Default::default()
            }));
        }
    }

    Ok(Success::default())
}

fn exits(task: &FolderManagerTask, data: &Vec<String>) -> Result<Success, Message> {
    for folder_url in data {
        let folder_path = Path::new(folder_url);
        if !folder_path.exists() {
            return Err(Message::FolderNotFound(MessageData {
                message: format!("Error folder {} not found", folder_path.display()),
                ..Default::default()
            }));
        };
    }

    Ok(Success::default())
}

fn check_exits(task: &FolderManagerTask, data: &Vec<String>) -> Result<Success, Message> {
    Ok(Success::default())
}