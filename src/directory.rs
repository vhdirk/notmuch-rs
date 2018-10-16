use std::ops::Drop;
use std::marker::PhantomData;

use utils::FromPtr;

use Database;
use Filenames;

use ffi;

#[derive(Debug)]
pub(crate) struct DirectoryPtr {
    pub ptr: *mut ffi::notmuch_directory_t
}

impl Drop for DirectoryPtr {
    fn drop(&mut self) {
        unsafe {
            ffi::notmuch_directory_destroy(self.ptr)
        };
    }
}

#[derive(Debug)]
pub struct Directory<'d>{
    handle: DirectoryPtr,
    phantom: PhantomData<&'d Database>,
}

impl<'d> Directory<'d>{
    pub fn child_directories(self: &'d Self) -> Filenames<'d>{
        Filenames::from_ptr(unsafe {
            ffi::notmuch_directory_get_child_directories(self.handle.ptr)
        })
    }
}

impl<'d> FromPtr<*mut ffi::notmuch_directory_t> for Directory<'d> {
    fn from_ptr(ptr: *mut ffi::notmuch_directory_t) -> Directory<'d> {
        Directory{
            handle: DirectoryPtr{ptr},
            phantom: PhantomData
        }
    }
}

unsafe impl<'d> Send for Directory<'d>{}
unsafe impl<'d> Sync for Directory<'d>{}
