#![allow(dead_code, unused_variables)]

extern crate rs_release;

use os_info::Type;
use crate::task::task::{Success, Task};
use crate::task::task_type::TaskType;
use crate::config::remote_config::{RemoteConfig, Dependencies};
use crate::config::state_config::{get_task_complete, set_task_complete};
use crate::context::context::Context;
use crate::message::message::{Message, MessageData};
use crate::task::task_impl::commons::run_command_task::{Cmd, RunCommandInputData, RunCommandTask};
use crate::task_manager::task_manager::TaskManager;
use crate::term::log_level::LogLevel::L2;

#[derive(Default)]
pub struct InstallDependenciesTask {
    dependencies: String
}

#[derive(Debug, Clone)]
pub struct InstallDependenciesOutputData {
    dependencies: String,
}

impl Task for InstallDependenciesTask {
    fn prepare(self: &mut Self, context: &mut Context, config: &RemoteConfig) -> Result<bool, Message> {
        if get_task_complete(&self.get_type()) {
            return Ok(false);
        };

        self.dependencies = get_dependencies_from_os(&config.dependencies)?;
        Ok(true)
    }

    fn run(self: &Self, context: &mut Context, config: &RemoteConfig) -> Result<Success, Message> {
        let command_input_result = get_install_command_from_os(&self.dependencies)?;
        TaskManager{}.start(vec![
                    Box::new(RunCommandTask { input_data: command_input_result, command_description: "Installing the necessary dependencies".to_string() })
                ], config, L2, context )
    }

    fn check(self: &Self, context: &mut Context, config: &RemoteConfig) -> Result<Success, Message> {
        set_task_complete(&self.get_type());
        Ok(Success::default())
    }

    fn get_type(self: &Self) -> TaskType {
        TaskType::InstallDependences
    }

    fn get_id(self: &Self) -> String {
        "".to_string()
    }
}

fn get_dependencies_from_os(dependencies: &Dependencies) -> Result<String, Message> {
    match os_info::get().os_type() {
        Type::Macos => { Ok(dependencies.macos.join(" ")) }
        Type::Ubuntu => { Ok(dependencies.ubuntu.join(" ")) }
        Type::Debian => { Ok(dependencies.debian.join(" ")) }
        Type::OracleLinux => { Ok(dependencies.debian.join(" ")) }
        Type::Fedora => { Ok(dependencies.fedora.join(" ")) }
        Type::CentOS => {
            let mut extra_dependences = String::new();
            let os_release = get_os_release();
            if os_release == "7" {
                extra_dependences = dependencies.centos_7.join(" ");
            } else if os_release == "8" {
                extra_dependences = dependencies.centos_8.join(" ");
            }
            Ok(format!("{} {}", dependencies.centos.join(" "), extra_dependences))
        }
        Type::Redhat => {
            let mut extra_dependences = String::new();
            let os_release = get_os_release();
            if os_release == "7" {
                extra_dependences = dependencies.rhel_7.join(" ");
            } else if os_release == "8" {
                extra_dependences = dependencies.rhel_8.join(" ");
            }
            Ok(format!("{} {}", dependencies.rhel.join(" "), extra_dependences))
        }
        _ => {
            return Err(Message::GettingDependences(MessageData {
                message: "Error getting dependencies".to_string(),
                task: TaskType::InstallDependences,
                ..Default::default()
            }));
        }
    }
}

fn get_os_release() -> String {
    match rs_release::get_os_release() {
        Err(_) => "".to_string(),
        Ok(_os_release) => {
            _os_release.get("VERSION_ID").unwrap().to_string()
        }
    }
}

fn get_install_command_from_os(dependencies: &String) -> Result<RunCommandInputData, Message> {
    return match os_info::get().os_type() {
        Type::Macos => { Ok(build_macos_install_command(dependencies)) }
        Type::Ubuntu => { Ok(build_ubuntu_debian_install_command(dependencies)) }
        Type::Debian => { Ok(build_ubuntu_debian_install_command(dependencies)) }
        Type::OracleLinux => { Ok(build_ubuntu_debian_install_command(dependencies)) }
        Type::CentOS => { Ok(build_centos_fedora_rhel_install_command(dependencies)) }
        Type::Fedora => { Ok(build_centos_fedora_rhel_install_command(dependencies)) }
        Type::Redhat => { Ok(build_centos_fedora_rhel_install_command(dependencies)) }
        _ => {
            return Err(Message::GettingDependences(MessageData {
                message: "Error getting dependencies".to_string(),
                task: TaskType::InstallDependences,
                ..Default::default()
            }));
        }
    };
}

fn build_macos_install_command(dependencies: &String) -> RunCommandInputData {
    let mut args = Vec::from_iter(dependencies.split_whitespace().map(String::from));
    args.insert(0, Cmd::Install.as_string());
    RunCommandInputData { command: Cmd::Install.as_string(), args, ..Default::default() }
}

fn build_ubuntu_debian_install_command(dependencies: &String) -> RunCommandInputData {
    let mut args = Vec::from_iter(dependencies.split_whitespace().map(String::from));
    args.insert(0, Cmd::Install.as_string());
    args.insert(0, "-y".to_string());
    args.insert(0, Cmd::AptGet.as_string());
    RunCommandInputData { command: Cmd::Sudo.as_string(), args, ..Default::default() }
}

fn build_centos_fedora_rhel_install_command(dependencies: &String) -> RunCommandInputData {
    let mut args = Vec::from_iter(dependencies.split_whitespace().map(String::from));
    args.insert(0, "--skip-broken".to_string());
    args.insert(0, "-y".to_string());
    args.insert(0, Cmd::Install.as_string());
    args.insert(0, Cmd::Yum.as_string());
    RunCommandInputData { command: Cmd::Sudo.as_string(), args, ..Default::default() }
}

fn get_verify_command_from_os(dependencies: &String) -> Option<RunCommandInputData> {
    return match os_info::get().os_type() {
        Type::Macos => { Some(build_macos_verify_install_command(dependencies)) }
        _ => { None }
    };
}

fn build_macos_verify_install_command(dependencies: &String) -> RunCommandInputData {
    let mut args = Vec::from_iter(dependencies.split_whitespace().map(String::from));
    args.insert(0, Cmd::List.as_string());
    RunCommandInputData { command: Cmd::Brew.as_string(), args, ..Default::default() }
}