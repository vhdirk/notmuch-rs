use std::ops::Drop;

use ffi;
use utils::{ToStr, ScopedSupercow, ScopedPhantomcow};
use Messages;
use MessageOwner;
use Tags;
use TagsOwner;
use Query;

#[derive(Debug)]
pub struct Thread<'d, 'q>
where
    'd: 'q
{
    pub(crate) ptr: *mut ffi::notmuch_thread_t,
    pub(crate) marker: ScopedPhantomcow<'q, Query<'d>>,
}

impl<'d, 'q> Drop for Thread<'d, 'q>
where
    'd: 'q
{
    fn drop(&mut self) {
        unsafe { ffi::notmuch_thread_destroy(self.ptr) };
    }
}

impl<'d, 'q> MessageOwner for Thread<'d, 'q> where 'd: 'q {}
impl<'d, 'q> TagsOwner for Thread<'d, 'q> where 'd: 'q {}

impl<'d, 'q> Thread<'d, 'q>
where
    'd: 'q
{
    pub fn from_ptr<P>(ptr: *mut ffi::notmuch_thread_t, owner: P) -> Thread<'d, 'q>
    where
        P: Into<ScopedPhantomcow<'q, Query<'d>>>,
    {
        Thread {
            ptr,
            marker: owner.into(),
        }
    }

    pub fn id(self: &Self) -> String {
        let tid = unsafe { ffi::notmuch_thread_get_thread_id(self.ptr) };
        tid.to_str().unwrap().to_string()
    }

    pub fn total_messages(self: &Self) -> i32 {
        unsafe { ffi::notmuch_thread_get_total_messages(self.ptr) }
    }

    #[cfg(feature = "0.26")]
    pub fn total_files(self: &Self) -> i32 {
        unsafe { ffi::notmuch_thread_get_total_files(self.ptr) }
    }

    pub fn toplevel_messages(self: &Self) -> Messages<'_, Self> {
        <Self as ThreadExt<'d, 'q>>::toplevel_messages(self)
    }

    /// Get a `Messages` iterator for all messages in 'thread' in
    /// oldest-first order.
    pub fn messages(self: &Self) -> Messages<'_, Self> {
        <Self as ThreadExt<'d, 'q>>::messages(self)
    }

    pub fn tags(&self) -> Tags<'_, Self> {
        <Self as ThreadExt<'d, 'q>>::tags(self)
    }

    pub fn subject(self: &Self) -> String {
        let sub = unsafe { ffi::notmuch_thread_get_subject(self.ptr) };

        sub.to_str().unwrap().to_string()
    }

    pub fn authors(self: &Self) -> Vec<String> {
        let athrs = unsafe { ffi::notmuch_thread_get_authors(self.ptr) };

        athrs
            .to_str()
            .unwrap()
            .split(',')
            .map(|s| s.to_string())
            .collect()
    }

    /// Get the date of the oldest message in 'thread' as a time_t value.
    pub fn oldest_date(self: &Self) -> i64 {
        unsafe { ffi::notmuch_thread_get_oldest_date(self.ptr) as i64 }
    }

    /// Get the date of the newest message in 'thread' as a time_t value.
    pub fn newest_date(self: &Self) -> i64 {
        unsafe { ffi::notmuch_thread_get_newest_date(self.ptr) as i64 }
    }
}

pub trait ThreadExt<'d, 'q>
where
    'd: 'q
{
    fn tags<'s, S>(thread: S) -> Tags<'s, Thread<'d, 'q>>
    where
        S: Into<ScopedSupercow<'s, Thread<'d, 'q>>>,
    {
        let threadref = thread.into();
        Tags::from_ptr(
            unsafe { ffi::notmuch_thread_get_tags(threadref.ptr) },
            ScopedSupercow::phantom(threadref),
        )
    }

    fn toplevel_messages<'s, S>(thread: S) -> Messages<'s, Thread<'d, 'q>>
    where
        S: Into<ScopedSupercow<'s, Thread<'d, 'q>>>,
    {
        let threadref = thread.into();
        Messages::from_ptr(
            unsafe { ffi::notmuch_thread_get_toplevel_messages(threadref.ptr) },
            ScopedSupercow::phantom(threadref),
        )
    }

    /// Get a `Messages` iterator for all messages in 'thread' in
    /// oldest-first order.
    fn messages<'s, S>(thread: S) -> Messages<'s, Thread<'d, 'q>>
    where
        S: Into<ScopedSupercow<'s, Thread<'d, 'q>>>,
    {
        let threadref = thread.into();
        Messages::from_ptr(
            unsafe { ffi::notmuch_thread_get_messages(threadref.ptr) },
            ScopedSupercow::phantom(threadref),
        )
    }
}

impl<'d, 'q> ThreadExt<'d, 'q> for Thread<'d, 'q> where 'd: 'q {}

unsafe impl<'d, 'q> Send for Thread<'d, 'q> where 'd: 'q {}
unsafe impl<'d, 'q> Sync for Thread<'d, 'q> where 'd: 'q {}
