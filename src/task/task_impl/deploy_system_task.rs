#![allow(dead_code, unused_variables)]

use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use file_diff::diff_files;
use tinytemplate::TinyTemplate;
use crate::env::Env;
use crate::{Success, url_build};
use crate::config::config::{Config, Services};
use crate::task::cvm_error::CvmError;
use crate::task::task::Task;
use crate::task::task_type::TaskType;
use crate::utils::download_manager::download;
use serde::Serialize;

pub struct DeploySystemTask {}

///Requisitos para correr
/// - Que el sistema soporte SystemCtl
/// - Que exista la ruta /etc/systemd/system
/// - Permisos de administrador

impl Task for DeploySystemTask {
    fn run(self: &Self, _env: &mut Env, config: &Config) -> Result<Success, CvmError> {
        sudo::escalate_if_needed().expect("Super user permissions are required");
        for services in &config.services_item {
            let service_exist = systemctl::exists(&services.name).unwrap_or(false);
            if !service_exist {
                create_service(&services)?;
            }
        }
        Ok(Success {})
    }

    fn check(self: &Self, _env: &mut Env, config: &Config) -> Result<Success, CvmError> {
        Ok(Success {})
    }

    fn get_type(self: &Self) -> TaskType {
        TaskType::DeploySystem
    }
}

fn create_service(service: &Services) -> Result<Success, CvmError> {
    let service_file_download = download(&format!("{}{}", &service.url, &service.file), &service.file)?;
    let file_string = fs::read_to_string(&service_file_download)?;

    let template = create_template(service.name.as_str(), file_string.as_str())?;
    create_service_file(&template, &service.file, &service_file_download)
}

fn create_template(name: &str, file_string: &str) -> Result<String, CvmError> {
    let mut template = TinyTemplate::new();
    template.add_template(name, file_string)?;
    let context = TemplateContext { user: "root".to_string() };
    let text = template.render(name, &context)?;
    Ok(text)
}

fn create_service_file(template: &String, service_name: &String, service_file_download: &String) -> Result<Success, CvmError> {
    let service = url_build(vec!["/etc/systemd/system/", service_name.as_str()], false);
    let service_path = Path::new(service.as_str());

    let files_is_same = check_if_files_is_same(service_path, Path::new(service_file_download)).unwrap_or(false);

    if !service_path.exists() || (service_path.exists() && !files_is_same){
        let mut file = File::create(service_path)?;
        file.write_all(template.as_bytes())?;
    }
    Ok(Success {})
}

fn check_if_files_is_same(service_path: &Path, service_file_download: &Path) -> Result<bool, CvmError, >{
    let mut system_file = File::open(service_path)?;
    let mut download_file = File::open(service_file_download)?;
    Ok(diff_files(&mut system_file, &mut download_file))
}

#[derive(Serialize)]
struct TemplateContext {
    user: String,
}