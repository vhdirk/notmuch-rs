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
pub struct Message<'d>(
    pub(crate) *mut ffi::notmuch_message_t,
    marker::PhantomData<&'d mut Database>,
);

impl<'d> NewFromPtr<*mut ffi::notmuch_message_t> for Message<'d> {
    fn new(ptr: *mut ffi::notmuch_message_t) -> Message<'d> {
        Message(ptr, marker::PhantomData)
    }
}


impl<'d> ops::Drop for Message<'d> {
    fn drop(&mut self) {
        unsafe {
            ffi::notmuch_message_destroy(self.0)
        };
    }
}
