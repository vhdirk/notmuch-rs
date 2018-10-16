use std::ops::Drop;
use std::iter::Iterator;
use std::marker::PhantomData;
use std::path::PathBuf;
use std::ffi::CStr;

use utils::FromPtr;
use Database;
use ffi;

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
pub struct Filenames<'d>{
    pub(crate) handle: FilenamesPtr,
    phantom: PhantomData<&'d Database>
}

impl<'d> FromPtr<*mut ffi::notmuch_filenames_t> for Filenames<'d> {
    fn from_ptr(ptr: *mut ffi::notmuch_filenames_t) -> Filenames<'d> {
        Filenames{
            handle: FilenamesPtr{ptr},
            phantom: PhantomData
        }
    }
}

impl<'d> Iterator for Filenames<'d> {
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

unsafe impl<'d> Send for Filenames<'d>{}
unsafe impl<'d> Sync for Filenames<'d>{}
