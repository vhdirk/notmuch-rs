use std::ops::Drop;

use ffi;
use utils::ScopedPhantomcow;
use MessageOwner;
use Message;
use Tags;
use TagsOwner;

#[derive(Debug)]
pub struct Messages<'o, O>
where
    O: MessageOwner + 'o,
{
    pub(crate) ptr: *mut ffi::notmuch_messages_t,
    marker: ScopedPhantomcow<'o, O>,
}

// impl<'o, O> Drop for Messages<'o, O>
// where
//     O: MessageOwner + 'o,
// {
//     fn drop(self: &mut Self) {
//         unsafe { ffi::notmuch_messages_destroy(self.ptr) };
//     }
// }

impl<'o, O> Messages<'o, O>
where
    O: MessageOwner + 'o,
{
    pub(crate) fn from_ptr<P>(ptr: *mut ffi::notmuch_messages_t, owner: P) -> Messages<'o, O>
    where
        P: Into<ScopedPhantomcow<'o, O>>,
    {
        Messages {
            ptr,
            marker: owner.into(),
        }
    }
}

impl<'o, O> MessageOwner for Messages<'o, O> where O: MessageOwner + 'o {}
impl<'o, O> TagsOwner for Messages<'o, O> where O: MessageOwner + 'o {}

impl<'o, O> Messages<'o, O>
where
    O: MessageOwner + 'o,
{
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
    pub fn collect_tags<'m>(self: &'o Self) -> Tags<'m, Self> {
        Tags::from_ptr(
            unsafe { ffi::notmuch_messages_collect_tags(self.ptr) },
            self,
        )
    }
}

impl<'o, O> Iterator for Messages<'o, O>
where
    O: MessageOwner + 'o,
{
    type Item = Message<'o, O>;

    fn next(&mut self) -> Option<Self::Item> {
        let valid = unsafe { ffi::notmuch_messages_valid(self.ptr) };

        if valid == 0 {
            return None;
        }

        let cthrd = unsafe {
            let thrd = ffi::notmuch_messages_get(self.ptr);
            ffi::notmuch_messages_move_to_next(self.ptr);
            thrd
        };

        Some(Message::from_ptr(cthrd, ScopedPhantomcow::<'o, O>::share(&mut self.marker)))
    }
}



pub trait MessagesExt<'o, O>
where
    O: MessageOwner + 'o,
{
}

impl<'o, O> MessagesExt<'o, O> for Messages<'o, O> where O: MessageOwner + 'o {}


unsafe impl<'o, O> Send for Messages<'o, O> where O: MessageOwner + 'o {}
unsafe impl<'o, O> Sync for Messages<'o, O> where O: MessageOwner + 'o {}

#[cfg(test)]
mod tests {
    // This will not compile if ownership can't be subject to recursion
    fn descend<'o, O: 'o + super::MessageOwner, T: Iterator<Item=super::Message<'o, O>>>(iter: T)
            -> usize {
        iter.map(|msg| descend(msg.replies()) ).count()
    }
    
    use query::Query;
    use database;
    
    #[test]
    #[should_panic] // until test data is filled in
    fn recurse() -> () {
        match database::Database::open(
            &String::new(),
            database::DatabaseMode::ReadOnly,
        ) {
            /* This will not happen without test data, but will force the compiler to compile
             * the descend function.
             */
            Ok(db) => {
                let q = Query::create(db, &String::new()).unwrap();
                descend::<Query, super::Messages<Query>>(q.search_messages().unwrap());
            }
            Err(err) => {
                panic!("Got error while trying to open db: {:?}", err);
            }
        }
    }
}
