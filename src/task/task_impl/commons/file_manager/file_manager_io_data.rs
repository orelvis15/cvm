#![allow(dead_code, unused_variables)]

use crate::task::task_impl::task_input_data::TaskInputData;

/// Input for File Manager Task
/// * `action` -> FileManagerAction The action you are going to do
#[derive(Default)]
pub struct FileManagerInputData {
    pub action: TaskInputData
}

#[derive(Default)]
pub struct ResolveFileManagerInputData {
    pub action: FileManagerAction
}

/// Input for Download Task
/// * `Remove - Vec<String>` -> path for all files we will be remove
/// * `Check - Vec<String>` -> path for all files we will be check if exist
/// * `CreateFileString - (String, String)` -> (path, data) first arg is the file we will create the file
///                                                         seconds arg is the data we will write in the file
#[derive(Debug, Clone)]
pub enum FileManagerAction {
    Remove(Vec<String>),
    Check(Vec<String>),
    CreateFileString((String, String)), // path, data
}

impl Default for FileManagerAction {
    fn default() -> Self {
        FileManagerAction::Check(vec![])
    }
}

#[derive(Debug, Clone)]
pub struct FileManagerOutputData {
    pub operation: FileManagerAction
}