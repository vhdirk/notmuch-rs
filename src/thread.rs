use std::borrow::Cow;
use std::ops::Drop;
use std::rc::Rc;

use from_variants::FromVariants;

use ffi;
use utils::ToStr;
use Query;
use Threads;
use Messages;
use Tags;

#[derive(Clone, Debug, FromVariants)]
pub(crate) enum ThreadOwner {
    Query(Query),
    Threads(Threads),
}

#[derive(Debug)]
pub(crate) struct ThreadPtr(*mut ffi::notmuch_thread_t);

impl Drop for ThreadPtr {
    fn drop(&mut self) {
        unsafe { ffi::notmuch_thread_destroy(self.0) };
    }
}

#[derive(Clone, Debug)]
pub struct Thread {
    ptr: Rc<ThreadPtr>,
    owner: Box<ThreadOwner>,
}

impl Thread {
    pub(crate) fn from_ptr<P>(ptr: *mut ffi::notmuch_thread_t, owner: P) -> Thread
    where
        P: Into<ThreadOwner>,
    {
        Thread {
            ptr: Rc::new(ThreadPtr(ptr)),
            owner: Box::new(owner.into()),
        }
    }

    pub fn id(&self) -> &str {
        let tid = unsafe { ffi::notmuch_thread_get_thread_id(self.ptr.0) };
        tid.to_str().unwrap()
    }

    pub fn total_messages(&self) -> i32 {
        unsafe { ffi::notmuch_thread_get_total_messages(self.ptr.0) }
    }

    #[cfg(feature = "0.26")]
    pub fn total_files(&self) -> i32 {
        unsafe { ffi::notmuch_thread_get_total_files(self.ptr.0) }
    }

    pub fn toplevel_messages(&self) -> Messages {
        Messages::from_ptr(
            unsafe { ffi::notmuch_thread_get_toplevel_messages(self.ptr.0) },
            self.clone(),
        )
    }

    pub fn matched_messages(&self) -> i32 {
        unsafe { ffi::notmuch_thread_get_matched_messages(self.ptr.0) }
    }

    /// Get a `Messages` iterator for all messages in 'thread' in
    /// oldest-first order.
    pub fn messages(&self) -> Messages {
        Messages::from_ptr(
            unsafe { ffi::notmuch_thread_get_messages(self.ptr.0) },
            self.clone(),
        )
    }

    pub fn tags(&self) -> Tags {
        Tags::from_ptr(
            unsafe { ffi::notmuch_thread_get_tags(self.ptr.0) },
            self.clone(),
        )
    }

    pub fn subject(&self) -> Cow<'_, str> {
        let sub = unsafe { ffi::notmuch_thread_get_subject(self.ptr.0) };
        sub.to_string_lossy()
    }

    pub fn authors(&self) -> Vec<String> {
        let athrs = unsafe { ffi::notmuch_thread_get_authors(self.ptr.0) };

        athrs
            .to_string_lossy()
            .split(',')
            .map(|s| s.to_string())
            .collect()
    }

    /// Get the date of the oldest message in 'thread' as a time_t value.
    pub fn oldest_date(&self) -> i64 {
        unsafe { ffi::notmuch_thread_get_oldest_date(self.ptr.0) as i64 }
    }

    /// Get the date of the newest message in 'thread' as a time_t value.
    pub fn newest_date(&self) -> i64 {
        unsafe { ffi::notmuch_thread_get_newest_date(self.ptr.0) as i64 }
    }
}
