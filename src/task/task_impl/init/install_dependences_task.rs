#![allow(dead_code, unused_variables)]

extern crate rs_release;

use os_info::Type;
use crate::env::Env;
use crate::task::task::{Success, Task};
use crate::task::task_type::TaskType;
use crate::config::config::{Config, Dependencies};
use crate::error::message::{Message, Error};
use crate::task::task_impl::commons::run_command_task::{Cmd, RunCommandInputData, RunCommandTask};
use crate::Term;

pub struct InstallDependencesTask {}

#[derive(Debug, Clone)]
pub struct InstallDependenciesOutputData {
    pub dependencies: String,
}

impl Task for InstallDependencesTask {
    fn run(self: &Self, env: &mut Env, config: &Config, term: &mut Term) -> Result<Success, Message> {
        let dependece = &config.dependencies;
        let dependences_result = get_dependencies_from_os(dependece);
        let dependences: String;

        match dependences_result {
            Some(data) => {
                dependences = String::from(data)
            }
            None => {
                return Err(Message::GettingDependences(Error {
                    message: "Error getting dependencies".to_string(),
                    task: self.get_type(),
                    stack: vec![],
                }));
            }
        }

        let command_input_result = get_install_command_from_os(dependences.clone());

        match command_input_result {
            Some(input) => {
                *env = Env::InstallDependences(InstallDependenciesOutputData { dependencies: dependences });

                let cmd = RunCommandTask { input_data: input, command_description: "Installing the necessary dependencies".to_string() };
                let mut env_aux: Env = Env::Empty();
                cmd.run(&mut env_aux, config, term)
            }
            None => {
                return Err(Message::GettingDependences(Error {
                    message: "Error getting dependencies".to_string(),
                    task: self.get_type(),
                    stack: vec![],
                }));
            }
        }
    }

    fn check(self: &Self, env: &mut Env, config: &Config, term: &mut Term) -> Result<Success, Message> {
        Ok(Success {})
    }

    fn get_type(self: &Self) -> TaskType {
        TaskType::InstallDependences
    }
}

fn get_dependencies_from_os(dependencies: &Dependencies) -> Option<String> {
    match os_info::get().os_type() {
        Type::Macos => { Some(dependencies.macos.join(" ")) }
        Type::Ubuntu => { Some(dependencies.ubuntu.join(" ")) }
        Type::Debian => { Some(dependencies.debian.join(" ")) }
        Type::OracleLinux => { Some(dependencies.debian.join(" ")) }
        Type::Fedora => { Some(dependencies.fedora.join(" ")) }
        Type::CentOS => {
            let mut extra_dependences = String::new();
            let os_release = get_os_release();
            if os_release == "7" {
                extra_dependences = dependencies.centos_7.join(" ");
            } else if os_release == "7" {
                extra_dependences = dependencies.centos_8.join(" ");
            }
            Some(format!("{} {}", dependencies.centos.join(" "), extra_dependences))
        }
        Type::Redhat => {
            let mut extra_dependences = String::new();
            let os_release = get_os_release();
            if os_release == "7" {
                extra_dependences = dependencies.rhel_7.join(" ");
            } else if os_release == "7" {
                extra_dependences = dependencies.rhel_8.join(" ");
            }
            Some(format!("{} {}", dependencies.rhel.join(" "), extra_dependences))
        }
        _ => { None }
    }
}

///Arreglar release os
fn get_os_release() -> String {
    match rs_release::get_os_release() {
        Err(_) => "".to_string(),
        Ok(_os_release) => {
            "7".to_string()
            //os_release.get("VERSION_ID").unwrap()
        }
    }
}

fn get_install_command_from_os(dependences: String) -> Option<RunCommandInputData> {
    return match os_info::get().os_type() {
        Type::Macos => { Some(build_macos_install_command(dependences)) }
        Type::Ubuntu => { Some(build_ubuntu_debian_install_command(dependences)) }
        Type::Debian => { Some(build_ubuntu_debian_install_command(dependences)) }
        Type::OracleLinux => { Some(build_ubuntu_debian_install_command(dependences)) }
        Type::CentOS => { Some(build_centos_fedora_rhel_install_command(dependences)) }
        Type::Fedora => { Some(build_centos_fedora_rhel_install_command(dependences)) }
        Type::Redhat => { Some(build_centos_fedora_rhel_install_command(dependences)) }
        _ => { None }
    };
}

fn build_macos_install_command(dependences: String) -> RunCommandInputData {
    let mut args = Vec::from_iter(dependences.split_whitespace().map(String::from));
    args.insert(0, Cmd::Install.as_string());
    RunCommandInputData { command: Cmd::Install.as_string(), args, ..Default::default() }
}

fn build_ubuntu_debian_install_command(dependences: String) -> RunCommandInputData {
    let mut args = Vec::from_iter(dependences.split_whitespace().map(String::from));
    args.insert(0, Cmd::Install.as_string());
    args.insert(0, "-y".to_string());
    args.insert(0, Cmd::AptGet.as_string());
    RunCommandInputData { command: Cmd::Sudo.as_string(), args, ..Default::default() }
}

fn build_centos_fedora_rhel_install_command(dependences: String) -> RunCommandInputData {
    let mut args = Vec::from_iter(dependences.split_whitespace().map(String::from));
    args.insert(0, "-y".to_string());
    args.insert(0, Cmd::Install.as_string());
    RunCommandInputData { command: Cmd::Yum.as_string(), args, ..Default::default() }
}

fn get_verify_command_from_os(dependences: String) -> Option<RunCommandInputData> {
    return match os_info::get().os_type() {
        Type::Macos => { Some(build_macos_verify_install_command(dependences)) }
        _ => { None }
    };
}

fn build_macos_verify_install_command(dependences: String) -> RunCommandInputData {
    let mut args = Vec::from_iter(dependences.split_whitespace().map(String::from));
    args.insert(0, Cmd::List.as_string());
    RunCommandInputData { command: Cmd::Brew.as_string(), args, ..Default::default() }
}