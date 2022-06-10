#![allow(dead_code, unused_variables)]

use std::fmt;
use std::fmt::Formatter;
use crate::task::task_impl::check_update_task::CheckUpdateData;
use crate::task::task_impl::copy_bin_task::CopyBinInputData;
use crate::task::task_impl::run_command_task::{RunCommandInputData};
use crate::task::task_impl::use_version_task::UserVersionData;

#[derive(Debug, Clone)]
pub enum TaskType {
    RunCommand(RunCommandInputData),
    InstallDependences,
    InstallHaskellGsh,
    CreateFolderStructure,
    DownloadConfigFiles,
    InstallLibsodium,
    BuildCardanoNode,
    CopyBinFiles(CopyBinInputData),
    UseVersion(UserVersionData),
    DeploySystem,
    ServicesManager,
    CheckUpdate(CheckUpdateData),
    EmptyTask(String),
}

impl fmt::Display for TaskType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TaskType::RunCommand(data) => {
                write!(f, "Task: Run_Command_Task | command: {} | Dir: {}", format!("{} {:?}", data.command, data.args), data.current_dir)
            }
            TaskType::InstallDependences => write!(f, "Task: Install_Dependencies_Task"),
            TaskType::InstallHaskellGsh => write!(f, "Task: Install_Haskell_Gsh_Task"),
            TaskType::CreateFolderStructure => write!(f, "Task: Create_Folder_Structure_Task"),
            TaskType::DownloadConfigFiles => write!(f, "Task: Download_Config_File_Task"),
            TaskType::InstallLibsodium => write!(f, "Task: Install_Libsodium_Task"),
            TaskType::BuildCardanoNode => write!(f, "Task: Build_Cardano_Node_Task"),
            TaskType::CopyBinFiles(data) => {
                write!(f, "Task: Copy_Bin_File_Task Path: {} | FileName: {:?} | Version: {}", data.origin_path, data.files_names, data.version)
            }
            TaskType::UseVersion(data) => {
                write!(f, "Task: Use_Version_Task | version: {}", data.version)
            }
            TaskType::CheckUpdate(data) => {
                write!(f, "Task: Check_Update_Task | version: {}", data.version)
            }
            TaskType::EmptyTask(data) => {
                write!(f, "Task: Empty_Task | Data:{}", data)
            }
            TaskType::DeploySystem => write!(f, "Task: Deploy_System"),
            TaskType::ServicesManager => write!(f, "Task: Services_Manager"),
        }
    }
}
