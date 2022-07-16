use crate::task::task_impl::task_input_data::TaskInputData;

#[derive(Default)]
pub struct PermissionInputData{
    pub action: TaskInputData
}

#[derive(Default)]
pub struct ResolvePermissionInputData{
    pub action: PermissionAction
}

/// * `SetFilesPermission - Vec<(String, u32)>` -> (path, permission) firt arg path of the file to which the permissions are to be applied
///                                                                   seconds arg is the permission you want to apply
/// * `CheckWrite - Vec<String>` -> paths for all folder we will be remove
/// * `CheckRead - Vec<String>` -> paths of the folders that will be emptied
/// * `CheckExecution - Vec<String>` -> paths of the folders that will be check if exist
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PermissionAction {
    SetFilesPermission(Vec<(String, u32)>),
    CheckWrite(Vec<String>),
    CheckRead(Vec<String>),
    CheckExecution(Vec<String>),
}

impl Default for PermissionAction {
    fn default() -> Self {
        PermissionAction::CheckRead(vec![])
    }
}

#[derive(Default, Debug, Clone)]
pub struct PermissionOutputData {
    pub operation: PermissionAction,
}