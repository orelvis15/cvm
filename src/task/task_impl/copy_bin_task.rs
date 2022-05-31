use std::fs;
use std::path::Path;
use walkdir::WalkDir;
use crate::env::Env;
use crate::{Message, Success, url_build};
use crate::config::config::{get_config, get_project_dir};
use crate::task::message_type::MessageType;
use crate::task::task::Task;
use crate::task::task_type::TaskType;

pub struct CopyBinTask {
    pub input_data: CopyBinInputData,
}

pub struct CopyBinInputData {
    pub file_name: String,
    pub origin_path: String,
    pub version: String,
}

const BIN_FOLDER: &str = "bin";

impl Task for CopyBinTask {
    fn run(self: &Self, _env: &mut Env) -> Result<Success, Message> {
        let config = get_config();
        if let Err(error) = config {
            return Err(error);
        };

        let project_dir = get_project_dir();

        let bin_folder = url_build(vec![project_dir.as_str(), &config.as_ref().unwrap().workspace.workspace_folder.as_str(), BIN_FOLDER], false);
        let version_folder = url_build(vec![bin_folder.as_str(), &self.input_data.version], false);
        let version_folder_path = Path::new(version_folder.as_str());
        if !version_folder_path.exists() {
            let create_folder_result = fs::create_dir_all(version_folder_path);

            if let Err(error) = create_folder_result {
                return Err(Message {
                    code: 0,
                    message: "Error creating folder structure".to_string(),
                    kind: MessageType::Error,
                    task: "".to_string(),
                    stack: vec![error.to_string()],
                });
            }
        };

        build_copy_program_to_bin_folder_command(&self.input_data.file_name, &version_folder.to_string(), &self.input_data.origin_path)
    }

    fn check(self: &Self, _env: &mut Env) -> Result<Success, Message> {
        Ok(Success {})
    }

    fn get_type(self: &Self) -> TaskType {
        TaskType::CopyBinFiles
    }
}

fn build_copy_program_to_bin_folder_command(file_name: &String, destination_path: &String, origin_path: &String) -> Result<Success, Message> {
    for entry in WalkDir::new(origin_path) {
        let entry = entry.unwrap();
        if entry.file_name().to_str().unwrap() == file_name && entry.path().is_file() {
            let copy_result = fs::copy(entry.path(), format!("{}/{}", destination_path, file_name));
            if let Err(error) = copy_result {
                return Err(Message {
                    code: 0,
                    message: "Error copy file".to_string(),
                    kind: MessageType::Error,
                    task: "".to_string(),
                    stack: vec![error.to_string()],
                });
            } else {
                return Ok(Success {});
            }
        }
    }
    return Err(Message {
        code: 0,
        message: "Cardano executable not found".to_string(),
        kind: MessageType::Error,
        task: TaskType::BuildCardanoNode.to_string(),
        stack: vec![],
    });
}