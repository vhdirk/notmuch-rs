use std::{
    ops,
    marker,
    iter
};

use std::path::{
    PathBuf,
    Path
};

use std::ffi::{
    CString,
    CStr
};

use utils::{
    NewFromPtr,
};

use database;
use ffi;

#[derive(Debug)]
pub struct Filenames<'d>(
    *mut ffi::notmuch_filenames_t,
    marker::PhantomData<&'d mut database::Database>,
);

impl<'d> NewFromPtr<*mut ffi::notmuch_filenames_t> for Filenames<'d> {
    fn new(ptr: *mut ffi::notmuch_filenames_t) -> Filenames<'d> {
        Filenames(ptr, marker::PhantomData)
    }
}

impl<'d> ops::Drop for Filenames<'d> {
    fn drop(&mut self) {
        unsafe {
            ffi::notmuch_filenames_destroy(self.0)
        };
    }
}

impl<'d> iter::Iterator for Filenames<'d> {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {

        let valid = unsafe {
            ffi::notmuch_filenames_valid(self.0)
        };

        if valid == 0{
            return None
        }

        let ctag = unsafe {
            ffi::notmuch_filenames_move_to_next(self.0);
            CStr::from_ptr(ffi::notmuch_filenames_get(self.0))
        };

        Some(PathBuf::from(ctag.to_str().unwrap()))
    }
}
