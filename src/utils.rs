use std::{
    ffi,
    path,
    str,
};

use libc;

pub trait NewFromPtr<T> {
    fn new(ptr: T) -> Self;
}

pub trait ToCString {
    fn to_cstring(&self) -> Result<ffi::CString, ffi::NulError>;
}

impl<T: AsRef<path::Path>> ToCString for T {
    fn to_cstring(&self) -> Result<ffi::CString, ffi::NulError> {
        let path: &ffi::OsStr = self.as_ref().as_ref();
        path.to_cstring()
    }
}

pub trait ToStr {
    fn to_str<'a>(&self) -> Result<&'a str, str::Utf8Error>;
}

impl ToStr for *const libc::c_char {
    fn to_str<'a>(&self) -> Result<&'a str, str::Utf8Error> {
        str::from_utf8(unsafe {
            ffi::CStr::from_ptr(*self)
        }.to_bytes())
    }
}
