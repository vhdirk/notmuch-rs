use std::{
    ops,
    marker,
    iter
};
use std::rc::Rc;

use std::path::{
    PathBuf
};

use std::ffi::{
    CStr
};

use utils::{
    FromPtr,
    NewFromPtr
};

use database::DatabasePtr;
use directory::Directory;
use message::Message;
use ffi;


#[derive(Debug)]
pub(crate) struct FilenamesPtr {
    pub ptr: *mut ffi::notmuch_filenames_t
}

impl ops::Drop for FilenamesPtr {
    fn drop(&mut self) {
        unsafe {
            ffi::notmuch_filenames_destroy(self.ptr)
        };
    }
}

#[derive(Debug)]
enum FilenamesParent{
    Dir(Directory),
    Msg(Message)
}

#[derive(Debug)]
pub struct Filenames(pub(crate) Rc<FilenamesPtr>,  FilenamesParent);


impl NewFromPtr<*mut ffi::notmuch_filenames_t, Directory> for Filenames {
    fn new(ptr: *mut ffi::notmuch_filenames_t, parent: Directory) -> Filenames {
        Filenames(Rc::new(FilenamesPtr{ptr}), FilenamesParent::Dir(parent))
    }
}

impl NewFromPtr<*mut ffi::notmuch_filenames_t, Message> for Filenames {
    fn new(ptr: *mut ffi::notmuch_filenames_t, parent: Message) -> Filenames {
        Filenames(Rc::new(FilenamesPtr{ptr}), FilenamesParent::Msg(parent))
    }
}

impl iter::Iterator for Filenames {
    type Item = PathBuf;

    fn next(self: &mut Self) -> Option<Self::Item> {

        let valid = unsafe {
            ffi::notmuch_filenames_valid(self.0.ptr)
        };

        if valid == 0{
            return None
        }

        let ctag = unsafe {
            let t = ffi::notmuch_filenames_get(self.0.ptr);
            ffi::notmuch_filenames_move_to_next(self.0.ptr);
            CStr::from_ptr(t)
        };

        Some(PathBuf::from(ctag.to_str().unwrap()))
    }
}


unsafe impl Send for Filenames{}
