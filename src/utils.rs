use std::{
    ffi,
    str,
};
use libc;

pub trait FromPtr<T> {
    fn from_ptr(ptr: T) -> Self;
}

// pub trait NewFromPtr<T, P> {
//     fn new(ptr: T, parent: Rc<P>) -> Self;
// }

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

pub trait ToString {
    fn to_string(&self) -> String;
}

impl ToString for *const libc::c_char {
    fn to_string(&self) -> String {
        unsafe {
            ffi::CStr::from_ptr(*self).to_string_lossy().into_owned()
        }
    }
}


pub struct Owner;


