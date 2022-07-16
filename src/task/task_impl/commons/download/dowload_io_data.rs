use crate::task::task_impl::task_input_data::TaskInputData;

/// Input for Download Task
/// * `urls` -> Vec<String> All the urls that we are going to download
/// * `folder` -> String Folder where we are going to save
#[derive(Default)]
pub struct DownloadInputData{
    pub urls: TaskInputData,
    pub folder: TaskInputData,
}

#[derive(Default)]
pub struct ResolveDownloadInputData {
    pub urls: Vec<String>,
    pub folder: String
}

#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct DownloadOutputData {
    pub files: Vec<String>,
}