use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};
use std::thread;
use owo_colors::OwoColorize;

use crate::env::Env;
use crate::task::task::{Error, Success, Task};
use crate::task::task_type::TaskType;

pub struct RunCommandTask {
    pub input_data: RunCommandInputData,
}

pub struct RunCommandOutputData {
    pub tag: String,
    pub result: Result<Success, Error>,
}

impl RunCommandOutputData
{
    fn get_tag(&self) -> &String {
        &self.tag
    }

    fn get_data(&self) -> &RunCommandOutputData {
        self
    }
}

#[derive(Default, Clone)]
pub struct RunCommandInputData {
    pub command: String,
    pub args: Vec<String>,
    pub current_dir: String,
}

impl RunCommandInputData {
    pub fn to_string(self: Self) -> String{

        let mut command = String::new();
        command.push_str(self.command.as_str());
        command.push_str(" ");

        for arg in self.args {
            command.push_str(arg.as_str());
            command.push_str(" ");
        }
        format!("Command: {}\nCurrent dir: {}", command, self.current_dir).blue().to_string()
    }
}

impl Task for RunCommandTask {
    fn run(self: &Self, env: &mut Env) -> Result<Success, Error> {
        let mut command = build_command(&self.input_data.clone());

        let result = command.stdout(Stdio::piped())
            .spawn();

        let mut child: Child;
        match result {
            Ok(data) => {
                child = data;
            },
            Err(error) => {
                return Result::Err(Error {
                    code: 0,
                    message: format!("Failed to run command: {}, args: {:?}",self.input_data.command, self.input_data.args),
                    task: "RunCommandTask".to_string(),
                    stack: vec![error.to_string()],
                });
            }
        };

        watch_log_process(&mut child);

        let result = start_command(self.get_type().to_string(), child);
        let result_ = result.clone();

        let output = RunCommandOutputData { tag: "".to_string(), result };
        *env = Env::RunCommnad(output);

        result_
    }

    fn check(self: &Self, env: &mut Env) -> Result<Success, Error> {
        match env {
            Env::RunCommnad(output) => {
                let result = &output.result;
                match result {
                    Ok(_) => {
                        Result::Ok(Success{})
                    }
                    Err(error) => {
                        Result::Err(Error { code: 0, message: "An error occurred while executing a task".to_string(), task: String::from(error.clone().task), stack: error.clone().stack })
                    }
                }
            }
            _ => Result::Err(Error { code: 0, message: format!("task type {} is expected", self.get_type()), task: "".to_string(), stack: vec![] })
        }
    }

    fn get_type(self: &Self) -> TaskType {
        TaskType::Command
    }
}

pub fn build_command(input: &RunCommandInputData) -> Command {
    let mut cmd = Command::new(&input.command);

    for arg in &input.args {
        cmd.arg(arg);
    }

    if !&input.current_dir.is_empty() {
        cmd.current_dir(&input.current_dir);
    }

    return cmd;
}

fn watch_log_process(child: &mut Child) {
    let stdout = child.stdout.take().unwrap();

    thread::spawn(move || {
        let mut f = BufReader::new(stdout);
        loop {
            let mut buf = String::new();
            match f.read_line(&mut buf) {
                Ok(_) => {
                    print!("{}", buf)
                    /*match sender.send(buf){
                        Ok(_) => {continue;},
                        Err(_) =>{}
                    }*/
                }
                Err(_) => {
                    break;
                }
            }
        }
    });
}

fn start_command(task_type: String, mut child: Child) -> Result<Success, Error> {
    let handler = thread::spawn(move || {
        return child.wait();
    });

    match handler.join().unwrap() {
        Ok(code) => {
            if code.success() {
                Result::Ok(Success {})
            } else {
                Result::Err(Error {
                    code: 0,
                    message: "The command output an error".to_string(),
                    task: task_type,
                    stack: vec![],
                })
            }
        }
        Err(_) => {
            Result::Err(
                Error {
                    code: 0,
                    message: "Failed to run command".to_string(),
                    task: "".to_string(),
                    stack: vec![],
                })
        }
    }
}

pub enum Cmd {
    Install,
    Brew,
    AptGet,
    Sudo,
    Yum,
    List,
    Bash,
    Sh,
    Sed,
    Set,
    Ghc,
    Cabal,
    Ghcup,
    Git,
    Clone,
    Checkout,
    Make,
    Fetch,
    Build,
    Cp,
}

impl Cmd {
    pub fn as_str(&self) -> String {
        match self {
            Cmd::Install => "install".to_string(),
            Cmd::Brew => "brew".to_string(),
            Cmd::AptGet => "apt-get".to_string(),
            Cmd::Sudo => "sudo".to_string(),
            Cmd::Yum => "yum".to_string(),
            Cmd::List => "list".to_string(),
            Cmd::Bash => "bash".to_string(),
            Cmd::Sh => "sh".to_string(),
            Cmd::Sed => "sed".to_string(),
            Cmd::Set => "set".to_string(),
            Cmd::Ghc => "ghc".to_string(),
            Cmd::Cabal => "cabal".to_string(),
            Cmd::Ghcup => "./ghcup".to_string(),
            Cmd::Git => "git".to_string(),
            Cmd::Clone => "clone".to_string(),
            Cmd::Checkout => "checkout".to_string(),
            Cmd::Make => "make".to_string(),
            Cmd::Fetch => "fetch".to_string(),
            Cmd::Build => "build".to_string(),
            Cmd::Cp => "cp".to_string(),
        }
    }
}