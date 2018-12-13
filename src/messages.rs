use std::ops::Drop;

use supercow::{Phantomcow, Supercow};

use crate::ffi;
use crate::utils::{StreamingIterator, StreamingIteratorExt};
use crate::Message;
use crate::MessageOwner;
use crate::Tags;
use crate::TagsOwner;

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
pub struct Messages<'o, O>
where
    O: MessagesOwner,
{
    pub(crate) handle: MessagesPtr,
    marker: Phantomcow<'o, O>,
}

impl<'o, O> Messages<'o, O>
where
    O: MessagesOwner + 'o,
{
    pub(crate) fn from_ptr<P>(ptr: *mut ffi::notmuch_messages_t, owner: P) -> Messages<'o, O>
    where
        P: Into<Phantomcow<'o, O>>,
    {
        Messages {
            handle: MessagesPtr { ptr },
            marker: owner.into(),
        }
    }
}

impl<'o, O> MessageOwner for Messages<'o, O> where O: MessagesOwner + 'o {}
impl<'o, O> TagsOwner for Messages<'o, O> where O: MessagesOwner + 'o {}

impl<'o, O> Messages<'o, O>
where
    O: MessagesOwner + 'o,
{
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

impl<'s, 'o: 's, O> StreamingIterator<'s, Message<'s, Self>> for Messages<'o, O>
where
    O: MessagesOwner + 'o,
{
    fn next(&'s mut self) -> Option<Message<'s, Self>> {
        <Self as StreamingIteratorExt<'s, Message<'s, Self>>>::next(Supercow::borrowed(self))
    }
}

pub trait MessagesExt<'o, O>
where
    O: MessagesOwner + 'o,
{
}

impl<'o, O> MessagesExt<'o, O> for Messages<'o, O> where O: MessagesOwner + 'o {}

impl<'s, 'o: 's, O> StreamingIteratorExt<'s, Message<'s, Self>> for Messages<'o, O>
where
    O: MessagesOwner + 'o,
{
    fn next<S>(messages: S) -> Option<Message<'s, Self>>
    where
        S: Into<Supercow<'s, Messages<'o, O>>>,
    {
        let messagesref = messages.into();
        let valid = unsafe { ffi::notmuch_messages_valid(messagesref.handle.ptr) };

        if valid == 0 {
            return None;
        }

        let cmsg = unsafe {
            let msg = ffi::notmuch_messages_get(messagesref.handle.ptr);
            ffi::notmuch_messages_move_to_next(messagesref.handle.ptr);
            msg
        };

        Some(Message::from_ptr(cmsg, Supercow::phantom(messagesref)))
    }
}

unsafe impl<'o, O> Send for Messages<'o, O> where O: MessagesOwner + 'o {}
unsafe impl<'o, O> Sync for Messages<'o, O> where O: MessagesOwner + 'o {}
