#![allow(dead_code, unused_variables)]

use std::fs;
use std::string::String;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use crate::env::Env;
use crate::{Error, Success, Term};
use crate::config::remote_config::RemoteConfig;
use crate::message::message::Message;
use crate::task::task::Task;
use crate::task::task_type::TaskType;
use faccess::PathExt;

pub struct PermissionTask {
    pub input_data: PermissionAction,
}

impl Task for PermissionTask {
    fn run(self: &Self, _env: &mut Env, config: &RemoteConfig, term: &mut Term) -> Result<Success, Message> {
        match &self.input_data {
            PermissionAction::CheckWrite(data) => {
                check_write(data)
            }
            PermissionAction::CheckRead(data) => {
                check_read(data)
            }
            PermissionAction::CheckExecution(data) => {
                check_execution(data)
            }
            PermissionAction::SetFilesPermission(data) => {
                set_permission(data)
            }
        }
    }

    fn check(self: &Self, _env: &mut Env, config: &RemoteConfig, term: &mut Term) -> Result<Success, Message> {
        match &self.input_data {
            PermissionAction::SetFilesPermission(data) => {
                for (value, mode) in data {
                    let path = Path::new(value);
                    let file_permission = fs::metadata(&path).unwrap().permissions();
                    let expected_permission = fs::Permissions::from_mode(mode.clone());
                    if file_permission != expected_permission {
                        return Err(Message::SettingPermission(Error {
                            message: format!("could not apply permissions to route: {}", value),
                            task: TaskType::Permission("".to_string()),
                            stack: vec![],
                        }));
                    }
                };
                Ok(Success {})
            }
            _ => { Ok(Success {}) }
        }
    }

    fn get_type(self: &Self) -> TaskType {
        let output;
        match self.input_data {
            PermissionAction::SetFilesPermission(_) => {
                output = String::from("Setting permissions");
            }
            PermissionAction::CheckWrite(_) => {
                output = String::from("Checking write");
            }
            PermissionAction::CheckRead(_) => {
                output = String::from("Checking read");
            }
            PermissionAction::CheckExecution(_) => {
                output = String::from("Checking execution");
            }
        };
        TaskType::Permission(output)
    }
}

fn set_permission(data: &Vec<(String, u32)>) -> Result<Success, Message> {
    for (path, mode) in data {
        error_is_dir(path)?;
        fs::set_permissions(path, fs::Permissions::from_mode(mode.clone()))?;
    }
    Ok(Success {})
}

fn check_write(paths: &Vec<String>) -> Result<Success, Message> {
    for value in paths {
        let path = Path::new(value);
        if path.is_dir() {
            let result = check_write_folder(value);
            if result.is_err() { return result; }
        } else {
            let result = check_write_path(value);
            if result.is_err() { return result; }
        }
    }
    Ok(Success {})
}

fn check_read(paths: &Vec<String>) -> Result<Success, Message> {
    for value in paths {
        let path = Path::new(value);
        if path.is_dir() {
            let result = check_read_folder(value);
            if result.is_err() { return result; }
        } else {
            let result = check_read_path(value);
            if result.is_err() { return result; }
        }
    }
    Ok(Success {})
}

fn check_write_folder(value: &String) -> Result<Success, Message> {
    let path = Path::new(value);
    check_write_path(value)?;
    for entry in fs::read_dir(path)? {
        if !entry.unwrap().path().writable() {
            return Err(Message::NoWritePermission(Error {
                message: format!("You don't have write access to the path {}", value),
                task: TaskType::EmptyTask("".to_string()),
                stack: vec![],
            }));
        }
    };
    Ok(Success {})
}

fn check_write_path(value: &String) -> Result<Success, Message> {
    let path = Path::new(value);
    if !path.writable() {
        return Err(Message::NoWritePermission(Error {
            message: format!("You don't have write access to the path {}", value),
            task: TaskType::EmptyTask("".to_string()),
            stack: vec![],
        }));
    }
    Ok(Success {})
}

fn check_read_folder(value: &String) -> Result<Success, Message> {
    let path = Path::new(value);
    check_read_path(value)?;
    for entry in fs::read_dir(path)? {
        if !entry.unwrap().path().readable() {
            return Err(Message::NoReadPermission(Error {
                message: format!("You don't have read access to the path {}", value),
                task: TaskType::EmptyTask("".to_string()),
                stack: vec![],
            }));
        }
    };
    Ok(Success {})
}

fn check_read_path(value: &String) -> Result<Success, Message> {
    let path = Path::new(value);
    if !path.readable() {
        return Err(Message::NoReadPermission(Error {
            message: format!("You don't have read access to the path {}", value),
            task: TaskType::EmptyTask("".to_string()),
            stack: vec![],
        }));
    }
    Ok(Success {})
}

fn check_execution(paths: &Vec<String>) -> Result<Success, Message> {
    for value in paths {
        let path = Path::new(value);
        error_is_dir(value)?;
        for entry in fs::read_dir(path)? {
            if !entry.unwrap().path().executable() {
                return Err(Message::NoExecutionPermission(Error {
                    message: format!("You don't have executable access to the path {}", value),
                    task: TaskType::EmptyTask("".to_string()),
                    stack: vec![],
                }));
            }
        };
    }
    Ok(Success {})
}

fn error_is_dir(value: &String) -> Result<Success, Message> {
    let path = Path::new(value);
    if path.is_dir() {
        return Err(Message::IsDir(Error {
            message: format!("Cannot apply permissions to a directory: {}", value),
            task: TaskType::InstallDependences,
            stack: vec![],
        }));
    };
    Ok(Success {})
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PermissionAction {
    SetFilesPermission(Vec<(String, u32)>),
    CheckWrite(Vec<String>),
    CheckRead(Vec<String>),
    CheckExecution(Vec<String>),
}
