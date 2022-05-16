use std::ops::Drop;
use std::rc::Rc;

use ffi;
use utils::ToStr;
use Database;

#[derive(Debug)]
pub struct ConfigValuesPtr(*mut ffi::notmuch_config_values_t);

#[derive(Clone, Debug)]
pub struct ConfigValues {
    ptr: Rc<ConfigValuesPtr>,
    _owner: Database,
}

impl Drop for ConfigValuesPtr {
    fn drop(&mut self) {
        unsafe { ffi::notmuch_config_values_destroy(self.0) };
    }
}

impl ConfigValues {
    pub(crate) fn from_ptr(
        ptr: *mut ffi::notmuch_config_values_t,
        owner: Database,
    ) -> ConfigValues {
        ConfigValues {
            ptr: Rc::new(ConfigValuesPtr(ptr)),
            _owner: owner,
        }
    }
}

impl Iterator for ConfigValues {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let valid = unsafe { ffi::notmuch_config_values_valid(self.ptr.0) };

        if valid == 0 {
            return None;
        }

        let value = unsafe {
            let value = ffi::notmuch_config_values_get(self.ptr.0);
            ffi::notmuch_config_values_move_to_next(self.ptr.0);
            value
        };

        Some(value.to_string_lossy().to_string())
    }
}
