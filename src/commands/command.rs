#![allow(dead_code, unused_variables)]

use clap::ArgMatches;
use crate::config::config::Config;
use crate::{Message, Success};

pub trait Command {
    fn start(command: &ArgMatches, config: &Config) -> Result<Success, Message>;
}