#![allow(dead_code, unused_variables)]

use clap::ArgMatches;
use crate::{Message, Success};

pub trait CommandStrategy {
    fn start(command: &ArgMatches) -> Result<Success, Message>;
}