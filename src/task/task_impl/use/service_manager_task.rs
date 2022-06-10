#![allow(dead_code, unused_variables)]

use std::process::ExitStatus;
use crate::env::Env;
use crate::Success;
use crate::config::config::Config;
use crate::error::error::Message;
use crate::task::task::Task;
use crate::task::task_type::TaskType;

pub struct ServicesManagerTask {
    pub input_data: ServicesAction,
}

impl Task for ServicesManagerTask {
    fn run(self: &Self, _env: &mut Env, config: &Config) -> Result<Success, Message> {
        sudo::escalate_if_needed().expect("Super user permissions are required");
        for services in &config.services_item {
            exec_action(&self.input_data, services.name.as_str())?;
        }
        Ok(Success {})
    }

    fn check(self: &Self, _env: &mut Env, config: &Config) -> Result<Success, Message> {
        Ok(Success {})
    }

    fn get_type(self: &Self) -> TaskType {
        TaskType::ServicesManager
    }
}

fn exec_action(input_data: &ServicesAction, service_name: &str) -> Result<ExitStatus, Message> {
    match input_data {
        ServicesAction::START => {
            Ok(systemctl::restart(service_name)?)
        }
        ServicesAction::STOP => {
            Ok(systemctl::stop(service_name)?)
        }
    }
}

pub enum ServicesAction{
    START,
    STOP
}