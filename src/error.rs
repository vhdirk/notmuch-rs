use std::{
    error,
    fmt,
    io,
    result,
};

use ffi;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    NotmuchError(ffi::NotmuchStatus),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", error::Error::description(self))
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::IoError(ref e) => error::Error::description(e),
            Error::NotmuchError(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::IoError(ref e) => Some(e),
            Error::NotmuchError(ref e) => Some(e),
        }
    }
}

impl error::FromError<io::Error> for Error {
    fn from_error(err: io::Error) -> Error {
        Error::IoError(err)
    }
}

impl error::FromError<ffi::NotmuchStatus> for Error {
    fn from_error(err: ffi::NotmuchStatus) -> Error {
        Error::NotmuchError(err)
    }
}

impl error::FromError<ffi::notmuch_status_t> for Error {
    fn from_error(err: ffi::notmuch_status_t) -> Error {
        Error::NotmuchError(ffi::NotmuchStatus::from(err))
    }
}
