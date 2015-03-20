use std::{
    path,
    ptr,
};

use libc;

use error::Result;
use utils::ToCString;

use ffi;

// Re-exported under database module for pretty namespacin'.
pub use ffi::NotmuchDatabaseMode as Mode;

#[derive(Copy, Debug)]
pub struct Version(libc::c_uint);

pub struct Database(*mut ffi::notmuch_database_t);

impl Database {
    pub fn create<P: path::AsPath>(path: &P) -> Result<Database> {
        let path = path.as_path().to_cstring().unwrap();

        let mut db = ptr::null_mut();
        try!(unsafe {
            ffi::notmuch_database_create(path.as_ptr(), &mut db)
        }.as_result());

        Ok(Database(db))
    }
}
