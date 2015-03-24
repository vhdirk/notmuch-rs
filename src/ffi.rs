#![allow(dead_code, non_camel_case_types)]

//! Re-presentation of the notmuch C API.

use libc::{
    c_char,
    c_double,
    c_int,
    c_uint,
    c_void,
    time_t,
};

use std::{
    error,
    fmt,
    str,
};

use utils::{
    NotmuchType,
    ToStr,
};

pub type notmuch_bool_t = c_int;

pub type notmuch_compact_status_cb_t = extern fn(*const c_char, *mut c_void);

notmuch_enum! {
    #[repr(C)]
    #[derive(Copy, Debug)]
    pub enum notmuch_status_t => NotmuchStatus {
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
    pub fn is_ok(&self) -> bool {
       match *self {
            notmuch_status_t::NOTMUCH_STATUS_SUCCESS => true,
            _ => false,
        }
    }

    pub fn as_result(self) -> Result<(), Self> {
        match self.is_ok() {
            true => Ok(()),
            false => Err(self),
        }
    }
}

impl ToStr for NotmuchStatus {
    fn to_str<'a>(&self) -> Result<&'a str, str::Utf8Error> {
        unsafe {
            notmuch_status_to_string(self.to_notmuch_t())
        }.to_str()
    }
}

impl fmt::Display for NotmuchStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str().unwrap())
    }
}

impl error::Error for NotmuchStatus {
    fn description(&self) -> &str {
        self.to_str().unwrap()
    }
}

notmuch_enum! {
    #[repr(C)]
    #[derive(Copy, Debug)]
    pub enum notmuch_database_mode_t => NotmuchDatabaseMode {
        NOTMUCH_DATABASE_MODE_READ_ONLY => ReadOnly,
        NOTMUCH_DATABASE_MODE_READ_WRITE => ReadWrite
    }
}

notmuch_enum! {
    #[repr(C)]
    #[derive(Copy, Debug)]
    pub enum notmuch_sort_t => NotmuchSort {
        NOTMUCH_SORT_OLDEST_FIRST => OldestFirst,
        NOTMUCH_SORT_NEWEST_FIRST => NewestFirst,
        NOTMUCH_SORT_MESSAGE_ID => MessageID,
        NOTMUCH_SORT_UNSORTED => ReadWrite
    }
}

notmuch_enum! {
    #[repr(C)]
    #[derive(Copy, Debug)]
    pub enum notmuch_exclude_t => NotmuchExclude {
        NOTMUCH_EXCLUDE_FLAG => Flag,
        NOTMUCH_EXCLUDE_TRUE => True,
        NOTMUCH_EXCLUDE_FALSE => False,
        NOTMUCH_EXCLUDE_ALL => All
    }
}

notmuch_enum! {
    #[repr(C)]
    #[derive(Copy, Debug)]
    pub enum notmuch_message_flag_t => NotmuchMessageFlag {
        NOTMUCH_MESSAGE_FLAG_MATCH => Match,
        NOTMUCH_MESSAGE_FLAG_EXCLUDED => Excluded,
        NOTMUCH_MESSAGE_FLAG_GHOST => Ghost
    }
}

#[repr(C)] pub struct notmuch_database_t;
#[repr(C)] pub struct notmuch_query_t;
#[repr(C)] pub struct notmuch_threads_t;
#[repr(C)] pub struct notmuch_thread_t;
#[repr(C)] pub struct notmuch_messages_t;
#[repr(C)] pub struct notmuch_message_t;
#[repr(C)] pub struct notmuch_tags_t;
#[repr(C)] pub struct notmuch_directory_t;
#[repr(C)] pub struct notmuch_filenames_t;

