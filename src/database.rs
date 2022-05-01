use std::ffi::{CStr, CString};
use std::fmt::Debug;
use std::ops::Drop;
use std::path::Path;
use std::ptr;
use std::rc::Rc;

use libc;
use std::cmp::{Ordering, PartialEq, PartialOrd};

use error::{Error, Result};
use ffi;
use ffi::Status;
use utils::ToStr;
use ConfigList;
use Directory;
use IndexOpts;
use Message;
use Query;
use Tags;

// Re-exported under database module for pretty namespacin'.
pub use ffi::DatabaseMode;

#[derive(Clone, Debug)]
pub struct Revision {
    pub revision: libc::c_ulong,
    pub uuid: String,
}

impl PartialEq for Revision {
    fn eq(&self, other: &Revision) -> bool {
        self.uuid == other.uuid && self.revision == other.revision
    }
}

impl PartialOrd for Revision {
    fn partial_cmp(&self, other: &Revision) -> Option<Ordering> {
        if self.uuid != other.uuid {
            return None;
        }
        self.revision.partial_cmp(&other.revision)
    }
}

#[derive(Debug)]
pub(crate) struct DatabasePtr(*mut ffi::notmuch_database_t);

impl Drop for DatabasePtr {
    fn drop(&mut self) {
        unsafe { ffi::notmuch_database_destroy(self.0) };
    }
}

#[derive(Clone, Debug)]
pub struct Database {
    ptr: Rc<DatabasePtr>,
}

impl Database {
    pub fn create<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let path_str = CString::new(path.as_ref().to_str().unwrap()).unwrap();

        let mut db = ptr::null_mut();
        unsafe { ffi::notmuch_database_create(path_str.as_ptr(), &mut db) }.as_result()?;

