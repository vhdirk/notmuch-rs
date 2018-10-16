use std::ops::Drop;
use std::marker::PhantomData;
use std::path::PathBuf;
use std::ffi::CString;

use error::{Error, Result};

use ffi;
use utils::{
    ToStr,
    FromPtr
};
use Query;
use Messages;
use Filenames;
use Tags;


#[derive(Debug)]
pub(crate) struct MessagePtr {
    pub ptr: *mut ffi::notmuch_message_t
}

impl Drop for MessagePtr {
    fn drop(&mut self) {
        unsafe {
            ffi::notmuch_message_destroy(self.ptr)
        };
    }
}
 
#[derive(Debug)]
pub struct Message<'d:'q, 'q>{
    pub(crate) handle: MessagePtr,
    phantom: PhantomData<&'q Query<'d>>,
}

impl<'d, 'q> FromPtr<*mut ffi::notmuch_message_t> for Message<'d, 'q> {
    fn from_ptr(ptr: *mut ffi::notmuch_message_t) -> Message<'d, 'q> {
        Message{
            handle: MessagePtr{ptr},
            phantom: PhantomData
        }
    }
}

impl<'d, 'q> Message<'d, 'q>{

    pub fn id(self: &Self) -> String{
        let mid = unsafe {
            ffi::notmuch_message_get_message_id(self.handle.ptr)
        };
        mid.to_str().unwrap().to_string()
    }

    pub fn thread_id(self: &Self) -> String{
        let tid = unsafe {
            ffi::notmuch_message_get_thread_id(self.handle.ptr)
        };
        tid.to_str().unwrap().to_string()
    }

    pub fn replies(self: &'q Self) -> Messages<'d, 'q>{
        Messages::from_ptr(unsafe {
            ffi::notmuch_message_get_replies(self.handle.ptr)
        })
    }

    #[cfg(feature = "v0_26")]
    pub fn count_files(self: &Self) -> i32{
        unsafe {
            ffi::notmuch_message_count_files(self.handle.ptr)
        }
    }

    pub fn filenames(self: &'d Self) -> Filenames<'d>{
        Filenames::from_ptr(unsafe {
            ffi::notmuch_message_get_filenames(self.handle.ptr)
        })
    }

    pub fn filename(self: &Self) -> PathBuf{
        PathBuf::from(unsafe {
            ffi::notmuch_message_get_filename(self.handle.ptr)
        }.to_str().unwrap())
    }

    pub fn header(&self, name: &str) -> Result<&str> {
        let ret = unsafe {
            ffi::notmuch_message_get_header(self.handle.ptr,
                CString::new(name).unwrap().as_ptr())
        };
        if ret.is_null() {
            Err(Error::UnspecifiedError)
        } else {
            Ok(ret.to_str().unwrap())
        }
    }

    pub fn tags(self: &'d Self) -> Tags<'d>{
        Tags::from_ptr(unsafe {
            ffi::notmuch_message_get_tags(self.handle.ptr)
        })
    }
}

unsafe impl<'d, 'q> Send for Message<'d, 'q>{}
unsafe impl<'d, 'q> Sync for Message<'d, 'q>{}
