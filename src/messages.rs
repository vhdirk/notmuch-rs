use std::ops::Drop;

use ffi;
use utils::ScopedPhantomcow;
use MessageOwner;
use Message;
use Tags;
use TagsOwner;
use Query;

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
pub struct Messages<'d, 'q>
where
    'd: 'q,
{
    pub(crate) handle: MessagesPtr,
    marker: ScopedPhantomcow<'q, Query<'d>>,
}

impl<'d, 'q> Messages<'d, 'q>
where
    'd: 'q,
{
    pub(crate) fn from_ptr<P>(ptr: *mut ffi::notmuch_messages_t, owner: P) -> Messages<'d, 'q>
    where
        P: Into<ScopedPhantomcow<'q, Query<'d>>>,
    {
        Messages {
            handle: MessagesPtr { ptr },
            marker: owner.into(),
        }
    }
}

impl<'d, 'q> MessageOwner for Messages<'d, 'q> where 'd: 'q {}
impl<'d, 'q> TagsOwner for Messages<'d, 'q> where 'd: 'q {}

impl<'d, 'q> Messages<'d, 'q>
where
    'd: 'q,
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
    pub fn collect_tags<'m>(self: &'m Self) -> Tags<'m, Self> {
        Tags::from_ptr(
            unsafe { ffi::notmuch_messages_collect_tags(self.handle.ptr) },
            self,
        )
    }
}

impl<'d, 'q> Iterator for Messages<'d, 'q>
where
    'd: 'q,
{
    type Item = Message<'d, 'q>;

    fn next(&mut self) -> Option<Self::Item> {
        let valid = unsafe { ffi::notmuch_messages_valid(self.handle.ptr) };

        if valid == 0 {
            return None;
        }

        let cthrd = unsafe {
            let thrd = ffi::notmuch_messages_get(self.handle.ptr);
            ffi::notmuch_messages_move_to_next(self.handle.ptr);
            thrd
        };

        Some(Message::from_ptr(cthrd, ScopedPhantomcow::<'q, Query<'d>>::share(&mut self.marker)))
    }
}



pub trait MessagesExt<'d, 'q>
where
    'd: 'q,
{
}

impl<'d, 'q> MessagesExt<'q, 'q> for Messages<'d, 'q> where 'd: 'q {}


unsafe impl<'d, 'q> Send for Messages<'d, 'q> where 'd: 'q {}
unsafe impl<'d, 'q> Sync for Messages<'d, 'q> where 'd: 'q {}
