use std::rc::Rc;
use std::ops::Drop;
use std::fmt::Debug;

use from_variants::FromVariants;

use ffi;
use Query;
use Thread;

#[derive(Clone, Debug, FromVariants)]
pub(crate) enum ThreadsOwner {
    Query(Query),
}

#[derive(Debug)]
pub struct ThreadsPtr(*mut ffi::notmuch_threads_t);

impl Drop for ThreadsPtr
{
    fn drop(&mut self) {
        unsafe { ffi::notmuch_threads_destroy(self.0) };
    }
}

#[derive(Clone, Debug)]
pub struct Threads
{
    ptr: Rc<ThreadsPtr>,
    owner: Box<ThreadsOwner>,
}

impl Threads
{
    pub(crate) fn from_ptr<O>(ptr: *mut ffi::notmuch_threads_t, owner: O) -> Threads
    where
        O: Into<ThreadsOwner>,
    {
        Threads {
            ptr: Rc::new(ThreadsPtr(ptr)),
            owner: Box::new(owner.into()),
        }
    }
}

impl Iterator for Threads
{
    type Item = Thread;

    fn next(&mut self) -> Option<Self::Item> {
        let valid = unsafe { ffi::notmuch_threads_valid(self.ptr.0) };

        if valid == 0 {
            return None;
        }

        let cthrd = unsafe {
            let thrd = ffi::notmuch_threads_get(self.ptr.0);
            ffi::notmuch_threads_move_to_next(self.ptr.0);
            thrd
        };

        Some(Thread::from_ptr(cthrd, self.clone()))
    }
}
