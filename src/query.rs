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
use Database;
use Messages;


#[derive(Debug)]
pub struct Query<'d>(
    pub(crate) *mut ffi::notmuch_query_t,
    marker::PhantomData<&'d mut Database>,
);


impl<'d> Query<'d> {
    pub fn create(db: &'d Database, query_string: &String) -> Result<Self> {
        db.create_query(query_string)
    }

    /// Filter messages according to the query and return
    pub fn search_messages(self: &Self) -> Result<Option<Messages>>
    {
        let mut msgs = ptr::null_mut();
        try!(unsafe {
            ffi::notmuch_query_search_messages(
                self.0, &mut msgs,
            )
        }.as_result());

        match msgs.is_null() {
            false => Ok(None),
            true => Ok(Some(Messages::new(msgs))),
        }
    }
}

impl<'d> NewFromPtr<*mut ffi::notmuch_query_t> for Query<'d> {
    fn new(ptr: *mut ffi::notmuch_query_t) -> Query<'d> {
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
