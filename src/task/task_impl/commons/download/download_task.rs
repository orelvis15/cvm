#![allow(dead_code, unused_variables)]

use std::fs::File;
use std::io::Write;
use crate::context::context::Context;
use crate::{Message, Success, url_build};
use crate::config::remote_config::RemoteConfig;
use crate::context::storage::TaskOutputData;
use crate::task::task::{id_generator, Task};
use crate::task::task_impl::commons::download::dowload_io_data::{DownloadInputData, DownloadOutputData, ResolveDownloadInputData};
use crate::task::task_impl::task_input_data::TaskInputData;
use crate::task::task_type::TaskType;

#[derive(Default)]
pub struct DownloadTask {
    pub input_data: DownloadInputData,
    pub data: ResolveDownloadInputData
}

impl Task for DownloadTask {
    fn prepare(self: &mut Self, context: &mut Context, config: &RemoteConfig) -> Result<bool, Message> {
        let mut input_data = ResolveDownloadInputData::default();
        match &self.input_data.urls {
            TaskInputData::VecString(data) => {
                input_data.urls = data.to_owned();
            }
            _ => {}
        }
        match &self.input_data.urls {
            TaskInputData::String(data) => {
                input_data.folder = data.to_owned();
            }
            _ => {}
        }
        self.data = input_data;
        Ok(true)
    }

    fn run(self: &Self, context: &mut Context, config: &RemoteConfig) -> Result<Success, Message> {
        let mut files = vec![];
        for url in &self.data.urls {
            let file = download(url, &self.data.folder)?;
            files.push(file);
        }
        Ok(Success { value: TaskOutputData::Download(DownloadOutputData { files }) })
    }

    fn check(self: &Self, context: &mut Context, config: &RemoteConfig) -> Result<Success, Message> {
        Ok(Success::default())
    }

    fn get_type(self: &Self) -> TaskType {
        TaskType::Download(format!("Download files"))
    }

    fn get_id(self: &Self) -> String {
        let mut data = self.data.urls.to_owned();
        data.push(String::from(&self.data.folder));
        id_generator(&data)
    }
}

fn download(url: &String, folder: &String) -> Result<String, Message> {
    let file_name = get_name_from_url(url);
    let path = url_build(vec![folder, &file_name], false);
    let response = reqwest::blocking::get(url)?;
    let content = response.bytes()?;
    let mut file = File::create(&path)?;
    file.write_all(&content)?;
    Ok(path)
}

fn get_name_from_url(url: &String) -> String {
    let url_decode: Vec<&str> = url.split('/').collect();
    url_decode.last().unwrap().to_string()
}