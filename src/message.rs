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
pub struct Message<'d:'q, 'q>(
    pub(crate) *mut ffi::notmuch_message_t,
    PhantomData<&'q Query<'d>>,
);

impl<'d, 'q> NewFromPtr<*mut ffi::notmuch_message_t> for Message<'d, 'q> {
    fn new(ptr: *mut ffi::notmuch_message_t) -> Message<'d, 'q> {
        Message(ptr, PhantomData)
    }
}

impl<'d, 'q> Message<'d, 'q>{

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

    pub fn replies(self: &'q Self) -> Messages<'d, 'q>{
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


impl<'d, 'q> Drop for Message<'d, 'q> {
    fn drop(self: &mut Self) {
        unsafe {
            ffi::notmuch_message_destroy(self.0)
        };
    }
}

unsafe impl<'d, 'q> Send for Message<'d, 'q>{}
unsafe impl<'d, 'q> Sync for Message<'d, 'q>{}
