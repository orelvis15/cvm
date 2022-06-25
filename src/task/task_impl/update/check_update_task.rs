#![allow(dead_code, unused_variables)]

use std::collections::HashMap;
use std::fs::File;
use flate2::read::GzDecoder;
use tar::Archive;
use crate::env::Env;
use strfmt::strfmt;
use crate::{Success, Term};
use crate::config::remote_config::{Config, get_home_dir, Update};
use crate::error::message::{Message, Error};
use crate::task::task::Task;
use crate::task::task_type::TaskType;
use crate::utils::download_manager::download;

pub struct CheckUpdateTask {
    pub input_data: CheckUpdateData,
}

#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct CheckUpdateData {
    pub version: String,
}

impl Task for CheckUpdateTask {
    fn run(self: &Self, _env: &mut Env, config: &Config, term: &mut Term) -> Result<Success, Message> {
        if &config.update.last_cvm_version <= &self.input_data.version {
            return Err(Message::AlreadyLastUpdate(Error {
                message: "You already have the latest version".to_string(),
                task: self.get_type(),
                stack: vec![],
            }));
        };

        download_and_copy_version(&config.update.last_cvm_version, &config.init.git_assets, &config.update)
    }

    fn check(self: &Self, _env: &mut Env, config: &Config, term: &mut Term) -> Result<Success, Message> {
        Ok(Success {})
    }

    fn get_type(self: &Self) -> TaskType {
        TaskType::CheckUpdate(self.input_data.clone())
    }
}

fn download_and_copy_version(version: &String, base_url: &String, update_data: &Update) -> Result<Success, Message> {
    let home_dir = get_home_dir();
    if let Err(error) = home_dir {
        return Err(error);
    }

    let mut version_map = HashMap::new();
    version_map.insert("version".to_string(), version);

    let ver = strfmt(&update_data.version_pattern, &version_map).unwrap();

    let arch = std::env::consts::ARCH.clone();
    let mut arch_map = HashMap::new();
    arch_map.insert("arch".to_string(), arch.to_string());

    let asset = strfmt(&update_data.name_pattern, &arch_map).unwrap();
    let url = format!("{}/{}/{}", &base_url.as_str(), &ver.as_str(), &asset.as_str());

    let download_path = download(&url, format!("/{}", update_data.file_name).as_str())?;

    decompress(download_path, home_dir.unwrap())
}

fn decompress(file_uri: String, home_dir: String) -> Result<Success, Message> {
    let file = File::open(file_uri)?;
    let tar = GzDecoder::new(file);
    let mut archive = Archive::new(tar);
    archive.unpack(format!("{}/{}/", home_dir, ".cvm"))?;

    Ok(Success {})
}