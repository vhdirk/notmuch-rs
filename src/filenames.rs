use std::ops::Drop;
use std::iter::Iterator;
use std::marker::PhantomData;
use std::path::PathBuf;
use std::ffi::CStr;

use utils::FromPtr;
use Database;
use ffi;

pub trait FilenamesOwner{}

#[derive(Debug)]
pub(crate) struct FilenamesPtr {
    pub ptr: *mut ffi::notmuch_filenames_t
}

impl Drop for FilenamesPtr {
    fn drop(self: &mut Self) {
        let valid = unsafe {
            ffi::notmuch_filenames_valid(self.ptr)
        };

        if valid != 0 {
            unsafe {
                ffi::notmuch_filenames_destroy(self.ptr)
            };
        }
    }
}
 
#[derive(Debug)]
pub struct Filenames<'o, Owner: FilenamesOwner>{
    pub(crate) handle: FilenamesPtr,
    pub(crate) phantom: PhantomData<&'o Owner>
}

impl<'o, Owner: FilenamesOwner> FromPtr<*mut ffi::notmuch_filenames_t> for Filenames<'o, Owner> {
    fn from_ptr(ptr: *mut ffi::notmuch_filenames_t) -> Filenames<'o, Owner> {
        Filenames{
            handle: FilenamesPtr{ptr},
            phantom: PhantomData
        }
    }
}

impl<'o, Owner: FilenamesOwner> Iterator for Filenames<'o, Owner> {
    type Item = PathBuf;

    fn next(self: &mut Self) -> Option<Self::Item> {

        let valid = unsafe {
            ffi::notmuch_filenames_valid(self.handle.ptr)
        };

        if valid == 0{
            return None
        }

        let ctag = unsafe {
            let t = ffi::notmuch_filenames_get(self.handle.ptr);
            ffi::notmuch_filenames_move_to_next(self.handle.ptr);
            CStr::from_ptr(t)
        };

        Some(PathBuf::from(ctag.to_str().unwrap()))
    }
}

unsafe impl<'o, Owner: FilenamesOwner> Send for Filenames<'o, Owner>{}
unsafe impl<'o, Owner: FilenamesOwner> Sync for Filenames<'o, Owner>{}
