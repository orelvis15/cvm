#![allow(dead_code, unused_variables)]

use clap::ArgMatches;
use owo_colors::OwoColorize;
use walkdir::WalkDir;
use crate::{CvmError, Success, url_build};
use crate::utils::version_utils::{read_version, verify_version};
use crate::config::config::{Config, get_project_dir};
use crate::task::folders::Folder;

pub fn start(_command: &ArgMatches, config: &Config) -> Result<Success, CvmError> {
    let project_dir = get_project_dir();
    let bin_folder = url_build(vec![project_dir.as_str(), Folder::get(Folder::ROOT, &config), Folder::get(Folder::BIN, &config)], false);
    let current_folder = url_build(vec![bin_folder.as_str(), Folder::get(Folder::CURRENT, &config)], false);

    let current_version = read_version(&current_folder);

    for entry in WalkDir::new(bin_folder) {
        let entry = entry.unwrap();
        let name = entry.file_name().to_str().unwrap().to_string();
        if entry.path().is_dir() && verify_version(name.as_str()) {
            if name != current_version {
                println!("{}", name.red());
            } else {
                println!("{}{}", "-> ".yellow(), name.green());
            }
        };
    }
    Ok(Success {})
}