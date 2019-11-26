use std::sync::Arc;
use fixtures::{NotmuchCommand, MailBox};


struct QueryFixture {
    // Return a single thread with 2 messages
    pub mailbox: MailBox,
    pub query: notmuch::Query<'static>,
}

impl QueryFixture {
    pub fn new() -> Self{
        let mailbox = MailBox::new();

        let (msgid, _) = mailbox.deliver(None, Some("foo".to_string()), None, None, vec![],  true, None, false, false, false).unwrap();
        mailbox.deliver(None, Some("bar".to_string()), None, None, vec![], true, None, false, false, false).unwrap();
        mailbox.deliver(None, Some("baz".to_string()), None, None, vec![("In-Reply-To".to_string(), format!("<{}>", msgid))], true, None, false, false, false).unwrap();
        mailbox.deliver(None, Some("foo qux".to_string()), None, None, vec![],  true, None, false, false, false).unwrap();
        mailbox.deliver(None, Some("foo quux".to_string()), None, None, vec![],  true, None, false, false, false).unwrap();

        let cmd = NotmuchCommand::new(&mailbox.path());
        cmd.run(vec!["new"]).unwrap();

        let query = {
            let database = Arc::new(notmuch::Database::open(&mailbox.path(), notmuch::DatabaseMode::ReadWrite).unwrap());

            notmuch::Query::create(database, &"foo".to_string()).unwrap()
        };
    
        Self {
            mailbox,
            query
        }
    }
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
fn test_iter_threads_ext() {
    let q = QueryFixture::new();
    
    let threads = <notmuch::Query as notmuch::QueryExt>::search_threads(q.query).unwrap();

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
fn test_iter_messages_ext() {
    let q = QueryFixture::new();
    
    let messages = <notmuch::Query as notmuch::QueryExt>::search_messages(q.query).unwrap();

    let mut num = 0;
    for _message in messages {
        num += 1;
    }

    assert_eq!(num, 3);
    
}

