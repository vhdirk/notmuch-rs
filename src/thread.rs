use std;
use std::{
    ops,
    marker,
    ptr,
};

use error::Result;

use ffi;
use utils::{
    NewFromPtr,
};
use Query;

#[derive(Debug)]
pub struct Thread<'q, 'd:'q>(
    pub(crate) *mut ffi::notmuch_thread_t,
    marker::PhantomData<&'q mut Query<'d>>,
);

impl<'q, 'd> NewFromPtr<*mut ffi::notmuch_thread_t> for Thread<'q, 'd> {
    fn new(ptr: *mut ffi::notmuch_thread_t) -> Thread<'q, 'd> {
        Thread(ptr, marker::PhantomData)
    }
}

// impl<'d> Thread<'d>(
//
//
//
// };
//

impl<'q, 'd> ops::Drop for Thread<'q, 'd> {
    fn drop(&mut self) {
        unsafe {
            ffi::notmuch_thread_destroy(self.0)
        };
    }
}
