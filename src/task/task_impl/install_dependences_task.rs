extern crate rs_release;
use os_info::Type;
use crate::env::Env;
use crate::task::task::{Message, Success, Task};
use crate::task::task_type::TaskType;
use crate::config::config::{Dependencies, get_config};
use crate::task::message_type::MessageType;
use crate::task::task_impl::run_command_task::{Cmd, RunCommandInputData, RunCommandTask};

pub struct InstallDependencesTask {}

#[derive(Debug, Clone)]
pub struct InstallDependencesOutputData {
    pub dependences: String,
}

impl Task for InstallDependencesTask {
    fn run(self: &Self, env: &mut Env) -> Result<Success, Message> {
        let config = get_config();
        if let Err(error) = config {
            return Err(error);
        }

        // read the dependence file and build command
        let dependece = config.unwrap().dependencies;
        let dependences_result = get_dependences_from_os(dependece);
        let dependences: String;

        match dependences_result {
            Some(data) => {
                dependences = String::from(data)
            }
            None => {
                return Err(Message {
                    code: 0,
                    message: "Error getting dependencies".to_string(),
                    kind: MessageType::Error,
                    task: self.get_type().to_string(),
                    stack: vec![],
                });
            }
        }

        let command_input_result = get_install_command_from_os(dependences.clone());

        match command_input_result {
            Some(input) => {
                *env = Env::InstallDependences(InstallDependencesOutputData { dependences });

                let cmd = RunCommandTask { input_data: input };
                let mut env_aux: Env = Env::Empty();
                cmd.run(&mut env_aux)
            }
            None => {
                return Err(Message {
                    code: 0,
                    message: "Error getting dependencies".to_string(),
                    kind: MessageType::Error,
                    task: self.get_type().to_string(),
                    stack: vec![],
                });
            }
        }
    }

    fn check(self: &Self, env: &mut Env) -> Result<Success, Message> {
        let dependences: String;
        match env {
            Env::InstallDependences(output) => {
                dependences = output.clone().dependences;
            }
            _ => return Err(Message { code: 0, message: format!("task type {} is expected", self.get_type()), kind: MessageType::Error, task: "".to_string(), stack: vec![] })
        }

        let command_input_result = get_verify_command_from_os(dependences);

        match command_input_result {
            Some(input) => {
                let cmd = RunCommandTask { input_data: input };
                cmd.run(env)
            }
            None => {
                Ok(Success {})
                /*return Result::Err(Error {
                    code: 0,
                    message: "Error verify dependencies".to_string(),
                    task: self.task_type.to_string(),
                    stack: vec![],
                });*/
            }
        }
    }

    fn get_type(self: &Self) -> TaskType {
        TaskType::InstallDependences
    }
}

fn get_dependences_from_os(dependences: Dependencies) -> Option<String> {
    match os_info::get().os_type() {
        Type::Macos => { Some(dependences.macos.join(" ")) }
        Type::Ubuntu => { Some(dependences.ubuntu.join(" ")) }
        Type::Debian => { Some(dependences.debian.join(" ")) }
        Type::OracleLinux => { Some(dependences.debian.join(" ")) }
        Type::Fedora => { Some(dependences.fedora.join(" ")) }
        Type::CentOS => {
            let mut extra_dependences = String::new();
            let os_release = get_os_release();
            if os_release == "7" {
                extra_dependences = dependences.centos_7.join(" ");
            } else if os_release == "7" {
                extra_dependences = dependences.centos_8.join(" ");
            }
            Some(format!("{} {}", dependences.centos.join(" "), extra_dependences))
        }
        Type::Redhat => {
            let mut extra_dependences = String::new();
            let os_release = get_os_release();
            if os_release == "7" {
                extra_dependences = dependences.rhel_7.join(" ");
            } else if os_release == "7" {
                extra_dependences = dependences.rhel_8.join(" ");
            }
            Some(format!("{} {}", dependences.rhel.join(" "), extra_dependences))
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
    args.insert(0, Cmd::Install.as_str());
    RunCommandInputData { command: Cmd::Install.as_str(), args, ..Default::default() }
}

fn build_ubuntu_debian_install_command(dependences: String) -> RunCommandInputData {
    let mut args = Vec::from_iter(dependences.split_whitespace().map(String::from));
    args.insert(0, Cmd::Install.as_str());
    args.insert(0, "-y".to_string());
    args.insert(0, Cmd::AptGet.as_str());
    RunCommandInputData { command: Cmd::Sudo.as_str(), args, ..Default::default() }
}

fn build_centos_fedora_rhel_install_command(dependences: String) -> RunCommandInputData {
    let mut args = Vec::from_iter(dependences.split_whitespace().map(String::from));
    args.insert(0, "-y".to_string());
    args.insert(0, Cmd::Install.as_str());
    args.insert(0, Cmd::Yum.as_str());
    RunCommandInputData { command: Cmd::Sudo.as_str(), args, ..Default::default() }
}

fn get_verify_command_from_os(dependences: String) -> Option<RunCommandInputData> {
    return match os_info::get().os_type() {
        Type::Macos => { Some(build_macos_verify_install_command(dependences)) }
        _ => { None }
    };
}

fn build_macos_verify_install_command(dependences: String) -> RunCommandInputData {
    let mut args = Vec::from_iter(dependences.split_whitespace().map(String::from));
    args.insert(0, Cmd::List.as_str());
    RunCommandInputData { command: Cmd::Brew.as_str(), args, ..Default::default() }
}