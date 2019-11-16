use std::ops::Drop;
use std::ffi::{CStr, CString};
use supercow::Supercow;

use ffi;
use Database;
use Filenames;
use FilenamesOwner;
use utils::{ToStr, ScopedSupercow, ScopedPhantomcow};


#[derive(Debug)]
pub struct ConfigList<'d> {
    ptr: *mut ffi::notmuch_config_list_t,
    marker: ScopedPhantomcow<'d, Database>,
}

impl<'d> Drop for ConfigList<'d> {
    fn drop(&mut self) {
        unsafe { ffi::notmuch_config_list_destroy(self.ptr) };
    }
}

impl<'d> ConfigList<'d> {
    pub(crate) fn from_ptr<O>(ptr: *mut ffi::notmuch_config_list_t, owner: O) -> ConfigList<'d>
    where
        O: Into<ScopedPhantomcow<'d, Database>>,
    {
        ConfigList {
            ptr,
            marker: owner.into(),
        }
    }
}


impl<'d> Iterator for ConfigList<'d>
{
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        let valid = unsafe { ffi::notmuch_config_list_valid(self.ptr) };

        if valid == 0 {
            return None;
        }

        let (k, v) = unsafe {
            let key = ffi::notmuch_config_list_key(self.ptr);
            let value = ffi::notmuch_config_list_value(self.ptr);

            ffi::notmuch_config_list_move_to_next(self.ptr);

            (key, value)
        };

        Some((k.to_string_lossy().to_string(), v.to_string_lossy().to_string()))
    }
}

unsafe impl<'d> Send for ConfigList<'d> {}
unsafe impl<'d> Sync for ConfigList<'d> {}
