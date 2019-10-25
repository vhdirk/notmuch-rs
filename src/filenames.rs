use std::ffi::CStr;
use std::iter::Iterator;
use std::ops::Drop;
use std::path::PathBuf;

use ffi;
use utils::ScopedPhantomcow;

pub trait FilenamesOwner {}

#[derive(Debug)]
pub struct Filenames<'o, O>
where
    O: FilenamesOwner + 'o,
{
    pub(crate) ptr: *mut ffi::notmuch_filenames_t,
    pub(crate) marker: ScopedPhantomcow<'o, O>,
}

impl<'o, O> Drop for Filenames<'o, O>
where
    O: FilenamesOwner + 'o,
{
    fn drop(self: &mut Self) {
        unsafe { ffi::notmuch_filenames_destroy(self.ptr) };
    }
}

impl<'o, O> Filenames<'o, O>
where
    O: FilenamesOwner + 'o,
{
    pub(crate) fn from_ptr<P>(ptr: *mut ffi::notmuch_filenames_t, owner: P) -> Filenames<'o, O>
    where
        P: Into<ScopedPhantomcow<'o, O>>,
    {
        Filenames {
            ptr,
            marker: owner.into(),
        }
    }
}

impl<'o, O> Iterator for Filenames<'o, O>
where
    O: FilenamesOwner + 'o,
{
    type Item = PathBuf;

    fn next(self: &mut Self) -> Option<Self::Item> {
        let valid = unsafe { ffi::notmuch_filenames_valid(self.ptr) };

        if valid == 0 {
            return None;
        }

        let ctag = unsafe {
            let t = ffi::notmuch_filenames_get(self.ptr);
            ffi::notmuch_filenames_move_to_next(self.ptr);
            CStr::from_ptr(t)
        };

        Some(PathBuf::from(ctag.to_str().unwrap()))
    }
}

unsafe impl<'o, O> Send for Filenames<'o, O> where O: FilenamesOwner + 'o {}
unsafe impl<'o, O> Sync for Filenames<'o, O> where O: FilenamesOwner + 'o {}
