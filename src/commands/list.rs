use clap::ArgMatches;
use owo_colors::OwoColorize;
use walkdir::WalkDir;
use crate::{Message, Success};
use crate::utils::version_utils::verify_version;
use crate::config::config::{get_config, get_home_dir, get_project_dir};
use crate::config::enviroment::get_env;

const BIN_FOLDER: &str = "bin";

pub fn start(_command: &ArgMatches) -> Result<Success, Message> {
    let config = get_config();
    if let Err(error) = config {
        return Err(error);
    }

    let project_dir = get_project_dir();

    let enviromet = get_env();
    if let Err(error) = enviromet {
        return Err(error);
    }

    let bin_folder = format!("{}/{}/{}", project_dir, &config.as_ref().unwrap().workspace.workspace_folder, BIN_FOLDER);

    for entry in WalkDir::new(bin_folder) {
        let entry = entry.unwrap();
        let name = entry.file_name().to_str().unwrap().to_string();
        if entry.path().is_dir() && verify_version(name.as_str()) {
            if enviromet.clone().unwrap().active_version == name {
                println!("{}{}", "*".green(),name.green());
            } else {
                println!("{}", name.red());
            }
        };
    }
    Ok(Success {})
}