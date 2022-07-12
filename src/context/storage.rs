#![allow(dead_code, unused_variables)]

use crate::context::storage::TaskOutputData::Empty;
use crate::task::task_impl::commons::file_manager_task::FileManagerOutputData;
use crate::task::task_impl::commons::folder_manager_task::FolderManagerOutputData;
use crate::task::task_impl::commons::permission_task::PermissionOutputData;
use crate::task::task_impl::commons::run_command_task::RunCommandOutputData;

#[derive(Default)]
pub struct Storage {
    pub data: Vec<StructureData>
}

#[derive(Default)]
pub struct StructureData {
    pub task_manager_id: String,
    pub task_id: String,
    pub data: TaskOutputData,
}

#[derive(Debug, Clone)]
pub enum TaskOutputData {
    Empty(),
    RunCommand(RunCommandOutputData),
    FileManager(FileManagerOutputData),
    FolderManager(FolderManagerOutputData),
    Permission(PermissionOutputData),
}

impl Default for TaskOutputData {
    fn default() -> Self {
        Empty()
    }
}

impl Storage {
    pub fn add(&mut self, data: StructureData) {
        let _ = &self.data.push(data);
    }
}