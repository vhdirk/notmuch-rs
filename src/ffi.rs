#![allow(dead_code, non_camel_case_types)]

//! Re-presentation of the notmuch C API.

use libc::{c_char, c_double, c_int, c_uint, c_ulong, c_void, time_t};

use error::{Error, Result};
use std::{error, fmt, str};

use utils::ToStr;

notmuch_enum! {
    #[repr(C)]
    #[derive(Debug, Eq, PartialEq, Clone, Copy)]
    pub enum notmuch_status_t => Status {
        NOTMUCH_STATUS_SUCCESS => Success,
        NOTMUCH_STATUS_OUT_OF_MEMORY => OutOfMemory,
        NOTMUCH_STATUS_READ_ONLY_DATABASE => ReadOnlyDatabase,
        NOTMUCH_STATUS_XAPIAN_EXCEPTION => XapianException,
        NOTMUCH_STATUS_FILE_ERROR => FileError,
        NOTMUCH_STATUS_FILE_NOT_EMAIL => FileNotEmail,
        NOTMUCH_STATUS_DUPLICATE_MESSAGE_ID => DuplicateMessageID,
        NOTMUCH_STATUS_NULL_POINTER => NullPointer,
        NOTMUCH_STATUS_TAG_TOO_LONG => TagTooLong,
        NOTMUCH_STATUS_UNBALANCED_FREEZE_THAW => UnbalancedFreezeThaw,
        NOTMUCH_STATUS_UNBALANCED_ATOMIC => UnbalancedAtomic,
        NOTMUCH_STATUS_UNSUPPORTED_OPERATION => UnsupportedOperation,
        NOTMUCH_STATUS_UPGRADE_REQUIRED => UpgradeRequired,
        // Not an actual status value.  Just a way to find out how many
        // valid status values there are.
        NOTMUCH_STATUS_LAST_STATUS => LastStatus
    }
}

impl notmuch_status_t {
    pub fn is_ok(self) -> bool {
        match self {
            notmuch_status_t::NOTMUCH_STATUS_SUCCESS => true,
            _ => false,
        }
    }

    pub fn is_err(self) -> bool {
        !self.is_ok()
    }

    pub fn as_result(self) -> Result<()> {
        if self.is_ok() {
            Ok(())
        } else {
            Err(Error::NotmuchError(Status::from(self)))
        }
    }
}

impl ToStr for Status {
    fn to_str<'a>(&self) -> std::result::Result<&'a str, str::Utf8Error> {
        unsafe { notmuch_status_to_string((*self).into()) }.to_str()
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str().unwrap())
    }
}

impl error::Error for Status {
    fn description(&self) -> &str {
        self.to_str().unwrap()
    }
}

notmuch_enum! {
    #[repr(C)]
    #[derive(Debug, Eq, PartialEq, Clone, Copy)]
    pub enum notmuch_database_mode_t => DatabaseMode {
        NOTMUCH_DATABASE_MODE_READ_ONLY => ReadOnly,
        NOTMUCH_DATABASE_MODE_READ_WRITE => ReadWrite
    }
}

notmuch_enum! {
    #[repr(C)]
    #[derive(Debug, Eq, PartialEq, Clone, Copy)]
    pub enum notmuch_sort_t => Sort {
        NOTMUCH_SORT_OLDEST_FIRST => OldestFirst,
        NOTMUCH_SORT_NEWEST_FIRST => NewestFirst,
        NOTMUCH_SORT_MESSAGE_ID => MessageID,
        NOTMUCH_SORT_UNSORTED => Unsorted
    }
}

notmuch_enum! {
    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    pub enum notmuch_exclude_t => Exclude {
        NOTMUCH_EXCLUDE_FLAG => Flag,
        NOTMUCH_EXCLUDE_TRUE => True,
        NOTMUCH_EXCLUDE_FALSE => False,
        NOTMUCH_EXCLUDE_ALL => All
    }
}

notmuch_enum! {
    #[repr(C)]
    #[derive(Debug, Eq, PartialEq, Clone, Copy)]
    pub enum notmuch_message_flag_t => MessageFlag {
        NOTMUCH_MESSAGE_FLAG_MATCH => Match,
        NOTMUCH_MESSAGE_FLAG_EXCLUDED => Excluded,
        NOTMUCH_MESSAGE_FLAG_GHOST => Ghost
    }
}

notmuch_enum! {
    #[repr(C)]
    #[derive(Debug, Eq, PartialEq, Clone, Copy)]
    pub enum notmuch_decryption_policy_t => DecryptionPolicy {
        NOTMUCH_DECRYPT_FALSE => False,
        NOTMUCH_DECRYPT_TRUE => True,
        NOTMUCH_DECRYPT_AUTO => Auto,
        NOTMUCH_DECRYPT_NOSTASH => NoStash
    }
}

#[repr(C)]
pub struct notmuch_database_t(c_void);
#[repr(C)]
pub struct notmuch_query_t(c_void);
#[repr(C)]
pub struct notmuch_threads_t(c_void);
#[repr(C)]
pub struct notmuch_thread_t(c_void);
#[repr(C)]
pub struct notmuch_messages_t(c_void);
#[repr(C)]
pub struct notmuch_message_t(c_void);
#[repr(C)]
pub struct notmuch_tags_t(c_void);
#[repr(C)]
pub struct notmuch_directory_t(c_void);
#[repr(C)]
pub struct notmuch_filenames_t(c_void);
#[repr(C)]
pub struct notmuch_message_properties_t(c_void);
#[repr(C)]
pub struct notmuch_config_list_t(c_void);
#[repr(C)]
pub struct notmuch_indexopts_t(c_void);

pub type notmuch_compact_status_cb_t = extern "C" fn(message: *const c_char, closure: *mut c_void);
pub type notmuch_database_upgrade_cb_t = extern "C" fn(closure: *mut c_void, progress: c_double);

pub type notmuch_bool_t = c_int;
pub const TRUE: notmuch_bool_t = 1;
pub const FALSE: notmuch_bool_t = 0;

