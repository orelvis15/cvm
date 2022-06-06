use std::fmt::{Display, Formatter};
use clap::{Arg, ArgMatches, Command};
use crate::config::config::CommandItem;
use crate::{CvmError, EmptyTask, Error, Success};

const VERSION: &str = "0.0.1";

pub fn command_config() -> ArgMatches {
    let network = Args::NETWORK._to_string();
    let version = Args::VERSION._to_string();

    return Command::new("cvm")
        .version(VERSION)
        .about("Version manager for cardano node")
        .author("Orelvis L. <orelvis15@gmail.com>")
        .args(&[
            Arg::new("help")
                .short('h')
                .long("help"),
            Arg::new("version")
                .short('v')
                .long("version")
        ])
        .subcommand(Command::new(CommandsConfig::INIT.to_string())
            .about("Start the environment to be able to build a Cardano node")
            .args(&[
                Arg::new(network)
                    .short('n')
                    .long("network")
                    .help("For which network do you want to download the configuration files [MAINNET | TESTNET]")
                    .takes_value(true)]
            )).subcommand(Command::new(CommandsConfig::INSTALL.to_string())
        .about("Build the cardano node and make it available for use")
        .args(&[
            Arg::new(version)
                .short('v')
                .long("version")
                .help("Version of the cardano node that you want to install")
                .takes_value(true)]
        )).subcommand(Command::new(CommandsConfig::USE.to_string())
        .about("Change the current cardano-node to the new version")
        .args(&[
            Arg::new(version)
                .short('v')
                .long("version")
                .help("Version of the cardano node that you want to use")
                .takes_value(true)]
        )).subcommand(Command::new(CommandsConfig::LIST.to_string())
        .about("List all installed versions of cardano node"))
        .subcommand(Command::new(CommandsConfig::UPDATE.to_string())
            .about("Update to the new version of CVM if it exists"))
        .get_matches();
}

pub fn get_version() -> String {
    VERSION.to_string()
}

pub enum CommandsConfig {
    INIT,
    INSTALL,
    USE,
    LIST,
    UPDATE,
}

impl Display for CommandsConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandsConfig::INIT => write!(f, "init"),
            CommandsConfig::INSTALL => write!(f, "install"),
            CommandsConfig::USE => write!(f, "use"),
            CommandsConfig::LIST => write!(f, "list"),
            CommandsConfig::UPDATE => write!(f, "update"),
        }
    }
}

impl CommandsConfig {
    pub fn get_key(&self) -> &str {
        match &self {
            CommandsConfig::INIT => { "INIT" }
            CommandsConfig::INSTALL => { "INSTALL" }
            CommandsConfig::USE => { "USE" }
            CommandsConfig::LIST => { "LIST" }
            CommandsConfig::UPDATE => { "UPDATE" }
        }
    }

    pub fn is_enable(&self, commands_list: &Vec<CommandItem>) -> Result<Success, CvmError> {
        let is_enable = commands_list.iter().find(|cmd| {
            &cmd.key == &self.get_key().to_string()
        }).unwrap().enable;

        if is_enable {
           return Ok(Success{})
        };

        Err(CvmError::CommandNotFound(Error {
            message: "The command has been temporarily disabled to avoid errors".to_string(),
            task: EmptyTask("".to_string()),
            stack: vec![],
        }))
    }
}

pub enum Args {
    NETWORK,
    VERSION,
}

impl Args {
    pub fn _to_string(&self) -> &str {
        match self {
            Args::NETWORK => { "network" }
            Args::VERSION => { "version" }
        }
    }
}

impl Display for Args {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Args::NETWORK => write!(f, "network"),
            Args::VERSION => write!(f, "version")
        }
    }
}