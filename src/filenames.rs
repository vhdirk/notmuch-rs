use std::ops::Drop;
use std::iter::Iterator;
use std::marker::PhantomData;
use std::path::PathBuf;
use std::ffi::CStr;

use utils::{
    NewFromPtr,
};

use Database;
use ffi;

#[derive(Debug)]
pub struct Filenames<'d>(
    *mut ffi::notmuch_filenames_t,
    PhantomData<&'d Database>,
);

impl<'d> NewFromPtr<*mut ffi::notmuch_filenames_t> for Filenames<'d> {
    fn new(ptr: *mut ffi::notmuch_filenames_t) -> Filenames<'d> {
        Filenames(ptr, PhantomData)
    }
}

impl<'d> Drop for Filenames<'d> {
    fn drop(self: &mut Self) {
        let valid = unsafe {
            ffi::notmuch_filenames_valid(self.0)
        };

        if valid != 0 {
            unsafe {
                ffi::notmuch_filenames_destroy(self.0)
            };
        }
    }
}

impl<'d> Iterator for Filenames<'d> {
    type Item = PathBuf;

    fn next(self: &mut Self) -> Option<Self::Item> {

        let valid = unsafe {
            ffi::notmuch_filenames_valid(self.0)
        };

        if valid == 0{
            return None
        }

        let ctag = unsafe {
            let t = ffi::notmuch_filenames_get(self.0);
            ffi::notmuch_filenames_move_to_next(self.0);
            CStr::from_ptr(t)
        };

        Some(PathBuf::from(ctag.to_str().unwrap()))
    }
}


unsafe impl<'d> Send for Filenames<'d>{}
unsafe impl<'d> Sync for Filenames<'d>{}
