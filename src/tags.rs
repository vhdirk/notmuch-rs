use std::{
    ops,
    marker,
    iter
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
pub struct Tags<'d>(
    *mut ffi::notmuch_tags_t,
    marker::PhantomData<&'d database::Database>,
);

impl<'d> NewFromPtr<*mut ffi::notmuch_tags_t> for Tags<'d> {
    fn new(ptr: *mut ffi::notmuch_tags_t) -> Tags<'d> {
        Tags(ptr, marker::PhantomData)
    }
}

impl<'d> ops::Drop for Tags<'d> {
    fn drop(&mut self) {
        unsafe {
            ffi::notmuch_tags_destroy(self.0)
        };
    }
}

impl<'d> iter::Iterator for Tags<'d> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {

        let valid = unsafe {
            ffi::notmuch_tags_valid(self.0)
        };

        if valid == 0{
            return None
        }

        let ctag = unsafe {
            let t = ffi::notmuch_tags_get(self.0);
            ffi::notmuch_tags_move_to_next(self.0);

            CStr::from_ptr(t)
        };

        Some(ctag.to_str().unwrap().to_string())
    }
}

unsafe impl<'d> Send for Tags<'d>{}
unsafe impl<'d> Sync for Tags<'d>{}
