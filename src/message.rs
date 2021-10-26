use from_variants::FromVariants;
use std::borrow::Cow;
use std::ffi::CString;
use std::path::Path;
use std::ptr;
use std::rc::Rc;

use error::{Error, Result};
use ffi;
use utils::ToStr;
use Filenames;
use IndexOpts;
use MessageProperties;
use Database;
use Messages;
use Query;
use Thread;
use Tags;

#[derive(Clone, Debug, FromVariants)]
pub(crate) enum MessageOwner {
    Database(Database),
    Messages(Messages),
    Thread(Thread),
    Query(Query),
}

#[derive(Debug)]
pub(crate) struct MessagePtr(*mut ffi::notmuch_message_t);


impl Drop for MessagePtr {
    fn drop(&mut self) {
        unsafe { ffi::notmuch_message_destroy(self.0) };
    }
}

#[derive(Clone, Debug)]
pub struct Message {
    ptr: Rc<MessagePtr>,
    owner: Box<MessageOwner>,
}

impl Message {
    pub(crate) fn from_ptr<O>(ptr: *mut ffi::notmuch_message_t, owner: O) -> Message
    where
        O: Into<MessageOwner>,
    {
        Message {
            ptr: Rc::new(MessagePtr(ptr)),
            owner: Box::new(owner.into()),
        }
    }

