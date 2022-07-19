#![allow(dead_code, unused_variables)]

use std::io::{BufRead, BufReader};
use std::process::{Child, Command, ExitStatus, Stdio};
use crate::context::context::Context;
use std::thread;
use crate::config::remote_config::RemoteConfig;
use crate::context::storage::TaskOutputData;
use crate::message::message::{Message, MessageData};
use crate::task::task::{id_generator, Success, Task};
use crate::task::task_impl::commons::command::run_command_io_data::{ResolveRunCommandInputData, RunCommandInputData, RunCommandOutputData};
use crate::task::task_impl::task_input_data::TaskInputData;
use crate::task::task_type::TaskType;

#[derive(Default)]
pub struct RunCommandTask {
    pub input_data: RunCommandInputData,
    pub data: ResolveRunCommandInputData,
}

impl Task for RunCommandTask {
    fn prepare(self: &mut Self, context: &mut Context, config: &RemoteConfig) -> Result<bool, Message> {
        let mut input_data = ResolveRunCommandInputData::default();

        match &self.input_data.command {
            TaskInputData::String(data) => { input_data.command = data.to_owned() }
            _ => {}
        }

        match &self.input_data.args {
            TaskInputData::VecString(data) => { input_data.args = data.to_owned() }
            _ => {}
        }

        match &self.input_data.current_dir {
            TaskInputData::String(data) => { input_data.current_dir = data.to_owned() }
            _ => {}
        }

        match &self.input_data.description {
            TaskInputData::String(data) => { input_data.description = data.to_owned() }
            _ => {}
        }

        self.data = input_data;
        Ok(true)
    }

    fn run(self: &Self, context: &mut Context, config: &RemoteConfig) -> Result<Success, Message> {
        let mut command = build_command(&self.data);

        let result = command.spawn();

        let mut child: Child;
        match result {
            Ok(data) => {
                child = data;
            }
            Err(error) => {
                return Err(Message::FaileToRunCommand(MessageData {
                    message: format!("Failed to run command: {}, args: {:?}", self.data.command, self.data.args),
                    task: self.get_type(),
                    stack: vec![error.to_string()],
                    ..Default::default()
                }));
            }
        };
        watch_log_process(&mut child);
        let exit_code = start_command(self.get_type().to_string(), child, self)?;

        Ok(Success { value: TaskOutputData::RunCommand(RunCommandOutputData { tag: "".to_string(), code: exit_code.code().unwrap_or(0i32) }) })
    }

    fn check(self: &Self, context: &mut Context, config: &RemoteConfig) -> Result<Success, Message> {
        Ok(Success::default())
    }

    fn get_type(self: &Self) -> TaskType {
        TaskType::RunCommand(self.data.to_owned())
    }

    fn get_id(self: &Self) -> String {
        let value: Vec<String> = vec![self.data.command.clone(),
                                      self.data.args.join(&"")];
        id_generator(&value)
    }
}

pub fn build_command(input: &ResolveRunCommandInputData) -> Command {
    let cmd = Command::new(&input.command);
    let mut cmd = read_stdout(cmd);

    for arg in &input.args {
        cmd.arg(arg);
    }

    if !&input.current_dir.is_empty() {
        cmd.current_dir(&input.current_dir);
    }

    return cmd;
}

#[cfg(debug_assertions)]
fn read_stdout(mut command: Command) -> Command {
    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());
    command
}

#[cfg(not(debug_assertions))]
fn read_stdout(mut command: Command) -> Command {
    command.stdout(Stdio::null());
    command.stderr(Stdio::null());
    command
}

#[cfg(debug_assertions)]
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

#[cfg(not(debug_assertions))]
fn watch_log_process(child: &mut Child) {}

fn start_command(task_type: String, mut child: Child, _self: &RunCommandTask) -> Result<ExitStatus, Message> {
    let handler = child.wait();

    match handler {
        Ok(code) => {
            if code.success() {
                Ok(code)
            } else {
                Err(Message::CommandOutputError(MessageData {
                    message: "The command output an message".to_string(),
                    task: _self.get_type(),
                    stack: vec![code.to_string()],
                    ..Default::default()
                }))
            }
        }
        Err(error) => {
            Err(
                Message::FaileToRunCommand(MessageData {
                    message: "Failed to run command".to_string(),
                    task: _self.get_type(),
                    stack: vec![error.to_string()],
                    ..Default::default()
                }))
        }
    }
}

pub enum Cmd {
    Install,
    Brew,
    AptGet,
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
    All,
    Systemctl,
    DaemonReload,
    Update,
    Clean,
    Help,
    Sudo,
}

impl Cmd {
    pub fn as_string(&self) -> String {
        match self {
            Cmd::Install => "install".to_string(),
            Cmd::Brew => "brew".to_string(),
            Cmd::AptGet => "apt-get".to_string(),
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
            Cmd::Systemctl => { "systemctl".to_string() }
            Cmd::DaemonReload => { "daemon-reload".to_string() }
            Cmd::All => { "all".to_string() }
            Cmd::Update => { "update".to_string() }
            Cmd::Clean => { "clean".to_string() }
            Cmd::Help => { "help".to_string() }
            Cmd::Sudo => { "sudo".to_string() }
        }
    }
}