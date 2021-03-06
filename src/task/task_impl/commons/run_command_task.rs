#![allow(dead_code, unused_variables)]

use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};
use std::thread;
use crate::config::remote_config::RemoteConfig;
use crate::env::Env;
use crate::message::message::{Message, MessageData};
use crate::task::task::{Success, Task};
use crate::task::task_type::TaskType;
use crate::Term;

pub struct RunCommandTask {
    pub input_data: RunCommandInputData,
    pub command_description: String
}

pub struct RunCommandOutputData {
    pub tag: String,
    pub result: Result<Success, Message>,
}

impl RunCommandOutputData {
    fn get_tag(&self) -> &String {
        &self.tag
    }
    fn get_data(&self) -> &RunCommandOutputData {
        self
    }
}

#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct RunCommandInputData {
    pub command: String,
    pub args: Vec<String>,
    pub current_dir: String,
}

impl Task for RunCommandTask {

    fn prepare(self: &mut Self, env: &mut Env, config: &RemoteConfig, term: &mut Term) -> Result<bool, Message> {
        Ok(true)
    }

    fn run(self: &Self, env: &mut Env, config: &RemoteConfig, term: &mut Term) -> Result<Success, Message> {
        let mut command = build_command(&self.input_data.clone());

        let result = command.spawn();

        let mut child: Child;
        match result {
            Ok(data) => {
                child = data;
            }
            Err(error) => {
                return Err(Message::FaileToRunCommand(MessageData {
                    message: format!("Failed to run command: {}, args: {:?}", self.input_data.command, self.input_data.args),
                    task: self.get_type(),
                    stack: vec![error.to_string()],
                    ..Default::default()
                }));
            }
        };
        watch_log_process(&mut child);
        start_command(self.get_type().to_string(), child, self)
    }

    fn check(self: &Self, env: &mut Env, config: &RemoteConfig, term: &mut Term) -> Result<Success, Message> {
        Ok(Success{})
    }

    fn get_type(self: &Self) -> TaskType {
        TaskType::RunCommand(self.input_data.clone(), self.command_description.clone())
    }
}

pub fn build_command(input: &RunCommandInputData) -> Command {
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
fn read_stdout(mut command: Command) -> Command{
    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());
    command
}

#[cfg(not(debug_assertions))]
fn read_stdout(mut command: Command) -> Command{
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

fn start_command(task_type: String, mut child: Child, _self: &RunCommandTask) -> Result<Success, Message> {

    let handler = child.wait();

    match handler {
        Ok(code) => {
            if code.success() {
                Ok(Success {})
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
    V2Update,
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
            Cmd::Systemctl => {"systemctl".to_string()}
            Cmd::DaemonReload => {"daemon-reload".to_string()}
            Cmd::All => {"all".to_string()}
            Cmd::Update => {"update".to_string()}
            Cmd::Clean => {"clean".to_string()}
            Cmd::Help => {"help".to_string()}
            Cmd::Sudo => {"sudo".to_string()}
            Cmd::V2Update => {"v2-update".to_string()}
        }
    }
}