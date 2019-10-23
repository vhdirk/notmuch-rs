use std::ffi::{CStr, CString};
use std::ops::Drop;
use std::path::Path;
use std::ptr;

use supercow::Supercow;

use libc;

use error::{Error, Result};
use ffi;
use ffi::Status;
use utils::ToStr;
use Directory;
use Query;
use Tags;
use TagsOwner;
use Message;
use MessageOwner;
use IndexOpts;
use utils::ScopedSupercow;


// Re-exported under database module for pretty namespacin'.
pub use ffi::DatabaseMode;

#[derive(Copy, Clone, Debug)]
pub struct Version(libc::c_uint);

#[derive(Clone, Debug)]
pub struct Revision {
    pub revision: libc::c_ulong,
    pub uuid: String,
}


#[derive(Debug)]
pub struct Database {
    pub(crate) ptr: *mut ffi::notmuch_database_t,
}

impl Drop for Database {
    fn drop(&mut self) {
        unsafe { ffi::notmuch_database_destroy(self.ptr) };
    }
}

impl TagsOwner for Database {}
impl MessageOwner for Database {}

impl Database {
    pub fn create<P>(path: &P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let path_str = CString::new(path.as_ref().to_str().unwrap()).unwrap();

        let mut db = ptr::null_mut();
        unsafe { ffi::notmuch_database_create(path_str.as_ptr(), &mut db) }.as_result()?;

        Ok(Database {
            ptr: db,
        })
    }

    pub fn open<P>(path: &P, mode: DatabaseMode) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let path_str = CString::new(path.as_ref().to_str().unwrap()).unwrap();

        let mut db = ptr::null_mut();
        unsafe { ffi::notmuch_database_open(path_str.as_ptr(), mode.into(), &mut db) }
            .as_result()?;

