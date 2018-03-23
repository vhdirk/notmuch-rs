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
pub struct Messages<'q, 'd:'q>(
    // TODO: is this lifetime specifier correct?
    // query may outlive messages.
    pub(crate) *mut ffi::notmuch_messages_t,
    marker::PhantomData<&'q Query<'d>>
);

impl<'q, 'd:'q> NewFromPtr<*mut ffi::notmuch_messages_t> for Messages<'q, 'd> {
    fn new(ptr: *mut ffi::notmuch_messages_t) -> Messages<'q, 'd> {
        Messages(ptr, marker::PhantomData)
    }
}



impl<'q, 'd:'q> ops::Drop for Messages<'q, 'd> {
    fn drop(&mut self) {
        unsafe {
            ffi::notmuch_messages_destroy(self.0)
        };
    }
}
