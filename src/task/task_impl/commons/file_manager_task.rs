#![allow(dead_code, unused_variables)]

use std::fs;
use std::io::Write;
use std::path::Path;
use crate::context::context::Context;
use crate::{MessageData, Success};
use crate::config::remote_config::RemoteConfig;
use crate::context::storage::TaskOutputData;
use crate::message::message::Message;
use crate::task::task::{id_generator, Task};
use crate::task::task_type::TaskType;

pub struct FileManagerTask {
    pub input_data: FileManagerAction,
}

#[derive(Debug, Clone)]
pub struct FileManagerOutputData {
    pub operation: FileManagerAction
}

impl Task for FileManagerTask {
    fn prepare(self: &mut Self, context: &mut Context, config: &RemoteConfig) -> Result<bool, Message> {
        Ok(true)
    }

    fn run(self: &Self, context: &mut Context, config: &RemoteConfig) -> Result<Success, Message> {
        match &self.input_data {
            FileManagerAction::Remove(data) => {
                remove(self, data)
            }
            FileManagerAction::Check(data) => {
                exits(self, data)
            }
            FileManagerAction::CreateFileString((path, data)) => {
                create_file_string(path, data)
            }
        }?;
        Ok(Success{ value: TaskOutputData::FileManager(FileManagerOutputData { operation: self.input_data.to_owned() }) })
    }

    fn check(self: &Self, context: &mut Context, config: &RemoteConfig) -> Result<Success, Message> {
        match &self.input_data {
            FileManagerAction::Remove(data) => {
                check_remove(self, data)
            }
            _ => { Ok(Success::default()) }
        }
    }

    fn get_type(self: &Self) -> TaskType {
        TaskType::FolderManager("".to_string())
    }

    fn get_id(self: &Self) -> String {
        match &self.input_data {
            FileManagerAction::Remove(data) => { id_generator(data) }
            FileManagerAction::Check(data) => { id_generator(data) }
            FileManagerAction::CreateFileString((data_1, data_2)) => {
                id_generator(&vec![data_1.to_string(), data_2.to_string()])
            }
        }
    }
}

fn create_file_string(file_url: &String, data: &String) -> Result<Success, Message> {
    let path = Path::new(file_url);

    let mut file = fs::File::create(path)?;
    file.write_all(data.as_bytes())?;

    Ok(Success::default())
}

fn remove(task: &FileManagerTask, data: &Vec<String>) -> Result<Success, Message> {
    for file_path in data {
        let path = Path::new(file_path);
        if !path.exists() {
            return Err(Message::RemoveFile(MessageData {
                message: format!("Error trying remove file {}", path.display()),
                ..Default::default()
            }));
        };

        fs::remove_dir_all(path)?;
    }
    Ok(Success::default())
}

fn check_remove(task: &FileManagerTask, data: &Vec<String>) -> Result<Success, Message> {
    for file_path in data {
        let path = Path::new(file_path);
        if path.exists() {
            return Err(Message::RemoveFile(MessageData {
                message: format!("Error file {} could not be removed", path.display()),
                ..Default::default()
            }));
        };
    }
    Ok(Success::default())
}

fn exits(task: &FileManagerTask, data: &Vec<String>) -> Result<Success, Message> {
    for file_path in data {
        let path = Path::new(file_path);
        if !path.exists() {
            return Err(Message::FileNotFound(MessageData {
                message: format!("Error file {} not found", path.display()),
                ..Default::default()
            }));
        };
    }
    Ok(Success::default())
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FileManagerAction {
    Remove(Vec<String>),
    Check(Vec<String>),
    CreateFileString((String, String)), // path, data
}