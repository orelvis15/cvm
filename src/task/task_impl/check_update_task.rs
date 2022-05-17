use std::fs;
use std::fs::File;
use flate2::Compression;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use tar::Archive;
use crate::env::Env;
use crate::{Error, Success, url_build};
use crate::config::config::{get_config, get_home_dir};
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
    fn run(self: &Self, env: &mut Env) -> Result<Success, Error> {
        let config = get_config();
        if let Err(_) = config {
            return Err(error());
        }

        if &config.as_ref().unwrap().general.last_cvm_version <= &self.input_data.version {
            return Err(Error {
                code: 0,
                message: "You already have the latest version".to_string(),
                task: TaskType::CheckUpdate.to_string(),
                stack: vec![],
            });
        };

        download_and_copy_version(&config.as_ref().unwrap().general.last_cvm_version, &config.as_ref().unwrap().init.git_assets)
    }

    fn check(self: &Self, env: &mut Env) -> Result<Success, Error> {
        Ok(Success {})
    }

    fn get_type(self: &Self) -> TaskType {
        TaskType::CheckUpdate
    }
}

fn download_and_copy_version(version: &String, base_url: &String) -> Result<Success, Error> {
    let home_dir = get_home_dir();
    if let Err(error) = home_dir {
        return Err(error);
    }

    let ver = format!("v{}", version);
    let asset = format!("cvm-{}.tar.gz", std::env::consts::ARCH);
    let url = format!("{}/{}/{}", &base_url.as_str(), &ver.as_str(), &asset.as_str());

    let download_path = download(url, "/cvm.tar.gz");

    if let Err(error) = &download_path {
        return Err(error.clone());
    };

    decompress(download_path.unwrap(), home_dir.unwrap())
}

fn decompress(file_uri: String, home_dir: String) -> Result<Success, Error>{
    let file = File::open(file_uri);

    if let Err(_) = &file {
        return Err(Error{
            code: 0,
            message: "error abriendo archivo".to_string(),
            task: "".to_string(),
            stack: vec![]
        })
    };

    let tar = GzDecoder::new(file.unwrap());
    let mut archive = Archive::new(tar);


    let result = archive.unpack(format!("{}/{}", home_dir, ".cvm/"));

    if let Err(error) = &result {
        return Err(Error{
            code: 0,
            message: "error descomprimiendo archivo".to_string(),
            task: "".to_string(),
            stack: vec![error.to_string()]
        })
    };

    Ok(Success{})
}

fn error() -> Error {
    Error {
        code: 0,
        message: "Error trying to update".to_string(),
        task: "".to_string(),
        stack: vec![],
    }
}