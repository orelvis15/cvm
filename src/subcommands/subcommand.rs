#![allow(dead_code, unused_variables)]

use clap::ArgMatches;
use crate::config::config::Config;
use crate::{Message, Success, Term};

pub trait Command {
    fn start(command: &ArgMatches, config: &Config, term: &mut Term) -> Result<Success, Message>;
}