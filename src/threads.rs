use std::ops::Drop;
use std::iter::Iterator;
use std::marker::PhantomData;

use utils::NewFromPtr;
use Query;
use Thread;
use ffi;

#[derive(Debug)]
pub struct Threads<'d:'q, 'q>(
    *mut ffi::notmuch_threads_t,
    PhantomData<&'q Query<'d>>,
);

impl<'d, 'q> NewFromPtr<*mut ffi::notmuch_threads_t> for Threads<'d, 'q> {
    fn new(ptr: *mut ffi::notmuch_threads_t) -> Threads<'d, 'q> {
        Threads(ptr, PhantomData)
    }
}

impl<'d, 'q> Drop for Threads<'d, 'q> {
    fn drop(&mut self) {
        unsafe {
            ffi::notmuch_threads_destroy(self.0)
        };
    }
}

impl<'d, 'q> Iterator for Threads<'d, 'q> {
    type Item = Thread<'d, 'q>;

    fn next(self: &mut Self) -> Option<Self::Item> {

        let valid = unsafe {
            ffi::notmuch_threads_valid(self.0)
        };

        if valid == 0{
            return None;
        }

        let cthread = unsafe {
            let t = ffi::notmuch_threads_get(self.0);
            ffi::notmuch_threads_move_to_next(self.0);
            t
        };

        Some(Self::Item::new(cthread))
    }
}

unsafe impl<'d, 'q> Send for Threads<'d, 'q> {}
unsafe impl<'d, 'q> Sync for Threads<'d, 'q> {}
