use std::{
    ops,
    marker,
};
use std::rc::Rc;

use utils::{
    FromPtr,
};

use Database;
use Filenames;

use ffi;

// #[derive(Debug)]
// pub struct Directory{
//     ptr: *mut ffi::notmuch_directory_t,
//     db: Rc<Database>,
// }

#[derive(Debug)]
pub struct Directory<'d>(
    *mut ffi::notmuch_directory_t,
    marker::PhantomData<&'d Database>
);


impl<'d> Directory<'d>{

    pub fn child_directories(self: &Self) -> Filenames{
        Filenames::from_ptr(unsafe {
            ffi::notmuch_directory_get_child_directories(self.0)
        })
    }
}

impl<'d> FromPtr<*mut ffi::notmuch_directory_t> for Directory<'d> {
    fn from_ptr(ptr: *mut ffi::notmuch_directory_t) -> Directory<'d> {
        Directory(ptr, marker::PhantomData)
    }
}

impl<'d> ops::Drop for Directory<'d> {
    fn drop(self: &mut Self) {
        unsafe {
            ffi::notmuch_directory_destroy(self.0)
        };
    }
}

unsafe impl<'d> Send for Directory<'d>{}
