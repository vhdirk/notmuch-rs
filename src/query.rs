use std::{
    ops,
};

use error::Result;

use ffi;

use database::Database;

#[derive(Debug)]
pub struct Query(pub(crate) *mut ffi::notmuch_query_t);

impl Query {
    pub fn create(db: &Database, query_string: &String) -> Result<Self> {
        db.create_query(query_string)
    }
}


impl ops::Drop for Query {
    fn drop(&mut self) {
        unsafe {
            ffi::notmuch_query_destroy(self.0)
        };
    }
}
