use std::ops::Drop;
use std::iter::Iterator;
use std::marker::PhantomData;

use utils::FromPtr;
use Query;
use Thread;
use ffi;
use thread::ThreadOwner;

pub trait ThreadsOwner{}


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
pub struct Threads<'o, Owner: ThreadsOwner>{
    handle: ThreadsPtr,
    phantom: PhantomData<&'o Owner>,
}

impl<'o, Owner: ThreadsOwner> ThreadOwner for Threads<'o, Owner>{}


impl<'o, Owner: ThreadsOwner> FromPtr<*mut ffi::notmuch_threads_t> for Threads<'o, Owner> {
    fn from_ptr(ptr: *mut ffi::notmuch_threads_t) -> Threads<'o, Owner> {
        Threads{
            handle: ThreadsPtr{ptr},
            phantom: PhantomData
        }
    }
}

impl<'o, Owner: ThreadsOwner> Iterator for Threads<'o, Owner> {
    type Item = Thread<'o, Self>;

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

unsafe impl<'o, Owner: ThreadsOwner> Send for Threads<'o, Owner> {}
unsafe impl<'o, Owner: ThreadsOwner> Sync for Threads<'o, Owner> {}
