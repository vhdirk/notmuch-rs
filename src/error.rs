use std;
use std::{error, fmt, io, result};

use ffi;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    NotmuchError(ffi::Status),
    UnspecifiedError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::IoError(e) => e.fmt(f),
            Error::NotmuchError(e) => e.fmt(f),
            Error::UnspecifiedError => write!(f, "Generic notmuch error"),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self {
            Error::IoError(e) => Some(e),
            Error::NotmuchError(e) => Some(e),
            Error::UnspecifiedError => None
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
