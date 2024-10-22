use std::{error::Error, fmt};

use config::ConfigError;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    Config(ConfigError),
    Io(std::io::Error),
    Other(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Config(e) => write!(f, "Configuration Error: {}", e),
            AppError::Io(e) => write!(f, "IO Error: {}", e),
            AppError::Other(e) => write!(f, "Error: {}", e),
        }
    }
}
impl Error for AppError {}

impl From<ConfigError> for AppError {
    fn from(e: ConfigError) -> Self {
        AppError::Config(e)
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::Io(e)
    }
}

impl From<&str> for AppError {
    fn from(e: &str) -> Self {
        AppError::Other(e.into())
    }
}
