use std::ffi::CStr;
use std::ops::Drop;
use std::rc::Rc;

use ffi;
use Message;

#[derive(Debug)]
pub(crate) struct MessagePropertiesPtr(*mut ffi::notmuch_message_properties_t);

impl Drop for MessagePropertiesPtr {
    fn drop(&mut self) {
        unsafe { ffi::notmuch_message_properties_destroy(self.0) };
    }
}

#[derive(Clone, Debug)]
pub struct MessageProperties {
    ptr: Rc<MessagePropertiesPtr>,
    owner: Message,
}

impl MessageProperties {
    pub(crate) fn from_ptr(
        ptr: *mut ffi::notmuch_message_properties_t,
        owner: Message,
    ) -> MessageProperties {
        MessageProperties {
            ptr: Rc::new(MessagePropertiesPtr(ptr)),
            owner,
        }
    }
}

impl Iterator for MessageProperties {
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        let valid = unsafe { ffi::notmuch_message_properties_valid(self.ptr.0) };

        if valid == 0 {
            return None;
        }

        let (k, v) = unsafe {
            let key = CStr::from_ptr(ffi::notmuch_message_properties_key(self.ptr.0));
            let value = CStr::from_ptr(ffi::notmuch_message_properties_value(self.ptr.0));

            ffi::notmuch_message_properties_move_to_next(self.ptr.0);

            (key, value)
        };

        Some((
            k.to_string_lossy().to_string(),
            v.to_string_lossy().to_string(),
        ))
    }
}
