#![allow(dead_code, unused_variables)]

use crate::config::remote_config::RemoteConfig;
use crate::{Message, Term};
use crate::env::Env;
use crate::task::task_type::TaskType;

pub trait Task {
    fn run(self: &Self, env: &mut Env, config: &RemoteConfig, term: &mut Term) -> Result<Success, Message>;
    fn check(self: &Self, env: &mut Env, config: &RemoteConfig, term: &mut Term) -> Result<Success, Message>;
    fn get_type(self: &Self) -> TaskType;
}

#[derive(Debug, Clone)]
pub struct Success {}