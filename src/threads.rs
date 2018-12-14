use std::ops::Drop;

use supercow::{Phantomcow, Supercow};

use crate::ffi;
use crate::thread::{ThreadOwner, ThreadPtr};
use crate::Thread;
use crate::utils::{ScopedPhantomcow, ScopedSupercow};


#[derive(Debug)]
pub(crate) struct ThreadsPtr {
    pub ptr: *mut ffi::notmuch_threads_t,
}

impl Drop for ThreadsPtr {
    fn drop(&mut self) {
        unsafe { ffi::notmuch_threads_destroy(self.ptr) };
    }
}

#[derive(Debug)]
pub struct Threads<'o, O>
where
    O: ThreadOwner,
{
    handle: ThreadsPtr,
    marker: ScopedPhantomcow<'o, O>,
}

impl<'o, O> Threads<'o, O>
where
    O: ThreadOwner + 'o,
{
    pub fn from_ptr<P>(ptr: *mut ffi::notmuch_threads_t, owner: P) -> Threads<'o, O>
    where
        P: Into<ScopedPhantomcow<'o, O>>,
    {
        Threads {
            handle: ThreadsPtr { ptr },
            marker: owner.into(),
        }
    }
}

impl<'o, O> Iterator for Threads<'o, O>
where
    O: ThreadOwner,
{
    type Item = Thread<'o, O>;

    fn next(&mut self) -> Option<Thread<'o, O>> {
        let valid = unsafe { ffi::notmuch_threads_valid(self.handle.ptr) };

        if valid == 0 {
            return None;
        }

        let cthrd = unsafe {
            let thrd = ffi::notmuch_threads_get(self.handle.ptr);
            ffi::notmuch_threads_move_to_next(self.handle.ptr);
            thrd
        };

        Some(Thread::from_ptr(cthrd, ScopedPhantomcow::<'o, O>::share(&mut self.marker)))
    }
}


pub trait ThreadsExt<'o, O>
where
    O: ThreadOwner + 'o,
{
}

impl<'o, O> ThreadsExt<'o, O> for Threads<'o, O> where O: ThreadOwner + 'o {}


unsafe impl<'o, O> Send for Threads<'o, O> where O: ThreadOwner + 'o {}
unsafe impl<'o, O> Sync for Threads<'o, O> where O: ThreadOwner + 'o {}
