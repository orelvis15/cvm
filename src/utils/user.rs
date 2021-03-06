use std::env;
use users::{get_current_uid, get_user_by_uid};
use crate::{MessageData, Message};

pub fn get_current_user() -> Result<String, Message> {
    let user = get_user_by_uid(get_current_uid()).unwrap();
    if user.uid() != 0 {
        return Ok(String::from(user.name().to_str().unwrap()));
    }

    //if user is root return SUDO_USER var
    if let Some(sudo_user) = env::var_os("SUDO_USER") {
        return Ok(String::from(sudo_user.to_str().unwrap()));
    }

    Err(Message::UserNotFound(MessageData {
        message: "User not found".to_string(),
        ..Default::default()
    }))
}