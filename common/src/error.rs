//! Common error types

use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    OutOfMemory,
    InvalidArgument,
    NotFound,
    AlreadyExists,
    PermissionDenied,
    IoError,
    NetworkError,
    ParseError,
    Timeout,
    Unknown,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::OutOfMemory => write!(f, "Out of memory"),
            Error::InvalidArgument => write!(f, "Invalid argument"),
            Error::NotFound => write!(f, "Not found"),
            Error::AlreadyExists => write!(f, "Already exists"),
            Error::PermissionDenied => write!(f, "Permission denied"),
            Error::IoError => write!(f, "I/O error"),
            Error::NetworkError => write!(f, "Network error"),
            Error::ParseError => write!(f, "Parse error"),
            Error::Timeout => write!(f, "Timeout"),
            Error::Unknown => write!(f, "Unknown error"),
        }
    }
}

pub type Result<T> = core::result::Result<T, Error>;
