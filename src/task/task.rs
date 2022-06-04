#![allow(dead_code, unused_variables)]

use crate::config::config::Config;
use crate::CvmError;
use crate::env::Env;
use crate::task::task_type::TaskType;

pub trait Task {
    fn run(self: &Self, env: &mut Env, config: &Config) -> Result<Success, CvmError>;
    fn check(self: &Self, env: &mut Env, config: &Config) -> Result<Success, CvmError>;
    fn get_type(self: &Self) -> TaskType;
}

#[derive(Debug, Clone)]
pub struct Success {}