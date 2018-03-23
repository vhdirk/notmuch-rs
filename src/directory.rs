use std::{
    ops,
    marker,
};

use utils::{
    NewFromPtr,
};

use database;

use ffi;

#[derive(Debug)]
pub struct Directory<'d>(
    *mut ffi::notmuch_directory_t,
    marker::PhantomData<&'d mut database::Database>,
);

impl<'d> NewFromPtr<*mut ffi::notmuch_directory_t> for Directory<'d> {
    fn new(ptr: *mut ffi::notmuch_directory_t) -> Directory<'d> {
        Directory(ptr, marker::PhantomData)
    }
}

impl<'d> ops::Drop for Directory<'d> {
    fn drop(&mut self) {
        unsafe {
            ffi::notmuch_directory_destroy(self.0)
        };
    }
}
