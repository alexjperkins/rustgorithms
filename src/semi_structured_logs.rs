#![allow(unused)]

use std::fmt;

#[derive(Clone, PartialEq, Debug)]
enum LogLevel {
    Info,
    Warning,
    Error,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let level = format!("{:?}", self);
        write!(f, "{:?}", level.to_uppercase())
    }
}

fn log(level: LogLevel, message: &str) -> String {
    format!("[{}:] {}", level, message)
}

fn info(message: &str) -> String {
    log(LogLevel::Info, message)
}

fn warn(message: &str) -> String {
    log(LogLevel::Warning, message)
}

fn error(message: &str) -> String {
    log(LogLevel::Error, message)
}
