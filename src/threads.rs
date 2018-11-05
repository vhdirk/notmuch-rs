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
pub struct Threads<'o, Owner: ThreadsOwner + 'o> {
    handle: ThreadsPtr,
    marker: Phantomcow<'o, Owner>,
}

impl<'o, Owner: ThreadsOwner + 'o> ThreadOwner for Threads<'o, Owner> {}

impl<'o, Owner: ThreadsOwner + 'o> Threads<'o, Owner> {
    pub fn from_ptr<O: Into<Phantomcow<'o, Owner>>>(
        ptr: *mut ffi::notmuch_threads_t,
        owner: O,
    ) -> Threads<'o, Owner> {
        Threads {
            handle: ThreadsPtr { ptr },
            marker: owner.into(),
        }
    }
}

impl<'s, 'o: 's, Owner: ThreadsOwner + 'o> StreamingIterator<'s, Thread<'s, Self>>
    for Threads<'o, Owner>
{
    fn next(&'s mut self) -> Option<Thread<'s, Self>> {
        <Self as StreamingIteratorExt<'s, Thread<'s, Self>>>::next(Supercow::borrowed(self))
    }
}

pub trait ThreadsExt<'o, Owner: ThreadsOwner + 'o>{

}

impl<'o, Owner: ThreadsOwner + 'o> ThreadsExt<'o, Owner> for Threads<'o, Owner>{
    
}

impl<'s, 'o: 's, Owner: ThreadsOwner + 'o> StreamingIteratorExt<'s, Thread<'s, Self>> for Threads<'o, Owner>
{
    fn next<S: Into<Supercow<'s, Threads<'o, Owner>>>>(threads: S) -> Option<Thread<'s, Self>>{
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

unsafe impl<'o, Owner: ThreadsOwner + 'o> Send for Threads<'o, Owner> {}
unsafe impl<'o, Owner: ThreadsOwner + 'o> Sync for Threads<'o, Owner> {}
