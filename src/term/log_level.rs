#![allow(dead_code, unused_variables)]

use std::fmt::{Display, Formatter};

#[derive(Eq, PartialEq)]
pub enum LogLevel {
    L1,
    L2,
    L3,
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            LogLevel::L1 => write!(f, "L1"),
            LogLevel::L2 => write!(f, "L2"),
            LogLevel::L3 => write!(f, "L3"),
        }
    }
}