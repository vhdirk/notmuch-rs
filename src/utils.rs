use libc;
use std::{ffi, str};

use supercow::{Supercow, DefaultFeatures, NonSyncFeatures};
use supercow::ext::{BoxedStorage};

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

pub type ScopedNonSyncSupercow<'a, OWNED, BORROWED = OWNED> =
    Supercow<'a, OWNED, BORROWED,
             Box<NonSyncFeatures<'a> + 'a>,
             BoxedStorage>;

pub type ScopedPhantomcow<'a, OWNED, BORROWED = OWNED, 
                              SHARED = Box<DefaultFeatures<'a> + 'a>,
                              STORAGE = BoxedStorage> =
    Supercow<'a, OWNED, BORROWED, SHARED, STORAGE, ()>;

pub type ScopedSupercow<'a, OWNED, BORROWED = OWNED, SHARED = Box<DefaultFeatures<'a> + 'a>> =
    Supercow<'a, OWNED, BORROWED, SHARED, BoxedStorage>;



