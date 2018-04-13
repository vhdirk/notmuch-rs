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
use Query;
use Message;
use Tags;

#[derive(Debug)]
pub struct Messages<'q, 'd:'q>(
    // TODO: is this lifetime specifier correct?
    // query may outlive messages.
    pub(crate) *mut ffi::notmuch_messages_t,
    marker::PhantomData<&'q mut Query<'d>>,
);

impl<'q, 'd> NewFromPtr<*mut ffi::notmuch_messages_t> for Messages<'q, 'd> {
    fn new(ptr: *mut ffi::notmuch_messages_t) -> Messages<'q, 'd> {
        Messages(ptr, marker::PhantomData)
    }
}

impl<'q, 'd> Messages<'q, 'd>{

    pub fn collect_tags(self: &Self) -> Tags{
        Tags::new(unsafe {
            ffi::notmuch_messages_collect_tags(self.0)
        })
    }

}

impl<'q, 'd> ops::Drop for Messages<'q, 'd> {
    fn drop(&mut self) {
        unsafe {
            ffi::notmuch_messages_destroy(self.0)
        };
    }
}

impl<'q, 'd> iter::Iterator for Messages<'q, 'd> {
    type Item = Message<'q, 'd>;

    fn next(&mut self) -> Option<Self::Item> {

        let valid = unsafe {
            ffi::notmuch_messages_valid(self.0)
        };

        if valid == 0{
            return None
        }

        let cmsg = unsafe {
            let msg = ffi::notmuch_messages_get(self.0);
            ffi::notmuch_messages_move_to_next(self.0);
            msg
        };

        Some(Self::Item::new(cmsg))
    }
}

unsafe impl<'q, 'd> Send for Messages<'q, 'd>{}
