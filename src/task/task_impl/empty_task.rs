#![allow(dead_code, unused_variables)]

use crate::env::Env;
use crate::{CvmError, Success};
use crate::task::task::Task;
use crate::task::task_type::TaskType;

pub struct EmptyTask {}

impl Task for EmptyTask {
    fn run(self: &Self, env: &mut Env) -> Result<Success, CvmError> {
        Ok(Success{})
    }

    fn check(self: &Self, env: &mut Env) -> Result<Success, CvmError> {
        Ok(Success{})
    }

    fn get_type(self: &Self) -> TaskType {
        TaskType::EmptyTask("".to_string())
    }
}