#![allow(dead_code, unused_variables)]

use std::io::Write;
use std::fs;
use std::fs::File;
use std::os::unix::fs::PermissionsExt;
use crate::config::config::get_home_dir;
use crate::error::error::Message;
use crate::url_build;

pub fn download(url: &String, name: &str) -> Result<String, Message> {
    let home_dir = get_home_dir()?;

    let dir_tmp_main = format!("{}/.cvm/tmp/", home_dir);

    let mut path = String::new();
    path.push_str(&dir_tmp_main);
    path.push_str(&*name);

    let response = reqwest::blocking::get(&*url)?;

    let content = response.bytes();
    let mut file = File::create(&path)?;

    fs::set_permissions(&path, fs::Permissions::from_mode(0o755))?;

    file.write_all(&content.unwrap())?;

    Ok(path)
}

pub fn download_in_path(url: &String, path: String, name: String) -> Result<String, Message> {
    let file_path = url_build(vec![&path, &name], false);

    let response = reqwest::blocking::get(url)?;

    let content = response.bytes();

    let mut file = File::create(&file_path)?;

    fs::set_permissions(&path, fs::Permissions::from_mode(0o755))?;

    file.write_all(&content.unwrap())?;

    Ok(path)
}