#![allow(dead_code, unused_variables)]

use crate::config::remote_config::RemoteConfig;
use crate::{Message, Term};
use crate::env::Env;
use crate::task::task_type::TaskType;

pub trait Task {

    /// Prepare and check
    /// Prepare all data for the task
    /// Check if is necesary run task
    /// * `env` - Enviroment with all tasks data
    /// * `RemoteConfig` - The config object
    /// * `Term` - term Configuration
    fn prepare(self: &mut Self, env: &mut Env, config: &RemoteConfig, term: &mut Term) -> Result<bool, Message>;

    /// Run task
    /// * `env` - Enviroment with all tasks data
    /// * `RemoteConfig` - The config object
    /// * `Term` - term Configuration
    fn run(self: &Self, env: &mut Env, config: &RemoteConfig, term: &mut Term) -> Result<Success, Message>;

    /// Check if task is run successful
    /// * `env` - Enviroment with all tasks data
    /// * `RemoteConfig` - The config object
    /// * `Term` - term Configuration
    fn check(self: &Self, env: &mut Env, config: &RemoteConfig, term: &mut Term) -> Result<Success, Message>;

    /// Return taskType
    fn get_type(self: &Self) -> TaskType;
}

#[derive(Debug, Clone)]
pub struct Success {}