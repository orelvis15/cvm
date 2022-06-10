#![allow(dead_code, unused_variables)]

use clap::ArgMatches;
use owo_colors::OwoColorize;
use walkdir::WalkDir;
use crate::{CvmError, Success};
use crate::utils::version_utils::{read_version, verify_version};
use crate::config::config::Config;
use crate::task::folders::Folder;

pub fn start(_command: &ArgMatches, config: &Config) -> Result<Success, CvmError> {
    let bin_folder = Folder::get_path(Folder::BIN, &config);
    let current_folder = Folder::get_path(Folder::CURRENT, &config);

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