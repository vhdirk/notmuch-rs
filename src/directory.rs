use std::ffi::{CStr, CString};
use std::ops::Drop;
use std::path::Path;
use std::ptr;
use supercow::{Supercow, Phantomcow};

use error::Result;
use ffi;
use Database;
use Filenames;
use FilenamesOwner;

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
    marker: Phantomcow<'d, Database>,
}

impl<'d> FilenamesOwner for Directory<'d> {}

impl<'d> Directory<'d> {
    pub fn from_ptr<O: Into<Phantomcow<'d, Database>>>(
        ptr: *mut ffi::notmuch_directory_t,
        owner: O,
    ) -> Directory<'d> {
        Directory {
            handle: DirectoryPtr { ptr },
            marker: owner.into(),
        }
    }

    pub fn new<O: Into<Supercow<'d, Database>>, 
               P: AsRef<Path>>(owner: O, path: &P) -> Result<Option<Directory<'d>>> {
        let db = owner.into();
        let path_str = CString::new(path.as_ref().to_str().unwrap()).unwrap();

        let mut dir = ptr::null_mut();
        try!(
            unsafe {
                ffi::notmuch_database_get_directory(db.handle.ptr, path_str.as_ptr(), &mut dir)
            }
            .as_result()
        );

        if dir.is_null() {
            Ok(None)
        } else {
            Ok(Some(Directory {
                handle: DirectoryPtr { ptr: dir },
                marker: Supercow::phantom(db),
            }))
        }
    }


    pub fn child_directories(&self) -> Filenames<Self> {
        Filenames::from_ptr(
            unsafe { ffi::notmuch_directory_get_child_directories(self.handle.ptr) },
            self,
        )
    }
}

unsafe impl<'d> Send for Directory<'d> {}
unsafe impl<'d> Sync for Directory<'d> {}
