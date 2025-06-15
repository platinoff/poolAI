use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum PlatformError {
    ServiceError(String),
    SystemInfoError(String),
    IoError(std::io::Error),
    WindowsError(windows_service::Error),
    UnixError(String),
}

impl fmt::Display for PlatformError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlatformError::ServiceError(msg) => write!(f, "Service error: {}", msg),
            PlatformError::SystemInfoError(msg) => write!(f, "System info error: {}", msg),
            PlatformError::IoError(err) => write!(f, "IO error: {}", err),
            PlatformError::WindowsError(err) => write!(f, "Windows error: {}", err),
            PlatformError::UnixError(msg) => write!(f, "Unix error: {}", msg),
        }
    }
}

impl Error for PlatformError {}

impl From<std::io::Error> for PlatformError {
    fn from(err: std::io::Error) -> Self {
        PlatformError::IoError(err)
    }
}

impl From<windows_service::Error> for PlatformError {
    fn from(err: windows_service::Error) -> Self {
        PlatformError::WindowsError(err)
    }
}

impl From<String> for PlatformError {
    fn from(err: String) -> Self {
        PlatformError::ServiceError(err)
    }
}

impl From<&str> for PlatformError {
    fn from(err: &str) -> Self {
        PlatformError::ServiceError(err.to_string())
    }
} 