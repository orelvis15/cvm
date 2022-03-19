use std::fmt::{Display, Formatter};
use clap::{Arg, App, ArgMatches};

pub fn command_config() -> ArgMatches {
    let network = Args::NETWORK.to_string();
    let version = Args::VERSION.to_string();

    return App::new("cvm")
        .version("0.0.1")
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
        .subcommand(App::new(Commands::INIT.to_string())
            .about("Start the environment to be able to build a Cardano node")
            .args(&[
                Arg::new(network.as_str())
                    .short('n')
                    .long("network")
                    .help("For which network do you want to download the configuration files [MAINNET | TESTNET]")
                    .takes_value(true)]
            )).subcommand(App::new(Commands::INSTALL.to_string())
        .about("Build the cardano node and make it available for use")
        .args(&[
            Arg::new(version.as_str())
                .short('v')
                .long("version")
                .help("Version of the cardano node that you want to install")
                .takes_value(true)
                .requires(true)]
        )).subcommand(App::new(Commands::USE.to_string())
        .about("Change the current cardano-node to the new version")
        .args(&[
            Arg::new(version.as_str())
                .short('v')
                .long("version")
                .help("Version of the cardano node that you want to use")
                .takes_value(true)
                .requires(true)]
        )).subcommand(App::new(Commands::LIST.to_string())
        .about("List all installed versions of cardano node"))
        .get_matches();
}

pub enum Commands {
    INIT,
    INSTALL,
    USE,
    LIST,
}

impl Display for Commands {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Commands::INIT => write!(f, "init"),
            Commands::INSTALL => write!(f, "install"),
            Commands::USE => write!(f, "use"),
            Commands::LIST => write!(f, "list"),
        }
    }
}

pub enum Args {
    NETWORK,
    VERSION,
}

impl Display for Args {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Args::NETWORK => write!(f, "network"),
            Args::VERSION => write!(f, "version")
        }
    }
}