#![allow(dead_code, unused_variables)]

use sha256::digest;
use crate::config::remote_config::RemoteConfig;
use crate::Message;
use crate::context::context::Context;
use crate::task::task_type::TaskType;
use crate::context::storage::TaskOutputData;

pub trait Task {
    /// Prepare and check
    /// Prepare all data for the task
    /// Check if is necesary run task
    /// * `context` - contains storage and term config
    /// * `RemoteConfig` - The config object
    fn prepare(self: &mut Self, context: &mut Context, config: &RemoteConfig) -> Result<bool, Message>;

    /// Run task
    /// * `context` - contains storage and term config
    /// * `RemoteConfig` - The config object
    fn run(self: &Self, context: &mut Context, config: &RemoteConfig) -> Result<Success, Message>;

    /// Check if task is run successful
    /// * `context` - contains storage and term config
    /// * `RemoteConfig` - The config object
    fn check(self: &Self, context: &mut Context, config: &RemoteConfig) -> Result<Success, Message>;

    /// Return taskType
    fn get_type(self: &Self) -> TaskType;

    fn get_id(self: &Self) -> String;
}

pub fn id_generator(data: &Vec<String>) -> String{
    digest(data.join(&"|"))
}

#[derive(Debug, Clone, Default)]
pub struct Success {
    pub value: TaskOutputData
}