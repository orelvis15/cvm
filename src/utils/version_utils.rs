#![allow(dead_code, unused_variables)]

use std::fs;
use std::fs::File;
use std::path::Path;
use regex::Regex;
use serde::{Serialize, Deserialize};
use crate::error::message::{Message, Error};
use crate::task::task_type::TaskType::EmptyTask;
use crate::url_build;

pub const LATEST: &str = "latest";
const USER_AGENT: &str = "cvm";
const VERSION_FILE: &str = "version";

pub fn verify_version(version: &str) -> bool {
    let regex = Regex::new(r"^(\d+\.)?(\d+\.)?(\*|\d+)$").unwrap();
    regex.is_match(version) || version == LATEST
}

pub fn get_last_tag(url: &String) -> Result<String, Message> {
    let client = reqwest::blocking::Client::builder().user_agent(USER_AGENT).build();
    if let Ok(client) = client {
        if let Ok(response) = client.get(url).send() {
            if let Ok(test) = response.text() {
                let tag: Tag = serde_json::from_str(test.as_str()).unwrap();
                return Ok(tag.tag_name);
            }
        }
    }
    return Err(Message::CheckCardanoVersion(Error {
        message: "Error checking latest cardano node tag".to_string(),
        task: EmptyTask("".to_string()),
        stack: vec![],
    }));
}


pub fn write_version(current_folder: &String, current_version: &String) {
    let file_path = url_build(vec![current_folder, &VERSION_FILE.to_string()], false);
    let path = Path::new(file_path.as_str());

    if !path.exists() {
        let _ = File::create(path);
    }
    let _ = fs::write(path, current_version);
}

pub fn read_version(current_folder: &String) -> String {
    let file_path = url_build(vec![current_folder, &VERSION_FILE.to_string()], false);
    let path = Path::new(file_path.as_str());

    let value = "".to_string();

    if path.exists() {
        return fs::read_to_string(path).unwrap_or(value);
    };

    value
}

#[derive(Serialize, Deserialize)]
pub struct Tag {
    #[serde(rename = "tag_name")]
    pub tag_name: String,
}
