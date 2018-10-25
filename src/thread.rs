use std::ops::Drop;
use std::marker::PhantomData;
use ffi;
use utils::{
    FromPtr,
    ToStr
};
use Query;
use Messages;
use Tags;
use messages::MessagesOwner;
use tags::TagsOwner;

pub trait ThreadOwner{}


#[derive(Debug)]
pub(crate) struct ThreadPtr {
    pub ptr: *mut ffi::notmuch_thread_t
}

impl Drop for ThreadPtr {
    fn drop(&mut self) {
        unsafe {
            ffi::notmuch_thread_destroy(self.ptr)
        };
    }
}


#[derive(Debug)]
pub struct Thread<'o, Owner: ThreadOwner>{
    pub(crate) handle: ThreadPtr,
    phantom: PhantomData<&'o Owner>,
}

impl<'o, Owner: ThreadOwner> MessagesOwner for Thread<'o, Owner>{}
impl<'o, Owner: ThreadOwner> TagsOwner for Thread<'o, Owner>{}


impl<'o, Owner: ThreadOwner> FromPtr<*mut ffi::notmuch_thread_t> for Thread<'o, Owner> {
    fn from_ptr(ptr: *mut ffi::notmuch_thread_t) -> Thread<'o, Owner> {
        Thread{
            handle: ThreadPtr{ptr},
            phantom: PhantomData
        }
    }
}

impl<'o, Owner: ThreadOwner> Thread<'o, Owner>{

    pub fn id(self: &Self) -> String{
        let tid = unsafe {
            ffi::notmuch_thread_get_thread_id(self.handle.ptr)
        };
        tid.to_str().unwrap().to_string()
    }


    pub fn total_messages(self: &Self) -> i32{
        unsafe {
            ffi::notmuch_thread_get_total_messages(self.handle.ptr)
        }
    }

    #[cfg(feature = "0.26")]
    pub fn total_files(self: &Self) -> i32{
        unsafe {
            ffi::notmuch_thread_get_total_files(self.handle.ptr)
        }
    }


    pub fn toplevel_messages(self: &Self) -> Messages<Self>{
        Messages::from_ptr(unsafe {
            ffi::notmuch_thread_get_toplevel_messages(self.handle.ptr)
        })
    }

    /// Get a `Messages` iterator for all messages in 'thread' in
    /// oldest-first order.
    pub fn messages(self: &Self) -> Messages<Self>{
        Messages::from_ptr(unsafe {
            ffi::notmuch_thread_get_messages(self.handle.ptr)
        })
    }


    pub fn tags<'t>(self: &Self) -> Tags{
        Tags::from_ptr(unsafe {
            ffi::notmuch_thread_get_tags(self.handle.ptr)
        })
    }

    pub fn subject(self: &Self) -> String{
        let sub = unsafe {
            ffi::notmuch_thread_get_subject(self.handle.ptr)
        };

        sub.to_str().unwrap().to_string()
    }

    pub fn authors(self: &Self) -> Vec<String>{
        let athrs = unsafe {
            ffi::notmuch_thread_get_authors(self.handle.ptr)
        };

        athrs.to_str().unwrap().split(',').map(|s| s.to_string()).collect()
    }

    /// Get the date of the oldest message in 'thread' as a time_t value.
    pub fn oldest_date(self: &Self) -> i64 {
        unsafe {
            ffi::notmuch_thread_get_oldest_date(self.handle.ptr)
        }
    }

    /// Get the date of the newest message in 'thread' as a time_t value.
    pub fn newest_date(self: &Self) -> i64 {
       unsafe {
           ffi::notmuch_thread_get_newest_date(self.handle.ptr)
       }
    }
}


unsafe impl<'o, Owner: ThreadOwner> Send for Thread<'o, Owner> {}
unsafe impl<'o, Owner: ThreadOwner> Sync for Thread<'o, Owner> {}
