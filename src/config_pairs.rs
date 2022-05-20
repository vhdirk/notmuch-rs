use std::ops::Drop;
use std::rc::Rc;

use ffi;
use utils::ToStr;
use Database;

#[derive(Debug)]
pub struct ConfigPairsPtr(*mut ffi::notmuch_config_pairs_t);

#[derive(Clone, Debug)]
pub struct ConfigPairs {
    ptr: Rc<ConfigPairsPtr>,
    _owner: Database,
}

impl Drop for ConfigPairsPtr {
    fn drop(&mut self) {
        unsafe { ffi::notmuch_config_pairs_destroy(self.0) };
    }
}

impl ConfigPairs {
    pub(crate) fn from_ptr(ptr: *mut ffi::notmuch_config_pairs_t, owner: Database) -> ConfigPairs {
        ConfigPairs {
            ptr: Rc::new(ConfigPairsPtr(ptr)),
            _owner: owner,
        }
    }
}

impl Iterator for ConfigPairs {
    type Item = (String, Option<String>);

    fn next(&mut self) -> Option<Self::Item> {
        let valid = unsafe { ffi::notmuch_config_pairs_valid(self.ptr.0) };

        if valid == 0 {
            return None;
        }

        let (k, v) = unsafe {
            let k = ffi::notmuch_config_pairs_key(self.ptr.0);
            let v = ffi::notmuch_config_pairs_value(self.ptr.0);

            ffi::notmuch_config_pairs_move_to_next(self.ptr.0);

            (k, v)
        };

        let value = if v.is_null() {
            None
        } else {
            Some(v.to_string_lossy().to_string())
        };

        Some((k.to_string_lossy().to_string(), value))
    }
}
