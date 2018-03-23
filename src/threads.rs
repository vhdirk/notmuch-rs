use std::{
    ops,
    marker,
    iter
};

use utils::{
    NewFromPtr,
};

use Query;
use Thread;
use ffi;

#[derive(Debug)]
pub struct Threads<'q, 'd:'q>(
    *mut ffi::notmuch_threads_t,
    marker::PhantomData<&'q mut Query<'d>>,
);

impl<'q, 'd> NewFromPtr<*mut ffi::notmuch_threads_t> for Threads<'q, 'd> {
    fn new(ptr: *mut ffi::notmuch_threads_t) -> Threads<'q, 'd> {
        Threads(ptr, marker::PhantomData)
    }
}

impl<'q, 'd> ops::Drop for Threads<'q, 'd> {
    fn drop(&mut self) {
        unsafe {
            ffi::notmuch_threads_destroy(self.0)
        };
    }
}

impl<'q, 'd> iter::Iterator for Threads<'q, 'd> {
    type Item = Thread<'q, 'd>;

    fn next(&mut self) -> Option<Self::Item> {

        let valid = unsafe {
            ffi::notmuch_threads_valid(self.0)
        };

        if valid == 0{
            return None;
        }

        let cthread = unsafe {
            ffi::notmuch_threads_move_to_next(self.0);
            ffi::notmuch_threads_get(self.0)
        };

        Some(Self::Item::new(cthread))
    }
}
