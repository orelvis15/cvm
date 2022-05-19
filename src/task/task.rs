use owo_colors::OwoColorize;
use crate::env::Env;
use crate::task::message_type::MessageType;
use crate::task::task_type::TaskType;

pub trait Task {
    fn run(self: &Self, env: &mut Env) -> Result<Success, Message>;
    fn check(self: &Self, env: &mut Env) -> Result<Success, Message>;
    fn get_type(self: &Self) -> TaskType;
}

#[derive(Debug, Clone)]
pub struct Message {
    pub code: u8,
    pub message: String,
    pub kind: MessageType,
    pub task: String,
    pub stack: Vec<String>,
}

impl Message {
    pub fn to_string(self: Self) -> String {
        return match self.kind {
            MessageType::Error => { self.print_error() }
            MessageType::Warning => { self.print_warning() }
            MessageType::Info => { self.print_info() }
        };
    }

    fn print_error(self: Message) -> String {
        let mut stack_out = String::new();
        for stack in self.stack {
            stack_out.push_str(stack.as_str());
            stack_out.push_str("\n");
        }

        let mut output = format!("{}", self.message);

        if cfg!(debug_assertions) {
            output = format!("Code: {}\nError: {}\nTask: {}\nStack: {}\n", self.code, self.message, self.task, stack_out);
        }

        format!("{}", output.red())
    }

    fn print_warning(self: Message) -> String {
        let output = format!("{}", self.message);
        format!("{}", output.yellow())
    }

    fn print_info(self: Message) -> String {
        let output = format!("{}", self.message);
        format!("{}", output.blue())
    }
}

#[derive(Debug, Clone)]
pub struct Success {}

