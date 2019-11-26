use std::sync::Arc;
use fixtures::{NotmuchCommand, MailBox};


struct ThreadFixture {
    // Return a single thread with 2 messages
    pub mailbox: MailBox,
    pub thread: notmuch::Thread<'static, 'static>,
}

impl ThreadFixture {
    pub fn new() -> Self{
        let mailbox = MailBox::new();

        let (msgid, _) = mailbox.deliver(None, Some("foo".to_string()), None, None, vec![],  true, None, false, false, false).unwrap();
        mailbox.deliver(None, Some("bar".to_string()), None, None, vec![("In-Reply-To".to_string(), format!("<{}>", msgid))], true, None, false, false, false).unwrap();

        let cmd = NotmuchCommand::new(&mailbox.path());
        cmd.run(vec!["new"]).unwrap();

        let mut threads = {
            let database = Arc::new(notmuch::Database::open(&mailbox.path(), notmuch::DatabaseMode::ReadWrite).unwrap());

            let query = notmuch::Query::create(database.clone(), &"foo".to_string()).unwrap();

            <notmuch::Query as notmuch::QueryExt>::search_threads(query).unwrap()
        };
        let thread = threads.next().unwrap();
    
        Self {
            mailbox,
            thread
        }
    }
}

#[test]
fn test_threadid() {
    let thread = ThreadFixture::new();
    assert!(!thread.thread.id().is_empty());
}


#[test]
fn test_toplevel() {
    let thread = ThreadFixture::new();
    let msgs = thread.thread.toplevel_messages();

    assert_eq!(msgs.count(), 1);
}


#[test]
fn test_toplevel_reply() {
    let thread = ThreadFixture::new();
    let msg = thread.thread.toplevel_messages().next().unwrap();

    assert_eq!(msg.replies().count(), 1);
}

#[test]
fn test_iter() {
    let thread = ThreadFixture::new();
    let msg_count0 = thread.thread.messages().count() as i32;
    let msg_count1 = thread.thread.total_messages();

    assert_eq!(msg_count0, msg_count1);
}

#[test]
fn test_matched() {
    let thread = ThreadFixture::new();
    assert_eq!(thread.thread.matched_messages(), 1);
}


#[test]
fn test_authors() {
    let thread = ThreadFixture::new();

    assert_eq!(thread.thread.authors(), vec!["src@example.com".to_string()]);
}


#[test]
fn test_subject() {
    let thread = ThreadFixture::new();

    println!("{:?}", thread.thread.subject());
    assert_eq!(thread.thread.subject(), "Test mail");
}



#[test]
fn test_tags() {
    let thread = ThreadFixture::new();

    let tags: Vec<String> = thread.thread.tags().collect();
    assert!(tags.iter().any(|x| x == "inbox"));
}
 