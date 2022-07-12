#![allow(dead_code, unused_variables)]

use sha256::digest;
use crate::config::remote_config::RemoteConfig;
use crate::context::context::Context;
use crate::context::storage::{StructureData, TaskOutputData};
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
                let data = run_task(&task, context, config)?;
                check_task(&task, context, config)?;
                save_data(task_manager_id(&task_queue, &log_level),task.get_id(), data.value, context)
            }
        }
        Ok(Success::default())
    }
}

fn save_data(task_manager_id: String, task_id: String, data: TaskOutputData, context: &mut Context){
    context.storage.add(StructureData{
        task_manager_id,
        task_id,
        data
    });
}

fn task_manager_id(task_queue: &Vec<Box<dyn Task>>, log_level: &LogLevel) -> String{
    let mut tasks_ids: Vec<String> =  task_queue.iter().map(|task| task.get_id()).collect();
    tasks_ids.push(log_level.to_string());
    digest(tasks_ids.join(""))
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