use std::ops::Drop;
use std::iter::Iterator;
use std::rc::Rc;

use ffi;
use utils::{
    FromPtr,
    NewFromPtr
};
use query::Query;
use Message;
use Tags;

#[derive(Debug)]
pub(crate) struct MessagesPtr {
    pub(crate) ptr: *mut ffi::notmuch_messages_t
}

impl Drop for MessagesPtr {
    fn drop(&mut self) {

        let valid = unsafe {
            ffi::notmuch_messages_valid(self.ptr)
        };

        if valid != 0 {
            unsafe {
                ffi::notmuch_messages_destroy(self.ptr)
            };
        }
    }
}


#[derive(Debug)]
pub struct Messages(pub(crate) Rc<MessagesPtr>, Query);


impl NewFromPtr<*mut ffi::notmuch_messages_t, Query> for Messages {
    fn new(ptr: *mut ffi::notmuch_messages_t, parent: Query) -> Messages {
        Messages(Rc::new(MessagesPtr{ptr}), parent)
    }
}

impl Messages{

    pub fn collect_tags(self: &Self) -> Tags{
        Tags::from_ptr(unsafe {
            ffi::notmuch_messages_collect_tags(self.0.ptr)
        })
    }

}

impl Iterator for Messages {
    type Item = Message;

    fn next(&mut self) -> Option<Self::Item> {

        let valid = unsafe {
            ffi::notmuch_messages_valid(self.0.ptr)
        };

        if valid == 0{
            return None
        }

        let cmsg = unsafe {
            let msg = ffi::notmuch_messages_get(self.0.ptr);
            ffi::notmuch_messages_move_to_next(self.0.ptr);
            msg
        };

        Some(Self::Item::new(cmsg, self.1.clone()))
    }
}
