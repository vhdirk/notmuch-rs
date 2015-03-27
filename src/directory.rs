use std::{
    marker,
};

use utils::{
    NewFromPtr,
};

use database;

use ffi;

pub struct Directory<'d>(
    *mut ffi::notmuch_directory_t,
    marker::PhantomData<&'d mut database::Database>,
);

impl<'d> NewFromPtr<*mut ffi::notmuch_directory_t> for Directory<'d> {
    fn new(ptr: *mut ffi::notmuch_directory_t) -> Directory<'d> {
        Directory(ptr, marker::PhantomData)
    }
}
