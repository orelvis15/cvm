use std::fmt;
use std::fmt::Formatter;

pub enum TaskType{
    Command,
    InstallDependences,
    InstallHaskellGsh,
    CreateFolderStructure,
    DownloadConfigFiles,
    SetEnviromentVariable,
    InstallLibsodium,
    BuildCardanoNode,
    CopyBinFiles,
    UseVersion,
}

impl fmt::Display for TaskType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TaskType::Command => write!(f, "command"),
            TaskType::InstallDependences => write!(f, "install_dependencies"),
            TaskType::InstallHaskellGsh => write!(f, "install_haskell_gsh"),
            TaskType::CreateFolderStructure => write!(f, "create_folder_structure"),
            TaskType::DownloadConfigFiles => write!(f, "download_config_file"),
            TaskType::SetEnviromentVariable => write!(f, "set_environment_variable"),
            TaskType::InstallLibsodium => write!(f, "install_libsodium"),
            TaskType::BuildCardanoNode => write!(f, "build_cardano_node"),
            TaskType::CopyBinFiles => write!(f, "copy_bin_file"),
            TaskType::UseVersion => write!(f, "use_version"),
        }
    }
}