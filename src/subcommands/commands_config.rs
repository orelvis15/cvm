use std::fmt::{Display, Formatter};
use clap::{Arg, ArgMatches, Command};

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
            .arg(get_arg_force_init())
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
            .about("Remove cardano-node binaries from {version}")
            .arg(get_arg_version())
        )
        .subcommand(Command::new(CommandsConfig::CONFIG.to_string())
            .subcommand(Command::new(CommandsConfig::UPDATE.to_string())
                .about("Update configuration files and scripts to their latest released version as long as they have not been modified by someone")
                .arg(get_arg_force_config())
            )
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

fn get_arg_force_init() -> Arg<'static> {
    Arg::new(Args::FORCE._to_string())
        .short('f')
        .long(Args::FORCE._to_string())
        .help("Force rerun all initialization tasks")
}

fn get_arg_force_config() -> Arg<'static> {
    Arg::new(Args::FORCE._to_string())
        .short('f')
        .long(Args::FORCE._to_string())
        .help("Force update all configuration files and scripts to their latest released version")
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
    CONFIG,
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
            CommandsConfig::CONFIG => write!(f, "config"),
        }
    }
}

pub enum Args {
    NETWORK,
    VERSION,
    LATEST,
    FORCE,
}

impl Args {
    pub fn _to_string(&self) -> &str {
        match self {
            Args::NETWORK => { "network" }
            Args::VERSION => { "version" }
            Args::LATEST => { "latest" }
            Args::FORCE => { "force" }
        }
    }
}

impl Display for Args {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Args::NETWORK => write!(f, "network"),
            Args::VERSION => write!(f, "version"),
            Args::LATEST => write!(f, "latest"),
            Args::FORCE => write!(f, "force"),
        }
    }
}