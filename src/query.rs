use std::{
    ops,
    marker,
    ptr,
};
use std::rc::Rc;

use error::Result;

use ffi;
use utils::{
    FromPtr,
};
use Database;
use Messages;
use Threads;
use ffi::Sort;

#[derive(Debug)]
pub struct Query<'d>(
    pub(crate) *mut ffi::notmuch_query_t,
    marker::PhantomData<&'d Database>,
);


impl<'d> Query<'d> {
    pub fn create(db: &'d Database, query_string: &str) -> Result<Self> {
        db.create_query(query_string)
    }

    /// Specify the sorting desired for this query.
    pub fn set_sort(self: &Self, sort: Sort)
    {
        unsafe {
            ffi::notmuch_query_set_sort(
                self.0, sort.into(),
            )
        }
    }

    /// Return the sort specified for this query. See
    /// `set_sort`.
    pub fn sort(self: &Self) -> Sort
    {
        unsafe {
            ffi::notmuch_query_get_sort(
                self.0,
            )
        }.into()
    }


    /// Filter messages according to the query and return
    pub fn search_messages(self: &Self) -> Result<Messages>
    {
        let mut msgs = ptr::null_mut();
        try!(unsafe {
            ffi::notmuch_query_search_messages(
                self.0, &mut msgs,
            )
        }.as_result());

        Ok(Messages::from_ptr(msgs))
    }

    pub fn count_messages(self: &Self) -> Result<u32>
    {
        let mut cnt = 0;
        try!(unsafe {
            ffi::notmuch_query_count_messages(
                self.0, &mut cnt,
            )
        }.as_result());

        Ok(cnt)
    }

    pub fn search_threads(self: & Self) -> Result<Threads>
    {
        let mut thrds = ptr::null_mut();
        try!(unsafe {
            ffi::notmuch_query_search_threads(
                self.0, &mut thrds,
            )
        }.as_result());

        Ok(Threads::from_ptr(thrds))
    }

    pub fn count_threads(self: &Self) -> Result<u32>
    {
        let mut cnt = 0;
        try!(unsafe {
            ffi::notmuch_query_count_threads(
                self.0, &mut cnt,
            )
        }.as_result());

        Ok(cnt)
    }
}

impl<'d> FromPtr<*mut ffi::notmuch_query_t> for Query<'d> {
    fn from_ptr(ptr: *mut ffi::notmuch_query_t) -> Query<'d> {
        Query(ptr, marker::PhantomData)
    }
}


impl<'d> ops::Drop for Query<'d> {
    fn drop(&mut self) {
        unsafe {
            ffi::notmuch_query_destroy(self.0)
        };
    }
}

unsafe impl<'d> Send for Query<'d> {}