        Ok(Database {
            ptr: Rc::new(DatabasePtr(db)),
        })
    }

    #[deprecated = "Replaced with `open_with_config`"]
    pub fn open<P>(path: P, mode: DatabaseMode) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let path_str = CString::new(path.as_ref().to_str().unwrap()).unwrap();

        let mut db = ptr::null_mut();
        unsafe { ffi::notmuch_database_open(path_str.as_ptr(), mode.into(), &mut db) }
            .as_result()?;

        Ok(Database {
            ptr: Rc::new(DatabasePtr(db)),
        })
    }

    pub fn open_with_config<DP, CP>(
        database_path: Option<DP>,
        mode: DatabaseMode,
        config_path: Option<CP>,
        profile: Option<&str>,
    ) -> Result<Self>
    where
        DP: AsRef<Path>,
        CP: AsRef<Path>,
    {
        let database_path_str =
            database_path.map(|p| CString::new(p.as_ref().to_str().unwrap()).unwrap());
        let database_path_ptr = database_path_str
            .as_ref()
            .map(|p| p.as_ptr())
            .unwrap_or_else(|| ptr::null());

        let config_path_str =
            config_path.map(|p| CString::new(p.as_ref().to_str().unwrap()).unwrap());
        let config_path_ptr = config_path_str
            .as_ref()
            .map(|p| p.as_ptr())
            .unwrap_or_else(|| ptr::null());

        let profile_str = profile.map(|p| CString::new(p).unwrap());
        let profile_ptr = profile_str
            .as_ref()
            .map(|p| p.as_ptr())
            .unwrap_or_else(|| ptr::null());

        let mut db = ptr::null_mut();
        let mut error_message = ptr::null_mut();
        unsafe {
            ffi::notmuch_database_open_with_config(
                database_path_ptr,
                mode.into(),
                config_path_ptr,
                profile_ptr,
                &mut db,
                &mut error_message,
            )
        }
        .as_verbose_result(error_message)?;

        Ok(Database {
            ptr: Rc::new(DatabasePtr(db)),
        })
    }

    pub fn close(&self) -> Result<()> {
        unsafe { ffi::notmuch_database_close(self.ptr.0) }.as_result()?;

        Ok(())
    }

    pub fn compact<P, F>(path: P, backup_path: Option<&P>) -> Result<()>
    where
        P: AsRef<Path>,
        F: FnMut(&str),
    {
        let status: Option<F> = None;
        Database::_compact(path, backup_path, status)
    }

    pub fn compact_with_status<P, F>(path: P, backup_path: Option<&P>, status: F) -> Result<()>
    where
        P: AsRef<Path>,
        F: FnMut(&str),
    {
        Database::_compact(path, backup_path, Some(status))
    }

    fn _compact<P, F>(path: P, backup_path: Option<&P>, status: Option<F>) -> Result<()>
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
        }
        .as_result()?;

        Ok(())
    }

    pub fn path(&self) -> &Path {
        Path::new(
            unsafe { ffi::notmuch_database_get_path(self.ptr.0) }
                .to_str()
                .unwrap(),
        )
    }

    pub fn version(&self) -> u32 {
        unsafe { ffi::notmuch_database_get_version(self.ptr.0) }
    }

    #[cfg(feature = "v0_21")]
    pub fn revision(&self) -> Revision {
        let uuid_p: *const libc::c_char = ptr::null();
        let revision = unsafe {
            ffi::notmuch_database_get_revision(
                self.ptr.0,
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
        unsafe { ffi::notmuch_database_needs_upgrade(self.ptr.0) == 1 }
    }

    pub fn upgrade<F>(&self) -> Result<()>
    where
        F: FnMut(f64),
    {
        let status: Option<F> = None;
        self._upgrade(status)
    }

    pub fn upgrade_with_status<F>(&self, status: F) -> Result<()>
    where
        F: FnMut(f64),
    {
        self._upgrade(Some(status))
    }

    fn _upgrade<F>(&self, status: Option<F>) -> Result<()>
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
                self.ptr.0,
                if status.is_some() {
                    Some(wrapper::<F>)
                } else {
                    None
                },
                status.map_or(ptr::null_mut(), |f| &f as *const _ as *mut libc::c_void),
            )
        }
        .as_result()?;

        Ok(())
    }

    pub fn directory<P>(&self, path: P) -> Result<Option<Directory>>
    where
        P: AsRef<Path>,
    {
        let path_str = CString::new(path.as_ref().to_str().unwrap()).unwrap();

        let mut dir = ptr::null_mut();
        unsafe { ffi::notmuch_database_get_directory(self.ptr.0, path_str.as_ptr(), &mut dir) }
            .as_result()?;

        if dir.is_null() {
            Ok(None)
        } else {
            Ok(Some(Directory::from_ptr(dir, self.clone())))
        }
    }

    pub fn config_list(&self, prefix: &str) -> Result<ConfigList> {
        let prefix_str = CString::new(prefix).unwrap();

        let mut cfgs = ptr::null_mut();
        unsafe {
            ffi::notmuch_database_get_config_list(self.ptr.0, prefix_str.as_ptr(), &mut cfgs)
        }
        .as_result()?;

        Ok(ConfigList::from_ptr(cfgs, self.clone()))
    }

    pub fn create_query(&self, query_string: &str) -> Result<Query> {
        let query_str = CString::new(query_string).unwrap();

        let query = unsafe { ffi::notmuch_query_create(self.ptr.0, query_str.as_ptr()) };

        Ok(Query::from_ptr(query, self.clone()))
    }

    pub fn all_tags(&self) -> Result<Tags> {
        let tags = unsafe { ffi::notmuch_database_get_all_tags(self.ptr.0) };

        Ok(Tags::from_ptr(tags, self.clone()))
    }

    pub fn find_message(&self, message_id: &str) -> Result<Option<Message>> {
        let message_id_str = CString::new(message_id).unwrap();

        let mut msg = ptr::null_mut();
        unsafe {
            ffi::notmuch_database_find_message(self.ptr.0, message_id_str.as_ptr(), &mut msg)
        }
        .as_result()?;

        if msg.is_null() {
            Ok(None)
        } else {
            Ok(Some(Message::from_ptr(msg, self.clone())))
        }
    }

    pub fn find_message_by_filename<P>(&self, filename: &P) -> Result<Option<Message>>
    where
        P: AsRef<Path>,
    {
        let path_str = CString::new(filename.as_ref().to_str().unwrap()).unwrap();

        let mut msg = ptr::null_mut();
        unsafe {
            ffi::notmuch_database_find_message_by_filename(self.ptr.0, path_str.as_ptr(), &mut msg)
        }
        .as_result()?;

        if msg.is_null() {
            Ok(None)
        } else {
            Ok(Some(Message::from_ptr(msg, self.clone())))
        }
    }

    pub fn remove_message<P>(&self, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        match path.as_ref().to_str() {
            Some(path_str) => {
                let msg_path = CString::new(path_str).unwrap();

                unsafe { ffi::notmuch_database_remove_message(self.ptr.0, msg_path.as_ptr()) }
                    .as_result()
            }
            None => Err(Error::NotmuchError(Status::FileError)),
        }
    }

    pub fn default_indexopts(&self) -> Result<IndexOpts> {
        let opts = unsafe { ffi::notmuch_database_get_default_indexopts(self.ptr.0) };

        Ok(IndexOpts::from_ptr(opts, self.clone()))
    }

    pub fn index_file<P>(&self, path: P, indexopts: Option<IndexOpts>) -> Result<Message>
    where
        P: AsRef<Path>,
    {
        let opts = indexopts.map_or(ptr::null_mut(), |opt| opt.ptr.0);

        match path.as_ref().to_str() {
            Some(path_str) => {
                let msg_path = CString::new(path_str).unwrap();

                let mut msg = ptr::null_mut();
                unsafe {
                    ffi::notmuch_database_index_file(self.ptr.0, msg_path.as_ptr(), opts, &mut msg)
                }
                .as_result()?;

                Ok(Message::from_ptr(msg, self.clone()))
            }
            None => Err(Error::NotmuchError(Status::FileError)),
        }
    }

    pub fn begin_atomic(&self) -> Result<()> {
        unsafe { ffi::notmuch_database_begin_atomic(self.ptr.0) }.as_result()
    }

    pub fn end_atomic(&self) -> Result<()> {
        unsafe { ffi::notmuch_database_end_atomic(self.ptr.0) }.as_result()
    }
}

#[derive(Debug)]
pub struct AtomicOperation {
    database: Database,
}

impl AtomicOperation {
    pub fn new(database: &Database) -> Result<Self> {
        database.begin_atomic()?;
        Ok(AtomicOperation {
            database: database.clone(),
        })
    }
}

impl Drop for AtomicOperation {
    fn drop(&mut self) {
        let _ = self.database.end_atomic();
    }
}
