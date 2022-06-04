#![allow(dead_code, unused_variables)]

use std::str::FromStr;
use crate::config::config::Config;

#[derive(Debug, Clone, PartialEq)]
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
    CURRENT,
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
            Folder::CURRENT => "CURRENT",
        }
    }

    // Return folder name in config file for key passed for parameter
    pub fn get(key: Folder, config: &Config) -> &str {
        let folders = &config.structure_folder_item;
        folders.iter().find(|folder| folder.key == key.to_str()).unwrap().name.as_str()
    }
}

impl FromStr for Folder {

    type Err = ();

    fn from_str(input: &str) -> Result<Folder, Self::Err> {
        match input.to_uppercase().as_str() {
            "ROOT"  => Ok(Folder::ROOT),
            "SCRIPT"  => Ok(Folder::SCRIPT),
            "FILES"  => Ok(Folder::FILES),
            "DB"  => Ok(Folder::DB),
            "GUILDDB"  => Ok(Folder::GUILDDB),
            "SOCKETS"  => Ok(Folder::SOCKETS),
            "PRIV"  => Ok(Folder::PRIV),
            "TMP"  => Ok(Folder::TMP),
            "LOGS"  => Ok(Folder::LOGS),
            "BIN"  => Ok(Folder::BIN),
            "GIT"  => Ok(Folder::GIT),
            "CURRENT"  => Ok(Folder::CURRENT),
            _ => Ok(Folder::ROOT) // this case never execute
        }
    }
}