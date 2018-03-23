use std;
use std::{
    ops,
    marker,
    str,
    result
};

use error::Result;

use ffi;
use utils::{
    NewFromPtr,
    ToStr
};
use Query;
use Messages;
use Filenames;

#[derive(Debug)]
pub struct Message<'q, 'd:'q>(
    pub(crate) *mut ffi::notmuch_message_t,
    marker::PhantomData<&'q mut Query<'d>>,
);

impl<'q, 'd> NewFromPtr<*mut ffi::notmuch_message_t> for Message<'q, 'd> {
    fn new(ptr: *mut ffi::notmuch_message_t) -> Message<'q, 'd> {
        Message(ptr, marker::PhantomData)
    }
}

impl<'q, 'd> Message<'q, 'd>{

    pub fn id(self: &Self) -> result::Result<&'q str, str::Utf8Error>{
        let tid = unsafe {
            ffi::notmuch_message_get_message_id(self.0)
        };
        tid.to_str()
    }

    pub fn replies(self: &Self) -> Messages<'q, 'd>{
        Messages::new(unsafe {
            ffi::notmuch_message_get_replies(self.0)
        })
    }

    pub fn count_files(self: &Self) -> i32
    {
        unsafe {
            ffi::notmuch_message_count_files(self.0)
        }
    }

    pub fn filenames(self: &Self) -> Filenames<'d>{
        Filenames::new(unsafe {
            ffi::notmuch_message_get_filenames(self.0)
        })
    }
}


impl<'q, 'd> ops::Drop for Message<'q, 'd> {
    fn drop(&mut self) {
        unsafe {
            ffi::notmuch_message_destroy(self.0)
        };
    }
}
