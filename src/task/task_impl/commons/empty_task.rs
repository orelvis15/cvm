#![allow(dead_code, unused_variables)]

use crate::env::Env;
use crate::{Message, Success};
use crate::config::config::Config;
use crate::task::task::Task;
use crate::task::task_type::TaskType;

pub struct EmptyTask {}

impl Task for EmptyTask {
    fn run(self: &Self, env: &mut Env, config: &Config) -> Result<Success, Message> {
        Ok(Success{})
    }

    fn check(self: &Self, env: &mut Env, config: &Config) -> Result<Success, Message> {
        Ok(Success{})
    }

    fn get_type(self: &Self) -> TaskType {
        TaskType::EmptyTask("".to_string())
    }
}