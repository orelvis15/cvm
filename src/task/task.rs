#![allow(dead_code, unused_variables)]

use crate::config::remote_config::RemoteConfig;
use crate::Message;
use crate::context::context::Context;
use crate::task::task_type::TaskType;

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
}

#[derive(Debug, Clone)]
pub struct Success {}