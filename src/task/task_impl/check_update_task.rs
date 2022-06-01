use std::fs::File;
use flate2::read::GzDecoder;
use tar::Archive;
use crate::env::Env;
use crate::{Message, Success};
use crate::config::config::{get_config, get_home_dir};
use crate::task::message_type::MessageType;
use crate::task::task::Task;
use crate::task::task_type::TaskType;
use crate::utils::download_manager::download;

pub struct CheckUpdateTask {
    pub input_data: CheckUpdateData,
}

pub struct CheckUpdateData {
    pub version: String,
}

impl Task for CheckUpdateTask {
    fn run(self: &Self, _env: &mut Env) -> Result<Success, Message> {
        let config = get_config();
        if let Err(_) = config {
            return Err(error());
        }
        let config = config.as_ref().unwrap();

        if &config.general.last_cvm_version <= &self.input_data.version {
            return Err(Message {
                code: 0,
                message: "You already have the latest version".to_string(),
                kind: MessageType::Info,
                task: TaskType::CheckUpdate.to_string(),
                stack: vec![],
            });
        };

        download_and_copy_version(&config.general.last_cvm_version, &config.init.git_assets)
    }

    fn check(self: &Self, _env: &mut Env) -> Result<Success, Message> {
        Ok(Success {})
    }

    fn get_type(self: &Self) -> TaskType {
        TaskType::CheckUpdate
    }
}

fn download_and_copy_version(version: &String, base_url: &String) -> Result<Success, Message> {
    let home_dir = get_home_dir();
    if let Err(error) = home_dir {
        return Err(error);
    }

    let ver = format!("v{}", version);
    let asset = format!("cvm-{}.tar.gz", std::env::consts::ARCH);
    let url = format!("{}/{}/{}", &base_url.as_str(), &ver.as_str(), &asset.as_str());

    let download_path = download(&url, "/cvm.tar.gz");

    if let Err(error) = &download_path {
        return Err(error.clone());
    };

    decompress(download_path.unwrap(), home_dir.unwrap())
}

fn decompress(file_uri: String, home_dir: String) -> Result<Success, Message> {
    let file = File::open(file_uri);

    if let Err(_) = &file {
        return Err(error());
    };

    let tar = GzDecoder::new(file.unwrap());
    let mut archive = Archive::new(tar);


    let result = archive.unpack(format!("{}/{}", home_dir, ".cvm/"));

    if let Err(_) = &result {
        return Err(error());
    };

    Ok(Success {})
}

fn error() -> Message {
    Message {
        code: 0,
        message: "Error trying to update".to_string(),
        kind: MessageType::Error,
        task: "".to_string(),
        stack: vec![],
    }
}