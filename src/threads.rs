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
pub struct Threads<'d>(
    *mut ffi::notmuch_threads_t,
    marker::PhantomData<&'d mut database::Database>,
);

impl<'d> NewFromPtr<*mut ffi::notmuch_threads_t> for Threads<'d> {
    fn new(ptr: *mut ffi::notmuch_threads_t) -> Threads<'d> {
        Threads(ptr, marker::PhantomData)
    }
}

impl<'d> ops::Drop for Threads<'d> {
    fn drop(&mut self) {
        unsafe {
            ffi::notmuch_threads_destroy(self.0)
        };
    }
}
