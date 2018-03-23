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
use Query;

#[derive(Debug)]
pub struct Message<'q, 'd:'q>(
    pub(crate) *mut ffi::notmuch_message_t,
    marker::PhantomData<&'q mut Query<'d>>,
);

impl<'q, 'd> NewFromPtr<*mut ffi::notmuch_message_t> for Message<'q, 'd> {
    fn new(ptr: *mut ffi::notmuch_message_t) -> Message<'q, 'd> {
        Message(ptr, marker::PhantomData)
    }
}


impl<'q, 'd> ops::Drop for Message<'q, 'd> {
    fn drop(&mut self) {
        unsafe {
            ffi::notmuch_message_destroy(self.0)
        };
    }
}
