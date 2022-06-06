#![allow(dead_code, unused_variables)]

use std::{io};
use std::fmt::Debug;
use std::io::ErrorKind;
use owo_colors::OwoColorize;
use toml::de;
use crate::task::task_type::TaskType;

#[derive(Debug, Clone)]
pub struct Error {
    pub message: String,
    pub task: TaskType,
    pub stack: Vec<String>,
}

impl Error {
    pub fn to_string(&self) -> String {
        self.message.to_string()
    }
}

#[derive(Debug, Clone)]
pub enum CvmError {
    //Tasks Errors
    ErrorRunTask(Error),
    TaskType(Error),

    //App Features
    AlreadyLastUpdate(Error),
    ErrorUpdate(Error),
    CreateFolderStructure(Error),
    BinNotFound(Error),
    GettingDependences(Error),
    VersionInstaller(Error),
    VersionBadFormed(Error),
    CheckCardanoVersion(Error),

    //IO Errors
    FileNotFound(Error),
    FolderNotFound(Error),
    ParsingFile(Error),
    DownloadFile(Error),
    OpenFile(Error),
    CreateFolder(Error),
    WriteFile(Error),
    PermissionDenied(Error),
    DeletingFolder(Error),
    Copy(Error),

    //Commands Errors
    CommandNotFound(Error),
    FaileToRunCommand(Error),
    CommandOutputError(Error),

    Generic(Error),
}

impl CvmError {
    pub fn data(&self) -> &Error {
        match self {
            CvmError::ErrorRunTask(this) => { &this }
            CvmError::TaskType(this) => { &this }
            CvmError::AlreadyLastUpdate(this) => { &this }
            CvmError::ErrorUpdate(this) => { &this }
            CvmError::CreateFolderStructure(this) => { &this }
            CvmError::BinNotFound(this) => { &this }
            CvmError::GettingDependences(this) => { &this }
            CvmError::VersionInstaller(this) => { &this }
            CvmError::VersionBadFormed(this) => { &this }
            CvmError::CheckCardanoVersion(this) => { &this }
            CvmError::FileNotFound(this) => { &this }
            CvmError::FolderNotFound(this) => { &this }
            CvmError::ParsingFile(this) => { &this }
            CvmError::DownloadFile(this) => { &this }
            CvmError::CreateFolder(this) => { &this }
            CvmError::WriteFile(this) => { &this }
            CvmError::PermissionDenied(this) => { &this }
            CvmError::DeletingFolder(this) => { &this }
            CvmError::Copy(this) => { &this }
            CvmError::CommandNotFound(this) => { &this }
            CvmError::FaileToRunCommand(this) => { &this }
            CvmError::CommandOutputError(this) => { &this }
            CvmError::Generic(this) => { &this }
            CvmError::OpenFile(this) => { &this }
        }
    }

    #[cfg(debug_assertions)]
    pub fn print(&self){
        let message = format!("Message: {} \n{} \nStack:{:?}", self.data().message, self.data().task, self.data().stack);
        println!("{}", message.red())
    }

    #[cfg(not(debug_assertions))]
    pub fn print(&self){
        println!("{}", self.data().message.red())
    }
}

impl From<reqwest::Error> for CvmError {
    fn from(error: reqwest::Error) -> Self {
        let data = format!("{:?}", error.url());
        return CvmError::DownloadFile(
            Error {
                message: "Error download file".to_string(),
                task: TaskType::EmptyTask(data),
                stack: vec![error.to_string()],
            });
    }
}

impl From<de::Error> for CvmError {
    fn from(error: de::Error) -> Self {
        return CvmError::DownloadFile(
            Error {
                message: "Error try parsing config file".to_string(),
                task: TaskType::EmptyTask("".to_string()),
                stack: vec![error.to_string()],
            });
    }
}

impl From<clap::Error> for CvmError {
    fn from(error: clap::Error) -> Self {
        return CvmError::DownloadFile(
            Error {
                message: "Error executing command".to_string(),
                task: TaskType::EmptyTask("Clap error".to_string()),
                stack: vec![error.to_string()],
            });
    }
}

impl From<io::Error> for CvmError {
    fn from(error: io::Error) -> Self {
        return match error.kind() {
            ErrorKind::NotFound => {
                CvmError::FileNotFound(
                    Error {
                        message: "File not found".to_string(),
                        task: TaskType::EmptyTask("".to_string()),
                        stack: vec![error.to_string()],
                    })
            }
            ErrorKind::PermissionDenied => {
                CvmError::PermissionDenied(
                    Error {
                        message: "Error permission denied".to_string(),
                        task: TaskType::EmptyTask("".to_string()),
                        stack: vec![error.to_string()],
                    })
            }
            ErrorKind::Interrupted => {
                CvmError::WriteFile(
                    Error {
                        message: "Error writing file".to_string(),
                        task: TaskType::EmptyTask("".to_string()),
                        stack: vec![error.to_string()],
                    })
            }
            ErrorKind::AlreadyExists => {
                CvmError::WriteFile(
                    Error {
                        message: "The element already exists".to_string(),
                        task: TaskType::EmptyTask("".to_string()),
                        stack: vec![error.to_string()],
                    })
            }
            _ => {
                CvmError::Generic(
                    Error {
                        message: "A problem has occurred :(".to_string(),
                        task: TaskType::EmptyTask("".to_string()),
                        stack: vec![error.to_string()],
                    })
            }
        };
    }
}