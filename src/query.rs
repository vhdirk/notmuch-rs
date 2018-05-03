use std::ops::Drop;
use std::ptr;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use std::result;
use error::{Result, Error};

use ffi;
use utils::NewFromPtr;

use database::Database;
use messages::Messages;
use Threads;
use ffi::Sort;

#[derive(Debug)]
pub(crate) struct QueryPtr {
    pub(crate) ptr: *mut ffi::notmuch_query_t
}

impl Drop for QueryPtr {
    fn drop(&mut self) {
        unsafe {
            ffi::notmuch_query_destroy(self.ptr)
        };
    }
}

#[derive(Debug)]
pub struct Query(Arc<RwLock<QueryPtr>>, Database);


impl Query {
    pub fn create(db: Database, query_string: &str) -> Result<Self> {
        db.create_query(query_string)
    }

    /// Specify the sorting desired for this query.
    pub fn set_sort(self: &Self, sort: Sort) -> Result<()>
    {
        match self.0.try_write(){
            Ok(guard) => Ok(unsafe {
                ffi::notmuch_query_set_sort(guard.ptr, sort.into())
            }),
            Err(err) => Err(err.into())
        }
    }

    /// Return the sort specified for this query. See
    /// `set_sort`.
    pub fn sort(self: &Self) -> Result<Sort>
    {
        match self.0.try_read(){
            Ok(guard) => Ok(unsafe {
                    ffi::notmuch_query_get_sort(guard.ptr)
                }.into()
            ),
            Err(err) => Err(err.into())
        }
    }


    /// Filter messages according to the query and return
    pub fn search_messages(self: &Self) -> Result<Messages>
    {
        match self.0.try_read(){
            Ok(guard) => {
                let mut msgs = ptr::null_mut();
                unsafe {
                    ffi::notmuch_query_search_messages(guard.ptr, &mut msgs)
                }.as_result();

                Ok(Messages::new(msgs, self.clone()))
            },
            Err(err) => Err(err.into())
        }
    }

    pub fn count_messages(self: &Self) -> Result<u32>
    {
        match self.0.try_read(){
            Ok(guard) => {
                let mut cnt = 0;
                unsafe {
                    ffi::notmuch_query_count_messages(guard.ptr, &mut cnt)
                }.as_result();

                Ok(cnt)
            },
            Err(err) => Err(err.into())
        }
    }

    pub fn search_threads(self: & Self) -> Result<Threads>
    {
        match self.0.try_read(){
            Ok(guard) => {
                let mut thrds = ptr::null_mut();
                try!(unsafe {
                    ffi::notmuch_query_search_threads(guard.ptr, &mut thrds)
                }.as_result());

                Ok(Threads::new(thrds, self.clone()))
            },
            Err(err) => Err(err.into())
        }
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