    pub fn id(&self) -> Cow<'_, str> {
        let mid = unsafe { ffi::notmuch_message_get_message_id(self.ptr.0) };
        mid.to_string_lossy()
    }

    pub fn thread_id(&self) -> Cow<'_, str> {
        let tid = unsafe { ffi::notmuch_message_get_thread_id(self.ptr.0) };
        tid.to_string_lossy()
    }

    pub fn replies(&self) -> Messages {
        Messages::from_ptr(
            unsafe { ffi::notmuch_message_get_replies(self.ptr.0) },
            self.clone(),
        )
    }

    #[cfg(feature = "v0_26")]
    pub fn count_files(&self) -> i32 {
        unsafe { ffi::notmuch_message_count_files(self.ptr.0) }
    }

    pub fn filenames(&self) -> Filenames {
        Filenames::from_ptr(
            unsafe { ffi::notmuch_message_get_filenames(self.ptr.0) },
            self.clone(),
        )
    }

    pub fn filename(&self) -> &Path {
        Path::new(
            unsafe { ffi::notmuch_message_get_filename(self.ptr.0) }
                .to_str()
                .unwrap(),
        )
    }

    pub fn date(&self) -> i64 {
        unsafe { ffi::notmuch_message_get_date(self.ptr.0) as i64 }
    }

    pub fn header(&self, name: &str) -> Result<Option<Cow<'_, str>>> {
        let name = CString::new(name).unwrap();
        let ret = unsafe { ffi::notmuch_message_get_header(self.ptr.0, name.as_ptr()) };
        if ret.is_null() {
            Err(Error::UnspecifiedError)
        } else {
            let ret_str = ret.to_string_lossy();
            if ret_str.is_empty() {
                Ok(None)
            } else {
                Ok(Some(ret_str))
            }
        }
    }

    pub fn tags(&self) -> Tags {
        Tags::from_ptr(
            unsafe { ffi::notmuch_message_get_tags(self.ptr.0) },
            self.clone(),
        )
    }

    pub fn add_tag(&self, tag: &str) -> Result<()> {
        let tag = CString::new(tag).unwrap();
        unsafe { ffi::notmuch_message_add_tag(self.ptr.0, tag.as_ptr()) }.as_result()
    }

    pub fn remove_tag(&self, tag: &str) -> Result<()> {
        let tag = CString::new(tag).unwrap();
        unsafe { ffi::notmuch_message_remove_tag(self.ptr.0, tag.as_ptr()) }.as_result()
    }

    pub fn remove_all_tags(&self) -> Result<()> {
        unsafe { ffi::notmuch_message_remove_all_tags(self.ptr.0) }.as_result()
    }

    pub fn tags_to_maildir_flags(&self) -> Result<()> {
        unsafe { ffi::notmuch_message_tags_to_maildir_flags(self.ptr.0) }.as_result()
    }

    pub fn maildir_flags_to_tags(&self) -> Result<()> {
        unsafe { ffi::notmuch_message_maildir_flags_to_tags(self.ptr.0) }.as_result()
    }

    pub fn reindex(&self, indexopts: IndexOpts) -> Result<()> {
        unsafe { ffi::notmuch_message_reindex(self.ptr.0, indexopts.ptr.0) }.as_result()
    }

    pub fn freeze(&self) -> Result<()> {
        unsafe { ffi::notmuch_message_freeze(self.ptr.0) }.as_result()
    }

    pub fn thaw(&self) -> Result<()> {
        unsafe { ffi::notmuch_message_thaw(self.ptr.0) }.as_result()
    }

    pub fn properties(&self, key: &str, exact: bool) -> MessageProperties {
        let key_str = CString::new(key).unwrap();

        let props = unsafe {
            ffi::notmuch_message_get_properties(self.ptr.0, key_str.as_ptr(), exact as i32)
        };

        MessageProperties::from_ptr(props, self.clone())
    }

    pub fn remove_all_properties(&self, key: Option<&str>) -> Result<()> {
        match key {
            Some(k) => {
                let key_str = CString::new(k).unwrap();
                unsafe { ffi::notmuch_message_remove_all_properties(self.ptr.0, key_str.as_ptr()) }
                    .as_result()
            }
            None => {
                let p = ptr::null();
                unsafe { ffi::notmuch_message_remove_all_properties(self.ptr.0, p) }.as_result()
            }
        }
    }

    pub fn remove_all_properties_with_prefix(&self, prefix: Option<&str>) -> Result<()> {
        match prefix {
            Some(k) => {
                let key_str = CString::new(k).unwrap();
                unsafe {
                    ffi::notmuch_message_remove_all_properties_with_prefix(
                        self.ptr.0,
                        key_str.as_ptr(),
                    )
                }
                .as_result()
            }
            None => {
                let p = ptr::null();
                unsafe { ffi::notmuch_message_remove_all_properties_with_prefix(self.ptr.0, p) }
                    .as_result()
            }
        }
    }

    pub fn count_properties(&self, key: &str) -> Result<u32> {
        let key_str = CString::new(key).unwrap();
        let mut cnt = 0;
        unsafe { ffi::notmuch_message_count_properties(self.ptr.0, key_str.as_ptr(), &mut cnt) }
            .as_result()?;

        Ok(cnt)
    }

    pub fn property(&self, key: &str) -> Result<Cow<'_, str>> {
        let key_str = CString::new(key).unwrap();
        let mut prop = ptr::null();
        unsafe { ffi::notmuch_message_get_property(self.ptr.0, key_str.as_ptr(), &mut prop) }
            .as_result()?;

        if prop.is_null() {
            Err(Error::UnspecifiedError)
        } else {
            // TODO: the unwrap here is not good
            Ok(prop.to_string_lossy())
        }
    }

    pub fn add_property(&self, key: &str, value: &str) -> Result<()> {
        let key_str = CString::new(key).unwrap();
        let value_str = CString::new(value).unwrap();
        unsafe {
            ffi::notmuch_message_add_property(self.ptr.0, key_str.as_ptr(), value_str.as_ptr())
        }
        .as_result()
    }

    pub fn remove_property(&self, key: &str, value: &str) -> Result<()> {
        let key_str = CString::new(key).unwrap();
        let value_str = CString::new(value).unwrap();
        unsafe {
            ffi::notmuch_message_remove_property(self.ptr.0, key_str.as_ptr(), value_str.as_ptr())
        }
        .as_result()
    }
}

pub struct FrozenMessage(Message);

impl FrozenMessage {
    pub fn new(message: &Message) -> Result<Self> {
        message.freeze()?;
        Ok(FrozenMessage(message.clone()))
    }
}

impl Drop for FrozenMessage {
    fn drop(&mut self) {
        let _ = self.0.thaw();
    }
}
