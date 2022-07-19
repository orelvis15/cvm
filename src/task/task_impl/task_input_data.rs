#![allow(dead_code, unused_variables)]

use crate::resolvers::routes_resolve::RoutesResolve;
use crate::task::task_impl::commons::file_manager::file_manager_io_data::FileManagerAction;
use crate::task::task_impl::commons::folder_manager::folder_manager_io_data::FolderManagerAction;
use crate::task::task_impl::commons::permission::permission_io_data::PermissionAction;

#[derive(Debug, Clone)]
pub enum TaskInputData{
    Empty,
    //Natives
    String(String),
    I8(i8),
    VecString(Vec<String>),

    //Resolve by TaskManager
    Resolve(Resolve),

    //FileManager
    FileManager(FileManagerAction),

    //FolderManager
    FolderManager(FolderManagerAction),

    //Permission
    Permission(PermissionAction),

    //Route
    Route(RoutesResolve)
}

impl Default for TaskInputData {
    fn default() -> Self {
        TaskInputData::Empty
    }
}

#[derive(Debug, Clone)]
pub enum Resolve{
    Empty,
    ByPosition(i8),
    ById(i8),
}

impl Default for Resolve {
    fn default() -> Self {
        Resolve::Empty
    }
}