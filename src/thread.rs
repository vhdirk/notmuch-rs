use std::{
    ops,
    marker
};
use std::rc::Rc;
use ffi;
use utils::{
    FromPtr,
    ToStr,
    NewFromPtr
};
use query::Query;
use Messages;
use Tags;


#[derive(Debug)]
pub(crate) struct ThreadPtr {
    pub ptr: *mut ffi::notmuch_thread_t
}

impl ops::Drop for ThreadPtr {
    fn drop(&mut self) {
        unsafe {
            ffi::notmuch_thread_destroy(self.ptr)
        };
    }
}


#[derive(Debug)]
pub struct Thread(pub(crate) Rc<ThreadPtr>, Query);


impl NewFromPtr<*mut ffi::notmuch_thread_t, Query> for Thread {
    fn new(ptr: *mut ffi::notmuch_thread_t, parent: Query) -> Thread {
        Thread(Rc::new(ThreadPtr{ptr}), parent)
    }
}


impl Thread{

    pub fn id(self: &Self) -> String{
        let tid = unsafe {
            ffi::notmuch_thread_get_thread_id(self.0.ptr)
        };
        tid.to_str().unwrap().to_string()
    }


    pub fn total_messages(self: &Self) -> i32{
        unsafe {
            ffi::notmuch_thread_get_total_messages(self.0.ptr)
        }
    }
#[cfg(feature = "0.26")]
    pub fn total_files(self: &Self) -> i32{
        unsafe {
            ffi::notmuch_thread_get_total_files(self.0.ptr)
        }
    }


    pub fn toplevel_messages(self: &Self) -> Messages{
        Messages::new(unsafe {
            ffi::notmuch_thread_get_toplevel_messages(self.0.ptr)
        }, self.1.clone())
    }

    /// Get a `Messages` iterator for all messages in 'thread' in
    /// oldest-first order.
    pub fn messages(self: &Self) -> Messages{
        Messages::new(unsafe {
            ffi::notmuch_thread_get_messages(self.0.ptr)
        }, self.1.clone())
    }


    pub fn tags(self: &Self) -> Tags{
        Tags::from_ptr(unsafe {
            ffi::notmuch_thread_get_tags(self.0.ptr)
        })
    }

    pub fn subject(self: &Self) -> String{
        let sub = unsafe {
            ffi::notmuch_thread_get_subject(self.0.ptr)
        };

        sub.to_str().unwrap().to_string()
    }

    pub fn authors(self: &Self) -> Vec<String>{
        let athrs = unsafe {
            ffi::notmuch_thread_get_authors(self.0.ptr)
        };

        athrs.to_str().unwrap().split(',').map(|s| s.to_string()).collect()
    }

    /// Get the date of the oldest message in 'thread' as a time_t value.
    pub fn oldest_date(self: &Self) -> i64 {
        unsafe {
            ffi::notmuch_thread_get_oldest_date(self.0.ptr)
        }
    }

    /// Get the date of the newest message in 'thread' as a time_t value.
    pub fn newest_date(self: &Self) -> i64 {
       unsafe {
           ffi::notmuch_thread_get_newest_date(self.0.ptr)
       }
    }
}

impl Clone for Thread {
    fn clone(&self) -> Self {
        Thread(self.0.clone(), self.1.clone())
    }
}

unsafe impl Send for Thread {}
