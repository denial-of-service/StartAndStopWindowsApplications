use std::fmt;
use std::error::Error;
#[derive(Debug)]
pub(crate) enum CustomError {
    InvalidFileError(String),
    InvalidConfigError(String),
    StartingApplicationFailedError(String),
    StoppingProcessFailedError(String),
}

impl Error for CustomError {}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            CustomError::InvalidFileError(msg) => msg,
            CustomError::InvalidConfigError(msg) => msg,
            CustomError::StartingApplicationFailedError(msg) => msg,
            CustomError::StoppingProcessFailedError(msg) => msg,
        };
        write!(f, "{msg}")
    }
}