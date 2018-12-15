use std::ops::Drop;

use supercow::{Phantomcow, Supercow};
use utils::{StreamingIterator, StreamingIteratorExt};

use ffi;
use thread::ThreadOwner;
use Thread;

pub trait ThreadsOwner {}

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
    O: ThreadsOwner + 'o,
{
    handle: ThreadsPtr,
    marker: Phantomcow<'o, O>,
}

impl<'o, O> ThreadOwner for Threads<'o, O> where O: ThreadsOwner + 'o {}

impl<'o, O> Threads<'o, O>
where
    O: ThreadsOwner + 'o,
{
    pub fn from_ptr<P>(ptr: *mut ffi::notmuch_threads_t, owner: P) -> Threads<'o, O>
    where
        P: Into<Phantomcow<'o, O>>,
    {
        Threads {
            handle: ThreadsPtr { ptr },
            marker: owner.into(),
        }
    }
}

impl<'s, 'o: 's, O> StreamingIterator<'s, Thread<'s, Self>> for Threads<'o, O>
where
    O: ThreadsOwner + 'o,
{
    fn next(&'s mut self) -> Option<Thread<'s, Self>> {
        <Self as StreamingIteratorExt<'s, Thread<'s, Self>>>::next(Supercow::borrowed(self))
    }
}

pub trait ThreadsExt<'o, O>
where
    O: ThreadsOwner + 'o,
{
}

impl<'o, O> ThreadsExt<'o, O> for Threads<'o, O> where O: ThreadsOwner + 'o {}

impl<'s, 'o: 's, O> StreamingIteratorExt<'s, Thread<'s, Self>> for Threads<'o, O>
where
    O: ThreadsOwner + 'o,
{
    fn next<S>(threads: S) -> Option<Thread<'s, Self>>
    where
        S: Into<Supercow<'s, Threads<'o, O>>>,
    {
        let threadsref = threads.into();
        let valid = unsafe { ffi::notmuch_threads_valid(threadsref.handle.ptr) };

        if valid == 0 {
            return None;
        }

        let cmsg = unsafe {
            let msg = ffi::notmuch_threads_get(threadsref.handle.ptr);
            ffi::notmuch_threads_move_to_next(threadsref.handle.ptr);
            msg
        };

        Some(Thread::from_ptr(cmsg, Supercow::phantom(threadsref)))
    }
}

unsafe impl<'o, O> Send for Threads<'o, O> where O: ThreadsOwner + 'o {}
unsafe impl<'o, O> Sync for Threads<'o, O> where O: ThreadsOwner + 'o {}
