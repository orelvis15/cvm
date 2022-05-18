use regex::Regex;
use crate::{Message, MessageType};
use serde::{Serialize, Deserialize};

pub const LATEST: &str = "latest";
const USER_AGENT: &str = "cvm";

pub fn verify_version(version: &str) -> bool {
    let regex = Regex::new(r"^(\d+\.)?(\d+\.)?(\*|\d+)$").unwrap();
    regex.is_match(version) || version == LATEST
}

pub fn get_last_tag(url: String) -> Result<String, Message> {
    let client = reqwest::blocking::Client::builder().user_agent(USER_AGENT).build();
    if let Ok(client) = client {
        if let Ok(response) = client.get(url).send() {
            if let Ok(test) = response.text() {
                let tag: Tag = serde_json::from_str(test.as_str()).unwrap();
                return Result::Ok(tag.tag_name);
            }
        }
    }
    return Err(Message {
        code: 0,
        message: "Error checking latest cardano node tag".to_string(),
        kind: MessageType::Error,
        task: "".to_string(),
        stack: vec![]
    });
}

#[derive(Serialize, Deserialize)]
pub struct Tag {
    #[serde(rename = "tag_name")]
    pub tag_name: String,
}
