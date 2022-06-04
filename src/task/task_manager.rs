#![allow(dead_code, unused_variables)]

use owo_colors::OwoColorize;
use crate::task::task::{Success, Task};
use crate::env::Env;
use crate::task::cvm_error::CvmError;

pub fn start(task_queue: Vec<Box<dyn Task>>) -> Result<Success, CvmError> {
    let mut env: Env = Env::Empty();

    for task in task_queue {
        match run_task(&task, &mut env) {
            Ok(_) => {
                println!("{}",format!("the task finished successfully: {}", task.get_type()).green());
                match check_task(&task, &mut env) {
                    Ok(_) => {
                        println!("{}",format!("the check completed successfully: {}", task.get_type()).green());
                        continue;
                    },
                    Err(error) => {
                        return Err(error);
                    }
                }
            },
            Err(error) => {
                return Err(error);
            }
        }
    }
    Ok(Success{})
}

fn run_task(task: &Box<dyn Task>, env: &mut Env) -> Result<Success, CvmError> {
    println!("{}",format!("Task starts executing: {}", task.get_type()).yellow());
    task.run(env)
}

fn check_task(task: &Box<dyn Task>, env: &mut Env) -> Result<Success, CvmError>{
    println!("{}",format!("Start checking task: {}", task.get_type()).yellow());
    task.check(env)
}