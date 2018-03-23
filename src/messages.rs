use std::{
    ops,
    marker,
    iter
};

use error::Result;

use ffi;
use utils::{
    NewFromPtr,
};
use Database;
use Message;

#[derive(Debug)]
pub struct Messages<'d>(
    // TODO: is this lifetime specifier correct?
    // query may outlive messages.
    pub(crate) *mut ffi::notmuch_messages_t,
    marker::PhantomData<&'d mut Database>,
);

impl<'d> NewFromPtr<*mut ffi::notmuch_messages_t> for Messages<'d> {
    fn new(ptr: *mut ffi::notmuch_messages_t) -> Messages<'d> {
        Messages(ptr, marker::PhantomData)
    }
}

impl<'d> ops::Drop for Messages<'d> {
    fn drop(&mut self) {
        unsafe {
            ffi::notmuch_messages_destroy(self.0)
        };
    }
}

impl<'d> iter::Iterator for Messages<'d> {
    type Item = Message<'d>;

    fn next(&mut self) -> Option<Self::Item> {

        let valid = unsafe {
            ffi::notmuch_messages_valid(self.0)
        };

        if valid == 0{
            return None
        }

        let cmsg = unsafe {
            ffi::notmuch_messages_move_to_next(self.0);
            ffi::notmuch_messages_get(self.0)
        };

        Some(Message::new(cmsg))
    }
}
