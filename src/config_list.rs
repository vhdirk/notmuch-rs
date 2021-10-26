use std::ops::Drop;
use std::rc::Rc;

use Database;
use ffi;
use utils::ToStr;

#[derive(Debug)]
pub struct ConfigListPtr(*mut ffi::notmuch_config_list_t);

#[derive(Clone, Debug)]
pub struct ConfigList {
    ptr: Rc<ConfigListPtr>,
    owner: Database,
}

impl Drop for ConfigListPtr {
    fn drop(&mut self) {
        unsafe { ffi::notmuch_config_list_destroy(self.0) };
    }
}

impl ConfigList {
    pub(crate) fn from_ptr(
        ptr: *mut ffi::notmuch_config_list_t,
        owner: Database,
    ) -> ConfigList {
        ConfigList {
            ptr: Rc::new(ConfigListPtr(ptr)),
            owner,
        }
    }
}

impl Iterator for ConfigList {
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        let valid = unsafe { ffi::notmuch_config_list_valid(self.ptr.0) };

        if valid == 0 {
            return None;
        }

        let (k, v) = unsafe {
            let key = ffi::notmuch_config_list_key(self.ptr.0);
            let value = ffi::notmuch_config_list_value(self.ptr.0);

            ffi::notmuch_config_list_move_to_next(self.ptr.0);

            (key, value)
        };

        Some((
            k.to_string_lossy().to_string(),
            v.to_string_lossy().to_string(),
        ))
    }
}
