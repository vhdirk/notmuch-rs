use std::ffi::CStr;
use std::iter::Iterator;
use std::ops::Drop;
use std::path::PathBuf;
use std::rc::Rc;

use from_variants::FromVariants;

use Directory;
use Message;

use ffi;

#[derive(Clone, Debug, FromVariants)]
pub(crate) enum FilenamesOwner {
    Directory(Directory),
    Message(Message),
}

#[derive(Debug)]
pub(crate) struct FilenamesPtr(*mut ffi::notmuch_filenames_t);

impl Drop for FilenamesPtr {
    fn drop(&mut self) {
        unsafe { ffi::notmuch_filenames_destroy(self.0) };
    }
}

#[derive(Debug, Clone)]
pub struct Filenames {
    ptr: Rc<FilenamesPtr>,
    owner: FilenamesOwner,
}

impl Filenames {
    pub(crate) fn from_ptr<O>(ptr: *mut ffi::notmuch_filenames_t, owner: O) -> Self
    where
        O: Into<FilenamesOwner>,
    {
        Filenames {
            ptr: Rc::new(FilenamesPtr(ptr)),
            owner: owner.into(),
        }
    }
}

impl Iterator for Filenames {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        let valid = unsafe { ffi::notmuch_filenames_valid(self.ptr.0) };

        if valid == 0 {
            return None;
        }

        let ctag = unsafe {
            let t = ffi::notmuch_filenames_get(self.ptr.0);
            ffi::notmuch_filenames_move_to_next(self.ptr.0);
            CStr::from_ptr(t)
        };

        Some(PathBuf::from(ctag.to_str().unwrap()))
    }
}
