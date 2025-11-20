use regex;
use std::error::Error;
use std::fmt;
use std::io;
use std::sync::mpsc::SendError;
use std::sync::{MutexGuard, PoisonError};

#[derive(Debug)]
pub enum AppError {
    Regex(regex::Error),
    Io(io::Error),
    SendError(String),
    MutexPoisoned(String),
    ThreadPanic,
    InvalidPath,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(err) => write!(f, "IO error: {}", err),
            AppError::SendError(err) => write!(f, "Send error: {}", err),
            AppError::MutexPoisoned(err) => write!(f, "Mutex poisoned: {}", err),
            AppError::ThreadPanic => write!(f, "A worker thread panicked"),
            AppError::InvalidPath => write!(f, "Invalid path encountered"),
            AppError::Regex(err) => write!(f, "Regex error: {}", err),
        }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AppError::Io(err) => Some(err),
            AppError::Regex(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        AppError::Io(err)
    }
}

impl<T> From<SendError<T>> for AppError {
    fn from(err: SendError<T>) -> Self {
        AppError::SendError(err.to_string())
    }
}

impl<'a, T> From<PoisonError<MutexGuard<'a, T>>> for AppError {
    fn from(err: PoisonError<MutexGuard<'a, T>>) -> Self {
        AppError::MutexPoisoned(err.to_string())
    }
}

impl From<regex::Error> for AppError {
    fn from(err: regex::Error) -> Self {
        AppError::Regex(err)
    }
}
