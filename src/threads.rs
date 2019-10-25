use std::ops::Drop;

use ffi;
use Thread;
use Query;
use utils::ScopedPhantomcow;


#[derive(Debug)]
pub struct Threads<'d, 'q>
where
    'd: 'q
{
    ptr: *mut ffi::notmuch_threads_t,
    marker: ScopedPhantomcow<'q, Query<'d>>,
}

impl<'d, 'q> Drop for Threads<'d, 'q>
where
    'd: 'q,
{
    fn drop(&mut self) {
        unsafe { ffi::notmuch_threads_destroy(self.ptr) };
    }
}

impl<'d, 'q> Threads<'d, 'q>
where
    'd: 'q,
{
    pub(crate) fn from_ptr<P>(ptr: *mut ffi::notmuch_threads_t, owner: P) -> Threads<'d, 'q>
    where
        P: Into<ScopedPhantomcow<'q, Query<'d>>>,
    {
        Threads {
            ptr,
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
        let valid = unsafe { ffi::notmuch_threads_valid(self.ptr) };

        if valid == 0 {
            return None;
        }

        let cthrd = unsafe {
            let thrd = ffi::notmuch_threads_get(self.ptr);
            ffi::notmuch_threads_move_to_next(self.ptr);
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
