use std::ffi::CString;
use std::ops::Drop;
use std::path::PathBuf;
use supercow::Phantomcow;

use error::{Error, Result};
use ffi;
use utils::ToStr;
use Filenames;
use FilenamesOwner;
use Messages;
use MessagesOwner;
use Tags;
use TagsOwner;

pub trait MessageOwner {}

#[derive(Debug)]
pub(crate) struct MessagePtr {
    pub ptr: *mut ffi::notmuch_message_t,
}

impl Drop for MessagePtr {
    fn drop(&mut self) {
        unsafe { ffi::notmuch_message_destroy(self.ptr) };
    }
}

#[derive(Debug)]
pub struct Message<'o, Owner: MessageOwner + 'o> {
    pub(crate) handle: MessagePtr,
    marker: Phantomcow<'o, Owner>,
}

impl<'o, Owner: MessageOwner + 'o> MessagesOwner for Message<'o, Owner> {}
impl<'o, Owner: MessageOwner + 'o> FilenamesOwner for Message<'o, Owner> {}
impl<'o, Owner: MessageOwner + 'o> TagsOwner for Message<'o, Owner> {}

impl<'o, Owner: MessageOwner + 'o> Message<'o, Owner> {
    pub fn from_ptr<O: Into<Phantomcow<'o, Owner>>>(
        ptr: *mut ffi::notmuch_message_t,
        owner: O,
    ) -> Message<'o, Owner> {
        Message {
            handle: MessagePtr { ptr },
            marker: owner.into(),
        }
    }

    pub fn id(self: &Self) -> String {
        let mid = unsafe { ffi::notmuch_message_get_message_id(self.handle.ptr) };
        mid.to_str().unwrap().to_string()
    }

    pub fn thread_id(self: &Self) -> String {
        let tid = unsafe { ffi::notmuch_message_get_thread_id(self.handle.ptr) };
        tid.to_str().unwrap().to_string()
    }

    pub fn replies(self: &Self) -> Messages<Self> {
        Messages::from_ptr(
            unsafe { ffi::notmuch_message_get_replies(self.handle.ptr) },
            self,
        )
    }

    #[cfg(feature = "v0_26")]
    pub fn count_files(self: &Self) -> i32 {
        unsafe { ffi::notmuch_message_count_files(self.handle.ptr) }
    }

    pub fn filenames(self: &Self) -> Filenames<Self> {
        Filenames::from_ptr(
            unsafe { ffi::notmuch_message_get_filenames(self.handle.ptr) },
            self,
        )
    }

    pub fn filename(self: &Self) -> PathBuf {
        PathBuf::from(
            unsafe { ffi::notmuch_message_get_filename(self.handle.ptr) }
                .to_str()
                .unwrap(),
        )
    }

    pub fn header(&self, name: &str) -> Result<&str> {
        let ret = unsafe {
            ffi::notmuch_message_get_header(self.handle.ptr, CString::new(name).unwrap().as_ptr())
        };
        if ret.is_null() {
            Err(Error::UnspecifiedError)
        } else {
            Ok(ret.to_str().unwrap())
        }
    }

    pub fn tags(&self) -> Tags<Self> {
        Tags::from_ptr(
            unsafe { ffi::notmuch_message_get_tags(self.handle.ptr) },
            self,
        )
    }
}

unsafe impl<'o, Owner: MessageOwner + 'o> Send for Message<'o, Owner> {}
unsafe impl<'o, Owner: MessageOwner + 'o> Sync for Message<'o, Owner> {}
