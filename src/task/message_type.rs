use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone)]
pub enum MessageType{
    Error,
    Warning,
    Info,
}

impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MessageType::Error => write!(f, "error"),
            MessageType::Warning => write!(f, "warning"),
            MessageType::Info => write!(f, "info"),
        }
    }
}