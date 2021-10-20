use std::rc::Rc;
use std::cmp::PartialEq;
use std::ffi::CStr;
use std::iter::Iterator;
use std::ops::Drop;

use from_variants::FromVariants;

use ffi;
use Thread;
use Database;
use Message;
use Messages;

#[derive(Clone, Debug, FromVariants)]
pub(crate) enum TagsOwner {
    Database(Database),
    Message(Message),
    Messages(Messages),
    Thread(Thread),
}

#[derive(Debug)]
pub(crate) struct TagsPtr(*mut ffi::notmuch_tags_t);

impl Drop for TagsPtr
{
    fn drop(&mut self) {
        unsafe { ffi::notmuch_tags_destroy(self.0) };
    }
}

impl PartialEq for TagsPtr{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}


#[derive(Clone, Debug)]
pub struct Tags
{
    ptr: Rc<TagsPtr>,
    owner: TagsOwner,
}


impl Tags
{
    pub(crate) fn from_ptr<O>(ptr: *mut ffi::notmuch_tags_t, owner: O) -> Tags
    where
        O: Into<TagsOwner>,
    {
        Tags {
            ptr: Rc::new(TagsPtr(ptr)),
            owner: owner.into(),
        }
    }
}

impl Iterator for Tags
{
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let valid = unsafe { ffi::notmuch_tags_valid(self.ptr.0) };

        if valid == 0 {
            return None;
        }

        let ctag = unsafe {
            let t = ffi::notmuch_tags_get(self.ptr.0);
            ffi::notmuch_tags_move_to_next(self.ptr.0);

            CStr::from_ptr(t)
        };

        Some(ctag.to_string_lossy().to_string())
    }
}