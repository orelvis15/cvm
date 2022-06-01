use std::fmt;
use std::fmt::Formatter;
use crate::config::config::Config;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Folder {
    ROOT,
    SCRIPT,
    FILES,
    DB,
    GUILDDB,
    SOCKETS,
    PRIV,
    TMP,
    LOGS,
    BIN,
    GIT,
}

impl Folder {
    pub fn to_str(&self) -> &str {
        match self {
            Folder::ROOT => "ROOT",
            Folder::SCRIPT => "SCRIPT",
            Folder::FILES => "FILES",
            Folder::DB => "DB",
            Folder::GUILDDB => "GUILDDB",
            Folder::SOCKETS => "SOCKETS",
            Folder::PRIV => "PRIV",
            Folder::TMP => "TMP",
            Folder::LOGS => "LOGS",
            Folder::BIN => "BIN",
            Folder::GIT => "GIT",
        }
    }

    // Return folder name in config file for key passed for parameter
    pub fn get(key: Folder, config: &Config) -> &str {
        let folders = &config.structure_folder_item;
        folders.iter().find(|folder| folder.key == key.to_str()).unwrap().name.as_str()
    }
}