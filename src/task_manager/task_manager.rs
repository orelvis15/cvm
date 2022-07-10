#![allow(dead_code, unused_variables)]

use crate::config::remote_config::RemoteConfig;
use crate::context::context::Context;
use crate::task::task::{Success, Task};
use crate::message::message::Message;
use crate::term::log_level::LogLevel;

#[derive(Default)]
pub struct TaskManager {}

impl TaskManager {

    pub fn start(&self, mut task_queue: Vec<Box<dyn Task>>, config: &RemoteConfig, log_level: LogLevel, context: &mut Context) -> Result<Success, Message> {

        task_queue.reverse();
        while !task_queue.is_empty() {

            let mut task = task_queue.pop().unwrap();
            let prepare = prepare_task(&mut task, context, config)?;

            if prepare {
                context.term.print_task_message(task.get_type(), &log_level);
                run_task(&task, context, config)?;
                check_task(&task, context, config)?;
            }
        }
        Ok(Success {})
    }
}

fn prepare_task(task: &mut Box<dyn Task>, context: &mut Context, config: &RemoteConfig) -> Result<bool, Message> {
    task.prepare(context, config)
}

fn run_task(task: &Box<dyn Task>, context: &mut Context, config: &RemoteConfig) -> Result<Success, Message> {
    task.run(context, config)
}

fn check_task(task: &Box<dyn Task>, context: &mut Context, config: &RemoteConfig) -> Result<Success, Message> {
    task.check(context, config)
}