use std::ops::Drop;
use std::rc::Rc;
use std::path::PathBuf;

use ffi;
use utils::{
    ToStr,
    NewFromPtr
};
use query::Query;
use Messages;
use Filenames;

#[derive(Debug)]
pub(crate) struct MessagePtr {
    pub(crate) ptr: *mut ffi::notmuch_message_t
}

impl Drop for MessagePtr {
    fn drop(&mut self) {
        unsafe {
            ffi::notmuch_message_destroy(self.ptr)
        };
    }
}

#[derive(Debug)]
pub struct Message(pub(crate) Rc<MessagePtr>, Query);


impl NewFromPtr<*mut ffi::notmuch_message_t, Query> for Message {
    fn new(ptr: *mut ffi::notmuch_message_t, parent: Query) -> Message {
        Message(Rc::new(MessagePtr{ptr}), parent)
    }
}

impl Message{

    pub fn id(self: &Self) -> String{
        let mid = unsafe {
            ffi::notmuch_message_get_message_id(self.0.ptr)
        };
        mid.to_str().unwrap().to_string()
    }

    pub fn thread_id(self: &Self) -> String{
        let tid = unsafe {
            ffi::notmuch_message_get_thread_id(self.0.ptr)
        };
        tid.to_str().unwrap().to_string()
    }

    pub fn replies(self: &Self) -> Messages{
        Messages::new(unsafe {
            ffi::notmuch_message_get_replies(self.0.ptr)
        }, self.1.clone())
    }

    #[cfg(feature = "0.26")]
    pub fn count_files(self: &Self) -> i32{
        unsafe {
            ffi::notmuch_message_count_files(self.0.ptr)
        }
    }

    pub fn filenames(self: &Self) -> Filenames{
        Filenames::new(unsafe {
            ffi::notmuch_message_get_filenames(self.0.ptr)
        }, self.clone())
    }

    pub fn filename(self: &Self) -> PathBuf{
        PathBuf::from(unsafe {
            ffi::notmuch_message_get_filename(self.0.ptr)
        }.to_str().unwrap())
    }
}

impl Clone for Message {
    fn clone(&self) -> Self {
        Message(self.0.clone(), self.1.clone())
    }
}
