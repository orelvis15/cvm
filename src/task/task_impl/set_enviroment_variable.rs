use std::env;
use crate::env::Env;
use crate::task::task::{Message, Success, Task};
use crate::task::task_type::TaskType;
use globalenv::set_var;
use owo_colors::OwoColorize;

pub struct SetEnvironmentVariable {
    pub input_data: SetEnvironmentVariableInput,
}

const PATH_KEY: &str = "PATH";

impl Task for SetEnvironmentVariable {
    fn run(self: &Self, _env: &mut Env) -> Result<Success, Message> {
        if self.input_data.key == PATH_KEY {
            set_path(&self.input_data.value)
        }else {
            set_var(&self.input_data.key, &self.input_data.value);
        }
        Result::Ok(Success {})
    }

    fn check(self: &Self, _env: &mut Env) -> Result<Success, Message> {
        Result::Ok(Success {})
    }

    fn get_type(self: &Self) -> TaskType {
        TaskType::SetEnviromentVariable
    }
}

fn set_path(value: &String) {
    let path_result = env::var(PATH_KEY);
    if let Ok(path_value) = path_result {
        if !path_value.contains(value) {
            set_var(PATH_KEY, format!("$PATH:{}", value).as_str());
        }
    }
}

pub struct SetEnvironmentVariableInput {
    pub key: String,
    pub value: String,
}