use std::{
    ops,
    path,
    ptr,
};

use std::ffi::CString;
use std::os::raw::c_char;

use libc;

use error::Result;
use utils::{
    NewFromPtr,
    ToStr,
};

use ffi;

use database::Database;

#[derive(Debug)]
pub struct Query(pub(crate) *mut ffi::notmuch_query_t);

impl Query {
    pub fn create(db: &Database, query_string: &String) -> Result<Self> {
        db.create_query(query_string)
    }
}