#[link(name = "notmuch")]
extern {
    pub fn notmuch_status_to_string(
        status: notmuch_status_t,
    ) -> *const c_char;

    pub fn notmuch_database_create(
        path: *const c_char,
        database: *mut *mut notmuch_database_t,
    ) -> notmuch_status_t;

    pub fn notmuch_database_open(
        path: *const c_char,
        mode: notmuch_database_mode_t,
        database: *mut *mut notmuch_database_t,
    ) -> notmuch_status_t;

    pub fn notmuch_database_close(
        database: *mut notmuch_database_t,
    ) -> notmuch_status_t;

    pub fn notmuch_database_compact(
        path: *const c_char,
        backup_path: *const c_char,
        status_cb: Option<notmuch_compact_status_cb_t>,
        closure: *mut c_void,
    ) -> notmuch_status_t;

    pub fn notmuch_database_destroy(
        database: *mut notmuch_database_t,
    ) -> notmuch_status_t;

    pub fn notmuch_database_get_path(
        database: *mut notmuch_database_t,
    ) -> *const c_char;

    pub fn notmuch_database_get_version(
        database: *mut notmuch_database_t,
    ) -> c_uint;

    pub fn notmuch_database_needs_upgrade(
        database: *mut notmuch_database_t,
    ) -> notmuch_bool_t;

    pub fn notmuch_database_upgrade(
        database: *mut notmuch_database_t,
        progress_notify: Option<extern fn(*mut c_void, c_double)>,
        closure: *mut c_void,
    ) -> notmuch_status_t;

    pub fn notmuch_database_begin_atomic(
        notmuch: *mut notmuch_database_t,
    ) -> notmuch_status_t;

    pub fn notmuch_database_end_atomic(
        notmuch: *mut notmuch_database_t,
    ) -> notmuch_status_t;

    pub fn notmuch_database_get_directory(
        database: *mut notmuch_database_t,
        path: *const c_char,
        directory: *mut *mut notmuch_directory_t,
    ) -> notmuch_status_t;

    pub fn notmuch_database_add_message(
        database: *mut notmuch_database_t,
        filename: *const c_char,
        message: *mut *mut notmuch_message_t,
    ) -> notmuch_status_t;

    pub fn notmuch_database_remove_message(
        database: *mut notmuch_database_t,
        filename: *const c_char,
    ) -> notmuch_status_t;

    pub fn notmuch_database_find_message(
        database: *mut notmuch_database_t,
        message_id: *const c_char,
        message: *mut *mut notmuch_message_t,
    ) -> notmuch_status_t;

    pub fn notmuch_database_find_message_by_filename(
        database: *mut notmuch_database_t,
        filename: *const c_char,
        message: *mut *mut notmuch_message_t,
    ) -> notmuch_status_t;

    pub fn notmuch_database_get_all_tags(
        db: *mut notmuch_database_t,
    ) -> *mut notmuch_tags_t;

    pub fn notmuch_query_create(
        database: *mut notmuch_database_t,
        query_string: *const c_char,
    ) -> *mut notmuch_query_t;

    pub fn notmuch_query_get_query_string(
        query: *mut notmuch_query_t,
    ) -> *const c_char;

    pub fn notmuch_query_set_omit_excluded(
        query: *mut notmuch_query_t,
        omit_excluded: notmuch_exclude_t,
    );

    pub fn notmuch_query_set_sort(
        query: *mut notmuch_query_t,
        sort: notmuch_sort_t,
    );

    pub fn notmuch_query_get_sort(
        query: *mut notmuch_query_t,
    ) -> notmuch_sort_t;

    pub fn notmuch_query_add_tag_exclude(
        query: *mut notmuch_query_t,
        tag: *const c_char,
    );

    pub fn notmuch_query_search_threads(
        query: *mut notmuch_query_t,
    ) -> *mut notmuch_threads_t;

    pub fn notmuch_query_search_messages(
        query: *mut notmuch_query_t,
    ) -> *mut notmuch_messages_t;

    pub fn notmuch_query_destroy(
        query: *mut notmuch_query_t,
    );

    pub fn notmuch_threads_valid(
        threads: *mut notmuch_threads_t,
    ) -> notmuch_bool_t;

    pub fn notmuch_threads_get(
        threads: *mut notmuch_threads_t,
    ) -> *mut notmuch_thread_t;

    pub fn notmuch_threads_move_to_next(
        threads: *mut notmuch_threads_t,
    );

    pub fn notmuch_threads_destroy(
        threads: *mut notmuch_threads_t,
    );

    pub fn notmuch_query_count_messages(
        query: *mut notmuch_query_t,
    ) -> c_uint;

    pub fn notmuch_count_threads(
        query: *mut notmuch_query_t,
    ) -> c_uint;

    pub fn notmuch_thread_get_thread_id(
        thread: *mut notmuch_thread_t,
    ) -> *const c_char;

    pub fn notmuch_thread_get_total_messages(
        thread: *mut notmuch_thread_t,
    ) -> c_int;

    pub fn notmuch_thread_get_toplevel_messages(
        thread: *mut notmuch_thread_t,
    ) -> *mut notmuch_messages_t;

    pub fn notmuch_thread_get_messages(
        thread: *mut notmuch_thread_t,
    ) -> *mut notmuch_messages_t;

    pub fn notmuch_thread_get_matched_messages(
        thread: *mut notmuch_thread_t,
    ) -> c_int;

    pub fn notmuch_thread_get_authors(
        thread: *mut notmuch_thread_t,
    ) -> *const c_char;

    pub fn notmuch_thread_get_subject(
        thread: *mut notmuch_thread_t,
    ) -> *const c_char;

    pub fn notmuch_thread_get_oldest_date(
        thread: *mut notmuch_thread_t,
    ) -> time_t;

    pub fn notmuch_thread_get_newest_date(
        thread: *mut notmuch_thread_t,
    ) -> time_t;

    pub fn notmuch_thread_get_tags(
        thread: *mut notmuch_thread_t,
    ) -> *mut notmuch_tags_t;

    pub fn notmuch_thread_destroy(
        thread: *mut notmuch_thread_t,
    );

    pub fn notmuch_messages_valid(
        messages: *mut notmuch_messages_t,
    ) -> notmuch_bool_t;

    pub fn notmuch_messages_get(
        messages: *mut notmuch_messages_t,
    ) -> *mut notmuch_message_t;

    pub fn notmuch_messages_move_to_next(
        messages: *mut notmuch_messages_t,
    );

    pub fn notmuch_messages_destroy(
        messages: *mut notmuch_messages_t,
    );

    pub fn notmuch_messages_collect_tags(
        messages: *mut notmuch_messages_t,
    ) -> *mut notmuch_tags_t;

    pub fn notmuch_message_get_message_id(
        message: *mut notmuch_message_t,
    ) -> *const c_char;

    pub fn notmuch_message_get_thread_id(
        message: *mut notmuch_message_t,
    ) -> *const c_char;

    pub fn notmuch_message_get_replies(
        message: *mut notmuch_message_t,
    ) -> *mut notmuch_messages_t;

    pub fn notmuch_message_get_filename(
        message: *mut notmuch_message_t,
    ) -> *const c_char;

    pub fn notmuch_message_get_filenames(
        message: *mut notmuch_message_t,
    ) -> *mut notmuch_filenames_t;

    pub fn notmuch_message_get_flag(
        message: *mut notmuch_message_t,
        flag: notmuch_message_flag_t,
    ) -> notmuch_bool_t;

    pub fn notmuch_message_set_flag(
        message: *mut notmuch_message_t,
        flag: notmuch_message_flag_t,
        value: notmuch_bool_t,
    );

    pub fn notmuch_message_get_date(
        message: *mut notmuch_message_t,
    ) -> time_t;

    pub fn notmuch_message_get_header(
        message: *mut notmuch_message_t,
        header: *const c_char,
    ) -> *const c_char;

    pub fn notmuch_message_get_tags(
        message: *mut notmuch_message_t,
    ) -> *mut notmuch_tags_t;

    pub fn notmuch_message_add_tag(
        message: *mut notmuch_message_t,
        tag: *const c_char,
    ) -> notmuch_status_t;

    pub fn notmuch_message_remove_tag(
        message: *mut notmuch_message_t,
        tag: *const c_char,
    ) -> notmuch_status_t;

    pub fn notmuch_message_remove_all_tags(
        message: *mut notmuch_message_t,
    ) -> notmuch_status_t;

    pub fn notmuch_message_maildir_flags_to_tags(
        message: *mut notmuch_message_t,
    ) -> notmuch_status_t;

    pub fn notmuch_message_tags_to_maildir_flags(
        message: *mut notmuch_message_t,
    ) -> notmuch_status_t;

    pub fn notmuch_message_freeze(
        message: *mut notmuch_message_t,
    ) -> notmuch_status_t;

    pub fn notmuch_message_thaw(
        message: *mut notmuch_message_t,
    ) -> notmuch_status_t;

    pub fn notmuch_message_destroy(
        message: *mut notmuch_message_t,
    );

    pub fn notmuch_tags_valid(
        tags: *mut notmuch_tags_t,
    ) -> notmuch_bool_t;

    pub fn notmuch_tags_get(
        tags: *mut notmuch_tags_t,
    ) -> *const c_char;

    pub fn notmuch_tags_move_to_next(
        tags: *mut notmuch_tags_t,
    );

    pub fn notmuch_tags_destroy(
        tags: *mut notmuch_tags_t,
    );

    pub fn notmuch_directory_set_mtime(
        directory: *mut notmuch_directory_t,
        mtime: time_t,
    ) -> notmuch_status_t;

    pub fn notmuch_directory_get_mtime(
        directory: *mut notmuch_directory_t,
    ) -> time_t;

    pub fn notmuch_directory_get_child_files(
        directory: *mut notmuch_directory_t,
    ) -> *mut notmuch_filenames_t;

    pub fn notmuch_directory_get_child_directories(
        directory: *mut notmuch_directory_t,
    ) -> *mut notmuch_filenames_t;

    pub fn notmuch_directory_destroy(
        directory: *mut notmuch_directory_t,
    );

    pub fn notmuch_filenames_valid(
        filenames: *mut notmuch_filenames_t,
    ) -> notmuch_bool_t;

    pub fn notmuch_filenames_get(
        filenames: *mut notmuch_filenames_t,
    ) -> *const c_char;

    pub fn notmuch_filenames_move_to_next(
        filenames: *mut notmuch_filenames_t,
    );

    pub fn notmuch_filenames_destroy(
        filenames: *mut notmuch_filenames_t,
    );
}
