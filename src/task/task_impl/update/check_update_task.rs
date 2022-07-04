#![allow(dead_code, unused_variables)]

use std::collections::HashMap;
use std::fs::File;
use flate2::read::GzDecoder;
use tar::Archive;
use crate::env::Env;
use strfmt::strfmt;
use crate::{Success, Term};
use crate::config::remote_config::RemoteConfig;
use crate::message::message::{Message, MessageData, MessageKind};
use crate::task::task::Task;
use crate::task::task_type::TaskType;
use crate::utils::download_manager::download;
use crate::utils::folders::Folder;
use crate::utils::version_utils::get_last_cvm_version;

const GIT_DOWNLOAD_URL: &str = "https://github.com/orelvis15/cvm/releases/download";
const VERSION_PATTERN: &str = "v{version}";
const NAME_PATTERN: &str = "cvm-{arch}.tar.gz";
const FILE_NAME: &str = "cvm.tar.gz";

pub struct CheckUpdateTask {
    pub input_data: CheckUpdateData,
}

#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct CheckUpdateData {
    pub version: String,
}

impl Task for CheckUpdateTask {

    fn prepare(self: &mut Self, env: &mut Env, config: &RemoteConfig, term: &mut Term) -> Result<bool, Message> {
        Ok(true)
    }

    fn run(self: &Self, _env: &mut Env, config: &RemoteConfig, term: &mut Term) -> Result<Success, Message> {
        let last_version = get_last_cvm_version()?;
        let last_version = last_version.replace("v", "");
        if &last_version <= &self.input_data.version {
            return Err(Message::AlreadyLastUpdate(MessageData {
                message: "You already have the latest version".to_string(),
                task: self.get_type(),
                kind: MessageKind::Info,
                ..Default::default()
            }));
        };

        download_and_copy_version(&last_version)
    }

    fn check(self: &Self, _env: &mut Env, config: &RemoteConfig, term: &mut Term) -> Result<Success, Message> {
        Ok(Success {})
    }

    fn get_type(self: &Self) -> TaskType {
        TaskType::CheckUpdate(self.input_data.clone())
    }
}

fn download_and_copy_version(version: &String) -> Result<Success, Message> {
    let home_dir = Folder::get_home_dir()?;

    let mut version_map = HashMap::new();
    version_map.insert("version".to_string(), version);

    let ver = strfmt(VERSION_PATTERN, &version_map).unwrap();

    let arch = std::env::consts::ARCH.clone();
    let mut arch_map = HashMap::new();
    arch_map.insert("arch".to_string(), arch.to_string());

    let asset = strfmt(&NAME_PATTERN, &arch_map).unwrap();
    let url = format!("{}/{}/{}", GIT_DOWNLOAD_URL, &ver.as_str(), &asset.as_str());

    let download_path = download(&url, format!("/{}", &FILE_NAME).as_str())?;

    decompress(download_path, home_dir)
}

fn decompress(file_uri: String, home_dir: String) -> Result<Success, Message> {
    let file = File::open(file_uri)?;
    let tar = GzDecoder::new(file);
    let mut archive = Archive::new(tar);
    archive.unpack(format!("{}/{}/", home_dir, ".cvm"))?;

    Ok(Success {})
}