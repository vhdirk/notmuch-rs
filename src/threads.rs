use std::ops::Drop;

use ffi;
use Thread;
use Query;
use utils::ScopedPhantomcow;


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
pub struct Threads<'d, 'q>
where
    'd: 'q
{
    handle: ThreadsPtr,
    marker: ScopedPhantomcow<'q, Query<'d>>,
}

impl<'d, 'q> Threads<'d, 'q>
where
    'd: 'q,
{
    pub fn from_ptr<P>(ptr: *mut ffi::notmuch_threads_t, owner: P) -> Threads<'d, 'q>
    where
        P: Into<ScopedPhantomcow<'q, Query<'d>>>,
    {
        Threads {
            handle: ThreadsPtr { ptr },
            marker: owner.into(),
        }
    }
}

impl<'d, 'q> Iterator for Threads<'d, 'q>
where
    'd: 'q,
{
    type Item = Thread<'d, 'q>;

    fn next(&mut self) -> Option<Self::Item> {
        let valid = unsafe { ffi::notmuch_threads_valid(self.handle.ptr) };

        if valid == 0 {
            return None;
        }

        let cthrd = unsafe {
            let thrd = ffi::notmuch_threads_get(self.handle.ptr);
            ffi::notmuch_threads_move_to_next(self.handle.ptr);
            thrd
        };

        Some(Thread::from_ptr(cthrd, ScopedPhantomcow::<'q, Query<'d>>::share(&mut self.marker)))
    }
}


pub trait ThreadsExt<'d, 'q>
where
    'd: 'q,
{
}

impl<'d, 'q> ThreadsExt<'d, 'q> for Threads<'d, 'q> where 'd: 'q {}


unsafe impl<'d, 'q> Send for Threads<'d, 'q> where 'd: 'q {}
unsafe impl<'d, 'q> Sync for Threads<'d, 'q> where 'd: 'q {}
