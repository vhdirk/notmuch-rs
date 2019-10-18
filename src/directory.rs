use std::ops::Drop;
use supercow::Supercow;

use ffi;
use Database;
use Filenames;
use FilenamesOwner;
use utils::{ScopedSupercow, ScopedPhantomcow};


#[derive(Debug)]
pub struct Directory<'d> {
    ptr: *mut ffi::notmuch_directory_t,
    marker: ScopedPhantomcow<'d, Database>,
}

impl<'d> Drop for Directory<'d> {
    fn drop(&mut self) {
        unsafe { ffi::notmuch_directory_destroy(self.ptr) };
    }
}

impl<'d> FilenamesOwner for Directory<'d> {}

impl<'d> Directory<'d> {
    pub(crate) fn from_ptr<O>(ptr: *mut ffi::notmuch_directory_t, owner: O) -> Directory<'d>
    where
        O: Into<ScopedPhantomcow<'d, Database>>,
    {
        Directory {
            ptr,
            marker: owner.into(),
        }
    }

    pub fn child_directories(&self) -> Filenames<Self> {
        <Self as DirectoryExt>::child_directories(self)
    }
}

pub trait DirectoryExt<'d> {
    fn child_directories<'s, S>(directory: S) -> Filenames<'s, Directory<'d>>
    where
        S: Into<ScopedSupercow<'s, Directory<'d>>>,
    {
        let dir = directory.into();
        Filenames::from_ptr(
            unsafe { ffi::notmuch_directory_get_child_directories(dir.ptr) },
            Supercow::phantom(dir),
        )
    }
}

impl<'d> DirectoryExt<'d> for Directory<'d> {}

unsafe impl<'d> Send for Directory<'d> {}
unsafe impl<'d> Sync for Directory<'d> {}
