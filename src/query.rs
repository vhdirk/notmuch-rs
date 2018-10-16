use std::ops::Drop;
use std::ptr;
use std::marker::PhantomData;

use error::Result;

use ffi;
use utils::FromPtr;

use Database;
use Messages;
use Threads;
use ffi::Sort;

#[derive(Debug)]
pub(crate) struct QueryPtr {
    pub ptr: *mut ffi::notmuch_query_t
}

impl Drop for QueryPtr {
    fn drop(&mut self) {
        unsafe {
            ffi::notmuch_query_destroy(self.ptr)
        };
    }
}

#[derive(Debug)]
pub struct Query<'d>{
    pub(crate) handle: QueryPtr,
    phantom: PhantomData<&'d Database>,
}

impl<'d> FromPtr<*mut ffi::notmuch_query_t> for Query<'d> {
    fn from_ptr(ptr: *mut ffi::notmuch_query_t) -> Query<'d> {
        Query{
            handle: QueryPtr{ptr},
            phantom: PhantomData
        }
    }
}

impl<'d> Query<'d> {
    pub fn create(db: &'d Database, query_string: &str) -> Result<Self> {
        db.create_query(query_string)
    }

    /// Specify the sorting desired for this query.
    pub fn set_sort(self: &Self, sort: Sort)
    {
        unsafe {
            ffi::notmuch_query_set_sort(
                self.handle.ptr, sort.into(),
            )
        }
    }

    /// Return the sort specified for this query. See
    /// `set_sort`.
    pub fn sort(self: &Self) -> Sort
    {
        unsafe {
            ffi::notmuch_query_get_sort(
                self.handle.ptr,
            )
        }.into()
    }


    /// Filter messages according to the query and return
    pub fn search_messages<'q>(self: &'d Self) -> Result<Messages<'q, 'd>>
    {
        let mut msgs = ptr::null_mut();
        try!(unsafe {
            ffi::notmuch_query_search_messages(
                self.handle.ptr, &mut msgs,
            )
        }.as_result());

        Ok(Messages::from_ptr(msgs))
    }

    pub fn count_messages(self: &Self) -> Result<u32>
    {
        let mut cnt = 0;
        try!(unsafe {
            ffi::notmuch_query_count_messages(
                self.handle.ptr, &mut cnt,
            )
        }.as_result());

        Ok(cnt)
    }

    pub fn search_threads<'q>(self: &'d Self) -> Result<Threads<'q, 'd>>
    {
        let mut thrds = ptr::null_mut();
        try!(unsafe {
            ffi::notmuch_query_search_threads(
                self.handle.ptr, &mut thrds,
            )
        }.as_result());

        Ok(Threads::from_ptr(thrds))
    }

    pub fn count_threads(self: &Self) -> Result<u32>
    {
        let mut cnt = 0;
        try!(unsafe {
            ffi::notmuch_query_count_threads(
                self.handle.ptr, &mut cnt,
            )
        }.as_result());

        Ok(cnt)
    }
}


unsafe impl<'d> Send for Query<'d> {}
unsafe impl<'d> Sync for Query<'d> {}
