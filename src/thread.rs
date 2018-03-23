use std;
use std::{
    ops,
    marker,
    ptr,
};

use error::Result;

use ffi;
use utils::{
    NewFromPtr,
};
use Query;

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

}


impl<'q, 'd> ops::Drop for Thread<'q, 'd> {
    fn drop(&mut self) {
        unsafe {
            ffi::notmuch_thread_destroy(self.0)
        };
    }
}
