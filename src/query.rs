use std::ops::Drop;
use std::ptr;

use supercow::{Phantomcow, Supercow};

use error::Result;
use ffi;
use ffi::Sort;
use Database;
use Messages;
use MessageOwner;
use Threads;
use utils::ScopedSupercow;

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

impl<'d> MessageOwner for Query<'d> {}

impl<'d> Query<'d> {
    pub(crate) fn from_ptr<O>(ptr: *mut ffi::notmuch_query_t, owner: O) -> Query<'d>
    where
        O: Into<Phantomcow<'d, Database>>,
    {
        Query {
            handle: QueryPtr { ptr },
            marker: owner.into(),
        }
    }

    pub(crate) fn from_handle<O>(handle: QueryPtr, owner: O) -> Query<'d>
    where
        O: Into<Phantomcow<'d, Database>>,
    {
        Query {
            handle,
            marker: owner.into(),
        }
    }

    pub fn create<D>(db: D, query_string: &str) -> Result<Self>
    where
        D: Into<Supercow<'d, Database>>,
    {
        let dbref = db.into();
        dbref
            .handle
            .create_query(query_string)
            .map(move |handle| Query::from_handle(handle, Supercow::phantom(dbref)))
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
        unsafe { ffi::notmuch_query_count_messages(self.handle.ptr, &mut cnt) }.as_result()?;

        Ok(cnt)
    }

    pub fn search_threads<'q>(self: &'d Self) -> Result<Threads<'d, 'q>> {
        <Query<'d> as QueryExt>::search_threads(self)
    }

    pub fn count_threads(self: &Self) -> Result<u32> {
        let mut cnt = 0;
        unsafe { ffi::notmuch_query_count_threads(self.handle.ptr, &mut cnt) }.as_result()?;

        Ok(cnt)
    }
}

pub trait QueryExt<'d> {
    fn search_threads<'q, Q>(query: Q) -> Result<Threads<'d, 'q>>
    where
        Q: Into<ScopedSupercow<'q, Query<'d>>>,
    {
        let queryref = query.into();

        let mut thrds = ptr::null_mut();
        unsafe { ffi::notmuch_query_search_threads(queryref.handle.ptr, &mut thrds) }
            .as_result()?;

        Ok(Threads::from_ptr(thrds, ScopedSupercow::phantom(queryref)))
    }

    fn search_messages<'q, Q>(query: Q) -> Result<Messages<'q, Query<'d>>>
    where
        Q: Into<ScopedSupercow<'q, Query<'d>>>,
    {
        let queryref = query.into();

        let mut msgs = ptr::null_mut();
        unsafe { ffi::notmuch_query_search_messages(queryref.handle.ptr, &mut msgs) }
            .as_result()?;

        Ok(Messages::from_ptr(msgs, ScopedSupercow::phantom(queryref)))
    }
}

impl<'d> QueryExt<'d> for Query<'d> {}

unsafe impl<'d> Send for Query<'d> {}
unsafe impl<'d> Sync for Query<'d> {}
