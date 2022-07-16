use crate::task::task_impl::task_input_data::TaskInputData;

/// Input for run command Task
/// * `command` -> String Comando que se ejecutará
/// * `args` -> Vec<String> All the arg for the command
/// * `current_dir` -> String Folder where the command will run
/// * `description` -> String text will be printer in the terminal
#[derive(Default, Debug, Clone)]
pub struct RunCommandInputData {
    pub command: TaskInputData,
    pub args: TaskInputData,
    pub current_dir: TaskInputData,
    pub description: TaskInputData,
}

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct ResolveRunCommandInputData {
    pub command: String,
    pub args: Vec<String>,
    pub current_dir: String,
    pub description: String,
}

#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct RunCommandOutputData {
    pub tag: String,
    pub code: i32,
}