use std::ffi::CStr;
use std::iter::Iterator;
use std::ops::Drop;

use ffi;
use utils::ScopedPhantomcow;

pub trait TagsOwner {}

#[derive(Debug)]
pub struct Tags<'o, O> where
    O: TagsOwner + 'o,
{
    ptr: *mut ffi::notmuch_tags_t,
    marker: ScopedPhantomcow<'o, O>,
}

impl<'o, O> Drop for Tags<'o, O>
where
    O: TagsOwner + 'o,
{
    fn drop(&mut self) {
        unsafe { ffi::notmuch_tags_destroy(self.ptr) };
    }
}

impl<'o, O> Tags<'o, O>
where
    O: TagsOwner + 'o,
{
    pub(crate) fn from_ptr<P>(ptr: *mut ffi::notmuch_tags_t, owner: P) -> Tags<'o, O>
    where
        P: Into<ScopedPhantomcow<'o, O>>,
    {
        Tags {
            ptr,
            marker: owner.into(),
        }
    }
}

impl<'o, O> Iterator for Tags<'o, O>
where
    O: TagsOwner + 'o,
{
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let valid = unsafe { ffi::notmuch_tags_valid(self.ptr) };

        if valid == 0 {
            return None;
        }

        let ctag = unsafe {
            let t = ffi::notmuch_tags_get(self.ptr);
            ffi::notmuch_tags_move_to_next(self.ptr);

            CStr::from_ptr(t)
        };

        Some(ctag.to_str().unwrap().to_string())
    }
}

pub trait TagsExt<'o, O>
where
    O: TagsOwner + 'o,
{
}

impl<'o, O> TagsExt<'o, O> for Tags<'o, O> where O: TagsOwner + 'o {}

unsafe impl<'o, O> Send for Tags<'o, O> where O: TagsOwner + 'o {}
unsafe impl<'o, O> Sync for Tags<'o, O> where O: TagsOwner + 'o {}
