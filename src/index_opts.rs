use std::ops::Drop;
use std::rc::Rc;

use Database;
use error::Result;
use ffi;
use ffi::DecryptionPolicy;

#[derive(Debug)]
pub(crate) struct IndexOptsPtr(pub(crate) *mut ffi::notmuch_indexopts_t);

impl Drop for IndexOptsPtr {
    fn drop(&mut self) {
        unsafe { ffi::notmuch_indexopts_destroy(self.0) };
    }
}

#[derive(Debug)]
pub struct IndexOpts {
    pub(crate) ptr: Rc<IndexOptsPtr>,
    owner: Database,
}

impl IndexOpts {
    pub(crate) fn from_ptr(
        ptr: *mut ffi::notmuch_indexopts_t,
        owner: Database,
    ) -> IndexOpts {
        IndexOpts {
            ptr: Rc::new(IndexOptsPtr(ptr)),
            owner,
        }
    }

    pub fn set_decrypt_policy(&self, decrypt_policy: DecryptionPolicy) -> Result<()> {
        unsafe { ffi::notmuch_indexopts_set_decrypt_policy(self.ptr.0, decrypt_policy.into()) }
            .as_result()
    }

    pub fn decrypt_policy(&self) -> DecryptionPolicy {
        unsafe { ffi::notmuch_indexopts_get_decrypt_policy(self.ptr.0) }.into()
    }
}
