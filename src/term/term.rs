#![allow(dead_code, unused_variables)]

use std::io::{Stdout, Write};
use crossterm::{cursor, ExecutableCommand, QueueableCommand};
use crossterm::style::Stylize;
use crate::task::task_type::TaskType;
use crate::term::log_level::LogLevel;

pub struct Term {
    pub stdout: Stdout,
}

impl Term {
    pub fn init(&mut self) {
        let _ = &self.stdout.execute(cursor::Hide).unwrap();
    }

    pub fn print_task_message(&mut self, tasktype: TaskType, log_level: &LogLevel) {
        if tasktype.print() != "" {
            let _ = &self.print_message(&tasktype, &log_level);
        }
    }

    fn print_message(&mut self, tasktype: &TaskType, log_level: &LogLevel) {
        match log_level {
            LogLevel::L1 => {
                let _ = &self.stdout.write_all(format!("{}", tasktype.print().green()).as_bytes());
            }
            LogLevel::L2 => {
                let _ = &self.stdout.write_all(format!("  |--{}", tasktype.print().green()).as_bytes());
            }
            _ => {}
        }
        let _ = &self.stdout.queue(cursor::MoveToNextLine(1)).unwrap();
        let _ = &self.stdout.flush().unwrap();
    }

    pub fn exit(&mut self) {
        let _ = &self.stdout.execute(cursor::Show).unwrap();
    }
}