use std::ops::Drop;
use std::ptr;
use std::ffi::{CStr, CString};

use supercow::{Supercow, Phantomcow};

use error::Result;
use ffi;
use ffi::Sort;
use Database;
use Messages;
use MessagesOwner;
use Threads;
use ThreadsOwner;
use threads::ThreadsPtr;
use messages::MessagesPtr;

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

    pub(crate) fn from_ptr<O: Into<Phantomcow<'d, Database>>>(
        ptr: *mut ffi::notmuch_query_t,
        owner: O,
    ) -> Query<'d> {
        Query {
            handle: QueryPtr{ptr},
            marker: owner.into(),
        }
    }

    pub(crate) fn from_handle<O: Into<Phantomcow<'d, Database>>>(
        handle: QueryPtr,
        owner: O,
    ) -> Query<'d> {
        Query {
            handle,
            marker: owner.into(),
        }
    }

    pub fn create<D: Into<Supercow<'d, Database>>>(db: D,
                  query_string: &str) -> Result<Self> {

        let dbref = db.into();
        dbref.handle.create_query(query_string).map(move |handle|{
            Query::from_handle(handle, Supercow::phantom(dbref))
        })
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
        <Query as QueryExt>::search_messages(self)
    }

    pub fn count_messages(self: &Self) -> Result<u32> {
        let mut cnt = 0;
        try!(unsafe { ffi::notmuch_query_count_messages(self.handle.ptr, &mut cnt,) }.as_result());

        Ok(cnt)
    }

    pub fn search_threads<'q>(self: &'d Self) -> Result<Threads<'q, Self>> {
        <Query as QueryExt>::search_threads(self)
    }

    pub fn count_threads(self: &Self) -> Result<u32> {
        let mut cnt = 0;
        try!(unsafe { ffi::notmuch_query_count_threads(self.handle.ptr, &mut cnt,) }.as_result());

        Ok(cnt)
    }
}

pub trait QueryExt<'d>{
    
    fn search_threads<'q, Q: Into<Supercow<'q, Query<'d>>>>(query: Q) -> Result<Threads<'q, Query<'d>>>{
        let queryref = query.into();

        let mut thrds = ptr::null_mut();
        try!(
            unsafe { ffi::notmuch_query_search_threads(queryref.handle.ptr, &mut thrds) }.as_result()
        );

        Ok(Threads::from_ptr(thrds, Supercow::phantom(queryref)))
    }

    fn search_messages<'q, Q: Into<Supercow<'q, Query<'d>>>>(query: Q) -> Result<Messages<'q, Query<'d>>>{
        let queryref = query.into();

        let mut msgs = ptr::null_mut();
        try!(
            unsafe { ffi::notmuch_query_search_messages(queryref.handle.ptr, &mut msgs) }.as_result()
        );

        Ok(Messages::from_ptr(msgs, Supercow::phantom(queryref)))
    }

}

impl<'d> QueryExt<'d> for Query<'d>{}


unsafe impl<'d> Send for Query<'d> {}
unsafe impl<'d> Sync for Query<'d> {}
