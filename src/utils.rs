use libc;
use std::{ffi, str};
use std::borrow::Cow;
use supercow::{Supercow, DefaultFeatures/*, NonSyncFeatures*/};
use supercow::ext::{BoxedStorage};

pub trait ToStr {
    fn to_str<'a>(&self) -> Result<&'a str, str::Utf8Error>;

    fn to_str_unchecked<'a>(&self) -> &'a str;

    fn to_string_lossy<'a>(&self) -> Cow<'a, str>;
}

impl ToStr for *const libc::c_char {
    fn to_str<'a>(&self) -> Result<&'a str, str::Utf8Error> {
        str::from_utf8(unsafe { ffi::CStr::from_ptr(*self) }.to_bytes())
    }

    fn to_str_unchecked<'a>(&self) -> &'a str {
        unsafe { str::from_utf8_unchecked(ffi::CStr::from_ptr(*self).to_bytes()) }
    }

    fn to_string_lossy<'a>(&self) -> Cow<'a, str> {
        unsafe { ffi::CStr::from_ptr(*self) }.to_string_lossy()
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

// pub type ScopedNonSyncSupercow<'a, OWNED, BORROWED = OWNED> =
//     Supercow<'a, OWNED, BORROWED,
//              Box<NonSyncFeatures<'a> + 'a>,
//              BoxedStorage>;

pub type ScopedPhantomcow<'a, OWNED, BORROWED = OWNED, 
                              SHARED = Box<dyn DefaultFeatures<'a> + 'a>,
                              STORAGE = BoxedStorage> =
    Supercow<'a, OWNED, BORROWED, SHARED, STORAGE, ()>;

pub type ScopedSupercow<'a, OWNED, BORROWED = OWNED, SHARED = Box<dyn DefaultFeatures<'a> + 'a>> =
    Supercow<'a, OWNED, BORROWED, SHARED, BoxedStorage>;





