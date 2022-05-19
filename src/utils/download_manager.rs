use std::env;
use std::io::Write;
use std::fs;
use std::fs::File;
use std::os::unix::fs::PermissionsExt;
use reqwest::blocking::Response;
use crate::task::message_type::MessageType;
use crate::task::task::{Message};

pub fn download(url: String, name: &str) -> Result<String, Message> {
    let dir = env::temp_dir();

    let mut path = String::new();
    path.push_str(dir.to_str().unwrap());
    path.push_str(&*name);

    let res = reqwest::blocking::get(&*url);
    let response: Response;

    match res {
        Ok(data) => {
            response = data;
        }
        Err(error) => {
            return Err(Message {
                code: 0,
                message: format!("Error download file: {} from url: {}", name, url),
                kind: MessageType::Error,
                task: "download function".to_string(),
                stack: vec![error.to_string()],
            });
        }
    }

    let content = response.bytes();

    let file_result = File::create(&path);
    let mut file: File;

    match file_result {
        Ok(data) => {
            file = data
        }
        Err(error) => {
            return Err(Message {
                code: 0,
                message: format!("Error creating file: {}", name),
                kind: MessageType::Error,
                task: "download function".to_string(),
                stack: vec![error.to_string()],
            });
        }
    }

    let permission_result = fs::set_permissions(&path, fs::Permissions::from_mode(0o755));

    if let Err(error) = permission_result {
        return Err(Message{
            code: 0,
            message: "Error assigning permissions to the downloaded file".to_string(),
            kind: MessageType::Error,
            task: "".to_string(),
            stack: vec![error.to_string()]
        });
    }


    let write_result = file.write_all(&content.unwrap());

    if let Err(error) = write_result {
        return Err(Message{
            code: 0,
            message: "Error writing file".to_string(),
            kind: MessageType::Error,
            task: "".to_string(),
            stack: vec![error.to_string()]
        });
    }

    Ok(path)
}

pub fn download_in_path(url: &String, path: String, name: &str) -> Result<String, Message> {
    let file_path = format!("{}/{}", path, name);

    let res = reqwest::blocking::get(url);
    let response: Response;

    match res {
        Ok(data) => {
            response = data;
        }
        Err(error) => {
            return Err(Message {
                code: 0,
                message: format!("Error download file: {} from url: {}", name, url),
                kind: MessageType::Error,
                task: "".to_string(),
                stack: vec![error.to_string()],
            });
        }
    }

    let content = response.bytes();

    let file_result = File::create(&file_path);
    let mut file: File;

    match file_result {
        Ok(data) => {
            file = data
        }
        Err(error) => {
            return Err(Message {
                code: 0,
                message: format!("Error creating file: {}", name),
                kind: MessageType::Error,
                task: "".to_string(),
                stack: vec![error.to_string()],
            });
        }
    }

    let permission_result = fs::set_permissions(&path, fs::Permissions::from_mode(0o755));

    if let Err(error) = permission_result {
        return Err(Message{
            code: 0,
            message: "Error assigning permissions to the downloaded file".to_string(),
            kind: MessageType::Error,
            task: "".to_string(),
            stack: vec![error.to_string()]
        });
    }


    let write_result = file.write_all(&content.unwrap());

    if let Err(error) = write_result {
        return Err(Message{
            code: 0,
            message: "Error writing file".to_string(),
            kind: MessageType::Error,
            task: "".to_string(),
            stack: vec![error.to_string()]
        });
    }

    Ok(path)
}