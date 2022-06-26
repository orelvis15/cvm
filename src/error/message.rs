#![allow(dead_code, unused_variables)]

use std::io;
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
pub enum Message {
    //Tasks Errors
    ErrorRunTask(Error),
    TaskType(Error),
    Libsodium(Error),

    //App Features
    AlreadyLastUpdate(Error),
    ErrorUpdate(Error),
    CreateFolderStructure(Error),
    BinNotFound(Error),
    GettingDependences(Error),
    VersionInstaller(Error),
    VersionBadFormed(Error),
    CheckCardanoVersion(Error),
    UseVersion(Error),
    UpdateConfigFile(Error),

    //IO Errors
    FileNotFound(Error),
    RemoveFile(Error),
    FolderNotFound(Error),
    ParsingFile(Error),
    DownloadFile(Error),
    OpenFile(Error),
    CreateFolder(Error),
    WriteFile(Error),
    RemoveFolder(Error),
    Copy(Error),

    //Permission
    PermissionDenied(Error),
    NoWritePermission(Error),
    NoReadPermission(Error),
    NoExecutionPermission(Error),
    SettingPermission(Error),

    //Commands Errors
    CommandNotFound(Error),
    FaileToRunCommand(Error),
    CommandOutputError(Error),

    //Files - Directories
    IsDir(Error),
    IsFile(Error),

    //arguments
    ParseArg(Error),

    Generic(Error),
}

impl Message {
    pub fn data(&self) -> &Error {
        match self {
            Message::ErrorRunTask(this) => { &this }
            Message::TaskType(this) => { &this }
            Message::AlreadyLastUpdate(this) => { &this }
            Message::ErrorUpdate(this) => { &this }
            Message::CreateFolderStructure(this) => { &this }
            Message::BinNotFound(this) => { &this }
            Message::GettingDependences(this) => { &this }
            Message::VersionInstaller(this) => { &this }
            Message::VersionBadFormed(this) => { &this }
            Message::CheckCardanoVersion(this) => { &this }
            Message::FileNotFound(this) => { &this }
            Message::FolderNotFound(this) => { &this }
            Message::ParsingFile(this) => { &this }
            Message::DownloadFile(this) => { &this }
            Message::CreateFolder(this) => { &this }
            Message::WriteFile(this) => { &this }
            Message::RemoveFolder(this) => { &this }
            Message::Copy(this) => { &this }
            Message::CommandNotFound(this) => { &this }
            Message::FaileToRunCommand(this) => { &this }
            Message::CommandOutputError(this) => { &this }
            Message::Generic(this) => { &this }
            Message::OpenFile(this) => { &this }
            Message::ParseArg(this) => { &this }
            Message::RemoveFile(this) => { &this }
            Message::NoWritePermission(this) => { &this }
            Message::NoReadPermission(this) => { &this }
            Message::NoExecutionPermission(this) => { &this }
            Message::IsDir(this) => { &this }
            Message::IsFile(this) => { &this }
            Message::PermissionDenied(this) => { &this }
            Message::SettingPermission(this) => { &this }
            Message::Libsodium(this) => { &this }
            Message::UseVersion(this) => { &this }
            Message::UpdateConfigFile(this) => { &this }
        }
    }

    #[cfg(debug_assertions)]
    pub fn print(&self) {
        let message = format!("Message: {} \n{} \nStack:{:?}", self.data().message, self.data().task, self.data().stack);
        println!("{}", message.red())
    }

    #[cfg(not(debug_assertions))]
    pub fn print(&self) {
        println!("{}", self.data().message.red())
    }
}

impl From<reqwest::Error> for Message {
    fn from(error: reqwest::Error) -> Self {
        let data = format!("{:?}", error.url());
        return Message::DownloadFile(
            Error {
                message: "Error download file".to_string(),
                task: TaskType::EmptyTask(data),
                stack: vec![error.to_string()],
            });
    }
}

impl From<de::Error> for Message {
    fn from(error: de::Error) -> Self {
        return Message::DownloadFile(
            Error {
                message: "Error try parsing config file".to_string(),
                task: TaskType::EmptyTask("".to_string()),
                stack: vec![error.to_string()],
            });
    }
}

impl From<clap::Error> for Message {
    fn from(error: clap::Error) -> Self {
        return Message::DownloadFile(
            Error {
                message: "Error executing command".to_string(),
                task: TaskType::EmptyTask("Clap error".to_string()),
                stack: vec![error.to_string()],
            });
    }
}

impl From<tinytemplate::error::Error> for Message {
    fn from(error: tinytemplate::error::Error) -> Self {
        return Message::DownloadFile(
            Error {
                message: "Error trying to parse service file".to_string(),
                task: TaskType::EmptyTask("TiniTemplate error".to_string()),
                stack: vec![error.to_string()],
            });
    }
}

impl From<io::Error> for Message {
    fn from(error: io::Error) -> Self {
        return match error.kind() {
            ErrorKind::NotFound => {
                Message::FileNotFound(
                    Error {
                        message: "File not found".to_string(),
                        task: TaskType::EmptyTask("".to_string()),
                        stack: vec![error.to_string()],
                    })
            }
            ErrorKind::PermissionDenied => {
                Message::PermissionDenied(
                    Error {
                        message: "Error permission denied".to_string(),
                        task: TaskType::EmptyTask("".to_string()),
                        stack: vec![error.to_string()],
                    })
            }
            ErrorKind::Interrupted => {
                Message::WriteFile(
                    Error {
                        message: "Error writing file".to_string(),
                        task: TaskType::EmptyTask("".to_string()),
                        stack: vec![error.to_string()],
                    })
            }
            ErrorKind::AlreadyExists => {
                Message::WriteFile(
                    Error {
                        message: "The element already exists".to_string(),
                        task: TaskType::EmptyTask("".to_string()),
                        stack: vec![error.to_string()],
                    })
            }
            _ => {
                Message::Generic(
                    Error {
                        message: "A problem has occurred :(".to_string(),
                        task: TaskType::EmptyTask("".to_string()),
                        stack: vec![error.to_string()],
                    })
            }
        };
    }
}