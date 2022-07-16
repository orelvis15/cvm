#![allow(dead_code, unused_variables)]

use crate::context::storage::TaskOutputData::Empty;
use crate::task::task_impl::commons::command::run_command_io_data::RunCommandOutputData;
use crate::task::task_impl::commons::download::dowload_io_data::DownloadOutputData;
use crate::task::task_impl::commons::file_manager::file_manager_io_data::FileManagerOutputData;
use crate::task::task_impl::commons::folder_manager::folder_manager_io_data::FolderManagerOutputData;
use crate::task::task_impl::commons::permission::permission_io_data::PermissionOutputData;

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
    Download(DownloadOutputData),
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