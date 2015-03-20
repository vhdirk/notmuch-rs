use libc;

use ffi;

// Re-exported under database module for pretty namespacin'.
pub use ffi::NotmuchDatabaseMode as Mode;

#[derive(Copy, Debug)]
pub struct Version(libc::c_uint);

pub struct Database(*mut ffi::notmuch_database_t);
