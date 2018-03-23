use std::{
    ops,
    marker,
    iter
};

use utils::{
    NewFromPtr,
};

use Database;
use Thread;
use ffi;

#[derive(Debug)]
pub struct Threads<'d>(
    *mut ffi::notmuch_threads_t,
    marker::PhantomData<&'d mut Database>,
);

impl<'d> NewFromPtr<*mut ffi::notmuch_threads_t> for Threads<'d> {
    fn new(ptr: *mut ffi::notmuch_threads_t) -> Threads<'d> {
        Threads(ptr, marker::PhantomData)
    }
}

impl<'d> ops::Drop for Threads<'d> {
    fn drop(&mut self) {
        unsafe {
            ffi::notmuch_threads_destroy(self.0)
        };
    }
}

impl<'d> iter::Iterator for Threads<'d> {
    type Item = Thread<'d>;

    fn next(&mut self) -> Option<Self::Item> {

        let valid = unsafe {
            ffi::notmuch_threads_valid(self.0)
        };

        if valid == 0{
            return None
        }

        let cthread = unsafe {
            ffi::notmuch_threads_move_to_next(self.0);
            ffi::notmuch_threads_get(self.0)
        };

        Some(Thread::new(cthread))
    }
}
