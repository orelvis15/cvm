#![allow(dead_code, unused_variables)]

use crate::context::context::Context;
use crate::{Message, Success};
use crate::config::remote_config::RemoteConfig;
use crate::task::task::Task;
use crate::task::task_type::TaskType;

pub struct EmptyTask {}

impl Task for EmptyTask {

    fn prepare(self: &mut Self, context: &mut Context, config: &RemoteConfig) -> Result<bool, Message> {
        Ok(true)
    }

    fn run(self: &Self, context: &mut Context, config: &RemoteConfig) -> Result<Success, Message> {
        Ok(Success{})
    }

    fn check(self: &Self, context: &mut Context, config: &RemoteConfig) -> Result<Success, Message> {
        Ok(Success{})
    }

    fn get_type(self: &Self) -> TaskType {
        TaskType::EmptyTask("".to_string())
    }
}