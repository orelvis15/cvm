use std::fmt::{Display, Formatter};
use clap::{Arg, ArgMatches, Command};
use crate::config::remote_config::CommandItem;
use crate::{Message, EmptyTask, Error, Success};

pub fn command_config() -> ArgMatches {

    return Command::new("cvm")
        .version(env!("CARGO_PKG_VERSION"))
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
            .arg(get_arg_network())
        )
        .subcommand(Command::new(CommandsConfig::INSTALL.to_string())
            .about("Build the cardano node and make it available for use")
            .arg(get_arg_version())
        )
        .subcommand(Command::new(CommandsConfig::USE.to_string())
            .about("Change the current cardano-node to the new version")
            .arg(get_arg_version())
        )
        .subcommand(Command::new(CommandsConfig::REMOVE.to_string())
            .about("Remove cardano-node binaries from this version")
            .arg(get_arg_version())
        )
        .subcommand(Command::new(CommandsConfig::CLEAN.to_string())
            .about("Remove temporary and build files"))
        .subcommand(Command::new(CommandsConfig::LIST.to_string())
            .about("List all installed versions of cardano node"))
        .subcommand(Command::new(CommandsConfig::UPDATE.to_string())
            .about("Update to the new version of CVM if it exists"))
        .subcommand(Command::new(CommandsConfig::START.to_string())
            .about("Start cardano node services"))
        .subcommand(Command::new(CommandsConfig::STOP.to_string())
            .about("Stop cardano node services"))
        .get_matches();
}

fn get_arg_version() -> Arg<'static> {
    Arg::new(Args::VERSION._to_string()).default_value(Args::LATEST._to_string()).takes_value(true)
}

fn get_arg_network() -> Arg<'static> {
    Arg::new(Args::NETWORK._to_string()).default_value(Args::LATEST._to_string()).takes_value(true)
}

pub enum CommandsConfig {
    INIT,
    INSTALL,
    USE,
    REMOVE,
    LIST,
    UPDATE,
    START,
    STOP,
    CLEAN,
}

impl Display for CommandsConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandsConfig::INIT => write!(f, "init"),
            CommandsConfig::INSTALL => write!(f, "install"),
            CommandsConfig::USE => write!(f, "use"),
            CommandsConfig::REMOVE => write!(f, "remove"),
            CommandsConfig::LIST => write!(f, "list"),
            CommandsConfig::UPDATE => write!(f, "update"),
            CommandsConfig::START => write!(f, "start"),
            CommandsConfig::STOP => write!(f, "stop"),
            CommandsConfig::CLEAN => write!(f, "clean"),
        }
    }
}

impl CommandsConfig {
    pub fn get_key(&self) -> &str {
        match &self {
            CommandsConfig::INIT => { "INIT" }
            CommandsConfig::INSTALL => { "INSTALL" }
            CommandsConfig::USE => { "USE" }
            CommandsConfig::REMOVE => { "REMOVE" }
            CommandsConfig::LIST => { "LIST" }
            CommandsConfig::UPDATE => { "UPDATE" }
            CommandsConfig::START => { "START" }
            CommandsConfig::STOP => { "STOP" }
            CommandsConfig::CLEAN => { "CLEAN" }
        }
    }

    pub fn is_enable(&self, commands_list: &Vec<CommandItem>) -> Result<Success, Message> {
        let is_enable = commands_list.iter().find(|cmd| {
            &cmd.key == &self.get_key().to_string()
        }).unwrap().enable;

        if is_enable {
            return Ok(Success {});
        };

        Err(Message::CommandNotFound(Error {
            message: "The command has been temporarily disabled to avoid errors".to_string(),
            task: EmptyTask("".to_string()),
            stack: vec![],
        }))
    }
}

pub enum Args {
    NETWORK,
    VERSION,
    LATEST,
}

impl Args {
    pub fn _to_string(&self) -> &str {
        match self {
            Args::NETWORK => { "network" }
            Args::VERSION => { "version" }
            Args::LATEST => {"latest"}
        }
    }
}

impl Display for Args {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Args::NETWORK => write!(f, "network"),
            Args::VERSION => write!(f, "version"),
            Args::LATEST => write!(f, "latest"),
        }
    }
}