use std::{
    ops,
    path,
    ptr,
};

use libc;

use error::Result;
use utils::{
    NotmuchType,
    ToCString,
    ToStr,
};

use ffi;

// Re-exported under database module for pretty namespacin'.
pub use ffi::NotmuchDatabaseMode as Mode;

#[derive(Copy, Debug)]
pub struct Version(libc::c_uint);

pub struct Database(*mut ffi::notmuch_database_t);

impl Database {
    pub fn create<P: AsRef<path::Path>>(path: &P) -> Result<Database> {
        let path = path.to_cstring().unwrap();

        let mut db = ptr::null_mut();
        try!(unsafe {
            ffi::notmuch_database_create(path.as_ptr(), &mut db)
        }.as_result());

        Ok(Database(db))
    }

    pub fn open<P: AsRef<path::Path>>(path: &P, mode: Mode) -> Result<Database> {
        let path = path.to_cstring().unwrap();

        let mut db = ptr::null_mut();
        try!(unsafe {
            ffi::notmuch_database_open(
                path.as_ptr(), mode.to_notmuch_t(), &mut db,
            )
        }.as_result());

        Ok(Database(db))
    }

    pub fn close(self) -> Result<()> {
        try!(unsafe {
            ffi::notmuch_database_close(self.0)
        }.as_result());

        Ok(())
    }

    pub fn compact<P: AsRef<path::Path>, F: FnMut(&str)>(
        path: &P, backup_path: Option<&P>,
    ) -> Result<()> {
        let status: Option<F> = None;
        Database::_compact(path, backup_path, status)
    }

    pub fn compact_with_status<P: AsRef<path::Path>, F: FnMut(&str)>(
        path: &P, backup_path: Option<&P>, status: F,
    ) -> Result<()> {
        Database::_compact(path, backup_path, Some(status))
    }

    fn _compact<P: AsRef<path::Path>, F: FnMut(&str)>(
        path: &P, backup_path: Option<&P>, status: Option<F>,
    ) -> Result<()> {

        extern fn wrapper<F: FnMut(&str)>(
            message:*const libc::c_char, closure: *mut libc::c_void,
        ) {
            let closure = closure as *mut F;
            unsafe {
                (*closure)(message.to_str().unwrap())
            }
        }

        let path = path.to_cstring().unwrap();
        let backup_path = backup_path.map(|p| {
            p.to_cstring().unwrap()
        });

        try!(unsafe {
            ffi::notmuch_database_compact(
                path.as_ptr(), backup_path.map_or(ptr::null(), |p| p.as_ptr()),
                if status.is_some() { Some(wrapper::<F>) } else { None },
                status.map_or(ptr::null_mut(), |f| {
                    &f as *const _ as *mut libc::c_void
                }),
            )
        }.as_result());

        Ok(())
    }

    pub fn path(&self) -> &path::Path {
        path::Path::new(unsafe {
            ffi::notmuch_database_get_path(self.0)
        }.to_str().unwrap())
    }

    pub fn version(&self) -> Version {
        Version(unsafe {
            ffi::notmuch_database_get_version(self.0)
        })
    }

    pub fn needs_upgrade(&self) -> bool {
        unsafe {
            ffi::notmuch_database_needs_upgrade(self.0) == 1
        }
    }

    pub fn upgrade<F: FnMut(f64)>(&mut self) -> Result<()> {
        let status: Option<F> = None;
        self._upgrade(status)
    }

    pub fn upgrade_with_status<F: FnMut(f64)>(&mut self, status: F) -> Result<()> {
        self._upgrade(Some(status))
    }

    fn _upgrade<F: FnMut(f64)>(&mut self, status: Option<F>) -> Result<()> {

        extern fn wrapper<F: FnMut(f64)>(
            closure: *mut libc::c_void, progress: libc::c_double,
        ) {
            let closure = closure as *mut F;
            unsafe {
                (*closure)(progress as f64)
            }
        }

        try!(unsafe {
            ffi::notmuch_database_upgrade(
                self.0,
                if status.is_some() { Some(wrapper::<F>) } else { None },
                status.map_or(ptr::null_mut(), |f| {
                    &f as *const _ as *mut libc::c_void
                }),
            )
        }.as_result());

        Ok(())
    }
}

impl ops::Drop for Database {
    fn drop(&mut self) {
        unsafe {
            ffi::notmuch_database_destroy(self.0)
        };
    }
}
