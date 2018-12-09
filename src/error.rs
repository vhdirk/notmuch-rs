use std;
use std::{error, fmt, io, result};

use crate::ffi;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    NotmuchError(ffi::Status),
    UnspecifiedError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", error::Error::description(self))
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::IoError(ref e) => error::Error::description(e),
            Error::NotmuchError(ref e) => e.description(),
            Error::UnspecifiedError => "Generic notmuch error",
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            Error::IoError(ref e) => Some(e),
            Error::NotmuchError(ref e) => Some(e),
            Error::UnspecifiedError => None,
        }
    }
}

impl std::convert::From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}

impl std::convert::From<ffi::Status> for Error {
    fn from(err: ffi::Status) -> Error {
        Error::NotmuchError(err)
    }
}

impl std::convert::From<ffi::notmuch_status_t> for Error {
    fn from(err: ffi::notmuch_status_t) -> Error {
        Error::NotmuchError(ffi::Status::from(err))
    }
}
