use std::rc::Rc;
use std::ffi::{CStr, CString};
use std::ops::Drop;
use std::ptr;

use error::Result;
use ffi;
use ffi::{Exclude, Sort};
use Database;
use Messages;
use Threads;

#[derive(Debug)]
pub(crate) struct QueryPtr(*mut ffi::notmuch_query_t);

impl Drop for QueryPtr {
    fn drop(&mut self) {
        unsafe { ffi::notmuch_query_destroy(self.0) };
    }
}

#[derive(Clone, Debug)]
pub struct Query{
    ptr: Rc<QueryPtr>,
    owner: Box<Database>,
}

impl Query {
    pub(crate) fn from_ptr(ptr: *mut ffi::notmuch_query_t, owner: Database) -> Self
    {
        Query {
            ptr: Rc::new(QueryPtr(ptr)),
            owner: Box::new(owner),
        }
    }

    pub fn create(database: &Database, query_string: &str) -> Result<Self>
    {
        database.create_query(query_string)
    }

    pub fn query_string(&self) -> String
    {
        let qstring =
            unsafe { CStr::from_ptr(ffi::notmuch_query_get_query_string(self.ptr.0)) };
        qstring.to_str().unwrap().to_string()
    }

    /// Specify the sorting desired for this query.
    pub fn set_sort(&self, sort: Sort)
    {
        unsafe { ffi::notmuch_query_set_sort(self.ptr.0, sort.into()) }
    }

    /// Return the sort specified for this query. See
    /// `set_sort`.
    pub fn sort(&self) -> Sort
    {
        unsafe { ffi::notmuch_query_get_sort(self.ptr.0) }.into()
    }

    /// Filter messages according to the query and return
    pub fn search_messages(&self) -> Result<Messages>
    {
        let mut msgs = ptr::null_mut();
        unsafe { ffi::notmuch_query_search_messages(self.ptr.0, &mut msgs) }.as_result()?;

        Ok(Messages::from_ptr(msgs, self.clone()))
    }

    pub fn count_messages(&self) -> Result<u32>
    {
        let mut cnt = 0;
        unsafe { ffi::notmuch_query_count_messages(self.ptr.0, &mut cnt) }.as_result()?;

        Ok(cnt)
    }

    pub fn search_threads(&self) -> Result<Threads>
    {
        let mut thrds = ptr::null_mut();
        unsafe { ffi::notmuch_query_search_threads(self.ptr.0, &mut thrds) }.as_result()?;

        Ok(Threads::from_ptr(thrds, self.clone()))
    }

    pub fn count_threads(&self) -> Result<u32>
    {
        let mut cnt = 0;
        unsafe { ffi::notmuch_query_count_threads(self.ptr.0, &mut cnt) }.as_result()?;

        Ok(cnt)
    }

    pub fn add_tag_exclude(&self, tag: &str) -> Result<()>
    {
        let tag_str = CString::new(tag).unwrap();
        unsafe { ffi::notmuch_query_add_tag_exclude(self.ptr.0, tag_str.as_ptr()) }
            .as_result()
    }

    pub fn set_omit_excluded(&self, omit_excluded: Exclude)
    {
        unsafe { ffi::notmuch_query_set_omit_excluded(self.ptr.0, omit_excluded.into()) }
    }
}
