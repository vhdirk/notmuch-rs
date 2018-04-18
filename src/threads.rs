use std::{
    ops,
    marker,
    iter
};
use std::rc::Rc;

use utils::{
    FromPtr,
    NewFromPtr
};

use query::{Query, QueryPtr};
use Thread;
use ffi;

#[derive(Debug)]
pub(crate) struct ThreadsPtr {
    pub ptr: *mut ffi::notmuch_threads_t
}

impl ops::Drop for ThreadsPtr {
    fn drop(&mut self) {
        unsafe {
            ffi::notmuch_threads_destroy(self.ptr)
        };
    }
}


#[derive(Debug)]
pub struct Threads(pub(crate) Rc<ThreadsPtr>, Query);


impl NewFromPtr<*mut ffi::notmuch_threads_t, Query> for Threads {
    fn new(ptr: *mut ffi::notmuch_threads_t, parent: Query) -> Threads {
        Threads(Rc::new(ThreadsPtr{ptr}), parent)
    }
}



impl ops::Drop for Threads {
    fn drop(&mut self) {
        unsafe {
            ffi::notmuch_threads_destroy(self.0.ptr)
        };
    }
}

impl iter::Iterator for Threads {
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

unsafe impl Send for Threads {}
