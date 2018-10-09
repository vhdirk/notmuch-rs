use std::ops::Drop;
use std::marker::PhantomData;
use ffi;
use utils::{
    NewFromPtr,
    ToStr
};
use Query;
use Messages;
use Tags;

#[derive(Debug)]
pub struct Thread<'d:'q, 'q>(
    pub(crate) *mut ffi::notmuch_thread_t,
    PhantomData<&'q Query<'d>>,
);

impl<'d, 'q> NewFromPtr<*mut ffi::notmuch_thread_t> for Thread<'d, 'q> {
    fn new(ptr: *mut ffi::notmuch_thread_t) -> Thread<'d, 'q> {
        Thread(ptr, PhantomData)
    }
}

impl<'d, 'q> Thread<'d, 'q>{

    pub fn id(self: &Self) -> String{
        let tid = unsafe {
            ffi::notmuch_thread_get_thread_id(self.0)
        };
        tid.to_str().unwrap().to_string()
    }


    pub fn total_messages(self: &Self) -> i32{
        unsafe {
            ffi::notmuch_thread_get_total_messages(self.0)
        }
    }

    #[cfg(feature = "0.26")]
    pub fn total_files(self: &Self) -> i32{
        unsafe {
            ffi::notmuch_thread_get_total_files(self.0)
        }
    }


    pub fn toplevel_messages(self: &Self) -> Messages{
        Messages::new(unsafe {
            ffi::notmuch_thread_get_toplevel_messages(self.0)
        })
    }

    /// Get a `Messages` iterator for all messages in 'thread' in
    /// oldest-first order.
    pub fn messages(self: &Self) -> Messages{
        Messages::new(unsafe {
            ffi::notmuch_thread_get_messages(self.0)
        })
    }


    pub fn tags(self: &Self) -> Tags{
        Tags::new(unsafe {
            ffi::notmuch_thread_get_tags(self.0)
        })
    }

    pub fn subject(self: &Self) -> String{
        let sub = unsafe {
            ffi::notmuch_thread_get_subject(self.0)
        };

        sub.to_str().unwrap().to_string()
    }

    pub fn authors(self: &Self) -> Vec<String>{
        let athrs = unsafe {
            ffi::notmuch_thread_get_authors(self.0)
        };

        athrs.to_str().unwrap().split(',').map(|s| s.to_string()).collect()
    }

    /// Get the date of the oldest message in 'thread' as a time_t value.
    pub fn oldest_date(self: &Self) -> i64 {
        unsafe {
            ffi::notmuch_thread_get_oldest_date(self.0)
        }
    }

    /// Get the date of the newest message in 'thread' as a time_t value.
    pub fn newest_date(self: &Self) -> i64 {
       unsafe {
           ffi::notmuch_thread_get_newest_date(self.0)
       }
    }
}


impl<'d, 'q> Drop for Thread<'d, 'q> {
    fn drop(&mut self) {
        unsafe {
            ffi::notmuch_thread_destroy(self.0)
        };
    }
}

unsafe impl<'d, 'q> Send for Thread<'d, 'q> {}
unsafe impl<'d, 'q> Sync for Thread<'d, 'q> {}
