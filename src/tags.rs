use std::ffi::CStr;
use std::iter::Iterator;
use std::ops::Drop;

use supercow::Phantomcow;

use ffi;

pub trait TagsOwner {}

#[derive(Debug)]
pub(crate) struct TagsPtr {
    pub ptr: *mut ffi::notmuch_tags_t,
}

impl Drop for TagsPtr {
    fn drop(&mut self) {
        unsafe { ffi::notmuch_tags_destroy(self.ptr) };
    }
}

#[derive(Debug)]
pub struct Tags<'o, Owner: TagsOwner + 'o> {
    handle: TagsPtr,
    marker: Phantomcow<'o, Owner>,
}

impl<'o, Owner: TagsOwner + 'o> Tags<'o, Owner> {
    pub fn from_ptr<O: Into<Phantomcow<'o, Owner>>>(
        ptr: *mut ffi::notmuch_tags_t,
        owner: O,
    ) -> Tags<'o, Owner> {
        Tags {
            handle: TagsPtr { ptr },
            marker: owner.into(),
        }
    }
}

impl<'o, Owner: TagsOwner + 'o> Iterator for Tags<'o, Owner> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let valid = unsafe { ffi::notmuch_tags_valid(self.handle.ptr) };

        if valid == 0 {
            return None;
        }

        let ctag = unsafe {
            let t = ffi::notmuch_tags_get(self.handle.ptr);
            ffi::notmuch_tags_move_to_next(self.handle.ptr);

            CStr::from_ptr(t)
        };

        Some(ctag.to_str().unwrap().to_string())
    }
}

pub trait TagsExt<'o, Owner: TagsOwner + 'o> {}

impl<'o, Owner: TagsOwner + 'o> TagsExt<'o, Owner> for Tags<'o, Owner> {}

unsafe impl<'o, Owner: TagsOwner + 'o> Send for Tags<'o, Owner> {}
unsafe impl<'o, Owner: TagsOwner + 'o> Sync for Tags<'o, Owner> {}
