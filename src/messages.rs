use std::ops::Drop;
use std::iter::Iterator;
use std::marker::PhantomData;

use ffi;
use utils::{
    FromPtr,
};
use Query;
use Message;
use Tags;
use message::MessageOwner;

pub trait MessagesOwner{
}

#[derive(Debug)]
pub(crate) struct MessagesPtr {
    pub ptr: *mut ffi::notmuch_messages_t
}

impl Drop for MessagesPtr {
    fn drop(self: &mut Self) {
        let valid = unsafe {
            ffi::notmuch_messages_valid(self.ptr)
        };

        if valid == 0{
            return;
        }

        unsafe {
            ffi::notmuch_messages_destroy(self.ptr)
        };
    }
}


#[derive(Debug)]
pub struct Messages<'o, Owner: MessagesOwner>{
    pub(crate) handle: MessagesPtr,
    phantom: PhantomData<&'o Owner>,
}

impl<'o, Owner: MessagesOwner> FromPtr<*mut ffi::notmuch_messages_t> for Messages<'o, Owner> {
    fn from_ptr(ptr: *mut ffi::notmuch_messages_t) -> Messages<'o, Owner> {
        Messages{
            handle: MessagesPtr{ptr},
            phantom: PhantomData
        }
    }
}

impl<'o, Owner: MessagesOwner> MessageOwner for Messages<'o, Owner>{}


impl<'o, Owner: MessagesOwner> Messages<'o, Owner>{

    /**
     * Return a list of tags from all messages.
     *
     * The resulting list is guaranteed not to contain duplicated tags.
     *
     * WARNING: You can no longer iterate over messages after calling this
     * function, because the iterator will point at the end of the list.
     * We do not have a function to reset the iterator yet and the only
     * way how you can iterate over the list again is to recreate the
     * message list.
     *
     * The function returns NULL on error.
     */
    pub fn collect_tags(self: &'o Self) -> Tags{
        Tags::from_ptr(unsafe {
            ffi::notmuch_messages_collect_tags(self.handle.ptr)
        })
    }
}



impl<'o, Owner: MessagesOwner> Iterator for Messages<'o, Owner> {
    type Item = Message<'o, Self>;

    fn next(&mut self) -> Option<Self::Item> {

        let valid = unsafe {
            ffi::notmuch_messages_valid(self.handle.ptr)
        };

        if valid == 0{
            return None
        }

        let cmsg = unsafe {
            let msg = ffi::notmuch_messages_get(self.handle.ptr);
            ffi::notmuch_messages_move_to_next(self.handle.ptr);
            msg
        };

        Some(Self::Item::from_ptr(cmsg))
    }
}

unsafe impl<'o, Owner: MessagesOwner> Send for Messages<'o, Owner>{}
unsafe impl<'o, Owner: MessagesOwner> Sync for Messages<'o, Owner>{}
