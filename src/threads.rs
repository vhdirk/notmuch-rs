use std::ops::Drop;
use std::iter::Iterator;
use std::rc::Rc;

use utils::NewFromPtr;
use query::Query;
use Thread;
use ffi;

#[derive(Debug)]
pub(crate) struct ThreadsPtr {
    pub ptr: *mut ffi::notmuch_threads_t
}

impl Drop for ThreadsPtr {
    fn drop(&mut self) {
        let valid = unsafe {
            ffi::notmuch_threads_valid(self.ptr)
        };

        if valid != 0 {
            unsafe {
                ffi::notmuch_threads_destroy(self.ptr)
            };
        }
    }
}


#[derive(Debug)]
pub struct Threads(pub(crate) Rc<ThreadsPtr>, Query);


impl NewFromPtr<*mut ffi::notmuch_threads_t, Query> for Threads {
    fn new(ptr: *mut ffi::notmuch_threads_t, parent: Query) -> Threads {
        Threads(Rc::new(ThreadsPtr{ptr}), parent)
    }
}

impl Iterator for Threads {
    type Item = Thread;

    fn next(self: &mut Self) -> Option<Self::Item> {

        let valid = unsafe {
            ffi::notmuch_threads_valid(self.0.ptr)
        };

        if valid == 0{
            return None;
        }

        let cthread = unsafe {
            let t = ffi::notmuch_threads_get(self.0.ptr);
            ffi::notmuch_threads_move_to_next(self.0.ptr);
            t
        };

        Some(Self::Item::new(cthread, self.1.clone()))
    }
}