#[link(name = "notmuch")]
extern "C" {

    /// Get a string representation of a `notmuch_status_t` value.
    ///
    /// The result is read-only.
    pub fn notmuch_status_to_string(status: notmuch_status_t) -> *const c_char;

    /// Create a new, empty notmuch database located at 'path'.
    ///
    /// The path should be a top-level directory to a collection of
    /// plain-text email messages (one message per file). This call will
    /// create a new ".notmuch" directory within 'path' where notmuch will
    /// store its data.
    ///
    /// After a successful call to `notmuch_database_create`, the returned
    /// database will be open so the caller should call
    /// `notmuch_database_destroy` when finished with it.
    ///
    /// The database will not yet have any data in it
    /// (`notmuch_database_create` itself is a very cheap function). Messages
    /// contained within 'path' can be added to the database by calling
    /// `notmuch_database_add_message`.
    ///
    /// In case of any failure, this function returns an error status and
    /// sets *database to NULL (after printing an error message on stderr).
    ///
    /// Return value:
    ///
    /// * `notmuch_status_t::SUCCESS`: Successfully created the database.
    ///
    /// * `notmuch_status_t::NULL_POINTER`: The given 'path' argument is NULL.
    ///
    /// * `notmuch_status_t::OUT_OF_MEMORY`: Out of memory.
    ///
    /// * `notmuch_status_t::FILE_ERROR`: An error occurred trying to create the
    /// 	  database file (such as permission denied, or file not found,
    /// 	  etc.), or the database already exists.
    ///
    /// * `notmuch_status_t::XAPIAN_EXCEPTION`: A Xapian exception occurred.
    pub fn notmuch_database_create(
        path: *const c_char,
        database: *mut *mut notmuch_database_t,
    ) -> notmuch_status_t;

    /// Like `notmuch_database_create`, except optionally return an error
    /// message. This message is allocated by malloc and should be freed by
    /// the caller.
    pub fn notmuch_database_create_verbose(
        path: *const c_char,
        database: *mut *mut notmuch_database_t,
        error_message: *mut *const c_char,
    ) -> notmuch_status_t;

    /// Open an existing notmuch database located at 'path'.
    ///
    /// The database should have been created at some time in the past,
    /// (not necessarily by this process), by calling
    /// notmuch_database_create with 'path'. By default the database should be
    /// opened for reading only. In order to write to the database you need to
    /// pass the `notmuch_database_mode_t::READ_WRITE` mode.
    ///
    /// An existing notmuch database can be identified by the presence of a
    /// directory named ".notmuch" below 'path'.
    ///
    /// The caller should call notmuch_database_destroy when finished with
    /// this database.
    ///
    /// In case of any failure, this function returns an error status and
    /// sets *database to NULL (after printing an error message on stderr).
    ///
    /// Return value:
    ///
    /// * `notmuch_status_t::SUCCESS`: Successfully opened the database.
    ///
    /// * `notmuch_status_t::NULL_POINTER`: The given 'path' argument is NULL.
    ///
    /// * `notmuch_status_t::OUT_OF_MEMORY`: Out of memory.
    ///
    /// * `notmuch_status_t::FILE_ERROR`: An error occurred trying to open the
    /// 	  database file (such as permission denied, or file not found,
    /// 	  etc.), or the database version is unknown.
    ///
    /// * `notmuch_status_t::XAPIAN_EXCEPTION`: A Xapian exception occurred.
    pub fn notmuch_database_open(
        path: *const c_char,
        mode: notmuch_database_mode_t,
        database: *mut *mut notmuch_database_t,
    ) -> notmuch_status_t;

    /// Like notmuch_database_open, except optionally return an error
    /// message. This message is allocated by malloc and should be freed by
    /// the caller.
    pub fn notmuch_database_open_verbose(
        path: *const c_char,
        mode: notmuch_database_mode_t,
        database: *mut *mut notmuch_database_t,
        error_message: *mut *mut c_char,
    ) -> notmuch_status_t;

    /// Retrieve last status string for given database.
    pub fn notmuch_database_status_string(notmuch: *mut notmuch_database_t) -> *const c_char;

    /// Commit changes and close the given notmuch database.
    ///
    /// After `notmuch_database_close` has been called, calls to other
    /// functions on objects derived from this database may either behave
    /// as if the database had not been closed (e.g., if the required data
    /// has been cached) or may fail with a
    /// `notmuch_status_t::XAPIAN_EXCEPTION`. The only further operation
    /// permitted on the database itself is to call `notmuch_database_destroy`.
    ///
    /// `notmuch_database_close` can be called multiple times. Later calls have
    /// no effect.
    ///
    /// For writable databases, `notmuch_database_close` commits all changes
    /// to disk before closing the database.  If the caller is currently in
    /// an atomic section (there was a `notmuch_database_begin_atomic`
    /// without a matching `notmuch_database_end_atomic`), this will discard
    /// changes made in that atomic section (but still commit changes made
    /// prior to entering the atomic section).
    ///
    /// Return value:
    ///
    /// * `notmuch_status_t::SUCCESS`: Successfully closed the database.
    ///
    /// * `notmuch_status_t::XAPIAN_EXCEPTION`: A Xapian exception occurred; the
    /// 	  database has been closed but there are no guarantees the
    /// 	  changes to the database, if any, have been flushed to disk.
    pub fn notmuch_database_close(database: *mut notmuch_database_t) -> notmuch_status_t;

    /// Compact a notmuch database, backing up the original database to the
    /// given path.
    ///
    /// The database will be opened with notmuch_database_mode_t::READ_WRITE
    /// during the compaction process to ensure no writes are made.
    ///
    /// If the optional callback function 'status_cb' is non-NULL, it will
    /// be called with diagnostic and informational messages. The argument
    /// 'closure' is passed verbatim to any callback invoked.
    pub fn notmuch_database_compact(
        path: *const c_char,
        backup_path: *const c_char,
        status_cb: Option<notmuch_compact_status_cb_t>,
        closure: *mut c_void,
    ) -> notmuch_status_t;

    /// Destroy the notmuch database, closing it if necessary and freeing
    /// all associated resources.
    ///
    /// Return value as in `notmuch_database_close` if the database was open;
    /// `notmuch_database_destroy` itself has no failure modes.
    pub fn notmuch_database_destroy(database: *mut notmuch_database_t) -> notmuch_status_t;

    /// Return the database path of the given database.
    ///
    /// The return value is a string owned by notmuch so should not be
    /// modified nor freed by the caller.
    pub fn notmuch_database_get_path(database: *mut notmuch_database_t) -> *const c_char;

    /// Return the database format version of the given database.
    pub fn notmuch_database_get_version(database: *mut notmuch_database_t) -> c_uint;

    /// Can the database be upgraded to a newer database version?
    ///
    /// If this function returns TRUE, then the caller may call
    /// `notmuch_database_upgrade` to upgrade the database.  If the caller
    /// does not upgrade an out-of-date database, then some functions may
    /// fail with `notmuch_status_t::UPGRADE_REQUIRED`.  This always returns
    /// FALSE for a read-only database because there's no way to upgrade a
    /// read-only database.
    pub fn notmuch_database_needs_upgrade(database: *mut notmuch_database_t) -> notmuch_bool_t;

    /// Upgrade the current database to the latest supported version.
    ///
    /// This ensures that all current notmuch functionality will be
    /// available on the database.  After opening a database in read-write
    /// mode, it is recommended that clients check if an upgrade is needed
    /// (`notmuch_database_needs_upgrade`) and if so, upgrade with this
    /// function before making any modifications.  If
    /// `notmuch_database_needs_upgrade` returns FALSE, this will be a no-op.
    ///
    /// The optional progress_notify callback can be used by the caller to
    /// provide progress indication to the user. If non-NULL it will be
    /// called periodically with 'progress' as a floating-point value in
    /// the range of [0.0 .. 1.0] indicating the progress made so far in
    /// the upgrade process.  The argument 'closure' is passed verbatim to
    /// any callback invoked.
    pub fn notmuch_database_upgrade(
        database: *mut notmuch_database_t,
        progress_notify: Option<extern "C" fn(closure: *mut c_void, progress: c_double)>,
        closure: *mut c_void,
    ) -> notmuch_status_t;

    /// Begin an atomic database operation.
    ///
    /// Any modifications performed between a successful begin and a
    /// `notmuch_database_end_atomic` will be applied to the database
    /// atomically.  Note that, unlike a typical database transaction, this
    /// only ensures atomicity, not durability; neither begin nor end
    /// necessarily flush modifications to disk.
    ///
    /// Atomic sections may be nested.  begin_atomic and end_atomic must
    /// always be called in pairs.
    ///
    /// Return value:
    ///
    /// * `notmuch_status_t::SUCCESS`: Successfully entered atomic section.
    ///
    /// * `notmuch_status_t::XAPIAN_EXCEPTION`: A Xapian exception occurred;
    /// 	  atomic section not entered.
    pub fn notmuch_database_begin_atomic(notmuch: *mut notmuch_database_t) -> notmuch_status_t;

    /// Indicate the end of an atomic database operation.
    ///
    /// Return value:
    ///
    /// * `notmuch_status_t::SUCCESS`: Successfully completed atomic section.
    ///
    /// * `notmuch_status_t::XAPIAN_EXCEPTION`: A Xapian exception occurred;
    /// 	  atomic section not ended.
    ///
    /// * `notmuch_status_t::UNBALANCED_ATOMIC`: The database is not currently in
    /// 	  an atomic section.
    pub fn notmuch_database_end_atomic(notmuch: *mut notmuch_database_t) -> notmuch_status_t;

    /// Return the committed database revision and UUID.
    ///
    /// The database revision number increases monotonically with each
    /// commit to the database.  Hence, all messages and message changes
    /// committed to the database (that is, visible to readers) have a last
    /// modification revision <= the committed database revision.  Any
    /// messages committed in the future will be assigned a modification
    /// revision > the committed database revision.
    ///
    /// The UUID is a NUL-terminated opaque string that uniquely identifies
    /// this database.  Two revision numbers are only comparable if they
    /// have the same database UUID.
    pub fn notmuch_database_get_revision(
        notmuch: *mut notmuch_database_t,
        uuid: *mut *const c_char,
    ) -> c_ulong;

    /// Retrieve a directory object from the database for 'path'.
    ///
    /// Here, 'path' should be a path relative to the path of 'database'
    /// (see `notmuch_database_get_path`), or else should be an absolute path
    /// with initial components that match the path of 'database'.
    ///
    /// If this directory object does not exist in the database, this
    /// returns `notmuch_status_t::SUCCESS` and sets *directory to NULL.
    ///
    /// Otherwise the returned directory object is owned by the database
    /// and as such, will only be valid until `notmuch_database_destroy` is
    /// called.
    ///
    /// Return value:
    ///
    /// * `notmuch_status_t::SUCCESS`: Successfully retrieved directory.
    ///
    /// * `notmuch_status_t::NULL_POINTER`: The given 'directory' argument is NULL.
    ///
    /// * `notmuch_status_t::XAPIAN_EXCEPTION`: A Xapian exception occurred;
    /// 	  directory not retrieved.
    ///
    /// * `notmuch_status_t::UPGRADE_REQUIRED`: The caller must upgrade the
    ///   database to use this function.
    pub fn notmuch_database_get_directory(
        database: *mut notmuch_database_t,
        path: *const c_char,
        directory: *mut *mut notmuch_directory_t,
    ) -> notmuch_status_t;

    /// Add a message file to a database, indexing it for retrieval by
    /// future searches.  If a message already exists with the same message
    /// ID as the specified file, their indexes will be merged, and this
    /// new filename will also be associated with the existing message.
    ///
    /// Here, 'filename' should be a path relative to the path of
    /// 'database' (see notmuch_database_get_path), or else should be an
    /// absolute filename with initial components that match the path of
    /// 'database'.
    ///
    /// The file should be a single mail message (not a multi-message mbox)
    /// that is expected to remain at its current location, (since the
    /// notmuch database will reference the filename, and will not copy the
    /// entire contents of the file.
    ///
    /// If another message with the same message ID already exists in the
    /// database, rather than creating a new message, this adds the search
    /// terms from the identified file to the existing message's index, and
    /// adds 'filename' to the list of filenames known for the message.
    ///
    /// The 'indexopts' parameter can be NULL (meaning, use the indexing
    /// defaults from the database), or can be an explicit choice of
    /// indexing options that should govern the indexing of this specific
    /// 'filename'.
    ///
    /// If 'message' is not NULL, then, on successful return
    /// (NOTMUCH_STATUS_SUCCESS or NOTMUCH_STATUS_DUPLICATE_MESSAGE_ID) '*message'
    /// will be initialized to a message object that can be used for things
    /// such as adding tags to the just-added message. The user should call
    /// notmuch_message_destroy when done with the message. On any failure
    /// '*message' will be set to NULL.
    ///
    /// Return value:
    ///
    /// NOTMUCH_STATUS_SUCCESS: Message successfully added to database.
    ///
    /// NOTMUCH_STATUS_XAPIAN_EXCEPTION: A Xapian exception occurred,
    ///	message not added.
    ///
    /// NOTMUCH_STATUS_DUPLICATE_MESSAGE_ID: Message has the same message
    ///	ID as another message already in the database. The new
    ///	filename was successfully added to the message in the database
    ///	(if not already present) and the existing message is returned.
    ///
    /// NOTMUCH_STATUS_FILE_ERROR: an error occurred trying to open the
    ///	file, (such as permission denied, or file not found,
    ///	etc.). Nothing added to the database.
    ///
    /// NOTMUCH_STATUS_FILE_NOT_EMAIL: the contents of filename don't look
    ///	like an email message. Nothing added to the database.
    ///
    /// NOTMUCH_STATUS_READ_ONLY_DATABASE: Database was opened in read-only
    ///	mode so no message can be added.
    ///
    /// NOTMUCH_STATUS_UPGRADE_REQUIRED: The caller must upgrade the
    /// 	database to use this function.
    ///
    /// @since libnotmuch 5.1 (notmuch 0.26)
    pub fn notmuch_database_index_file(
        database: *mut notmuch_database_t,
        filename: *const c_char,
        indexopts: *mut notmuch_indexopts_t,
        message: *mut *mut notmuch_message_t,
    ) -> notmuch_status_t;

    /// Deprecated alias for notmuch_database_index_file called with
    /// NULL indexopts.
    ///
    /// @deprecated Deprecated as of libnotmuch 5.1 (notmuch 0.26). Please
    /// use notmuch_database_index_file instead.
    pub fn notmuch_database_add_message(
        database: *mut notmuch_database_t,
        filename: *const c_char,
        message: *mut *mut notmuch_message_t,
    ) -> notmuch_status_t;

    /// Remove a message filename from the given notmuch database. If the
    /// message has no more filenames, remove the message.
    ///
    /// If the same message (as determined by the message ID) is still
    /// available via other filenames, then the message will persist in the
    /// database for those filenames. When the last filename is removed for
    /// a particular message, the database content for that message will be
    /// entirely removed.
    ///
    /// Return value:
    ///
    /// * `notmuch_status_t::SUCCESS`: The last filename was removed and the
    /// 	  message was removed from the database.
    ///
    /// * `notmuch_status_t::XAPIAN_EXCEPTION`: A Xapian exception occurred,
    /// 	  message not removed.
    ///
    /// * `notmuch_status_t::DUPLICATE_MESSAGE_ID`: This filename was removed but
    /// 	  the message persists in the database with at least one other
    /// 	  filename.
    ///
    /// * `notmuch_status_t::READ_ONLY_DATABASE`: Database was opened in read-only
    /// 	  mode so no message can be removed.
    ///
    /// * `notmuch_status_t::UPGRADE_REQUIRED`: The caller must upgrade the
    ///   database to use this function.
    pub fn notmuch_database_remove_message(
        database: *mut notmuch_database_t,
        filename: *const c_char,
    ) -> notmuch_status_t;

    /// Find a message with the given message_id.
    ///
    /// If a message with the given message_id is found then, on successful return
    /// (`notmuch_status_t::SUCCESS`) '*message' will be initialized to a message
    /// object.  The caller should call `notmuch_message_destroy` when done with the
    /// message.
    ///
    /// On any failure or when the message is not found, this function initializes
    /// '*message' to NULL. This means, when `notmuch_status_t::SUCCESS` is returned, the
    /// caller is supposed to check '*message' for NULL to find out whether the
    /// message with the given message_id was found.
    ///
    /// Return value:
    ///
    /// * `notmuch_status_t::SUCCESS`: Successful return, check '*message'.
    ///
    /// * `notmuch_status_t::NULL_POINTER`: The given 'message' argument is NULL
    ///
    /// * `notmuch_status_t::OUT_OF_MEMORY`: Out of memory, creating message object
    ///
    /// * `notmuch_status_t::XAPIAN_EXCEPTION`: A Xapian exception occurred
    pub fn notmuch_database_find_message(
        database: *mut notmuch_database_t,
        message_id: *const c_char,
        message: *mut *mut notmuch_message_t,
    ) -> notmuch_status_t;

    /// Find a message with the given filename.
    ///
    /// If the database contains a message with the given filename then, on
    /// successful return (`notmuch_status_t::SUCCESS`) '*message' will be initialized to
    /// a message object. The caller should call `notmuch_message_destroy` when done
    /// with the message.
    ///
    /// On any failure or when the message is not found, this function initializes
    /// '*message' to NULL. This means, when `notmuch_status_t::SUCCESS` is returned, the
    /// caller is supposed to check '*message' for NULL to find out whether the
    /// message with the given filename is found.
    ///
    /// Return value:
    ///
    /// * `notmuch_status_t::SUCCESS`: Successful return, check '*message'
    ///
    /// * `notmuch_status_t::NULL_POINTER`: The given 'message' argument is NULL
    ///
    /// * `notmuch_status_t::OUT_OF_MEMORY`: Out of memory, creating the message object
    ///
    /// * `notmuch_status_t::XAPIAN_EXCEPTION`: A Xapian exception occurred
    ///
    /// * `notmuch_status_t::UPGRADE_REQUIRED`: The caller must upgrade the
    ///   database to use this function.
    pub fn notmuch_database_find_message_by_filename(
        database: *mut notmuch_database_t,
        filename: *const c_char,
        message: *mut *mut notmuch_message_t,
    ) -> notmuch_status_t;

    /// Return a list of all tags found in the database.
    ///
    /// This function creates a list of all tags found in the database. The
    /// resulting list contains all tags from all messages found in the database.
    ///
    /// On error this function returns NULL.
    pub fn notmuch_database_get_all_tags(db: *mut notmuch_database_t) -> *mut notmuch_tags_t;

    /// Create a new query for 'database'.
    ///
    /// Here, 'database' should be an open database, (see
    /// notmuch_database_open and `notmuch_database_create`).
    ///
    /// For the query string, we'll document the syntax here more
    /// completely in the future, but it's likely to be a specialized
    /// version of the general Xapian query syntax:
    ///
    /// https://xapian.org/docs/queryparser.html
    ///
    /// As a special case, passing either a length-zero string, (that is ""),
    /// or a string consisting of a single asterisk (that is "*"), will
    /// result in a query that returns all messages in the database.
    ///
    /// See `notmuch_query_set_sort` for controlling the order of results.
    /// See notmuch_query_search_messages and `notmuch_query_search_threads`
    /// to actually execute the query.
    ///
    /// User should call `notmuch_query_destroy` when finished with this
    /// query.
    ///
    /// Will return NULL if insufficient memory is available.
    pub fn notmuch_query_create(
        database: *mut notmuch_database_t,
        query_string: *const c_char,
    ) -> *mut notmuch_query_t;

    /// Return the query_string of this query. See `notmuch_query_create`.
    pub fn notmuch_query_get_query_string(query: *mut notmuch_query_t) -> *const c_char;

    /// Return the notmuch database of this query. See `notmuch_query_create`.
    pub fn notmuch_query_get_database(query: *const notmuch_query_t) -> *mut notmuch_database_t;

    /// Specify whether to omit excluded results or simply flag them.  By
    /// default, this is set to TRUE.
    ///
    /// If set to TRUE or ALL, `notmuch_query_search_messages` will omit excluded
    /// messages from the results, and `notmuch_query_search_threads` will omit
    /// threads that match only in excluded messages.  If set to TRUE,
    /// `notmuch_query_search_threads` will include all messages in threads that
    /// match in at least one non-excluded message.  Otherwise, if set to ALL,
    /// `notmuch_query_search_threads` will omit excluded messages from all threads.
    ///
    /// If set to FALSE or FLAG then both `notmuch_query_search_messages` and
    /// `notmuch_query_search_threads` will return all matching
    /// messages/threads regardless of exclude status. If set to FLAG then
    /// the exclude flag will be set for any excluded message that is
    /// returned by `notmuch_query_search_messages`, and the thread counts
    /// for threads returned by `notmuch_query_search_threads` will be the
    /// number of non-excluded messages/matches. Otherwise, if set to
    /// FALSE, then the exclude status is completely ignored.
    ///
    /// The performance difference when calling
    /// `notmuch_query_search_messages` should be relatively small (and both
    /// should be very fast).  However, in some cases,
    /// `notmuch_query_search_threads` is very much faster when omitting
    /// excluded messages as it does not need to construct the threads that
    /// only match in excluded messages.
    pub fn notmuch_query_set_omit_excluded(
        query: *mut notmuch_query_t,
        omit_excluded: notmuch_exclude_t,
    );

    /// Specify the sorting desired for this query.
    pub fn notmuch_query_set_sort(query: *mut notmuch_query_t, sort: notmuch_sort_t);

    /// Return the sort specified for this query. See
    /// `notmuch_query_set_sort`.
    pub fn notmuch_query_get_sort(query: *mut notmuch_query_t) -> notmuch_sort_t;

    /// Add a tag that will be excluded from the query results by default.
    /// This exclusion will be overridden if this tag appears explicitly in
    /// the query.
    pub fn notmuch_query_add_tag_exclude(query: *mut notmuch_query_t, tag: *const c_char) -> notmuch_status_t;

    /// Execute a query for threads, returning a `notmuch_threads_t` object
    /// which can be used to iterate over the results. The returned threads
    /// object is owned by the query and as such, will only be valid until
    /// `notmuch_query_destroy`.
    ///
    /// Typical usage might be:
    ///
    /// ```norun
    /// notmuch_query_t *query;
    /// notmuch_threads_t *threads;
    /// notmuch_thread_t *thread;
    ///
    /// query = notmuch_query_create (database, query_string);
    ///
    /// for (threads = notmuch_query_search_threads (query);
    ///      notmuch_threads_valid (threads);
    ///      notmuch_threads_move_to_next (threads))
    /// {
    ///     thread = notmuch_threads_get (threads);
    ///     ....
    ///     notmuch_thread_destroy (thread);
    /// }
    ///
    /// notmuch_query_destroy (query);
    /// ```
    ///
    /// Note: If you are finished with a thread before its containing
    /// query, you can call `notmuch_thread_destroy` to clean up some memory
    /// sooner (as in the above example). Otherwise, if your thread objects
    /// are long-lived, then you don't need to call `notmuch_thread_destroy`
    /// and all the memory will still be reclaimed when the query is
    /// destroyed.
    ///
    /// Note that there's no explicit destructor needed for the
    /// `notmuch_threads_t` object. (For consistency, we do provide a
    /// `notmuch_threads_destroy` function, but there's no good reason
    /// to call it if the query is about to be destroyed).
    ///
    /// @since libnotmuch 4.2 (notmuch 0.20)
    pub fn notmuch_query_search_threads(
        query: *mut notmuch_query_t,
        out: *mut *mut notmuch_threads_t,
    ) -> notmuch_status_t;

    /// Execute a query for messages, returning a `notmuch_messages_t` object
    /// which can be used to iterate over the results. The returned
    /// messages object is owned by the query and as such, will only be
    /// valid until `notmuch_query_destroy`.
    ///
    /// Typical usage might be:
    ///
    /// ```norun
    /// notmuch_query_t *query;
    /// notmuch_messages_t *messages;
    /// notmuch_message_t *message;
    ///
    /// query = notmuch_query_create (database, query_string);
    ///
    /// for (messages = notmuch_query_search_messages (query);
    ///      notmuch_messages_valid (messages);
    ///      notmuch_messages_move_to_next (messages))
    /// {
    ///     message = notmuch_messages_get (messages);
    ///     ....
    ///     notmuch_message_destroy (message);
    /// }
    ///
    /// notmuch_query_destroy (query);
    /// ```
    ///
    /// Note: If you are finished with a message before its containing
    /// query, you can call `notmuch_message_destroy` to clean up some memory
    /// sooner (as in the above example). Otherwise, if your message
    /// objects are long-lived, then you don't need to call
    /// `notmuch_message_destroy` and all the memory will still be reclaimed
    /// when the query is destroyed.
    ///
    /// Note that there's no explicit destructor needed for the
    /// `notmuch_messages_t` object. (For consistency, we do provide a
    /// `notmuch_messages_destroy` function, but there's no good
    /// reason to call it if the query is about to be destroyed).
    ///
    /// If a Xapian exception occurs this function will return NULL.
    ///
    /// @since libnotmuch 5 (notmuch 0.25)
    pub fn notmuch_query_search_messages(
        query: *mut notmuch_query_t,
        out: *mut *mut notmuch_messages_t,
    ) -> notmuch_status_t;

    /// Destroy a `notmuch_query_t` along with any associated resources.
    ///
    /// This will in turn destroy any `notmuch_threads_t` and
    /// `notmuch_messages_t` objects generated by this query, (and in
    /// turn any notmuch_thread_t and `notmuch_message_t` objects generated
    /// from those results, etc.), if such objects haven't already been
    /// destroyed.
    pub fn notmuch_query_destroy(query: *mut notmuch_query_t);

    /// Is the given 'threads' iterator pointing at a valid thread.
    ///
    /// When this function returns TRUE, `notmuch_threads_get` will return a
    /// valid object. Whereas when this function returns FALSE,
    /// `notmuch_threads_get` will return NULL.
    ///
    /// If passed a NULL pointer, this function returns FALSE
    ///
    /// See the documentation of `notmuch_query_search_threads` for example
    /// code showing how to iterate over a `notmuch_threads_t` object.
    pub fn notmuch_threads_valid(threads: *mut notmuch_threads_t) -> notmuch_bool_t;

    /// Get the current thread from 'threads' as a `notmuch_thread_t`.
    ///
    /// Note: The returned thread belongs to 'threads' and has a lifetime
    /// identical to it (and the query to which it belongs).
    ///
    /// See the documentation of `notmuch_query_search_threads` for example
    /// code showing how to iterate over a `notmuch_threads_t` object.
    ///
    /// If an out-of-memory situation occurs, this function will return
    /// NULL.
    pub fn notmuch_threads_get(threads: *mut notmuch_threads_t) -> *mut notmuch_thread_t;

    /// Move the 'threads' iterator to the next thread.
    ///
    /// If 'threads' is already pointing at the last thread then the
    /// iterator will be moved to a point just beyond that last thread,
    /// (where `notmuch_threads_valid` will return FALSE and
    /// `notmuch_threads_get` will return NULL).
    ///
    /// See the documentation of `notmuch_query_search_threads` for example
    /// code showing how to iterate over a `notmuch_threads_t` object.
    pub fn notmuch_threads_move_to_next(threads: *mut notmuch_threads_t);

    /// Destroy a `notmuch_threads_t` object.
    ///
    /// It's not strictly necessary to call this function. All memory from
    /// the `notmuch_threads_t` object will be reclaimed when the
    /// containing query object is destroyed.
    pub fn notmuch_threads_destroy(threads: *mut notmuch_threads_t);

    /// Return the number of messages matching a search.
    ///
    /// This function performs a search and returns the number of matching
    /// messages.
    ///
    /// @returns
    ///
    /// `notmuch_status_t::SUCCESS`: query completed successfully.
    ///
    /// `notmuch_status_t::XAPIAN_EXCEPTION`: a Xapian exception occured. The
    ///      value of *count is not defined.
    ///
    /// @since libnotmuch 4.3 (notmuch 0.21)
    pub fn notmuch_query_count_messages(
        query: *mut notmuch_query_t,
        count: *mut c_uint,
    ) -> notmuch_status_t;

    /// Return the number of threads matching a search.
    ///
    /// This function performs a search and returns the number of unique thread IDs
    /// in the matching messages. This is the same as number of threads matching a
    /// search.
    ///
    /// Note that this is a significantly heavier operation than
    /// `notmuch_query_count_messages`{_st}().
    ///
    /// @returns
    ///
    /// * `notmuch_status_t::OUT_OF_MEMORY`: Memory allocation failed. The value
    ///   of *count is not defined

    /// * `notmuch_status_t::SUCCESS`: query completed successfully.
    ///
    /// * `notmuch_status_t::XAPIAN_EXCEPTION`: a Xapian exception occured. The
    ///      value of *count is not defined.
    ///
    /// @since libnotmuch 4.3 (notmuch 0.21)
    pub fn notmuch_query_count_threads(
        query: *mut notmuch_query_t,
        count: *mut c_uint,
    ) -> notmuch_status_t;

    /// Get the thread ID of 'thread'.
    ///
    /// The returned string belongs to 'thread' and as such, should not be
    /// modified by the caller and will only be valid for as long as the
    /// thread is valid, (which is until `notmuch_thread_destroy` or until
    /// the query from which it derived is destroyed).
    pub fn notmuch_thread_get_thread_id(thread: *mut notmuch_thread_t) -> *const c_char;

    /// Get the total number of messages in 'thread'.
    ///
    /// This count consists of all messages in the database belonging to
    /// this thread. Contrast with `notmuch_thread_get_matched_messages`().
    pub fn notmuch_thread_get_total_messages(thread: *mut notmuch_thread_t) -> c_int;

    /// Get the total number of files in 'thread'.
    ///
    /// This sums notmuch_message_count_files over all messages in the
    /// thread
    /// @returns Non-negative integer
    /// @since libnotmuch 5.0 (notmuch 0.25)
    ///
    pub fn notmuch_thread_get_total_files(thread: *mut notmuch_thread_t) -> c_int;

    /// Get a `notmuch_messages_t` iterator for the top-level messages in
    /// 'thread' in oldest-first order.
    ///
    /// This iterator will not necessarily iterate over all of the messages
    /// in the thread. It will only iterate over the messages in the thread
    /// which are not replies to other messages in the thread.
    ///
    /// The returned list will be destroyed when the thread is destroyed.
    pub fn notmuch_thread_get_toplevel_messages(
        thread: *mut notmuch_thread_t,
    ) -> *mut notmuch_messages_t;

    /// Get a `notmuch_thread_t` iterator for all messages in 'thread' in
    /// oldest-first order.
    ///
    /// The returned list will be destroyed when the thread is destroyed.
    pub fn notmuch_thread_get_messages(thread: *mut notmuch_thread_t) -> *mut notmuch_messages_t;

    /// Get the number of messages in 'thread' that matched the search.
    ///
    /// This count includes only the messages in this thread that were
    /// matched by the search from which the thread was created and were
    /// not excluded by any exclude tags passed in with the query (see
    /// `notmuch_query_add_tag_exclude`). Contrast with
    /// `notmuch_thread_get_total_messages`() .
    pub fn notmuch_thread_get_matched_messages(thread: *mut notmuch_thread_t) -> c_int;

    /// Get the authors of 'thread' as a UTF-8 string.
    ///
    /// The returned string is a comma-separated list of the names of the
    /// authors of mail messages in the query results that belong to this
    /// thread.
    ///
    /// The string contains authors of messages matching the query first, then
    /// non-matched authors (with the two groups separated by '|'). Within
    /// each group, authors are ordered by date.
    ///
    /// The returned string belongs to 'thread' and as such, should not be
    /// modified by the caller and will only be valid for as long as the
    /// thread is valid, (which is until `notmuch_thread_destroy` or until
    /// the query from which it derived is destroyed).
    pub fn notmuch_thread_get_authors(thread: *mut notmuch_thread_t) -> *const c_char;

    /// Get the subject of 'thread' as a UTF-8 string.
    ///
    /// The subject is taken from the first message (according to the query
    /// order---see `notmuch_query_set_sort`) in the query results that
    /// belongs to this thread.
    ///
    /// The returned string belongs to 'thread' and as such, should not be
    /// modified by the caller and will only be valid for as long as the
    /// thread is valid, (which is until `notmuch_thread_destroy` or until
    /// the query from which it derived is destroyed).
    pub fn notmuch_thread_get_subject(thread: *mut notmuch_thread_t) -> *const c_char;

    /// Get the date of the oldest message in 'thread' as a time_t value.
    pub fn notmuch_thread_get_oldest_date(thread: *mut notmuch_thread_t) -> time_t;

    /// Get the date of the newest message in 'thread' as a time_t value.
    pub fn notmuch_thread_get_newest_date(thread: *mut notmuch_thread_t) -> time_t;

    /// Get the tags for 'thread', returning a `notmuch_tags_t` object which
    /// can be used to iterate over all tags.
    ///
    /// Note: In the Notmuch database, tags are stored on individual
    /// messages, not on threads. So the tags returned here will be all
    /// tags of the messages which matched the search and which belong to
    /// this thread.
    ///
    /// The tags object is owned by the thread and as such, will only be
    /// valid for as long as the thread is valid, (for example, until
    /// `notmuch_thread_destroy` or until the query from which it derived is
    /// destroyed).
    ///
    /// Typical usage might be:
    ///
    /// ```norun
    /// notmuch_thread_t *thread;
    /// notmuch_tags_t *tags;
    /// const char *tag;
    ///
    /// thread = notmuch_threads_get (threads);
    ///
    /// for (tags = notmuch_thread_get_tags (thread);
    ///      notmuch_tags_valid (tags);
    ///      notmuch_tags_move_to_next (tags))
    /// {
    ///     tag = notmuch_tags_get (tags);
    ///     ....
    /// }
    ///
    /// notmuch_thread_destroy (thread);
    /// ```
    ///
    /// Note that there's no explicit destructor needed for the
    /// `notmuch_tags_t` object. (For consistency, we do provide a
    /// `notmuch_tags_destroy` function, but there's no good reason to call
    /// it if the message is about to be destroyed).
    pub fn notmuch_thread_get_tags(thread: *mut notmuch_thread_t) -> *mut notmuch_tags_t;

    /// Destroy a `notmuch_thread_t` object.
    pub fn notmuch_thread_destroy(thread: *mut notmuch_thread_t);

    /// Is the given 'messages' iterator pointing at a valid message.
    ///
    /// When this function returns TRUE, `notmuch_messages_get` will return a
    /// valid object. Whereas when this function returns FALSE,
    /// `notmuch_messages_get` will return NULL.
    ///
    /// See the documentation of `notmuch_query_search_messages` for example
    /// code showing how to iterate over a `notmuch_messages_t` object.
    pub fn notmuch_messages_valid(messages: *mut notmuch_messages_t) -> notmuch_bool_t;

    /// Get the current message from 'messages' as a `notmuch_message_t`.
    ///
    /// Note: The returned message belongs to 'messages' and has a lifetime
    /// identical to it (and the query to which it belongs).
    ///
    /// See the documentation of `notmuch_query_search_messages` for example
    /// code showing how to iterate over a `notmuch_messages_t` object.
    ///
    /// If an out-of-memory situation occurs, this function will return
    /// NULL.
    pub fn notmuch_messages_get(messages: *mut notmuch_messages_t) -> *mut notmuch_message_t;

    /// Move the 'messages' iterator to the next message.
    ///
    /// If 'messages' is already pointing at the last message then the
    /// iterator will be moved to a point just beyond that last message,
    /// (where `notmuch_messages_valid` will return FALSE and
    /// `notmuch_messages_get` will return NULL).
    ///
    /// See the documentation of `notmuch_query_search_messages` for example
    /// code showing how to iterate over a `notmuch_messages_t` object.
    pub fn notmuch_messages_move_to_next(messages: *mut notmuch_messages_t);

    /// Destroy a `notmuch_messages_t` object.
    ///
    /// It's not strictly necessary to call this function. All memory from
    /// the `notmuch_messages_t` object will be reclaimed when the containing
    /// query object is destroyed.
    pub fn notmuch_messages_destroy(messages: *mut notmuch_messages_t);

    /// Return a list of tags from all messages.
    ///
    /// The resulting list is guaranteed not to contain duplicated tags.
    ///
    /// WARNING: You can no longer iterate over messages after calling this
    /// function, because the iterator will point at the end of the list.
    /// We do not have a function to reset the iterator yet and the only
    /// way how you can iterate over the list again is to recreate the
    /// message list.
    ///
    /// The function returns NULL on error.
    pub fn notmuch_messages_collect_tags(messages: *mut notmuch_messages_t) -> *mut notmuch_tags_t;

    /// Get the message ID of 'message'.
    ///
    /// The returned string belongs to 'message' and as such, should not be
    /// modified by the caller and will only be valid for as long as the
    /// message is valid, (which is until the query from which it derived
    /// is destroyed).
    ///
    /// This function will not return NULL since Notmuch ensures that every
    /// message has a unique message ID, (Notmuch will generate an ID for a
    /// message if the original file does not contain one).
    pub fn notmuch_message_get_message_id(message: *mut notmuch_message_t) -> *const c_char;

    /// Get the thread ID of 'message'.
    ///
    /// The returned string belongs to 'message' and as such, should not be
    /// modified by the caller and will only be valid for as long as the
    /// message is valid, (for example, until the user calls
    /// `notmuch_message_destroy` on 'message' or until a query from which it
    /// derived is destroyed).
    ///
    /// This function will not return NULL since Notmuch ensures that every
    /// message belongs to a single thread.
    pub fn notmuch_message_get_thread_id(message: *mut notmuch_message_t) -> *const c_char;

    /// Get a `notmuch_messages_t` iterator for all of the replies to
    /// 'message'.
    ///
    /// Note: This call only makes sense if 'message' was ultimately
    /// obtained from a `notmuch_thread_t` object, (such as by coming
    /// directly from the result of calling
    /// `notmuch_thread_get_toplevel_messages` or by any number of subsequent
    /// calls to `notmuch_message_get_replies`).
    ///
    /// If 'message' was obtained through some non-thread means, (such as
    /// by a call to `notmuch_query_search_messages`), then this function
    /// will return NULL.
    ///
    /// If there are no replies to 'message', this function will return
    /// NULL. (Note that `notmuch_messages_valid` will accept that NULL
    /// value as legitimate, and simply return FALSE for it.)
    pub fn notmuch_message_get_replies(message: *mut notmuch_message_t) -> *mut notmuch_messages_t;

    /// Get the total number of files associated with a message.
    /// @returns Non-negative integer
    /// @since libnotmuch 5.0 (notmuch 0.25)
    pub fn notmuch_message_count_files(message: *mut notmuch_message_t) -> c_int;

    /// Get a filename for the email corresponding to 'message'.
    ///
    /// The returned filename is an absolute filename, (the initial
    /// component will match `notmuch_database_get_path`() ).
    ///
    /// The returned string belongs to the message so should not be
    /// modified or freed by the caller (nor should it be referenced after
    /// the message is destroyed).
    ///
    /// Note: If this message corresponds to multiple files in the mail
    /// store, (that is, multiple files contain identical message IDs),
    /// this function will arbitrarily return a single one of those
    /// filenames. See `notmuch_message_get_filenames` for returning the
    /// complete list of filenames.
    pub fn notmuch_message_get_filename(message: *mut notmuch_message_t) -> *const c_char;

    /// Get all filenames for the email corresponding to 'message'.
    ///
    /// Returns a `notmuch_filenames_t` iterator listing all the filenames
    /// associated with 'message'. These files may not have identical
    /// content, but each will have the identical Message-ID.
    ///
    /// Each filename in the iterator is an absolute filename, (the initial
    /// component will match `notmuch_database_get_path`() ).
    pub fn notmuch_message_get_filenames(
        message: *mut notmuch_message_t,
    ) -> *mut notmuch_filenames_t;

    /// Re-index the e-mail corresponding to 'message' using the supplied index options
    ///
    /// Returns the status of the re-index operation.  (see the return
    /// codes documented in notmuch_database_index_file)
    ///
    /// After reindexing, the user should discard the message object passed
    /// in here by calling notmuch_message_destroy, since it refers to the
    /// original message, not to the reindexed message.
    pub fn notmuch_message_reindex(
        message: *mut notmuch_message_t,
        indexopts: *mut notmuch_indexopts_t
    ) -> notmuch_status_t;
    
    /// Get a value of a flag for the email corresponding to 'message'.
    pub fn notmuch_message_get_flag(
        message: *mut notmuch_message_t,
        flag: notmuch_message_flag_t,
    ) -> notmuch_bool_t;

    /// Set a value of a flag for the email corresponding to 'message'.
    pub fn notmuch_message_set_flag(
        message: *mut notmuch_message_t,
        flag: notmuch_message_flag_t,
        value: notmuch_bool_t,
    );

    /// Get the date of 'message' as a time_t value.
    ///
    /// For the original textual representation of the Date header from the
    /// message call `notmuch_message_get_header`() with a header value of
    /// "date".
    pub fn notmuch_message_get_date(message: *mut notmuch_message_t) -> time_t;

    /// Get the value of the specified header from 'message' as a UTF-8 string.
    ///
    /// Common headers are stored in the database when the message is
    /// indexed and will be returned from the database.  Other headers will
    /// be read from the actual message file.
    ///
    /// The header name is case insensitive.
    ///
    /// The returned string belongs to the message so should not be
    /// modified or freed by the caller (nor should it be referenced after
    /// the message is destroyed).
    ///
    /// Returns an empty string ("") if the message does not contain a
    /// header line matching 'header'. Returns NULL if any error occurs.
    pub fn notmuch_message_get_header(
        message: *mut notmuch_message_t,
        header: *const c_char,
    ) -> *const c_char;

    /// Get the tags for 'message', returning a `notmuch_tags_t` object which
    /// can be used to iterate over all tags.
    ///
    /// The tags object is owned by the message and as such, will only be
    /// valid for as long as the message is valid, (which is until the
    /// query from which it derived is destroyed).
    ///
    /// Typical usage might be:
    ///
    /// ```norun
    /// notmuch_message_t *message;
    /// notmuch_tags_t *tags;
    /// const char *tag;
    ///
    /// message = notmuch_database_find_message (database, message_id);
    ///
    /// for (tags = `notmuch_message_get_tags` (message);
    ///      notmuch_tags_valid (tags);
    ///      notmuch_tags_move_to_next (tags))
    /// {
    ///     tag = notmuch_tags_get (tags);
    ///     ....
    /// }
    ///
    /// notmuch_message_destroy (message);
    /// ```
    ///
    /// Note that there's no explicit destructor needed for the
    /// `notmuch_tags_t` object. (For consistency, we do provide a
    /// `notmuch_tags_destroy` function, but there's no good reason to call
    /// it if the message is about to be destroyed).
    pub fn notmuch_message_get_tags(message: *mut notmuch_message_t) -> *mut notmuch_tags_t;

    /// Add a tag to the given message.
    ///
    /// Return value:
    ///
    /// * `notmuch_status_t::SUCCESS`: Tag successfully added to message
    ///
    /// * `notmuch_status_t::NULL_POINTER`: The 'tag' argument is NULL
    ///
    /// * `notmuch_status_t::TAG_TOO_LONG`: The length of 'tag' is too long
    /// 	  (exceeds TAG_MAX)
    ///
    /// * `notmuch_status_t::READ_ONLY_DATABASE`: Database was opened in read-only
    /// 	  mode so message cannot be modified.
    pub fn notmuch_message_add_tag(
        message: *mut notmuch_message_t,
        tag: *const c_char,
    ) -> notmuch_status_t;

    /// Remove a tag from the given message.
    ///
    /// Return value:
    ///
    /// * `notmuch_status_t::SUCCESS`: Tag successfully removed from message
    /// * `notmuch_status_t::NULL_POINTER`: The 'tag' argument is NULL
    /// * `notmuch_status_t::TAG_TOO_LONG`: The length of 'tag' is too long (exceeds `TAG_MAX`)
    /// * `notmuch_status_t::READ_ONLY_DATABASE`: Database was opened in read-only mode so message
    ///   cannot be modified.
    pub fn notmuch_message_remove_tag(
        message: *mut notmuch_message_t,
        tag: *const c_char,
    ) -> notmuch_status_t;

    /// Remove all tags from the given message.
    ///
    /// See `notmuch_message_freeze` for an example showing how to safely
    /// replace tag values.
    ///
    /// `notmuch_status_t::READ_ONLY_DATABASE`: Database was opened in read-only
    /// 	mode so message cannot be modified.
    pub fn notmuch_message_remove_all_tags(message: *mut notmuch_message_t) -> notmuch_status_t;

    /// Add/remove tags according to maildir flags in the message filename(s).
    ///
    /// This function examines the filenames of 'message' for maildir flags, and adds or removes
    /// tags on 'message' as follows when these flags are present:
    ///
    /// ```norun
    /// Flag	Action if present
    /// ----	-----------------
    /// 'D'	Adds the "draft" tag to the message
    /// 'F'	Adds the "flagged" tag to the message
    /// 'P'	Adds the "passed" tag to the message
    /// 'R'	Adds the "replied" tag to the message
    /// 'S'	Removes the "unread" tag from the message
    /// ```
    ///
    /// For each flag that is not present, the opposite action (add/remove)
    /// is performed for the corresponding tags.
    ///
    /// Flags are identified as trailing components of the filename after a
    /// sequence of ":2,".
    ///
    /// If there are multiple filenames associated with this message, the
    /// flag is considered present if it appears in one or more
    /// filenames. (That is, the flags from the multiple filenames are
    /// combined with the logical OR operator.)
    ///
    /// A client can ensure that notmuch database tags remain synchronized
    /// with maildir flags by calling this function after each call to
    /// `notmuch_database_add_message`. See also
    /// `notmuch_message_tags_to_maildir_flags` for synchronizing tag changes
    /// back to maildir flags.
    pub fn notmuch_message_maildir_flags_to_tags(
        message: *mut notmuch_message_t,
    ) -> notmuch_status_t;

    /// Rename message filename(s) to encode tags as maildir flags.
    ///
    /// Specifically, for each filename corresponding to this message:
    ///
    /// If the filename is not in a maildir directory, do nothing.  (A
    /// maildir directory is determined as a directory named "new" or
    /// "cur".) Similarly, if the filename has invalid maildir info,
    /// (repeated or outof-ASCII-order flag characters after ":2,"), then
    /// do nothing.
    ///
    /// If the filename is in a maildir directory, rename the file so that
    /// its filename ends with the sequence ":2," followed by zero or more
    /// of the following single-character flags (in ASCII order):
    ///
    ///   'D' iff the message has the "draft" tag
    ///   'F' iff the message has the "flagged" tag
    ///   'P' iff the message has the "passed" tag
    ///   'R' iff the message has the "replied" tag
    ///   'S' iff the message does not have the "unread" tag
    ///
    /// Any existing flags unmentioned in the list above will be preserved
    /// in the renaming.
    ///
    /// Also, if this filename is in a directory named "new", rename it to
    /// be within the neighboring directory named "cur".
    ///
    /// A client can ensure that maildir filename flags remain synchronized
    /// with notmuch database tags by calling this function after changing
    /// tags, (after calls to `notmuch_message_add_tag`,
    /// notmuch_message_remove_tag, or `notmuch_message_freeze`/
    /// notmuch_message_thaw). See also `notmuch_message_maildir_flags_to_tags`
    /// for synchronizing maildir flag changes back to tags.
    pub fn notmuch_message_tags_to_maildir_flags(
        message: *mut notmuch_message_t,
    ) -> notmuch_status_t;

    /// Freeze the current state of 'message' within the database.
    ///
    /// This means that changes to the message state, (via
    /// notmuch_message_add_tag, `notmuch_message_remove_tag`, and
    /// `notmuch_message_remove_all_tags`), will not be committed to the
    /// database until the message is thawed with `notmuch_message_thaw`.
    ///
    /// Multiple calls to freeze/thaw are valid and these calls will
    /// "stack". That is there must be as many calls to thaw as to freeze
    /// before a message is actually thawed.
    ///
    /// The ability to do freeze/thaw allows for safe transactions to
    /// change tag values. For example, explicitly setting a message to
    /// have a given set of tags might look like this:
    ///
    /// ```norun
    /// notmuch_message_freeze (message);
    ///
    /// notmuch_message_remove_all_tags (message);
    ///
    /// for (i = 0; i < NUM_TAGS; i++)
    ///     notmuch_message_add_tag (message, tags[i]);
    ///
    /// notmuch_message_thaw (message);
    /// ```
    ///
    /// With freeze/thaw used like this, the message in the database is
    /// guaranteed to have either the full set of original tag values, or
    /// the full set of new tag values, but nothing in between.
    ///
    /// Imagine the example above without freeze/thaw and the operation
    /// somehow getting interrupted. This could result in the message being
    /// left with no tags if the interruption happened after
    /// notmuch_message_remove_all_tags but before `notmuch_message_add_tag`.
    ///
    /// Return value:
    ///
    /// `notmuch_status_t::SUCCESS`: Message successfully frozen.
    ///
    /// `notmuch_status_t::READ_ONLY_DATABASE`: Database was opened in read-only
    /// 	mode so message cannot be modified.
    pub fn notmuch_message_freeze(message: *mut notmuch_message_t) -> notmuch_status_t;

    /// Thaw the current 'message', synchronizing any changes that may have
    /// occurred while 'message' was frozen into the notmuch database.
    ///
    /// See `notmuch_message_freeze` for an example of how to use this
    /// function to safely provide tag changes.
    ///
    /// Multiple calls to freeze/thaw are valid and these calls with
    /// "stack". That is there must be as many calls to thaw as to freeze
    /// before a message is actually thawed.
    ///
    /// Return value:
    ///
    /// `notmuch_status_t::SUCCESS`: Message successfully thawed, (or at least
    /// 	its frozen count has successfully been reduced by 1).
    ///
    /// `notmuch_status_t::UNBALANCED_FREEZE_THAW`: An attempt was made to thaw
    /// 	an unfrozen message. That is, there have been an unbalanced
    /// 	number of calls to `notmuch_message_freeze` and
    /// 	`notmuch_message_thaw`.
    pub fn notmuch_message_thaw(message: *mut notmuch_message_t) -> notmuch_status_t;

    /// Destroy a `notmuch_message_t` object.
    ///
    /// It can be useful to call this function in the case of a single
    /// query object with many messages in the result, (such as iterating
    /// over the entire database). Otherwise, it's fine to never call this
    /// function and there will still be no memory leaks. (The memory from
    /// the messages get reclaimed when the containing query is destroyed.)
    pub fn notmuch_message_destroy(message: *mut notmuch_message_t);

    /// Retrieve the value for a single property key
    ///
    /// *value* is set to a string owned by the message or NULL if there is
    /// no such key. In the case of multiple values for the given key, the
    /// first one is retrieved.
    ///
    /// @returns
    /// - `notmuch_status_t::NULL_POINTER`: *value* may not be NULL.
    /// - `notmuch_status_t::SUCCESS`: No error occured.
    /// @since libnotmuch 4.4 (notmuch 0.23)
    pub fn notmuch_message_get_property(
        message: *mut notmuch_message_t,
        key: *const c_char,
        value: *mut *const c_char,
    ) -> notmuch_status_t;

    /// Add a (key,value) pair to a message
    ///
    /// @returns
    /// - `notmuch_status_t::ILLEGAL_ARGUMENT`: *key* may not contain an '=' character.
    /// - `notmuch_status_t::NULL_POINTER`: Neither *key* nor *value* may be NULL.
    /// - `notmuch_status_t::SUCCESS`: No error occured.
    /// @since libnotmuch 4.4 (notmuch 0.23)
    pub fn notmuch_message_add_property(
        message: *mut notmuch_message_t,
        key: *const c_char,
        value: *const c_char,
    ) -> notmuch_status_t;

    ///
    /// Remove a `(key,value)` pair from a message.
    ///
    /// It is not an error to remove a non-existant `(key,value)` pair
    ///
    /// @returns
    /// - `notmuch_status_t::ILLEGAL_ARGUMENT`: `key` may not contain an '=' character.
    /// - `notmuch_status_t::NULL_POINTER`: Neither `key` nor *value* may be NULL.
    /// - `notmuch_status_t::SUCCESS`: No error occured.
    /// @since libnotmuch 4.4 (notmuch 0.23)
    pub fn notmuch_message_remove_property(
        message: *mut notmuch_message_t,
        key: *const c_char,
        value: *const c_char,
    ) -> notmuch_status_t;

    /// Remove all `(key,value)` pairs from the given message.
    ///
    /// @param[in,out] message  message to operate on.
    /// @param[in]     key      key to delete properties for. If NULL, delete
    ///			   properties for all keys
    /// @returns
    /// - `notmuch_status_::READ_ONLY_DATABASE`: Database was opened in
    ///   read-only mode so message cannot be modified.
    /// - `notmuch_status_t::SUCCESS`: No error occured.
    ///
    /// @since libnotmuch 4.4 (notmuch 0.23)
    pub fn notmuch_message_remove_all_properties(
        message: *mut notmuch_message_t,
        key: *const c_char,
    ) -> notmuch_status_t;

    /// Get the properties for *message*, returning a
    /// `notmuch_message_properties_t` object which can be used to iterate over
    /// all properties.
    ///
    /// The `notmuch_message_properties_t` object is owned by the message and as
    /// such, will only be valid for as long as the message is valid, (which is
    /// until the query from which it derived is destroyed).
    ///
    /// @param[in] message  The message to examine
    /// @param[in] key      key or key prefix
    /// @param[in] exact    if TRUE, require exact match with key. Otherwise
    ///		       treat as prefix.
    ///
    /// Typical usage might be:
    ///
    /// ```norun
    /// notmuch_message_properties_t *list;
    ///
    /// for (list = notmuch_message_get_properties (message, "testkey1", TRUE);
    ///      notmuch_message_properties_valid (list); notmuch_message_properties_move_to_next (list)) {
    ///    printf("%s\n", notmuch_message_properties_value(list));
    /// }
    ///
    /// notmuch_message_properties_destroy (list);
    /// ```
    ///
    /// Note that there's no explicit destructor needed for the
    /// `notmuch_message_properties_t` object. (For consistency, we do provide a
    /// `notmuch_message_properities_destroy` function, but there's no good
    /// reason to call it if the message is about to be destroyed).
    ///
    /// @since libnotmuch 4.4 (notmuch 0.23)
    ///
    pub fn notmuch_message_get_properties(
        message: *mut notmuch_message_t,
        key: *const c_char,
        exact: notmuch_bool_t,
    ) -> *mut notmuch_message_properties_t;


    /// Return the number of properties named "key" belonging to the specific message.
    ///
    /// @param[in] message  The message to examine
    /// @param[in] key      key to count
    /// @param[out] count   The number of matching properties associated with this message.
    ///
    /// @returns
    ///
    /// NOTMUCH_STATUS_SUCCESS: successful count, possibly some other error.
    ///
    /// @since libnotmuch 5.2 (notmuch 0.27)
    pub fn notmuch_message_count_properties(
        message: *mut notmuch_message_t,
        key: *const c_char,
        count: *mut c_uint,
    ) -> notmuch_status_t;

    ///  Is the given *properties* iterator pointing at a valid `(key,value)` pair.
    ///
    ///  When this function returns TRUE, `notmuch_message_properties_{key,value}`
    ///  will return a valid string, and `notmuch_message_properties_move_to_next`
    ///  will do what it says. Whereas when this function returns FALSE, calling any
    ///  of these functions results in undefined behaviour.
    ///
    ///  See the documentation of `notmuch_message_properties_get` for example code
    ///  showing how to iterate over a `notmuch_message_properties_t` object.
    ///
    ///  @since libnotmuch 4.4 (notmuch 0.23)
    pub fn notmuch_message_properties_valid(
        properties: *const notmuch_message_properties_t,
    ) -> notmuch_bool_t;

    /// Move the *properties* iterator to the next `(key,value)` pair
    ///
    /// If *properties* is already pointing at the last pair then the iterator
    /// will be moved to a point just beyond that last pair, (where
    /// `notmuch_message_properties_valid` will return FALSE).
    ///
    /// See the documentation of `notmuch_message_get_properties` for example
    /// code showing how to iterate over a `notmuch_message_properties_t` object.
    ///
    /// @since libnotmuch 4.4 (notmuch 0.23)
    pub fn notmuch_message_properties_move_to_next(properties: *mut notmuch_message_properties_t);

    /// Return the `key` from the current `(key,value)` pair.
    ///
    /// this could be useful if iterating for a prefix
    ///
    /// @since libnotmuch 4.4 (notmuch 0.23)
    ///
    pub fn notmuch_message_properties_key(
        properties: *mut notmuch_message_properties_t,
    ) -> *const c_char;

    /// Return the `value` from the current `(key,value)` pair.
    ///
    /// This could be useful if iterating for a prefix.
    ///
    /// @since libnotmuch 4.4 (notmuch 0.23)
    pub fn notmuch_message_properties_value(
        properties: *const notmuch_message_properties_t,
    ) -> *const c_char;

    /// Destroy a `notmuch_message_properties_t` object.
    ///
    /// It's not strictly necessary to call this function. All memory from
    /// the `notmuch_message_properties_t` object will be reclaimed when the
    /// containing message object is destroyed.
    ///
    /// @since libnotmuch 4.4 (notmuch 0.23)
    pub fn notmuch_message_properties_destroy(properties: *mut notmuch_message_properties_t);

    /// Is the given 'tags' iterator pointing at a valid tag.
    ///
    /// When this function returns TRUE, `notmuch_tags_get` will return a
    /// valid string. Whereas when this function returns FALSE,
    /// `notmuch_tags_get` will return NULL.
    ///
    /// See the documentation of `notmuch_message_get_tags` for example code
    /// showing how to iterate over a `notmuch_tags_t` object.
    pub fn notmuch_tags_valid(tags: *mut notmuch_tags_t) -> notmuch_bool_t;

    /// Get the current tag from 'tags' as a string.
    ///
    /// Note: The returned string belongs to 'tags' and has a lifetime
    /// identical to it (and the query to which it ultimately belongs).
    ///
    /// See the documentation of `notmuch_message_get_tags` for example code
    /// showing how to iterate over a `notmuch_tags_t` object.
    pub fn notmuch_tags_get(tags: *mut notmuch_tags_t) -> *const c_char;

    /// Move the 'tags' iterator to the next tag.
    ///
    /// If 'tags' is already pointing at the last tag then the iterator
    /// will be moved to a point just beyond that last tag, (where
    /// notmuch_tags_valid will return FALSE and `notmuch_tags_get` will
    /// return NULL).
    ///
    /// See the documentation of `notmuch_message_get_tags` for example code
    /// showing how to iterate over a `notmuch_tags_t` object.
    pub fn notmuch_tags_move_to_next(tags: *mut notmuch_tags_t);

    /// Destroy a `notmuch_tags_t` object.
    ///
    /// It's not strictly necessary to call this function. All memory from
    /// the `notmuch_tags_t` object will be reclaimed when the containing
    /// message or query objects are destroyed.
    pub fn notmuch_tags_destroy(tags: *mut notmuch_tags_t);

    /// Store an mtime within the database for 'directory'.
    ///
    /// The 'directory' should be an object retrieved from the database
    /// with `notmuch_database_get_directory` for a particular path.
    ///
    /// The intention is for the caller to use the mtime to allow efficient
    /// identification of new messages to be added to the database. The
    /// recommended usage is as follows:
    ///
    /// * Read the mtime of a directory from the filesystem
    ///
    /// * Call add_message for all mail files in the directory
    ///
    /// * Call `notmuch_directory_set_mtime` with the mtime read from the filesystem.
    ///
    /// Then, when wanting to check for updates to the directory in the
    /// future, the client can call `notmuch_directory_get_mtime` and know
    /// that it only needs to add files if the mtime of the directory and
    /// files are newer than the stored timestamp.
    ///
    /// Note: The `notmuch_directory_get_mtime` function does not allow the
    /// caller to distinguish a timestamp of 0 from a non-existent
    /// timestamp. So don't store a timestamp of 0 unless you are
    /// comfortable with that.
    ///
    /// Return value:
    ///
    /// * `notmuch_status_t::SUCCESS`: mtime successfully stored in database.
    ///
    /// * `notmuch_status_t::XAPIAN_EXCEPTION`: A Xapian exception occurred, mtime not stored.
    ///
    /// * `notmuch_status_t::READ_ONLY_DATABASE`: Database was opened in read-only mode so
    ///    directory mtime cannot be modified.
    pub fn notmuch_directory_set_mtime(
        directory: *mut notmuch_directory_t,
        mtime: time_t,
    ) -> notmuch_status_t;

    /// Get the mtime of a directory, (as previously stored with
    /// `notmuch_directory_set_mtime`).
    ///
    /// Returns 0 if no mtime has previously been stored for this
    /// directory.
    pub fn notmuch_directory_get_mtime(directory: *mut notmuch_directory_t) -> time_t;

    /// Get a `notmuch_filenames_t` iterator listing all the filenames of
    /// messages in the database within the given directory.
    ///
    /// The returned filenames will be the basename-entries only (not
    /// complete paths).
    pub fn notmuch_directory_get_child_files(
        directory: *mut notmuch_directory_t,
    ) -> *mut notmuch_filenames_t;

    /// Get a `notmuch_filenames_t` iterator listing all the filenames of
    /// sub-directories in the database within the given directory.
    ///
    /// The returned filenames will be the basename-entries only (not
    /// complete paths).
    pub fn notmuch_directory_get_child_directories(
        directory: *mut notmuch_directory_t,
    ) -> *mut notmuch_filenames_t;

    /// Delete directory document from the database, and destroy the
    /// `notmuch_directory_t` object. Assumes any child directories and files
    /// have been deleted by the caller.
    ///
    /// @since libnotmuch 4.3 (notmuch 0.21)
    pub fn notmuch_directory_delete(directory: *mut notmuch_directory_t) -> notmuch_status_t;

    /// Destroy a `notmuch_directory_t` object.
    pub fn notmuch_directory_destroy(directory: *mut notmuch_directory_t);

    /// Is the given 'filenames' iterator pointing at a valid filename.
    ///
    /// When this function returns TRUE, `notmuch_filenames_get` will return
    /// a valid string. Whereas when this function returns FALSE,
    /// `notmuch_filenames_get` will return NULL.
    ///
    /// It is acceptable to pass NULL for 'filenames', in which case this
    /// function will always return FALSE.
    pub fn notmuch_filenames_valid(filenames: *mut notmuch_filenames_t) -> notmuch_bool_t;

    /// Get the current filename from 'filenames' as a string.
    ///
    /// Note: The returned string belongs to 'filenames' and has a lifetime
    /// identical to it (and the directory to which it ultimately belongs).
    ///
    /// It is acceptable to pass NULL for 'filenames', in which case this
    /// function will always return NULL.
    pub fn notmuch_filenames_get(filenames: *mut notmuch_filenames_t) -> *const c_char;

    /// Move the 'filenames' iterator to the next filename.
    ///
    /// If 'filenames' is already pointing at the last filename then the
    /// iterator will be moved to a point just beyond that last filename,
    /// (where `notmuch_filenames_valid` will return FALSE and
    /// `notmuch_filenames_get` will return NULL).
    ///
    /// It is acceptable to pass NULL for 'filenames', in which case this
    /// function will do nothing.
    pub fn notmuch_filenames_move_to_next(filenames: *mut notmuch_filenames_t);

    /// Destroy a `notmuch_filenames_t` object.
    ///
    /// It's not strictly necessary to call this function. All memory from
    /// the `notmuch_filenames_t` object will be reclaimed when the
    /// containing directory object is destroyed.
    ///
    /// It is acceptable to pass NULL for 'filenames', in which case this
    /// function will do nothing.
    pub fn notmuch_filenames_destroy(filenames: *mut notmuch_filenames_t);

    /// set config 'key' to 'value'
    ///
    /// @since libnotmuch 4.4 (notmuch 0.23)
    pub fn notmuch_database_set_config(
        db: *mut notmuch_database_t,
        key: *const c_char,
        value: *const c_char,
    ) -> notmuch_status_t;

    /// retrieve config item 'key', assign to  'value'
    ///
    /// keys which have not been previously set with n_d_set_config will
    /// return an empty string.
    ///
    /// return value is allocated by malloc and should be freed by the
    /// caller.
    ///
    /// @since libnotmuch 4.4 (notmuch 0.23)
    pub fn notmuch_database_get_config(
        db: *mut notmuch_database_t,
        key: *const c_char,
        value: *mut *mut c_char,
    ) -> notmuch_status_t;

    /// Create an iterator for all config items with keys matching a given prefix
    ///
    /// @since libnotmuch 4.4 (notmuch 0.23)
    pub fn notmuch_database_get_config_list(
        db: *mut notmuch_database_t,
        prefix: *const c_char,
        out: *mut *mut notmuch_config_list_t,
    ) -> notmuch_status_t;

    /// Is 'config_list' iterator valid (i.e. _key, _value, _move_to_next can be called).
    ///
    /// @since libnotmuch 4.4 (notmuch 0.23)
    pub fn notmuch_config_list_valid(config_list: *mut notmuch_config_list_t) -> notmuch_bool_t;

    /// return key for current config pair
    ///
    /// return value is owned by the iterator, and will be destroyed by the
    /// next call to `notmuch_config_list_key` or `notmuch_config_list_destroy`.
    ///
    /// @since libnotmuch 4.4 (notmuch 0.23)
    pub fn notmuch_config_list_key(config_list: *mut notmuch_config_list_t) -> *const c_char;

    /// return 'value' for current config pair
    ///
    /// return value is owned by the iterator, and will be destroyed by the
    /// next call to `notmuch_config_list_value` or notmuch `config_list_destroy`
    ///
    /// @since libnotmuch 4.4 (notmuch 0.23)
    pub fn notmuch_config_list_value(config_list: *mut notmuch_config_list_t) -> *const c_char;

    /// move 'config_list' iterator to the next pair
    ///
    /// @since libnotmuch 4.4 (notmuch 0.23)
    pub fn notmuch_config_list_move_to_next(config_list: *mut notmuch_config_list_t);

    /// free any resources held by 'config_list'
    ///
    /// @since libnotmuch 4.4 (notmuch 0.23)
    pub fn notmuch_config_list_destroy(config_list: *mut notmuch_config_list_t);

    /// get the current default indexing options for a given database.
    ///
    /// This object will survive until the database itself is destroyed,
    /// but the caller may also release it earlier with
    /// notmuch_indexopts_destroy.
    ///
    /// This object represents a set of options on how a message can be
    /// added to the index.  At the moment it is a featureless stub.
    ///
    /// @since libnotmuch 5.1 (notmuch 0.26)
    pub fn notmuch_database_get_default_indexopts(db: *mut notmuch_database_t) -> *mut notmuch_indexopts_t;


    //// 
    //// Stating a policy about how to decrypt messages.
    ////
    //// See index.decrypt in notmuch-config(1) for more details.
    //// 
    //// typedef enum {
    ////     NOTMUCH_DECRYPT_FALSE,
    ////     NOTMUCH_DECRYPT_TRUE,
    ////     NOTMUCH_DECRYPT_AUTO,
    ////     NOTMUCH_DECRYPT_NOSTASH,
    //// } notmuch_decryption_policy_t;
    ////
    //// 
    //// Specify whether to decrypt encrypted parts while indexing.
    ////
    //// Be aware that the index is likely sufficient to reconstruct the
    //// cleartext of the message itself, so please ensure that the notmuch
    //// message index is adequately protected. DO NOT SET THIS FLAG TO TRUE
    //// without considering the security of your index.
    ////
    //// @since libnotmuch 5.1 (notmuch 0.26)
    pub fn notmuch_indexopts_set_decrypt_policy(options: *mut notmuch_indexopts_t,
                                                decrypt_policy: notmuch_decryption_policy_t) -> notmuch_status_t;

    //// Return whether to decrypt encrypted parts while indexing.
    //// see notmuch_indexopts_set_decrypt_policy.
    ////  
    //// @since libnotmuch 5.1 (notmuch 0.26)
    pub fn notmuch_indexopts_get_decrypt_policy(options: *const notmuch_indexopts_t) -> notmuch_decryption_policy_t;


    /// Destroy a notmuch_indexopts_t object.
    /// 
    /// @since libnotmuch 5.1 (notmuch 0.26)
    pub fn notmuch_indexopts_destroy(options: *mut notmuch_indexopts_t);

    /// interrogate the library for compile time features
    ///
    /// @since libnotmuch 4.4 (notmuch 0.23)
    pub fn notmuch_built_with(name: *const c_char) -> notmuch_bool_t;
}
