use std;
use std::{
    ops,
    marker,
    str,
    result
};
use std::ffi::{CString, CStr};

use error::Result;

use ffi;
use utils::{
    NewFromPtr,
    ToStr
};
use Query;
use Messages;
use Tags;

#[derive(Debug)]
pub struct Thread<'q, 'd:'q>(
    pub(crate) *mut ffi::notmuch_thread_t,
    marker::PhantomData<&'q mut Query<'d>>,
);

impl<'q, 'd> NewFromPtr<*mut ffi::notmuch_thread_t> for Thread<'q, 'd> {
    fn new(ptr: *mut ffi::notmuch_thread_t) -> Thread<'q, 'd> {
        Thread(ptr, marker::PhantomData)
    }
}

impl<'q, 'd> Thread<'q, 'd>{

    pub fn id(self: &Self) -> result::Result<&'q str, str::Utf8Error>{
        let tid = unsafe {
            ffi::notmuch_thread_get_thread_id(self.0)
        };
        tid.to_str()
    }


    pub fn total_messages(self: &Self) -> i32{
        unsafe {
            ffi::notmuch_thread_get_total_messages(self.0)
        }
    }

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

}


impl<'q, 'd> ops::Drop for Thread<'q, 'd> {
    fn drop(&mut self) {
        unsafe {
            ffi::notmuch_thread_destroy(self.0)
        };
    }
}
