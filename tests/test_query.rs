use fixtures::{MailBox, NotmuchCommand};
use std::sync::Arc;

struct QueryFixture {
    pub mailbox: MailBox,
    pub database: notmuch::Database,
    pub query: notmuch::Query,
}

impl QueryFixture {
    pub fn new() -> Self {
        let mailbox = MailBox::new();

        let (msgid, _) = mailbox
            .deliver(
                None,
                Some("foo".to_string()),
                None,
                None,
                vec![],
                true,
                None,
                false,
                false,
                false,
            )
            .unwrap();
        mailbox
            .deliver(
                None,
                Some("bar".to_string()),
                None,
                None,
                vec![],
                true,
                None,
                false,
                false,
                false,
            )
            .unwrap();
        mailbox
            .deliver(
                None,
                Some("baz".to_string()),
                None,
                None,
                vec![("In-Reply-To".to_string(), format!("<{}>", msgid))],
                true,
                None,
                false,
                false,
                false,
            )
            .unwrap();
        mailbox
            .deliver(
                None,
                Some("foo qux".to_string()),
                None,
                None,
                vec![],
                true,
                None,
                false,
                false,
                false,
            )
            .unwrap();
        mailbox
            .deliver(
                None,
                Some("foo quux".to_string()),
                None,
                None,
                vec![],
                true,
                None,
                false,
                false,
                false,
            )
            .unwrap();

        let cmd = NotmuchCommand::new(&mailbox.path());
        cmd.run(vec!["new"]).unwrap();

        let database = notmuch::Database::open(&mailbox.path(), notmuch::DatabaseMode::ReadWrite).unwrap();

        let query = notmuch::Query::create(&database, &"foo".to_string()).unwrap();
        Self { mailbox, database, query }
    }
}

#[test]
fn test_find_message() {
    let f = QueryFixture::new();
    let mut messages = f.query.search_messages().unwrap();

    let message_from_query = messages.next().unwrap();

    let message_from_id = f.database.find_message(&message_from_query.id()).unwrap().unwrap();

    dbg!("got message");
    message_from_id.tags();
    drop(message_from_query);

    // If notmuch would reuse the message pointer, the following call would produce a segfault.
    // Luckily, that is not the case.

    dbg!("got message");
    message_from_id.tags();
}

#[test]
fn test_iter_threads() {
    let q = QueryFixture::new();
    let threads = q.query.search_threads().unwrap();

    let mut num = 0;
    for _thread in threads {
        num += 1;
    }

    assert_eq!(num, 3);
}

#[test]
fn test_iter_messages() {
    let q = QueryFixture::new();
    let messages = q.query.search_messages().unwrap();

    let mut num = 0;
    for _message in messages {
        num += 1;
    }

    assert_eq!(num, 3);
}

#[test]
fn test_iter_messages_drop() {
    let q = QueryFixture::new();

    let mut messages = q.query.search_messages().unwrap();
    let message1 = messages.next().unwrap();
    dbg!("got message");
    message1.tags();

    drop(message1);

    let message2 = messages.next().unwrap();
    dbg!("got message");
    message2.tags();
    drop(message2);
}
#[test]
fn test_iter_messages_drop2() {
    let q = QueryFixture::new();

    let message = {
        let mut messages = q.query.search_messages().unwrap();
        messages.next().unwrap()
    };
    dbg!("got message");
    message.tags();
}

#[test]
fn test_iter_threads_drop() {
    let q = QueryFixture::new();

    let mut threads = q.query.search_threads().unwrap();
    let thread1 = threads.next().unwrap();

    dbg!("got thread");
    drop(thread1);

    let thread2 = threads.next().unwrap();
    drop(threads);

    dbg!("got thread");
    drop(thread2);
}

