use std::ops::Drop;

use supercow::{Phantomcow, Supercow};

use crate::ffi;
use crate::utils::{ScopedSupercow, ScopedPhantomcow};
use crate::Message;
use crate::MessageOwner;
use crate::Tags;
use crate::TagsOwner;

#[derive(Debug)]
pub struct MessagesPtr {
    pub ptr: *mut ffi::notmuch_messages_t,
}

impl Drop for MessagesPtr {
    fn drop(self: &mut Self) {
        unsafe { ffi::notmuch_messages_destroy(self.ptr) };
    }
}

#[derive(Debug)]
pub struct Messages<'o, O>
where
    O: MessageOwner,
{
    pub(crate) handle: MessagesPtr,
    marker: ScopedPhantomcow<'o, O>,
}

impl<'o, O> Messages<'o, O>
where
    O: MessageOwner + 'o,
{
    pub(crate) fn from_ptr<P>(ptr: *mut ffi::notmuch_messages_t, owner: P) -> Messages<'o, O>
    where
        P: Into<ScopedPhantomcow<'o, O>>,
    {
        Messages {
            handle: MessagesPtr { ptr },
            marker: owner.into(),
        }
    }
}

impl<'o, O> MessageOwner for Messages<'o, O> where O: MessageOwner + 'o {}
impl<'o, O> TagsOwner for Messages<'o, O> where O: MessageOwner + 'o {}

impl<'o, O> Messages<'o, O>
where
    O: MessageOwner + 'o,
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

pub trait MessagesExt<'o, O>
where
    O: MessageOwner + 'o,
{
}

impl<'o, O> MessagesExt<'o, O> for Messages<'o, O> where O: MessageOwner + 'o {}


unsafe impl<'o, O> Send for Messages<'o, O> where O: MessageOwner + 'o {}
unsafe impl<'o, O> Sync for Messages<'o, O> where O: MessageOwner + 'o {}
