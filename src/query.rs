use std::{
    ops,
    marker
};

use error::Result;

use ffi;
use utils::{
    NewFromPtr,
};
use Database;


#[derive(Debug)]
pub struct Query<'d>(
    pub(crate) *mut ffi::notmuch_query_t,
    marker::PhantomData<&'d mut Database>,
);


impl<'d> Query<'d> {
    pub fn create(db: &'d Database, query_string: &String) -> Result<Self> {
        db.create_query(query_string)
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
