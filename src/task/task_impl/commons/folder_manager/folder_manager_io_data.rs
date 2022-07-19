use crate::task::task_impl::task_input_data::TaskInputData;

/// Input for Folder Manager Task
/// * `action` -> FolderManagerAction The action you are going to do
#[derive(Default)]
pub struct FolderManagerInputData{
    pub action: TaskInputData
}

#[derive(Default)]
pub struct ResolveFolderManagerInputData{
    pub action: FolderManagerAction
}

/// * `Create - Vec<(String, String)>` -> (path, folderName) firt arg path from where the folders will be created
///                                                         seconds arg is the name for the folders
/// * `Remove - Vec<String>` -> paths for all folders we will be remove
/// * `Clean - Vec<String>` -> paths of the folders that will be emptied
/// * `Exist - Vec<String>` -> paths of the folders that will be check if exist
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FolderManagerAction {
    Create(Vec<(String, String)>),
    Remove(Vec<String>),
    Clean(Vec<String>),
    Exits(Vec<String>),
}

impl Default for FolderManagerAction {
    fn default() -> Self {
        FolderManagerAction::Exits(vec![])
    }
}

#[derive(Default, Debug, Clone)]
pub struct FolderManagerOutputData {
    pub operation: FolderManagerAction,
}