
use owo_colors::OwoColorize;
use crate::task::task::{Error, Success, Task};
use crate::env::Env;

pub fn start(task_list: Vec<Box<dyn Task>>) -> Result<Success, Error> {
    let mut env: Env = Env::Empty();

    for task in task_list {
        match run_task(&task, &mut env) {
            Ok(_) => {
                println!("{}",format!("the task finished successfully: {}", task.get_type()).green());
                match check_task(&task, &mut env) {
                    Ok(_) => {
                        println!("{}",format!("the check completed successfully: {}", task.get_type()).green());
                        continue;
                    },
                    Err(error) => {
                        return Result::Err(error);
                    }
                }
            },
            Err(error) => {
                return Result::Err(error);
            }
        }
    }
    Result::Ok(Success{})
}

fn run_task(task: &Box<dyn Task>, env: &mut Env) -> Result<Success, Error> {
    println!("{}",format!("Task starts executing: {}", task.get_type()).yellow());
    task.run(env)
}

fn check_task(task: &Box<dyn Task>, env: &mut Env) -> Result<Success, Error>{
    println!("{}",format!("Start checking task: {}", task.get_type()).yellow());
    task.check(env)
}