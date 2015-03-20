use std::{
    ffi,
    str,
};

use std::os::unix::ffi::OsStrExt;

use libc;

pub trait NotmuchEnum {
    type NotmuchT;

    fn from_notmuch_t(notmuch_t: Self::NotmuchT) -> Self;
    fn to_notmuch_t(self) -> Self::NotmuchT;
}

pub trait ToCString {
    fn to_cstring(&self) -> Result<ffi::CString, ffi::NulError>;
}

impl<T: ffi::AsOsStr> ToCString for T {
    fn to_cstring(&self) -> Result<ffi::CString, ffi::NulError> {
        self.as_os_str().to_cstring()
    }
}

pub trait ToStr {
    fn to_str(&self) -> Result<&str, str::Utf8Error>;
}

impl ToStr for *const libc::c_char {
    fn to_str(&self) -> Result<&str, str::Utf8Error> {
        str::from_utf8(unsafe {
            ffi::CStr::from_ptr(*self)
        }.to_bytes())
    }
}

pub trait ToStaticStr {
    fn to_static_str(&self) -> Result<&'static str, str::Utf8Error>;
}

impl ToStaticStr for *const libc::c_char {
    fn to_static_str(&self) -> Result<&'static str, str::Utf8Error> {
        str::from_utf8(unsafe {
            ffi::CStr::from_ptr(*self)
        }.to_bytes())
    }
}
