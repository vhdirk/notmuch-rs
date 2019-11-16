use std::ffi::{CString, CStr};
use std::path::{Path, PathBuf};
use std::cell::RefCell;
use std::borrow::Cow;
use std::ptr;

use supercow::{Supercow};

use error::{Error, Result};
use ffi;
use utils::{ToStr, ScopedPhantomcow, ScopedSupercow};
use Filenames;
use FilenamesOwner;
use Messages;
use MessageProperties;
use Tags;
use TagsOwner;
use IndexOpts;

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


// impl<'o, O> PartialEq for Message<'o, O>
// where
//     O: MessageOwner + 'o
// {
//     fn eq(self: &Self, other: &Message<'o, O>) -> bool{
//         self.id() == other.id()
//     }
// }

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

    pub fn id(self: &Self) -> Cow<'_, str> {
        let mid = unsafe { ffi::notmuch_message_get_message_id(self.ptr) };
        mid.to_string_lossy()
    }

    pub fn thread_id(self: &Self) -> Cow<'_, str> {
        let tid = unsafe { ffi::notmuch_message_get_thread_id(self.ptr) };
        tid.to_string_lossy()
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

    pub fn header(&self, name: &str) -> Result<Option<Cow<'_, str>>> {
        let name = CString::new(name).unwrap();
        let ret = unsafe { ffi::notmuch_message_get_header(self.ptr, name.as_ptr()) };
        if ret.is_null() {
            Err(Error::UnspecifiedError)
        } else {
            let ret_str = ret.to_string_lossy();
            if ret_str.is_empty() {
                Ok(None)
            } else{
                Ok(Some(ret_str))
            }
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

    pub fn reindex<'d>(self: &Self, indexopts: IndexOpts<'d>) -> Result<()> {
        unsafe { ffi::notmuch_message_reindex(self.ptr, indexopts.ptr) }.as_result()
    }

    pub fn freeze(self: &Self) -> Result<()> {
        unsafe { ffi::notmuch_message_freeze(self.ptr) }.as_result()
    }

    pub fn thaw(self: &Self) -> Result<()> {
        unsafe { ffi::notmuch_message_thaw(self.ptr) }.as_result()
    }

    pub fn properties<'m>(&'m self, key: &str, exact: bool) -> MessageProperties<'m, 'o, O> {
        <Self as MessageExt<'o, O>>::properties(self, key, exact)
    }

    pub fn remove_all_properties(&self, key: Option<&str>) -> Result<()>
    {
        match key {
            Some(k) => {
                let key_str = CString::new(k).unwrap();
                unsafe {
                    ffi::notmuch_message_remove_all_properties(self.ptr, key_str.as_ptr())
                }.as_result()
            },
            None => {
                let p = ptr::null();
                unsafe {
                    ffi::notmuch_message_remove_all_properties(self.ptr, p)
                }.as_result()
            }
        }
    }

    pub fn remove_all_properties_with_prefix(&self, prefix: Option<&str>) -> Result<()>
    {
        match prefix {
            Some(k) => {
                let key_str = CString::new(k).unwrap();
                unsafe {
                    ffi::notmuch_message_remove_all_properties_with_prefix(self.ptr, key_str.as_ptr())
                }.as_result()
            },
            None => {
                let p = ptr::null();
                unsafe {
                    ffi::notmuch_message_remove_all_properties_with_prefix(self.ptr, p)
                }.as_result()
            }
        }
    }


    pub fn count_properties(&self, key: &str) -> Result<u32>
    {
        let key_str = CString::new(key).unwrap();
        let mut cnt = 0;
        unsafe {
            ffi::notmuch_message_count_properties(self.ptr, key_str.as_ptr(), &mut cnt)
        }.as_result()?;

        Ok(cnt)
    }

    pub fn property(&self, key: &str) -> Result<Cow<'_, str>>
    {
        let key_str = CString::new(key).unwrap();
        let mut prop = ptr::null();
        unsafe {
            ffi::notmuch_message_get_property(self.ptr, key_str.as_ptr(), &mut prop)
        }.as_result()?;

        if prop.is_null() {
            Err(Error::UnspecifiedError)
        } else {
            // TODO: the unwrap here is not good
            Ok(prop.to_string_lossy())
        }
    }

    pub fn add_property(&self, key: &str, value: &str) -> Result<()>
    {
        let key_str = CString::new(key).unwrap();
        let value_str = CString::new(value).unwrap();
        unsafe {
            ffi::notmuch_message_add_property(self.ptr, key_str.as_ptr(), value_str.as_ptr())
        }.as_result()
    }

    pub fn remove_property(&self, key: &str, value: &str) -> Result<()>
    {
        let key_str = CString::new(key).unwrap();
        let value_str = CString::new(value).unwrap();
        unsafe {
            ffi::notmuch_message_remove_property(self.ptr, key_str.as_ptr(), value_str.as_ptr())
        }.as_result()
    }
}

pub trait MessageExt<'o, O>
where
    O: MessageOwner + 'o,
{
    fn tags<'m, M>(message: M) -> Tags<'m, Message<'o, O>>
    where
        M: Into<ScopedSupercow<'m, Message<'o, O>>>,
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

    fn filenames<'m, M>(message: M) -> Filenames<'m, Message<'o, O>>
    where
        M: Into<ScopedSupercow<'m, Message<'o, O>>>,
    {
        let messageref = message.into();
        Filenames::from_ptr(
            unsafe { ffi::notmuch_message_get_filenames(messageref.ptr) },
            Supercow::phantom(messageref),
        )
    }

    fn properties<'m, M>(message: M, key: &str, exact: bool) -> MessageProperties<'m, 'o, O>
    where
        M: Into<ScopedSupercow<'m, Message<'o, O>>>,
    {
        let messageref = message.into();
        let key_str = CString::new(key).unwrap();

        let props = unsafe {
            ffi::notmuch_message_get_properties(messageref.ptr, key_str.as_ptr(), exact as i32)
        };

        MessageProperties::from_ptr(props, Supercow::phantom(messageref))
    }
}

impl<'o, O> MessageExt<'o, O> for Message<'o, O> where O: MessageOwner + 'o {}

unsafe impl<'o, O> Send for Message<'o, O> where O: MessageOwner + 'o {}
unsafe impl<'o, O> Sync for Message<'o, O> where O: MessageOwner + 'o {}


pub struct FrozenMessage<'m ,'o, O>
where
    O: MessageOwner + 'o
{
    message: ScopedSupercow<'m, Message<'o, O>>
}


impl<'m, 'o, O> FrozenMessage<'m, 'o, O>
where
    O: MessageOwner + 'o
{
    pub fn new<M>(message: M) -> Result<Self>
    where
        M: Into<ScopedSupercow<'m, Message<'o, O>>>
    {
        let msg = message.into();
        msg.freeze()?;
        Ok(FrozenMessage{
            message: msg
        })
    }
}

impl<'m, 'o, O> Drop for FrozenMessage<'m, 'o, O>
where
    O: MessageOwner + 'o
{
    fn drop(&mut self) {
        let _ = self.message.thaw();
    }
}


