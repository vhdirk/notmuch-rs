use std::ops::Drop;
use supercow::{Supercow, Phantomcow};

use error::{Error, Result};
use ffi;
use ffi::DecryptionPolicy;
use Database;
use Filenames;
use FilenamesOwner;
use utils::{ScopedSupercow, ScopedPhantomcow};


#[derive(Debug)]
pub struct IndexOpts<'d> {
    pub(crate) ptr: *mut ffi::notmuch_indexopts_t,
    marker: ScopedPhantomcow<'d, Database>,
}

impl<'d> Drop for IndexOpts<'d> {
    fn drop(&mut self) {
        unsafe { ffi::notmuch_indexopts_destroy(self.ptr) };
    }
}

impl<'d> IndexOpts<'d> {
    pub(crate) fn from_ptr<O>(ptr: *mut ffi::notmuch_indexopts_t, owner: O) -> IndexOpts<'d>
    where
        O: Into<ScopedPhantomcow<'d, Database>>,
    {
        IndexOpts {
            ptr,
            marker: owner.into(),
        }
    }

    pub fn set_decrypt_policy(self: &Self, decrypt_policy: DecryptionPolicy) -> Result<()> {
        unsafe { ffi::notmuch_indexopts_set_decrypt_policy(self.ptr, decrypt_policy.into()) }.as_result()
    }

    pub fn decrypt_policy(self: &Self) -> DecryptionPolicy {
        unsafe { ffi::notmuch_indexopts_get_decrypt_policy(self.ptr)}.into()
    }
}

unsafe impl<'d> Send for IndexOpts<'d> {}
unsafe impl<'d> Sync for IndexOpts<'d> {}