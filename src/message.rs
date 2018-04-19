use std::ops::Drop;
use std::marker::PhantomData;
use std::path::PathBuf;

use ffi;
use utils::{
    ToStr,
    NewFromPtr
};
use Query;
use Messages;
use Filenames;

#[derive(Debug)]
pub struct Message<'q, 'd:'q>(
    pub(crate) *mut ffi::notmuch_message_t,
    PhantomData<&'q Query<'d>>,
);

impl<'q, 'd> NewFromPtr<*mut ffi::notmuch_message_t> for Message<'q, 'd> {
    fn new(ptr: *mut ffi::notmuch_message_t) -> Message<'q, 'd> {
        Message(ptr, PhantomData)
    }
}

impl<'q, 'd> Message<'q, 'd>{

    pub fn id(self: &Self) -> String{
        let mid = unsafe {
            ffi::notmuch_message_get_message_id(self.0)
        };
        mid.to_str().unwrap().to_string()
    }

    pub fn thread_id(self: &Self) -> String{
        let tid = unsafe {
            ffi::notmuch_message_get_thread_id(self.0)
        };
        tid.to_str().unwrap().to_string()
    }

    pub fn replies(self: &'q Self) -> Messages<'q, 'd>{
        Messages::new(unsafe {
            ffi::notmuch_message_get_replies(self.0)
        })
    }

    #[cfg(feature = "v0_26")]
    pub fn count_files(self: &Self) -> i32{
        unsafe {
            ffi::notmuch_message_count_files(self.0)
        }
    }

    pub fn filenames(self: &'d Self) -> Filenames<'d>{
        Filenames::new(unsafe {
            ffi::notmuch_message_get_filenames(self.0)
        })
    }

    pub fn filename(self: &Self) -> PathBuf{
        PathBuf::from(unsafe {
            ffi::notmuch_message_get_filename(self.0)
        }.to_str().unwrap())
    }
}


impl<'q, 'd> Drop for Message<'q, 'd> {
    fn drop(self: &mut Self) {
        unsafe {
            ffi::notmuch_message_destroy(self.0)
        };
    }
}

unsafe impl<'q, 'd> Send for Message<'q, 'd>{}
unsafe impl<'q, 'd> Sync for Message<'q, 'd>{}
