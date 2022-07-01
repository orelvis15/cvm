#![allow(dead_code, unused_variables)]

use crate::env::Env;
use crate::{Message, Success, Term};
use crate::config::remote_config::RemoteConfig;
use crate::task::task::Task;
use crate::task::task_type::TaskType;

pub struct EmptyTask {}

impl Task for EmptyTask {

    fn prepare(self: &mut Self, env: &mut Env, config: &RemoteConfig, term: &mut Term) -> Result<bool, Message> {
        Ok(true)
    }

    fn run(self: &Self, env: &mut Env, config: &RemoteConfig, term: &mut Term) -> Result<Success, Message> {
        Ok(Success{})
    }

    fn check(self: &Self, env: &mut Env, config: &RemoteConfig, term: &mut Term) -> Result<Success, Message> {
        Ok(Success{})
    }

    fn get_type(self: &Self) -> TaskType {
        TaskType::EmptyTask("".to_string())
    }
}