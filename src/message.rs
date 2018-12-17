use std::ffi::CString;
use std::path::PathBuf;
use supercow::{Supercow};

use error::{Error, Result};
use ffi;
use utils::{ToStr, ScopedPhantomcow, ScopedSupercow};
use Filenames;
use FilenamesOwner;
use Messages;
use Tags;
use TagsOwner;
use Query;

pub trait MessageOwner: Send + Sync {}

#[derive(Debug)]
pub(crate) struct MessagePtr {
    pub ptr: *mut ffi::notmuch_message_t,
}

#[derive(Debug)]
pub struct Message<'d, 'q>
where
    'd: 'q,
{
    pub(crate) handle: MessagePtr,
    marker: ScopedPhantomcow<'q, Query<'d>>,
}

impl<'d, 'q> MessageOwner for Message<'d, 'q> where 'd: 'q {}
impl<'d, 'q> FilenamesOwner for Message<'d, 'q> where 'd: 'q {}
impl<'d, 'q> TagsOwner for Message<'d, 'q> where 'd: 'q {}

impl<'d, 'q> Message<'d, 'q>
where
    'd: 'q,
{
    pub fn from_ptr<P>(ptr: *mut ffi::notmuch_message_t, owner: P) -> Message<'d, 'q>
    where
        P: Into<ScopedPhantomcow<'q, Query<'d>>>,
    {
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

    pub fn replies(self: &mut Self) -> Messages<'d, 'q>
    {
        Messages::from_ptr(
            unsafe { ffi::notmuch_message_get_replies(self.handle.ptr) },
            ScopedPhantomcow::<'q, Query<'d>>::share(&mut self.marker),
        )
    }

    #[cfg(feature = "v0_26")]
    pub fn count_files(self: &Self) -> i32 {
        unsafe { ffi::notmuch_message_count_files(self.handle.ptr) }
    }

    pub fn filenames(self: &Self) -> Filenames<Self> {
        <Self as MessageExt<'d, 'q>>::filenames(self)
    }

    pub fn filename(self: &Self) -> PathBuf {
        PathBuf::from(
            unsafe { ffi::notmuch_message_get_filename(self.handle.ptr) }
                .to_str()
                .unwrap(),
        )
    }

    pub fn date(&self) -> i64 {
        unsafe { ffi::notmuch_message_get_date(self.handle.ptr) as i64 }
    }

    pub fn header(&self, name: &str) -> Result<Option<&str>> {
        let name = CString::new(name).unwrap();
        let ret = unsafe { ffi::notmuch_message_get_header(self.handle.ptr, name.as_ptr()) };
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
        <Self as MessageExt<'d, 'q>>::tags(self)
    }

    pub fn add_tag(self: &Self, tag: &str) -> Result<()> {
        let tag = CString::new(tag).unwrap();
        unsafe { ffi::notmuch_message_add_tag(self.handle.ptr, tag.as_ptr()) }.as_result()
    }

    pub fn remove_tag(self: &Self, tag: &str) -> Result<()> {
        let tag = CString::new(tag).unwrap();
        unsafe { ffi::notmuch_message_remove_tag(self.handle.ptr, tag.as_ptr()) }.as_result()
    }

    pub fn remove_all_tags(self: &Self) -> Result<()> {
        unsafe { ffi::notmuch_message_remove_all_tags(self.handle.ptr) }.as_result()
    }
}

pub trait MessageExt<'d, 'q>
where
    'd: 'q,
{
    fn tags<'s, S>(message: S) -> Tags<'s, Message<'d, 'q>>
    where
        S: Into<ScopedSupercow<'s, Message<'d, 'q>>>,
    {
        let messageref = message.into();
        Tags::from_ptr(
            unsafe { ffi::notmuch_message_get_tags(messageref.handle.ptr) },
            Supercow::phantom(messageref),
        )
    }

    // fn replies<'s, S>(message: S) -> Messages<'d, 'q>
    // where
    //     S: Into<ScopedSupercow<'s, Message<'d, 'q>>>,
    // {
    //     let messageref = message.into();
    //     Messages::from_ptr(
    //         unsafe { ffi::notmuch_message_get_replies(messageref.handle.ptr) },
    //         Supercow::phantom(messageref),
    //     )
    // }

    fn filenames<'s, S>(message: S) -> Filenames<'s, Message<'d, 'q>>
    where
        S: Into<ScopedSupercow<'s, Message<'d, 'q>>>,
    {
        let messageref = message.into();
        Filenames::from_ptr(
            unsafe { ffi::notmuch_message_get_filenames(messageref.handle.ptr) },
            Supercow::phantom(messageref),
        )
    }
}

impl<'d, 'q> MessageExt<'d, 'q> for Message<'d, 'q> where 'd: 'q {}

unsafe impl<'d, 'q> Send for Message<'d, 'q> where 'd: 'q {}
unsafe impl<'d, 'q> Sync for Message<'d, 'q> where 'd: 'q {}
