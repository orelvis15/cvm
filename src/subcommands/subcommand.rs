#![allow(dead_code, unused_variables)]

use clap::ArgMatches;
use crate::config::remote_config::RemoteConfig;
use crate::{Message, Success, Term};

pub trait Command {
    fn start(command: &ArgMatches, config: &RemoteConfig, term: &mut Term) -> Result<Success, Message>;
}