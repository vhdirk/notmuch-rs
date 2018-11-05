use libc;
use std::{ffi, str};

use supercow::Supercow;

pub trait ToStr {
    fn to_str<'a>(&self) -> Result<&'a str, str::Utf8Error>;
}

impl ToStr for *const libc::c_char {
    fn to_str<'a>(&self) -> Result<&'a str, str::Utf8Error> {
        str::from_utf8(unsafe { ffi::CStr::from_ptr(*self) }.to_bytes())
    }
}

pub trait ToString {
    fn to_string(&self) -> String;
}

impl ToString for *const libc::c_char {
    fn to_string(&self) -> String {
        unsafe { ffi::CStr::from_ptr(*self).to_string_lossy().into_owned() }
    }
}

/// A streaming iterator, as found in https://github.com/emk/rust-streaming
pub trait StreamingIterator<'a, T> {
    /// Return either the next item in the sequence, or `None` if all items
    /// have been consumed.
    fn next(&'a mut self) -> Option<T>;
}

pub trait StreamingIteratorExt<'a, T> {
    /// Return either the next item in the sequence, or `None` if all items
    /// have been consumed.
    fn next<S: Into<Supercow<'a, Self>>>(s: S) -> Option<T>
      where Self: Sized + 'a;
}

