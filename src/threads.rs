use std::ops::Drop;
use std::iter::Iterator;
use std::marker::PhantomData;

use utils::FromPtr;
use Query;
use Thread;
use ffi;

#[derive(Debug)]
pub(crate) struct ThreadsPtr {
    pub ptr: *mut ffi::notmuch_threads_t
}

impl Drop for ThreadsPtr {
    fn drop(&mut self) {
        unsafe {
            ffi::notmuch_threads_destroy(self.ptr)
        };
    }
}

#[derive(Debug)]
pub struct Threads<'d:'q, 'q>{
    handle: ThreadsPtr,
    phantom: PhantomData<&'q Query<'d>>,
}

impl<'d, 'q> FromPtr<*mut ffi::notmuch_threads_t> for Threads<'d, 'q> {
    fn from_ptr(ptr: *mut ffi::notmuch_threads_t) -> Threads<'d, 'q> {
        Threads{
            handle: ThreadsPtr{ptr},
            phantom: PhantomData
        }
    }
}

impl<'d, 'q> Iterator for Threads<'d, 'q> {
    type Item = Thread<'d, 'q>;

    fn next(self: &mut Self) -> Option<Self::Item> {

        let valid = unsafe {
            ffi::notmuch_threads_valid(self.handle.ptr)
        };

        if valid == 0{
            return None;
        }

        let cthread = unsafe {
            let t = ffi::notmuch_threads_get(self.handle.ptr);
            ffi::notmuch_threads_move_to_next(self.handle.ptr);
            t
        };

        Some(Self::Item::from_ptr(cthread))
    }
}

unsafe impl<'d, 'q> Send for Threads<'d, 'q> {}
unsafe impl<'d, 'q> Sync for Threads<'d, 'q> {}
