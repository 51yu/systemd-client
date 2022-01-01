use std::fmt::{Debug, Display};
use std::result;
use thiserror::Error;

#[derive(Debug, Error)]
pub struct Error(Box<ErrorImpl>);

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum ErrorImpl {
    #[error("dbus error, detail: {0:?}")]
    DBus(#[from] dbus::Error),
    #[error("into string error, detail: {0:?}")]
    IntoString(#[from] std::ffi::IntoStringError),
    #[error("io error, detail: {0:?}")]
    Io(#[from] std::io::Error),
}

impl From<dbus::Error> for Error {
    fn from(err: dbus::Error) -> Self {
        Error(Box::new(ErrorImpl::DBus(err)))
    }
}

impl From<std::ffi::IntoStringError> for Error {
    fn from(err: std::ffi::IntoStringError) -> Self {
        Error(Box::new(ErrorImpl::IntoString(err)))
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error(Box::new(ErrorImpl::Io(err)))
    }
}
