#![allow(dead_code, unused_variables)]

use crate::config::remote_config::RemoteConfig;
use crate::task::task::{Success, Task};
use crate::env::Env;
use crate::message::message::Message;
use crate::Term;
use crate::term::log_level::LogLevel;

#[derive(Default)]
pub struct TaskManager {}

impl TaskManager {

    pub fn start(&self, mut task_queue: Vec<Box<dyn Task>>, config: &RemoteConfig, term: &mut Term, log_level: LogLevel) -> Result<Success, Message> {
        let mut env: Env = Env::Empty();

        task_queue.reverse();
        while !task_queue.is_empty() {

            let mut task = task_queue.pop().unwrap();
            let prepare = prepare_task(&mut task, &mut env, config, term)?;

            if prepare {
                term.print_task_message(task.get_type(), &log_level);
                run_task(&task, &mut env, config, term)?;
                check_task(&task, &mut env, config, term)?;
            }
        }
        Ok(Success {})
    }
}

fn prepare_task(task: &mut Box<dyn Task>, env: &mut Env, config: &RemoteConfig, term: &mut Term) -> Result<bool, Message> {
    task.prepare(env, config, term)
}

fn run_task(task: &Box<dyn Task>, env: &mut Env, config: &RemoteConfig, term: &mut Term) -> Result<Success, Message> {
    task.run(env, config, term)
}

fn check_task(task: &Box<dyn Task>, env: &mut Env, config: &RemoteConfig, term: &mut Term) -> Result<Success, Message> {
    task.check(env, config, term)
}