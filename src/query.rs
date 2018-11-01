use std::ops::Drop;
use std::ptr;
use supercow::Phantomcow;

use error::Result;
use ffi;
use ffi::Sort;
use Database;
use Messages;
use MessagesOwner;
use Threads;
use ThreadsOwner;

#[derive(Debug)]
pub(crate) struct QueryPtr {
    pub ptr: *mut ffi::notmuch_query_t,
}

impl Drop for QueryPtr {
    fn drop(&mut self) {
        unsafe { ffi::notmuch_query_destroy(self.ptr) };
    }
}

#[derive(Debug)]
pub struct Query<'d> {
    pub(crate) handle: QueryPtr,
    marker: Phantomcow<'d, Database>,
}

impl<'d> ThreadsOwner for Query<'d> {}
impl<'d> MessagesOwner for Query<'d> {}

impl<'d> Query<'d> {
    pub fn from_ptr<O: Into<Phantomcow<'d, Database>>>(
        ptr: *mut ffi::notmuch_query_t,
        owner: O,
    ) -> Query<'d> {
        Query {
            handle: QueryPtr { ptr },
            marker: owner.into(),
        }
    }

    pub fn create(db: &'d Database, query_string: &str) -> Result<Self> {
        db.create_query(query_string)
    }

    /// Specify the sorting desired for this query.
    pub fn set_sort(self: &Self, sort: Sort) {
        unsafe { ffi::notmuch_query_set_sort(self.handle.ptr, sort.into()) }
    }

    /// Return the sort specified for this query. See
    /// `set_sort`.
    pub fn sort(self: &Self) -> Sort {
        unsafe { ffi::notmuch_query_get_sort(self.handle.ptr) }.into()
    }

    /// Filter messages according to the query and return
    pub fn search_messages<'q>(self: &'d Self) -> Result<Messages<'q, Self>> {
        let mut msgs = ptr::null_mut();
        try!(
            unsafe { ffi::notmuch_query_search_messages(self.handle.ptr, &mut msgs,) }.as_result()
        );

        Ok(Messages::from_ptr(msgs, self))
    }

    pub fn count_messages(self: &Self) -> Result<u32> {
        let mut cnt = 0;
        try!(unsafe { ffi::notmuch_query_count_messages(self.handle.ptr, &mut cnt,) }.as_result());

        Ok(cnt)
    }

    pub fn search_threads<'q>(self: &'d Self) -> Result<Threads<'q, Self>> {
        let mut thrds = ptr::null_mut();
        try!(
            unsafe { ffi::notmuch_query_search_threads(self.handle.ptr, &mut thrds,) }.as_result()
        );

        Ok(Threads::from_ptr(thrds, self))
    }

    pub fn count_threads(self: &Self) -> Result<u32> {
        let mut cnt = 0;
        try!(unsafe { ffi::notmuch_query_count_threads(self.handle.ptr, &mut cnt,) }.as_result());

        Ok(cnt)
    }
}

unsafe impl<'d> Send for Query<'d> {}
unsafe impl<'d> Sync for Query<'d> {}
