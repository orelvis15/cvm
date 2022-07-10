#![allow(dead_code, unused_variables)]

use clap::ArgMatches;
use crate::{Message, Success};
use crate::context::context::Context;

pub trait CommandStrategy {
    fn start(command: &ArgMatches, context: &mut Context) -> Result<Success, Message>;
}