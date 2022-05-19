use std::env;
use crate::env::Env;
use crate::task::task::{Message, Success, Task};
use crate::task::task_type::TaskType;
use globalenv::set_var;
use crate::MessageType;

pub struct SetEnvironmentVariable {
    pub input_data: SetEnvironmentVariableInput,
}

const PATH_KEY: &str = "PATH";

impl Task for SetEnvironmentVariable {
    fn run(self: &Self, _env: &mut Env) -> Result<Success, Message> {
        if self.input_data.key == PATH_KEY {
            let set_result = set_path(&self.input_data.value);
            if let Err(_) = set_result {
                return Err(show_error());
            }
        } else {
            let set_result = set_var(&self.input_data.key, &self.input_data.value);
            if let Err(_) = set_result {
                return Err(show_error());
            }
        }
        Ok(Success {})
    }

    fn check(self: &Self, _env: &mut Env) -> Result<Success, Message> {
        Ok(Success {})
    }

    fn get_type(self: &Self) -> TaskType {
        TaskType::SetEnviromentVariable
    }
}

fn set_path(value: &String) -> Result<Success, Message> {
    let path_result = env::var(PATH_KEY);
    if let Ok(path_value) = path_result {
        if !path_value.contains(value) {
            let set_result = set_var(PATH_KEY, format!("$PATH:{}", value).as_str());

            if let Err(_) = set_result {
                return Err(show_error());
            }
        }
    }
    Ok(Success {})
}

pub struct SetEnvironmentVariableInput {
    pub key: String,
    pub value: String,
}

fn show_error() -> Message {
    return Message {
        code: 0,
        message: "Error creating environment variable".to_string(),
        kind: MessageType::Error,
        task: "".to_string(),
        stack: vec![],
    };
}