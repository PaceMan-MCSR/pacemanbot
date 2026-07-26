#[derive(PartialEq, PartialOrd)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
}

impl From<&str> for LogLevel {
    fn from(s: &str) -> Self {
        match s {
            "error" | "ERROR" => LogLevel::Error,
            "warn" | "WARN" => LogLevel::Warn,
            "info" | "INFO" => LogLevel::Info,
            "debug" | "DEBUG" => LogLevel::Debug,
            _ => LogLevel::Info,
        }
    }
}

impl LogLevel {
    pub fn to_log_prefix(&self) -> String {
        match self {
            LogLevel::Error => "[ERROR]".to_string(),
            LogLevel::Warn => "[WARN]".to_string(),
            LogLevel::Info => "[INFO]".to_string(),
            LogLevel::Debug => "[DEBUG]".to_string(),
        }
    }
}
