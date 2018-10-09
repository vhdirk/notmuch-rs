use std::ops::Drop;
use std::iter::Iterator;
use std::marker::PhantomData;

use ffi;
use utils::{
    NewFromPtr,
};
use Query;
use Message;
use Tags;

#[derive(Debug)]
pub struct Messages<'d:'q, 'q>(
    // TODO: is this lifetime specifier correct?
    // query may outlive messages.
    pub(crate) *mut ffi::notmuch_messages_t,
    PhantomData<&'q Query<'d>>,
);

impl<'d, 'q> NewFromPtr<*mut ffi::notmuch_messages_t> for Messages<'d, 'q> {
    fn new(ptr: *mut ffi::notmuch_messages_t) -> Messages<'d, 'q> {
        Messages(ptr, PhantomData)
    }
}

impl<'d, 'q> Messages<'d, 'q>{

    pub fn collect_tags(self: &'d Self) -> Tags<'d>{
        Tags::new(unsafe {
            ffi::notmuch_messages_collect_tags(self.0)
        })
    }

}

impl<'d, 'q> Drop for Messages<'d, 'q> {
    fn drop(self: &mut Self) {
        let valid = unsafe {
            ffi::notmuch_messages_valid(self.0)
        };

        if valid == 0{
            return;
        }

        unsafe {
            ffi::notmuch_messages_destroy(self.0)
        };
    }
}

impl<'d, 'q> Iterator for Messages<'d, 'q> {
    type Item = Message<'d, 'q>;

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

unsafe impl<'d, 'q> Send for Messages<'d, 'q>{}
unsafe impl<'d, 'q> Sync for Messages<'d, 'q>{}