        Ok(Database {
            ptr: db,
        })
    }

    pub fn close(&mut self) -> Result<()> {
        unsafe { ffi::notmuch_database_close(self.ptr) }.as_result()?;

        Ok(())
    }

    pub fn compact<P, F>(path: &P, backup_path: Option<&P>) -> Result<()>
    where
        P: AsRef<Path>,
        F: FnMut(&str),
    {
        let status: Option<F> = None;
        Database::_compact(path, backup_path, status)
    }

    pub fn compact_with_status<P, F>(path: &P, backup_path: Option<&P>, status: F) -> Result<()>
    where
        P: AsRef<Path>,
        F: FnMut(&str),
    {
        Database::_compact(path, backup_path, Some(status))
    }

    fn _compact<P, F>(path: &P, backup_path: Option<&P>, status: Option<F>) -> Result<()>
    where
        P: AsRef<Path>,
        F: FnMut(&str),
    {
        extern "C" fn wrapper<F: FnMut(&str)>(
            message: *const libc::c_char,
            closure: *mut libc::c_void,
        ) {
            let closure = closure as *mut F;
            unsafe { (*closure)(message.to_str().unwrap()) }
        }

        let path_str = CString::new(path.as_ref().to_str().unwrap()).unwrap();

        let backup_path = backup_path.map(|p| CString::new(p.as_ref().to_str().unwrap()).unwrap());

        unsafe {
            ffi::notmuch_database_compact(
                path_str.as_ptr(),
                backup_path.map_or(ptr::null(), |p| p.as_ptr()),
                if status.is_some() {
                    Some(wrapper::<F>)
                } else {
                    None
                },
                status.map_or(ptr::null_mut(), |f| &f as *const _ as *mut libc::c_void),
            )
        }.as_result()?;

        Ok(())
    }

    pub fn path(&self) -> &Path {
        Path::new(
            unsafe { ffi::notmuch_database_get_path(self.ptr) }
                .to_str()
                .unwrap(),
        )
    }

    pub fn version(&self) -> Version {
        Version(unsafe { ffi::notmuch_database_get_version(self.ptr) })
    }

    #[cfg(feature = "v0_21")]
    pub fn revision(&self) -> Revision {
        let uuid_p: *const libc::c_char = ptr::null();
        let revision = unsafe {
            ffi::notmuch_database_get_revision(
                self.ptr,
                (&uuid_p) as *const _ as *mut *const libc::c_char,
            )
        };

        let uuid = unsafe { CStr::from_ptr(uuid_p) };

        Revision {
            revision,
            uuid: uuid.to_string_lossy().into_owned(),
        }
    }

    pub fn needs_upgrade(&self) -> bool {
        unsafe { ffi::notmuch_database_needs_upgrade(self.ptr) == 1 }
    }

    pub fn upgrade<F>(&mut self) -> Result<()>
    where
        F: FnMut(f64),
    {
        let status: Option<F> = None;
        self._upgrade(status)
    }

    pub fn upgrade_with_status<F>(&mut self, status: F) -> Result<()>
    where
        F: FnMut(f64),
    {
        self._upgrade(Some(status))
    }

    fn _upgrade<F>(&mut self, status: Option<F>) -> Result<()>
    where
        F: FnMut(f64),
    {
        #[allow(trivial_numeric_casts)]
        extern "C" fn wrapper<F>(closure: *mut libc::c_void, progress: libc::c_double)
        where
            F: FnMut(f64),
        {
            let closure = closure as *mut F;
            unsafe { (*closure)(progress as f64) }
        }

        unsafe {
            ffi::notmuch_database_upgrade(
                self.ptr,
                if status.is_some() {
                    Some(wrapper::<F>)
                } else {
                    None
                },
                status.map_or(ptr::null_mut(), |f| &f as *const _ as *mut libc::c_void),
            )
        }.as_result()?;

        Ok(())
    }

    pub fn directory<'d, P>(&'d self, path: &P) -> Result<Option<Directory<'d>>>
    where
        P: AsRef<Path>,
    {
        <Self as DatabaseExt>::directory(self, path)
    }

    pub fn create_query<'d>(&'d self, query_string: &str) -> Result<Query<'d>> {
        <Self as DatabaseExt>::create_query(self, query_string)
    }

    pub fn all_tags<'d>(&'d self) -> Result<Tags<'d, Self>> {
        <Self as DatabaseExt>::all_tags(self)
    }

    pub fn find_message<'d>(&'d self, message_id: &str) -> Result<Option<Message<'d, Self>>> {
        <Self as DatabaseExt>::find_message(self, message_id)
    }

    pub fn find_message_by_filename<'d, P>(&'d self, filename: &P) -> Result<Option<Message<'d, Self>>>
    where
        P: AsRef<Path>,
    {
        <Self as DatabaseExt>::find_message_by_filename(self, filename)
    }

    pub fn remove_message<'d, P>(&'d self, path: &P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        <Self as DatabaseExt>::remove_message(self, path)
    }

    pub fn get_default_indexopts<'d, P>(&'d self) -> Result<IndexOpts<'d>>
    {
        <Self as DatabaseExt>::get_default_indexopts(self)
    }

    pub fn index_file<'d, P>(&'d self, path: &P, indexopts: Option<IndexOpts<'d>>) -> Result<Message<'d, Self>>
    where
        P: AsRef<Path>,
    {
        <Self as DatabaseExt>::index_file(self, path, indexopts)
    }

    pub fn begin_atomic(&self) -> Result<()> {
        unsafe { ffi::notmuch_database_begin_atomic(self.ptr) }.as_result()
    }

    pub fn end_atomic(&self) -> Result<()> {
        unsafe { ffi::notmuch_database_end_atomic(self.ptr) }.as_result()
    }
}

pub trait DatabaseExt {
    fn create_query<'d, D>(database: D, query_string: &str) -> Result<Query<'d>>
    where
        D: Into<Supercow<'d, Database>>,
    {
        let dbref = database.into();
        let query_str = CString::new(query_string).unwrap();

        let query = unsafe { ffi::notmuch_query_create(dbref.ptr, query_str.as_ptr()) };

        Ok(Query::from_ptr(query, Supercow::phantom(dbref)))
    }

    fn all_tags<'d, D>(database: D) -> Result<Tags<'d, Database>>
    where
        D: Into<ScopedSupercow<'d, Database>>,
    {
        let dbref = database.into();

        let tags = unsafe { ffi::notmuch_database_get_all_tags(dbref.ptr) };

        Ok(Tags::from_ptr(tags, ScopedSupercow::phantom(dbref)))
    }

