#![allow(dead_code, unused_variables)]

use owo_colors::OwoColorize;
use crate::config::config::Config;
use crate::task::task::{Success, Task};
use crate::env::Env;
use crate::task::cvm_error::CvmError;

pub struct TaskManager {}

impl TaskManager {
    pub fn start(task_queue: Vec<Box<dyn Task>>, config: &Config) -> Result<Success, CvmError> {

        let mut env: Env = Env::Empty();

        for task in task_queue {
            match run_task(&task, &mut env, config) {
                Ok(_) => {
                    println!("{}", format!("the task finished successfully: {}", task.get_type()).green());
                    match check_task(&task, &mut env, config) {
                        Ok(_) => {
                            println!("{}", format!("the check completed successfully: {}", task.get_type()).green());
                            continue;
                        }
                        Err(error) => {
                            return Err(error);
                        }
                    }
                }
                Err(error) => {
                    return Err(error);
                }
            }
        }
        Ok(Success {})
    }
}

fn run_task(task: &Box<dyn Task>, env: &mut Env, config: &Config) -> Result<Success, CvmError> {
    println!("{}", format!("Task starts executing: {}", task.get_type()).yellow());
    task.run(env, config)
}

fn check_task(task: &Box<dyn Task>, env: &mut Env, config: &Config) -> Result<Success, CvmError> {
    println!("{}", format!("Start checking task: {}", task.get_type()).yellow());
    task.check(env, config)
}