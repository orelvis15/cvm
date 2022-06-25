#![allow(dead_code, unused_variables)]

use std::fs;
use std::io::Write;
use std::path::Path;
use crate::env::Env;
use crate::{Error, Success, Term};
use crate::config::remote_config::Config;
use crate::error::message::Message;
use crate::task::task::Task;
use crate::task::task_type::TaskType;

pub struct FileManagerTask {
    pub input_data: FileManagerAction,
}

impl Task for FileManagerTask {
    fn run(self: &Self, _env: &mut Env, config: &Config, term: &mut Term) -> Result<Success, Message> {
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
        }
    }

    fn check(self: &Self, _env: &mut Env, config: &Config, term: &mut Term) -> Result<Success, Message> {
        match &self.input_data {
            FileManagerAction::Remove(data) => {
                check_remove(self, data)
            }
            _ => { Ok(Success {}) }
        }
    }

    fn get_type(self: &Self) -> TaskType {
        TaskType::FolderManager("".to_string())
    }
}

fn create_file_string(file_url: &String, data: &String) -> Result<Success, Message> {
    let path = Path::new(file_url);

    let mut file = fs::File::create(path)?;
    file.write_all(data.as_bytes())?;

    Ok(Success{})
}

fn remove(task: &FileManagerTask, data: &Vec<String>) -> Result<Success, Message> {
    for file_path in data {
        let path = Path::new(file_path);
        if !path.exists() {
            return Err(Message::RemoveFile(Error {
                message: format!("Error trying remove file {}", path.display()),
                task: task.get_type(),
                stack: vec![],
            }));
        };

        fs::remove_dir_all(path)?;
    }
    Ok(Success {})
}

fn check_remove(task: &FileManagerTask, data: &Vec<String>) -> Result<Success, Message> {
    for file_path in data {
        let path = Path::new(file_path);
        if path.exists() {
            return Err(Message::RemoveFile(Error {
                message: format!("Error file {} could not be removed", path.display()),
                task: task.get_type(),
                stack: vec![],
            }));
        };
    }
    Ok(Success {})
}

fn exits(task: &FileManagerTask, data: &Vec<String>) -> Result<Success, Message> {
    for file_path in data {
        let path = Path::new(file_path);
        if !path.exists() {
            return Err(Message::FileNotFound(Error {
                message: format!("Error file {} not found", path.display()),
                task: task.get_type(),
                stack: vec![],
            }));
        };
    }
    Ok(Success {})
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FileManagerAction {
    Remove(Vec<String>),
    Check(Vec<String>),
    CreateFileString((String, String)), // path, data
}