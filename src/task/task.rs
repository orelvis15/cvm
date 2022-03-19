use owo_colors::OwoColorize;
use crate::env::Env;
use crate::task::task_type::TaskType;

pub trait Task {
    fn run(self: &Self, env: &mut Env) -> Result<Success, Error>;
    fn check(self: &Self, env: &mut Env) -> Result<Success, Error>;
    fn get_type(self: &Self) -> TaskType;
}

#[derive(Debug, Clone, Default)]
pub struct Error {
    pub code: u8,
    pub message: String,
    pub task: String,
    pub stack: Vec<String>,
}

impl Error {
    pub fn to_string(self: Self) -> String{

        let mut stack_out = String::new();
        for stack in self.stack{
            stack_out.push_str(stack.as_str());
            stack_out.push_str("\n");
        }
        let output = format!("Code: {}\nError: {}\nTask: {}\nStack: {}\n", self.code, self.message, self.task, stack_out);
        format!("{}", output.red())
    }
}

#[derive(Debug, Clone)]
pub struct Success {}