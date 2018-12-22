use std::ffi::CString;
use std::path::PathBuf;
use std::cell::RefCell;
use supercow::{Supercow};

use error::{Error, Result};
use ffi;
use utils::{ToStr, ScopedPhantomcow, ScopedSupercow};
use Filenames;
use FilenamesOwner;
use Messages;
use Tags;
use TagsOwner;

pub trait MessageOwner: Send + Sync {}

#[derive(Debug)]
pub struct Message<'o, O>
where
    O: MessageOwner + 'o,
{
    pub(crate) ptr: *mut ffi::notmuch_message_t,
    marker: RefCell<ScopedPhantomcow<'o, O>>,
}

impl<'o, O> MessageOwner for Message<'o, O> where O: MessageOwner + 'o {}
impl<'o, O> FilenamesOwner for Message<'o, O> where O: MessageOwner + 'o {}
impl<'o, O> TagsOwner for Message<'o, O> where O: MessageOwner + 'o {}

impl<'o, O> Message<'o, O>
where
    O: MessageOwner + 'o,
{
    pub(crate) fn from_ptr<P>(ptr: *mut ffi::notmuch_message_t, owner: P) -> Message<'o, O>
    where
        P: Into<ScopedPhantomcow<'o, O>>,
    {
        Message {
            ptr,
            marker: RefCell::new(owner.into()),
        }
    }

    pub fn id(self: &Self) -> String {
        let mid = unsafe { ffi::notmuch_message_get_message_id(self.ptr) };
        mid.to_str().unwrap().to_string()
    }

    pub fn thread_id(self: &Self) -> String {
        let tid = unsafe { ffi::notmuch_message_get_thread_id(self.ptr) };
        tid.to_str().unwrap().to_string()
    }

    pub fn replies(self: &Self) -> Messages<'o, O> {
        Messages::<'o, O>::from_ptr(
            unsafe { ffi::notmuch_message_get_replies(self.ptr) },
            // will never panic since the borrow is released immediately
            ScopedPhantomcow::<'o, O>::share(&mut *(self.marker.borrow_mut()))
        )
    }

    #[cfg(feature = "v0_26")]
    pub fn count_files(self: &Self) -> i32 {
        unsafe { ffi::notmuch_message_count_files(self.ptr) }
    }

    pub fn filenames(self: &Self) -> Filenames<Self> {
        <Self as MessageExt<'o, O>>::filenames(self)
    }

    pub fn filename(self: &Self) -> PathBuf {
        PathBuf::from(
            unsafe { ffi::notmuch_message_get_filename(self.ptr) }
                .to_str()
                .unwrap(),
        )
    }

    pub fn date(&self) -> i64 {
        unsafe { ffi::notmuch_message_get_date(self.ptr) as i64 }
    }

    pub fn header(&self, name: &str) -> Result<Option<&str>> {
        let name = CString::new(name).unwrap();
        let ret = unsafe { ffi::notmuch_message_get_header(self.ptr, name.as_ptr()) };
        if ret.is_null() {
            Err(Error::UnspecifiedError)
        } else {
            Ok(match ret.to_str().unwrap() {
                "" => None,
                ret => Some(ret),
            })
        }
    }

    pub fn tags(&self) -> Tags<Self> {
        <Self as MessageExt<'o, O>>::tags(self)
    }

    pub fn add_tag(self: &Self, tag: &str) -> Result<()> {
        let tag = CString::new(tag).unwrap();
        unsafe { ffi::notmuch_message_add_tag(self.ptr, tag.as_ptr()) }.as_result()
    }

    pub fn remove_tag(self: &Self, tag: &str) -> Result<()> {
        let tag = CString::new(tag).unwrap();
        unsafe { ffi::notmuch_message_remove_tag(self.ptr, tag.as_ptr()) }.as_result()
    }

    pub fn remove_all_tags(self: &Self) -> Result<()> {
        unsafe { ffi::notmuch_message_remove_all_tags(self.ptr) }.as_result()
    }

    pub fn tags_to_maildir_flags(self: &Self) -> Result<()> {
        unsafe { ffi::notmuch_message_tags_to_maildir_flags(self.ptr) }.as_result()
    }

    pub fn maildir_flags_to_tags(self: &Self) -> Result<()> {
        unsafe { ffi::notmuch_message_maildir_flags_to_tags(self.ptr) }.as_result()
    }
}

pub trait MessageExt<'o, O>
where
    O: MessageOwner + 'o,
{
    fn tags<'s, S>(message: S) -> Tags<'s, Message<'o, O>>
    where
        S: Into<ScopedSupercow<'s, Message<'o, O>>>,
    {
        let messageref = message.into();
        Tags::from_ptr(
            unsafe { ffi::notmuch_message_get_tags(messageref.ptr) },
            Supercow::phantom(messageref),
        )
    }

    // fn replies<'s, S>(message: S) -> Messages<'s, Message<'o, O>>
    // where
    //     S: Into<ScopedSupercow<'s, Message<'o, O>>>,
    // {
    //     let messageref = message.into();
    //     Messages::from_ptr(
    //         unsafe { ffi::notmuch_message_get_replies(messageref.ptr) },
    //         Supercow::phantom(messageref),
    //     )
    // }

    fn filenames<'s, S>(message: S) -> Filenames<'s, Message<'o, O>>
    where
        S: Into<ScopedSupercow<'s, Message<'o, O>>>,
    {
        let messageref = message.into();
        Filenames::from_ptr(
            unsafe { ffi::notmuch_message_get_filenames(messageref.ptr) },
            Supercow::phantom(messageref),
        )
    }
}

impl<'o, O> MessageExt<'o, O> for Message<'o, O> where O: MessageOwner + 'o {}

unsafe impl<'o, O> Send for Message<'o, O> where O: MessageOwner + 'o {}
unsafe impl<'o, O> Sync for Message<'o, O> where O: MessageOwner + 'o {}
