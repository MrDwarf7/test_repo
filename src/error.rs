use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    SystemTimeError(std::time::SystemTimeError),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::SystemTimeError(error) => write!(f, "SystemTimeError: {}", error),
        }
    }
}

impl From<std::time::SystemTimeError> for Error {
    fn from(error: std::time::SystemTimeError) -> Self {
        Error::SystemTimeError(error)
    }
}
