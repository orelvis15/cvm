#![allow(dead_code, unused_variables)]

use clap::ArgMatches;
use crossterm::style::Stylize;
use fs_extra::dir::get_size;
use walkdir::WalkDir;
use crate::{CommandStrategy, config, Message, Success};
use crate::utils::version_utils::verify_version;
use crate::config::state_config::get_state;
use crate::utils::folders::Folder;

pub struct List{}

impl CommandStrategy for List{
    fn start(command: &ArgMatches) -> Result<Success, Message> {

        let config = config::remote_config::get_remote_config()?;

        let bin_folder = Folder::get_path(Folder::BIN, &config);
        let current_folder = Folder::get_path(Folder::CURRENT, &config);

        let current_version = get_state()?.r#use.version;

        for entry in WalkDir::new(bin_folder) {
            let entry = entry.unwrap();
            let name = entry.file_name().to_str().unwrap().to_string();
            if entry.path().is_dir() && verify_version(name.as_str()).is_ok() {

                let size = get_size(entry.path()).unwrap() / 1024 / 1024;
                let size_format = format!("{} MB", size);

                if name != current_version {
                    print(format!("{}  {}", name.red(), size_format.red()));
                } else {
                    print(format!("{}{}  {}", "-> ".yellow(), name.green(), size_format.green()));
                }
            };
        }
        Ok(Success {})
    }
}

fn print(message: String){
    println!("{}", message);
}