    fn directory<'d, D, P>(database: D, path: &P) -> Result<Option<Directory<'d>>>
    where
        D: Into<ScopedSupercow<'d, Database>>,
        P: AsRef<Path>,
    {
        let dbref = database.into();

        let path_str = CString::new(path.as_ref().to_str().unwrap()).unwrap();

        let mut dir = ptr::null_mut();
        unsafe {
            ffi::notmuch_database_get_directory(dbref.ptr, path_str.as_ptr(), &mut dir)
        }.as_result()?;

        if dir.is_null() {
            Ok(None)
        } else {
            Ok(Some(Directory::from_ptr(dir, Supercow::phantom(dbref))))
        }
    }

    fn find_message<'d, D>(database: D, message_id: &str) -> Result<Option<Message<'d, Database>>>
    where
        D: Into<ScopedSupercow<'d, Database>>
    {
        let dbref = database.into();
        let message_id_str = CString::new(message_id).unwrap();

        let mut msg = ptr::null_mut();
        unsafe {
            ffi::notmuch_database_find_message(dbref.ptr, message_id_str.as_ptr(), &mut msg)
        }.as_result()?;

        if msg.is_null() {
            Ok(None)
        } else {
            Ok(Some(Message::from_ptr(msg, Supercow::phantom(dbref))))
        }
    }

    fn find_message_by_filename<'d, D, P>(database: D, filename: &P) -> Result<Option<Message<'d, Database>>>
    where
        D: Into<ScopedSupercow<'d, Database>>,
        P: AsRef<Path>
    {
        let dbref = database.into();
        let path_str = CString::new(filename.as_ref().to_str().unwrap()).unwrap();

        let mut msg = ptr::null_mut();
        unsafe {
            ffi::notmuch_database_find_message_by_filename(dbref.ptr, path_str.as_ptr(), &mut msg)
        }.as_result()?;

        if msg.is_null() {
            Ok(None)
        } else {
            Ok(Some(Message::from_ptr(msg, Supercow::phantom(dbref))))
        }
    }

    fn remove_message<'d, D, P>(database: D, path: &P) -> Result<()>
    where
        D: Into<ScopedSupercow<'d, Database>>,
        P: AsRef<Path>,
    {
        let dbref = database.into();
        match path.as_ref().to_str() {
            Some(path_str) => {
                let msg_path = CString::new(path_str).unwrap();

                unsafe { ffi::notmuch_database_remove_message(dbref.ptr, msg_path.as_ptr()) }
                    .as_result()
            }
            None => Err(Error::NotmuchError(Status::FileError)),
        }
    }

    fn get_default_indexopts<'d, D>(database: D) -> Result<IndexOpts<'d>>
    where
        D: Into<ScopedSupercow<'d, Database>>
    {
        let dbref = database.into();

        let opts = unsafe { ffi::notmuch_database_get_default_indexopts(dbref.ptr) };

        Ok(IndexOpts::from_ptr(opts, ScopedSupercow::phantom(dbref)))
    }


    fn index_file<'d, D, P>(database: D, path: &P, indexopts: Option<IndexOpts<'d>>) -> Result<Message<'d, Database>>
    where
        D: Into<ScopedSupercow<'d, Database>>,
        P: AsRef<Path>,
    {
        let dbref = database.into();

        let opts = indexopts.map_or(ptr::null_mut(), |opt| opt.ptr);

        match path.as_ref().to_str() {
            Some(path_str) => {
                let msg_path = CString::new(path_str).unwrap();

                let mut msg = ptr::null_mut();
                unsafe { ffi::notmuch_database_index_file(dbref.ptr, msg_path.as_ptr(), opts, &mut msg) }
                    .as_result()?;
                
                Ok(Message::from_ptr(msg, Supercow::phantom(dbref)))
            }
            None => Err(Error::NotmuchError(Status::FileError)),
        }
    }
}

impl DatabaseExt for Database {}

unsafe impl Send for Database {}
unsafe impl Sync for Database {}
