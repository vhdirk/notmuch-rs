use std::{
    ops,
    marker
};

use error::Result;

use ffi;
use utils::{
    NewFromPtr,
};
use Database;
use Query;

#[derive(Debug)]
pub struct Messages<'q>(
    // TODO: is this lifetime specifier correct?
    pub(crate) *mut ffi::notmuch_messages_t,
    marker::PhantomData<&'q Query<'q>>
);

impl<'q> NewFromPtr<*mut ffi::notmuch_messages_t> for Messages<'q> {
    fn new(ptr: *mut ffi::notmuch_messages_t) -> Messages<'q> {
        Messages(ptr, marker::PhantomData)
    }
}


impl<'q> ops::Drop for Messages<'q> {
    fn drop(&mut self) {
        unsafe {
            ffi::notmuch_messages_destroy(self.0)
        };
    }
}
