use std;
use std::{
    ops,
    marker,
    ptr,
};

use error::Result;

use ffi;
use utils::{
    NewFromPtr,
};
use Database;

#[derive(Debug)]
pub struct Thread<'d>(
    pub(crate) *mut ffi::notmuch_thread_t,
    marker::PhantomData<&'d mut Database>,
);

impl<'d> NewFromPtr<*mut ffi::notmuch_thread_t> for Thread<'d> {
    fn new(ptr: *mut ffi::notmuch_thread_t) -> Thread<'d> {
        Thread(ptr, marker::PhantomData)
    }
}

impl<'d> ops::Drop for Thread<'d> {
    fn drop(&mut self) {
        unsafe {
            ffi::notmuch_thread_destroy(self.0)
        };
    }
}
