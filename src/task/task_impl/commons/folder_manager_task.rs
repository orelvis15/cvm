#![allow(dead_code, unused_variables)]

use std::fs;
use std::path::Path;
use crate::env::Env;
use crate::{Error, Success, Term, url_build};
use crate::config::config::Config;
use crate::error::message::Message;
use crate::task::task::Task;
use crate::task::task_type::TaskType;

pub struct FolderManagerTask {
    pub input_data: FolderManagerAction,
}

impl Task for FolderManagerTask {
    fn run(self: &Self, _env: &mut Env, config: &Config, term: &mut Term) -> Result<Success, Message> {
        match &self.input_data {
            FolderManagerAction::Create(data) => {
                create(self, data)
            }
            FolderManagerAction::Remove(data) => {
                delete(self, data)
            }
            FolderManagerAction::Clean(data) => {
                clean(self, data)
            }
        }
    }

    fn check(self: &Self, _env: &mut Env, config: &Config, term: &mut Term) -> Result<Success, Message> {
        Ok(Success {})
    }

    fn get_type(self: &Self) -> TaskType {
        TaskType::FolderManager("".to_string())
    }
}

fn create(task: &FolderManagerTask, data: &Vec<(String, String)>) -> Result<Success, Message>{

    for (folder_path, folder_name) in data{

        let path = Path::new(folder_path);
        let full = url_build(vec![folder_path, folder_name], false);
        let full_path = Path::new(&full);

        if !path.exists() || folder_name.is_empty() || full_path.exists(){
            return Err(Message::FolderNotFound(Error{
                message: "Error trying create folder".to_string(),
                task: task.get_type(),
                stack: vec![]
            }));
        };

        fs::create_dir(full_path)?;
    }

    Ok(Success{})
}

fn delete(task: &FolderManagerTask, data: &Vec<String>) -> Result<Success, Message>{
    for folder_path in data {

        let path = Path::new(folder_path);
        if !path.exists() {
            return Err(Message::FolderNotFound(Error{
                message: "Error trying remove folder".to_string(),
                task: task.get_type(),
                stack: vec![]
            }));
        };

        fs::remove_dir_all(path)?;
    }
    Ok(Success{})
}

fn clean(task: &FolderManagerTask, data: &Vec<String>) -> Result<Success, Message>{
    for folder_path in data {
        let path = Path::new(folder_path);
        if !path.exists() {
            return Err(Message::FolderNotFound(Error{
                message: "Error trying remove folder".to_string(),
                task: task.get_type(),
                stack: vec![]
            }));
        };

        for entry in fs::read_dir(path)? {
            if entry.as_ref().unwrap().path().is_dir(){
                fs::remove_dir_all(entry.as_ref().unwrap().path())?
            }else {
                fs::remove_file(entry.as_ref().unwrap().path())?;
            }
        };
    }
    Ok(Success{})
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FolderManagerAction{
    Create(Vec<(String, String)>), //Path for create, Folder name
    Remove(Vec<String>), // Folder path
    Clean(Vec<String>), // Folder path
}