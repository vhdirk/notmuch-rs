use std::ops::Drop;
use std::iter::Iterator;
use std::marker::PhantomData;
use std::ffi::CStr;

use utils::{
    FromPtr,
};

use Database;
use ffi;

pub trait TagsOwner{}


#[derive(Debug)]
pub struct Tags<'o, Owner: TagsOwner + 'o>(
    *mut ffi::notmuch_tags_t,
    PhantomData<&'o Owner>,
);

impl<'o, Owner: TagsOwner + 'o> FromPtr<*mut ffi::notmuch_tags_t> for Tags<'o, Owner> {
    fn from_ptr(ptr: *mut ffi::notmuch_tags_t) -> Tags<'o, Owner> {
        Tags(ptr, PhantomData)
    }
}

impl<'o, Owner: TagsOwner + 'o> Drop for Tags<'o, Owner> {
    fn drop(&mut self) {
        unsafe {
            ffi::notmuch_tags_destroy(self.0)
        };
    }
}

impl<'o, Owner: TagsOwner + 'o> Iterator for Tags<'o, Owner> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {

        let valid = unsafe {
            ffi::notmuch_tags_valid(self.0)
        };

        if valid == 0{
            return None
        }

        let ctag = unsafe {
            let t = ffi::notmuch_tags_get(self.0);
            ffi::notmuch_tags_move_to_next(self.0);

            CStr::from_ptr(t)
        };

        Some(ctag.to_str().unwrap().to_string())
    }
}

unsafe impl<'o, Owner: TagsOwner + 'o> Send for Tags<'o, Owner>{}
unsafe impl<'o, Owner: TagsOwner + 'o> Sync for Tags<'o, Owner>{}
