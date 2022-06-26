#![allow(dead_code, unused_variables)]

use crate::config::remote_config::RemoteConfig;
use crate::task::task::{Success, Task};
use crate::env::Env;
use crate::error::message::Message;
use crate::Term;
use crate::term::log_level::LogLevel;

pub struct TaskManager {}

impl TaskManager {

    pub fn start(&self, mut task_queue: Vec<Box<dyn Task>>, config: &RemoteConfig, term: &mut Term, log_level: LogLevel) -> Result<Success, Message> {
        let mut env: Env = Env::Empty();

        task_queue.reverse();
        while !task_queue.is_empty() {

            let task = task_queue.pop().unwrap();
            term.print_task_message(task.get_type(), &log_level);

            if let Err(error) = run_task(&task, &mut env, config, term) {
                return Err(error);
            }

            if let Err(error) = check_task(&task, &mut env, config, term) {
                return Err(error);
            }
        }

        Ok(Success {})
    }
}

fn run_task(task: &Box<dyn Task>, env: &mut Env, config: &RemoteConfig, term: &mut Term) -> Result<Success, Message> {
    task.run(env, config, term)
}

fn check_task(task: &Box<dyn Task>, env: &mut Env, config: &RemoteConfig, term: &mut Term) -> Result<Success, Message> {
    task.check(env, config, term)
}