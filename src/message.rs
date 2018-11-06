use std::ffi::CString;
use std::ops::Drop;
use std::path::PathBuf;
use supercow::{Phantomcow, Supercow};

use error::{Error, Result};
use ffi;
use ffi::Status;
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
        <Self as MessageExt<'o, Owner>>::replies(self)
    }

    #[cfg(feature = "v0_26")]
    pub fn count_files(self: &Self) -> i32 {
        unsafe { ffi::notmuch_message_count_files(self.handle.ptr) }
    }

    pub fn filenames(self: &Self) -> Filenames<Self> {
        <Self as MessageExt<'o, Owner>>::filenames(self)
    }

    pub fn filename(self: &Self) -> PathBuf {
        PathBuf::from(
            unsafe { ffi::notmuch_message_get_filename(self.handle.ptr) }
                .to_str()
                .unwrap(),
        )
    }

    pub fn date(&self) -> i64 {
        unsafe { ffi::notmuch_message_get_date(self.handle.ptr) }
    }

    pub fn header(&self, name: &str) -> Result<Option<&str>> {
        let name = CString::new(name).unwrap();
        let ret = unsafe { ffi::notmuch_message_get_header(self.handle.ptr, name.as_ptr()) };
        if ret.is_null() {
            Err(Error::UnspecifiedError)
        } else {
            Ok(match ret.to_str().unwrap() {
                "" => None,
                ret => Some(ret)
            })
        }
    }

    pub fn tags(&self) -> Tags<Self> {
        <Self as MessageExt<'o, Owner>>::tags(self)
    }

    pub fn add_tag(self: &Self, tag: &str) -> Status {
        let tag = CString::new(tag).unwrap();
        Status::from(unsafe { ffi::notmuch_message_add_tag(self.handle.ptr, tag.as_ptr()) })
    }

    pub fn remove_tag(self: &Self, tag: &str) -> Status {
        let tag = CString::new(tag).unwrap();
        Status::from(unsafe { ffi::notmuch_message_remove_tag(self.handle.ptr, tag.as_ptr()) })
    }

    pub fn remove_all_tags(self: &Self) -> Status {
        Status::from(unsafe { ffi::notmuch_message_remove_all_tags(self.handle.ptr) })
    }
}

pub trait MessageExt<'o, Owner: MessageOwner + 'o> {
    fn tags<'s, S: Into<Supercow<'s, Message<'o, Owner>>>>(
        message: S,
    ) -> Tags<'s, Message<'o, Owner>> {
        let messageref = message.into();
        Tags::from_ptr(
            unsafe { ffi::notmuch_message_get_tags(messageref.handle.ptr) },
            Supercow::phantom(messageref),
        )
    }

    fn replies<'s, S: Into<Supercow<'s, Message<'o, Owner>>>>(
        message: S,
    ) -> Messages<'s, Message<'o, Owner>> {
        let messageref = message.into();
        Messages::from_ptr(
            unsafe { ffi::notmuch_message_get_replies(messageref.handle.ptr) },
            Supercow::phantom(messageref),
        )
    }

    fn filenames<'s, S: Into<Supercow<'s, Message<'o, Owner>>>>(
        message: S,
    ) -> Filenames<'s, Message<'o, Owner>> {
        let messageref = message.into();
        Filenames::from_ptr(
            unsafe { ffi::notmuch_message_get_filenames(messageref.handle.ptr) },
            Supercow::phantom(messageref),
        )
    }
}

impl<'o, Owner: MessageOwner + 'o> MessageExt<'o, Owner> for Message<'o, Owner> {}

unsafe impl<'o, Owner: MessageOwner + 'o> Send for Message<'o, Owner> {}
unsafe impl<'o, Owner: MessageOwner + 'o> Sync for Message<'o, Owner> {}
