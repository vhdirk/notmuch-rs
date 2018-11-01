use std::ops::Drop;

use supercow::{Phantomcow, Supercow};

use ffi;
use utils::StreamingIterator;
use Message;
use MessageOwner;
use Tags;
use TagsOwner;

pub trait MessagesOwner {}

#[derive(Debug)]
pub struct MessagesPtr {
    pub ptr: *mut ffi::notmuch_messages_t,
}

impl Drop for MessagesPtr {
    fn drop(self: &mut Self) {
        let valid = unsafe { ffi::notmuch_messages_valid(self.ptr) };

        if valid == 0 {
            return;
        }

        unsafe { ffi::notmuch_messages_destroy(self.ptr) };
    }
}

#[derive(Debug)]
pub struct Messages<'o, Owner: MessagesOwner + 'o> {
    pub(crate) handle: MessagesPtr,
    marker: Phantomcow<'o, Owner>,
}

impl<'o, Owner: MessagesOwner + 'o> Messages<'o, Owner> {
    pub fn from_ptr<O: Into<Phantomcow<'o, Owner>>>(
        ptr: *mut ffi::notmuch_messages_t,
        owner: O,
    ) -> Messages<'o, Owner> {
        Messages {
            handle: MessagesPtr { ptr },
            marker: owner.into(),
        }
    }
}

impl<'o, Owner: MessagesOwner + 'o> MessageOwner for Messages<'o, Owner> {}
impl<'o, Owner: MessagesOwner + 'o> TagsOwner for Messages<'o, Owner> {}

impl<'o, Owner: MessagesOwner + 'o> Messages<'o, Owner> {
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
    pub fn collect_tags<'m>(self: &'o Self) -> Tags<'m, Self> {
        Tags::from_ptr(
            unsafe { ffi::notmuch_messages_collect_tags(self.handle.ptr) },
            self,
        )
    }
}

impl<'s, 'o: 's, Owner: MessagesOwner + 'o> StreamingIterator<'s, Message<'s, Self>>
    for Messages<'o, Owner>
{
    fn next(&'s mut self) -> Option<Message<'s, Self>> {
        let valid = unsafe { ffi::notmuch_messages_valid(self.handle.ptr) };

        if valid == 0 {
            return None;
        }

        let cmsg = unsafe {
            let msg = ffi::notmuch_messages_get(self.handle.ptr);
            ffi::notmuch_messages_move_to_next(self.handle.ptr);
            msg
        };

        Some(Message::from_ptr(cmsg, Supercow::borrowed(self)))
    }
}

unsafe impl<'o, Owner: MessagesOwner + 'o> Send for Messages<'o, Owner> {}
unsafe impl<'o, Owner: MessagesOwner + 'o> Sync for Messages<'o, Owner> {}
