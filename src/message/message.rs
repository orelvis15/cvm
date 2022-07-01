#![allow(dead_code, unused_variables)]

use std::io;
use std::fmt::Debug;
use std::io::ErrorKind;
use crossterm::style::Stylize;
use toml::de;
use crate::task::task_type::TaskType;
use core::default::Default;

#[derive(Debug, Clone, Default)]
pub struct MessageData {
    pub message: String,
    pub task: TaskType,
    pub stack: Vec<String>,
    pub kind: MessageKind,
}

impl MessageData {
    pub fn to_string(&self) -> String {
        self.message.to_string()
    }
}

#[derive(Debug, Clone)]
pub enum MessageKind {
    Error,
    Info,
    Warning,
}

impl Default for MessageKind {
    fn default() -> Self {
        MessageKind::Error
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    //Tasks Errors
    ErrorRunTask(MessageData),
    TaskType(MessageData),
    Libsodium(MessageData),

    //App Features
    NewUpdate(MessageData),
    AlreadyLastUpdate(MessageData),
    ErrorUpdate(MessageData),
    CreateFolderStructure(MessageData),
    BinNotFound(MessageData),
    GettingDependences(MessageData),
    VersionInstaller(MessageData),
    VersionBadFormed(MessageData),
    CheckCardanoVersion(MessageData),
    UseVersion(MessageData),
    UpdateConfigFile(MessageData),
    ProjectNotInit(MessageData),
    VersionExist(MessageData),

    //IO Errors
    FileNotFound(MessageData),
    RemoveFile(MessageData),
    FolderNotFound(MessageData),
    ParsingFile(MessageData),
    DownloadFile(MessageData),
    OpenFile(MessageData),
    CreateFolder(MessageData),
    WriteFile(MessageData),
    RemoveFolder(MessageData),
    Copy(MessageData),
    UserNotFound(MessageData),

    //Permission
    PermissionDenied(MessageData),
    NoWritePermission(MessageData),
    NoReadPermission(MessageData),
    NoExecutionPermission(MessageData),
    SettingPermission(MessageData),

    //Commands Errors
    CommandNotFound(MessageData),
    FaileToRunCommand(MessageData),
    CommandOutputError(MessageData),

    //Files - Directories
    IsDir(MessageData),
    IsFile(MessageData),

    //arguments
    ParseArg(MessageData),

    Generic(MessageData),
}

impl Message {
    pub fn data(&self) -> &MessageData {
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
            Message::ProjectNotInit(this) => { &this }
            Message::VersionExist(this) => { &this }
            Message::UserNotFound(this) => { &this }
            Message::NewUpdate(this) => { &this }
        }
    }

    #[cfg(debug_assertions)]
    pub fn print(&self) {
        let message = format!("Message: {} \n{} \nStack:{:?}", self.data().message, self.data().task, self.data().stack);
        match self.data().kind {
            MessageKind::Info => { println!("{}", message.blue()) }
            MessageKind::Warning => { println!("{}", message.yellow()) }
            _ => { println!("{}", message.red()) }
        }
    }

    #[cfg(not(debug_assertions))]
    pub fn print(&self) {
        match self.data().kind {
            MessageKind::Info => { println!("{}", self.data().message.blue()) }
            MessageKind::Warning => { println!("{}", self.data().message.yellow()) }
            _ => { println!("{}", self.data().message.red()) }
        }
    }
}

impl From<reqwest::Error> for Message {
    fn from(error: reqwest::Error) -> Self {
        let data = format!("{:?}", error.url());
        return Message::DownloadFile(
            MessageData {
                message: format!("{} {}", "Error download file", data),
                task: TaskType::EmptyTask(data),
                stack: vec![error.to_string()],
                ..Default::default()
            });
    }
}

impl From<de::Error> for Message {
    fn from(error: de::Error) -> Self {
        return Message::DownloadFile(
            MessageData {
                message: "Error try parsing config file".to_string(),
                task: TaskType::EmptyTask("".to_string()),
                stack: vec![error.to_string()],
                ..Default::default()
            });
    }
}

impl From<clap::Error> for Message {
    fn from(error: clap::Error) -> Self {
        return Message::DownloadFile(
            MessageData {
                message: "Error executing command".to_string(),
                stack: vec![error.to_string()],
                ..Default::default()
            });
    }
}

impl From<tinytemplate::error::Error> for Message {
    fn from(error: tinytemplate::error::Error) -> Self {
        return Message::DownloadFile(
            MessageData {
                message: "Error trying to parse service file".to_string(),
                task: TaskType::EmptyTask("TiniTemplate message".to_string()),
                stack: vec![error.to_string()],
                ..Default::default()
            });
    }
}

impl From<io::Error> for Message {
    fn from(error: io::Error) -> Self {
        return match error.kind() {
            ErrorKind::NotFound => {
                Message::FileNotFound(
                    MessageData {
                        message: "File not found".to_string(),
                        task: TaskType::EmptyTask("".to_string()),
                        stack: vec![error.to_string()],
                        ..Default::default()
                    })
            }
            ErrorKind::PermissionDenied => {
                Message::PermissionDenied(
                    MessageData {
                        message: "Error permission denied".to_string(),
                        task: TaskType::EmptyTask("".to_string()),
                        stack: vec![error.to_string()],
                        ..Default::default()
                    })
            }
            ErrorKind::Interrupted => {
                Message::WriteFile(
                    MessageData {
                        message: "Error writing file".to_string(),
                        task: TaskType::EmptyTask("".to_string()),
                        stack: vec![error.to_string()],
                        ..Default::default()
                    })
            }
            ErrorKind::AlreadyExists => {
                Message::WriteFile(
                    MessageData {
                        message: "The element already exists".to_string(),
                        task: TaskType::EmptyTask("".to_string()),
                        stack: vec![error.to_string()],
                        ..Default::default()
                    })
            }
            _ => {
                Message::Generic(
                    MessageData {
                        message: "A problem has occurred :(".to_string(),
                        task: TaskType::EmptyTask("".to_string()),
                        stack: vec![error.to_string()],
                        ..Default::default()
                    })
            }
        };
    }
}