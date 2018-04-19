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
    NewFromPtr
};
use database::{Database, DatabasePtr};
use messages::Messages;
use Threads;
use ffi::Sort;

#[derive(Debug)]
pub(crate) struct QueryPtr {
    pub ptr: *mut ffi::notmuch_query_t
}

impl ops::Drop for QueryPtr {
    fn drop(&mut self) {
        unsafe {
            ffi::notmuch_query_destroy(self.ptr)
        };
    }
}

impl !Send for QueryPtr {}
impl !Sync for QueryPtr {}

#[derive(Debug)]
pub struct Query(Rc<QueryPtr>, Database);


impl Query {
    pub fn create(db: Database, query_string: &str) -> Result<Self> {
        db.create_query(query_string)
    }

    /// Specify the sorting desired for this query.
    pub fn set_sort(self: &Self, sort: Sort)
    {
        unsafe {
            ffi::notmuch_query_set_sort(
                self.0.ptr, sort.into(),
            )
        }
    }

    /// Return the sort specified for this query. See
    /// `set_sort`.
    pub fn sort(self: &Self) -> Sort
    {
        unsafe {
            ffi::notmuch_query_get_sort(
                self.0.ptr,
            )
        }.into()
    }


    /// Filter messages according to the query and return
    pub fn search_messages(self: &Self) -> Result<Messages>
    {
        let mut msgs = ptr::null_mut();
        try!(unsafe {
            ffi::notmuch_query_search_messages(
                self.0.ptr, &mut msgs,
            )
        }.as_result());

        Ok(Messages::new(msgs, self.clone()))
    }

    pub fn count_messages(self: &Self) -> Result<u32>
    {
        let mut cnt = 0;
        try!(unsafe {
            ffi::notmuch_query_count_messages(
                self.0.ptr, &mut cnt,
            )
        }.as_result());

        Ok(cnt)
    }

    pub fn search_threads(self: & Self) -> Result<Threads>
    {
        let mut thrds = ptr::null_mut();
        try!(unsafe {
            ffi::notmuch_query_search_threads(
                self.0.ptr, &mut thrds,
            )
        }.as_result());

        Ok(Threads::new(thrds, self.clone()))
    }

    pub fn count_threads(self: &Self) -> Result<u32>
    {
        let mut cnt = 0;
        try!(unsafe {
            ffi::notmuch_query_count_threads(
                self.0.ptr, &mut cnt,
            )
        }.as_result());

        Ok(cnt)
    }
}

// impl FromPtr<*mut ffi::notmuch_query_t> for Query {
//     fn from_ptr(ptr: *mut ffi::notmuch_query_t) -> Query {
//         Query(ptr, marker::PhantomData)
//     }
// }

impl NewFromPtr<*mut ffi::notmuch_query_t, Database> for Query {
    fn new(ptr: *mut ffi::notmuch_query_t, parent: Database) -> Query {
        Query(Rc::new(QueryPtr{ptr}), parent)
    }
}

impl Clone for Query {
    fn clone(&self) -> Self {
        Query(self.0.clone(), self.1.clone())
    }
}


// unsafe impl Send for Query {}
// impl !Sync for Query{}
