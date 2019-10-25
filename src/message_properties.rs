use std::ops::Drop;
use std::ffi::{CStr, CString};
use supercow::Supercow;

use ffi;
use Message;
use MessageOwner;
use MessageExt;
use utils::{ScopedSupercow, ScopedPhantomcow};


#[derive(Debug)]
pub struct MessageProperties<'m, 'o, O>
where
    O: MessageOwner + 'o
{
    ptr: *mut ffi::notmuch_message_properties_t,
    marker: ScopedPhantomcow<'m, Message<'o, O>>,
}

impl<'m, 'o, O> Drop for MessageProperties<'m, 'o, O>
where
    O: MessageOwner + 'o
{
    fn drop(&mut self) {
        unsafe { ffi::notmuch_message_properties_destroy(self.ptr) };
    }
}

impl<'m, 'o, O> MessageProperties<'m, 'o, O>
where
    O: MessageOwner + 'o
{
    pub(crate) fn from_ptr<S>(ptr: *mut ffi::notmuch_message_properties_t, owner: S) -> MessageProperties<'m, 'o, O>
    where
        S: Into<ScopedPhantomcow<'m, Message<'o, O>>>,
    {
        MessageProperties {
            ptr,
            marker: owner.into(),
        }
    }
}


impl<'m, 'o, O> Iterator for MessageProperties<'m, 'o, O>
where
    O: MessageOwner + 'o
{
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        let valid = unsafe { ffi::notmuch_message_properties_valid(self.ptr) };

        if valid == 0 {
            return None;
        }

        let (k, v) = unsafe {
            let key = CStr::from_ptr(ffi::notmuch_message_properties_key(self.ptr));
            let value = CStr::from_ptr(ffi::notmuch_message_properties_value(self.ptr));

            ffi::notmuch_message_properties_move_to_next(self.ptr);

            (key, value)
        };

        Some((k.to_str().unwrap().to_string(), v.to_str().unwrap().to_string()))
    }
}

unsafe impl<'m, 'o, O> Send for MessageProperties<'m, 'o, O> where
    O: MessageOwner + 'o {}
unsafe impl<'m, 'o, O> Sync for MessageProperties<'m, 'o, O> where
    O: MessageOwner + 'o {}
