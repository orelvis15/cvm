#![allow(dead_code, unused_variables)]

use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use crate::context::context::Context;
use file_diff::diff_files;
use tinytemplate::TinyTemplate;
use crate::{Success, url_build};
use crate::config::remote_config::{RemoteConfig, Services};
use crate::message::message::Message;
use crate::task::task::Task;
use crate::task::task_type::TaskType;
use crate::utils::download_manager::download;
use serde::Serialize;
use crate::task::task_impl::commons::run_command_task::{Cmd, RunCommandInputData, RunCommandTask};
use crate::task_manager::task_manager::TaskManager;
use crate::term::log_level::LogLevel::L2;
use crate::utils::user::get_current_user;

pub struct DeploySystemTask {}

///Requisitos para correr
/// - Que el sistema soporte SystemCtl
/// - Que exista la ruta /etc/systemd/system
/// - Permisos de administrador

impl Task for DeploySystemTask {
    fn prepare(self: &mut Self, context: &mut Context, config: &RemoteConfig) -> Result<bool, Message> {
        sudo::escalate_if_needed().expect("Super user permissions are required");
        Ok(true)
    }

    fn run(self: &Self, context: &mut Context, config: &RemoteConfig) -> Result<Success, Message> {
        for services in &config.services_item {
            create_service(&services)?;
        }

        TaskManager::default().start(vec![
            Box::new(RunCommandTask { input_data: build_reset_daemon_command(), command_description: "Reset systemctl daemon".to_string() }),
        ], config, L2, context)
    }

    fn check(self: &Self, context: &mut Context, config: &RemoteConfig) -> Result<Success, Message> {
        Ok(Success::default())
    }

    fn get_type(self: &Self) -> TaskType {
        TaskType::DeploySystem
    }

    fn get_id(self: &Self) -> String {
        "".to_string()
    }
}

fn create_service(service: &Services) -> Result<Success, Message> {
    let service_file_download = download(&format!("{}{}", &service.url, &service.file), &service.file)?;
    let file_string = fs::read_to_string(&service_file_download)?;

    let template = create_template(service.name.as_str(), file_string.as_str())?;
    create_service_file(&template, &service.file, &service_file_download)
}

fn create_template(name: &str, file_string: &str) -> Result<String, Message> {
    let mut template = TinyTemplate::new();
    template.add_template(name, file_string)?;
    let context = TemplateContext { user: get_current_user()?.to_string() };
    let text = template.render(name, &context)?;
    Ok(text)
}

fn create_service_file(template: &String, service_name: &String, service_file_download: &String) -> Result<Success, Message> {
    let service = url_build(vec![&"/etc/systemd/system/".to_string(), service_name], false);
    let service_path = Path::new(service.as_str());

    let files_is_same = check_if_files_is_same(service_path, Path::new(service_file_download)).unwrap_or(false);

    if !service_path.exists() || (service_path.exists() && !files_is_same) {
        let mut file = File::create(service_path)?;
        file.write_all(template.as_bytes())?;
    }
    Ok(Success::default())
}

fn check_if_files_is_same(service_path: &Path, service_file_download: &Path) -> Result<bool, Message, > {
    let mut system_file = File::open(service_path)?;
    let mut download_file = File::open(service_file_download)?;
    Ok(diff_files(&mut system_file, &mut download_file))
}

fn build_reset_daemon_command() -> RunCommandInputData {
    let args = vec![Cmd::DaemonReload.as_string()];
    RunCommandInputData { command: Cmd::Systemctl.as_string(), args, current_dir: "".to_string() }
}

#[derive(Serialize)]
struct TemplateContext {
    user: String,
}