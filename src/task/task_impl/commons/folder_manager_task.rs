#![allow(dead_code, unused_variables)]

use std::fs;
use std::path::Path;
use crate::env::Env;
use crate::{MessageData, Success, Term, url_build};
use crate::config::remote_config::RemoteConfig;
use crate::message::message::Message;
use crate::task::task::Task;
use crate::task::task_type::TaskType;

pub struct FolderManagerTask {
    pub input_data: FolderManagerAction,
}

impl Task for FolderManagerTask {

    fn prepare(self: &mut Self, env: &mut Env, config: &RemoteConfig, term: &mut Term) -> Result<bool, Message> {
        Ok(true)
    }

    fn run(self: &Self, _env: &mut Env, config: &RemoteConfig, term: &mut Term) -> Result<Success, Message> {
        match &self.input_data {
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
        }
    }

    fn check(self: &Self, _env: &mut Env, config: &RemoteConfig, term: &mut Term) -> Result<Success, Message> {
        match &self.input_data {
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

    Ok(Success {})
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

    Ok(Success {})
}

fn remove(task: &FolderManagerTask, data: &Vec<String>) -> Result<Success, Message> {
    for folder_url in data {
        let folder_path = Path::new(folder_url);
        if !folder_path.exists() {
            continue;
        };

        fs::remove_dir_all(folder_path)?;
    }

    Ok(Success {})
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

    Ok(Success {})
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

    Ok(Success {})
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

    Ok(Success {})
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

    Ok(Success {})
}

fn check_exits(task: &FolderManagerTask, data: &Vec<String>) -> Result<Success, Message> {
    Ok(Success {})
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FolderManagerAction {
    Create(Vec<(String, String)>),
    //Path for create, Folder name
    Remove(Vec<String>),
    // Folder path
    Clean(Vec<String>),
    // Folder path
    Exits(Vec<String>), // Folder path
}