#![allow(dead_code, unused_variables)]

extern crate strfmt;

use std::collections::HashMap;
use std::fs::File;
use flate2::read::GzDecoder;
use tar::Archive;
use crate::env::Env;
use crate::{Message, MessageData, Success, Term, url_build};
use crate::config::remote_config::{DownloadInstall, RemoteConfig};
use crate::message::message::MessageKind;
use crate::task::task::Task;
use crate::task::task_type::TaskType;
use crate::utils::download_manager::download_in_path;
use crate::utils::folders::Folder;
use strfmt::strfmt;

pub struct DownloadInstallTask {
    pub version: String,
}

impl Task for DownloadInstallTask {
    fn prepare(self: &mut Self, env: &mut Env, config: &RemoteConfig, term: &mut Term) -> Result<bool, Message> {
        Ok(true)
    }

    fn run(self: &Self, env: &mut Env, config: &RemoteConfig, term: &mut Term) -> Result<Success, Message> {
        let bin_name = "cnode.tar.gz";
        let bin_folder = Folder::get_path(Folder::BIN, &config);
        let version_folder = url_build(vec![&bin_folder, &self.version], false);

        let url = build_download_url(&config.download_install, &self.version)?;
        let file = download_in_path(&url, version_folder.clone(), bin_name.to_string())?;

        decompress(&file, &version_folder)
    }

    fn check(self: &Self, env: &mut Env, config: &RemoteConfig, term: &mut Term) -> Result<Success, Message> {
        Ok(Success {})
    }

    fn get_type(self: &Self) -> TaskType {
        TaskType::DownloadInstall("".to_string())
    }
}

fn decompress(file_uri: &String, folder: &String) -> Result<Success, Message> {
    let file = File::open(file_uri)?;
    let tar = GzDecoder::new(file);
    let mut archive = Archive::new(tar);
    archive.unpack(folder)?;
    Ok(Success {})
}

fn build_download_url(data: &DownloadInstall, version: &String) -> Result<String, Message> {
    let version_position = &data.versions.iter().position(|item| item == version);
    if version_position.is_none() {
        return Err(Message::VersionExist(MessageData {
            message: "The version does not exist yet".to_string(),
            kind: MessageKind::Info,
            ..Default::default()
        }));
    }

    let mut vars = HashMap::new();
    vars.insert("version".to_string(), version);

    let end_url_with_version = strfmt(&data.end_url, &vars).unwrap();
    let build_id = data.build_id.get(version_position.unwrap()).unwrap();

    let url_result = format!("{}{}{}", &data.base_url, build_id, &end_url_with_version);
    Ok(url_result)
}