use std::ops::Drop;
use supercow::{Phantomcow, Supercow};

use crate::ffi;
use crate::Database;
use crate::Filenames;
use crate::FilenamesOwner;
use crate::utils::{ScopedSupercow, ScopedPhantomcow};

#[derive(Debug)]
pub(crate) struct DirectoryPtr {
    pub ptr: *mut ffi::notmuch_directory_t,
}

impl Drop for DirectoryPtr {
    fn drop(&mut self) {
        unsafe { ffi::notmuch_directory_destroy(self.ptr) };
    }
}

#[derive(Debug)]
pub struct Directory<'d> {
    handle: DirectoryPtr,
    marker: ScopedPhantomcow<'d, Database>,
}

impl<'d> FilenamesOwner for Directory<'d> {}

impl<'d> Directory<'d> {
    pub fn from_ptr<O>(ptr: *mut ffi::notmuch_directory_t, owner: O) -> Directory<'d>
    where
        O: Into<ScopedPhantomcow<'d, Database>>,
    {
        Directory {
            handle: DirectoryPtr { ptr },
            marker: owner.into(),
        }
    }

    pub fn child_directories(&self) -> Filenames<'_, Self> {
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
            unsafe { ffi::notmuch_directory_get_child_directories(dir.handle.ptr) },
            Supercow::phantom(dir),
        )
    }
}

impl<'d> DirectoryExt<'d> for Directory<'d> {}

unsafe impl<'d> Send for Directory<'d> {}
unsafe impl<'d> Sync for Directory<'d> {}
