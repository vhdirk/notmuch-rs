use std::ops::Drop;
use std::rc::Rc;

use ffi;
use Database;
use Filenames;

#[derive(Debug)]
pub(crate) struct DirectoryPtr(*mut ffi::notmuch_directory_t);

#[derive(Debug, Clone)]
pub struct Directory {
    ptr: Rc<DirectoryPtr>,
    owner: Database,
}

impl Drop for Directory {
    fn drop(&mut self) {
        unsafe { ffi::notmuch_directory_destroy(self.ptr.0) };
    }
}

impl Directory {
    pub(crate) fn from_ptr(
        ptr: *mut ffi::notmuch_directory_t,
        owner: Database,
    ) -> Directory {
        Directory {
            ptr: Rc::new(DirectoryPtr(ptr)),
            owner,
        }
    }

    fn child_directories(&self) -> Filenames {
        Filenames::from_ptr(
            unsafe { ffi::notmuch_directory_get_child_directories(self.ptr.0) },
            self.clone(),
        )
    }
}
