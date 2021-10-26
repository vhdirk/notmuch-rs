use std::marker::PhantomData;
use std::ops::Deref;
use std::ops::DerefMut;
use std::rc::Rc;

use from_variants::FromVariants;

use ffi;
use Query;
use Thread;
use Message;
use Tags;

#[derive(Clone, Debug, FromVariants)]
pub(crate) enum MessagesOwner {
    Query(Query),
    Message(Message),
    Messages(Messages),
    Thread(Thread),
}

#[derive(Debug)]
pub(crate) struct MessagesPtr(*mut ffi::notmuch_messages_t);

impl Drop for MessagesPtr {
    fn drop(&mut self) {
        unsafe { ffi::notmuch_messages_destroy(self.0) };
    }
}

#[derive(Clone, Debug)]
pub struct Messages {
    ptr: Rc<MessagesPtr>,
    owner: Box<MessagesOwner>,
}

impl Messages {
    pub(crate) fn from_ptr<O>(ptr: *mut ffi::notmuch_messages_t, owner: O) -> Messages
    where
        O: Into<MessagesOwner>,
    {
        Messages {
            ptr: Rc::new(MessagesPtr(ptr)),
            owner: Box::new(owner.into()),
        }
    }
}

impl Messages {
    /**
     * Return a list of tags from all messages.
     *
     * The resulting list is guaranteed not to contain duplicated tags.
     *
     * WARNING: You can no longer iterate over messages after calling this
     * function, because the iterator will point at the end of the list.
     * We do not have a function to reset the iterator yet and the only
     * way how you can iterate over the list again is to recreate the
     * message list.
     *
     * The function returns NULL on error.
     */
    pub fn collect_tags(&self) -> Tags {
        Tags::from_ptr(
            unsafe { ffi::notmuch_messages_collect_tags(self.ptr.0) },
            self.clone(),
        )
    }
}

impl Iterator for Messages {
    type Item = Message;

    fn next(&mut self) -> Option<Self::Item> {
        let valid = unsafe { ffi::notmuch_messages_valid(self.ptr.0) };

        if valid == 0 {
            return None;
        }

        let cmsg = unsafe {
            let msg = ffi::notmuch_messages_get(self.ptr.0);
            ffi::notmuch_messages_move_to_next(self.ptr.0);
            msg
        };

        Some(Message::from_ptr(cmsg, self.clone()))
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    // This will not compile if ownership can't be subject to recursion
    fn descend<T>(iter: T) -> usize
    where
        T: Iterator<Item = super::Message>,
    {
        iter.map(|msg| descend(msg.replies())).count()
    }

    use database;
    use query::Query;

    #[test]
    #[should_panic] // until test data is filled in
    fn recurse() {
        match database::Database::open(&String::new(), database::DatabaseMode::ReadOnly) {
            /* This will not happen without test data, but will force the compiler to compile
             * the descend function.
             */
            Ok(db) => {
                let q = Query::create(&db, &String::new()).unwrap();
                descend(q.search_messages().unwrap());
            }
            Err(err) => {
                panic!("Got error while trying to open db: {:?}", err);
            }
        }
    }
}
