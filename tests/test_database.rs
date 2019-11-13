use fixtures::{NotmuchCommand, MailBox};

// #[test]
// // fn test_config_pathname_default(){

// //      monkeypatch.delenv('NOTMUCH_CONFIG', raising=False)
// //      user = pathlib.Path('~/.notmuch-config').expanduser()
// //      assert dbmod._config_pathname() == user

// // }

mod database {

    use super::*;

    #[test]
    fn test_create(){
        let mailbox = MailBox::new();
        let db = notmuch::Database::create(&mailbox.path());
        assert!(db.is_ok());

        assert!(mailbox.path().join(".notmuch/xapian").exists());
    }

    #[test]
    fn test_create_already_open(){
        let mailbox = MailBox::new();
        let db1 = notmuch::Database::create(&mailbox.path());
        assert!(db1.is_ok());

        let db2 = notmuch::Database::create(&mailbox.path());
        assert!(db2.is_err());
    }


    #[test]
    fn test_create_existing(){
        let mailbox = MailBox::new();
        notmuch::Database::create(&mailbox.path()).unwrap();

        let db2 = notmuch::Database::create(&mailbox.path());
        assert!(db2.is_err());
    }


    #[test]
    fn test_close(){
        let mailbox = MailBox::new();
        let db = notmuch::Database::create(&mailbox.path()).unwrap();

        assert!(db.close().is_ok());
    }

    #[test]
    fn test_drop_noclose(){
        let mailbox = MailBox::new();
        let db = notmuch::Database::create(&mailbox.path()).unwrap();

        drop(db);
    }

    #[test]
    fn test_close_drop(){
        let mailbox = MailBox::new();
        let db = notmuch::Database::create(&mailbox.path()).unwrap();
        db.close().unwrap();
        drop(db);
    }

    #[test]
    fn test_path(){
        let mailbox = MailBox::new();
        let db = notmuch::Database::create(&mailbox.path()).unwrap();
        assert!(db.path() == mailbox.path());
    }

    #[test]
    fn test_version(){
        let mailbox = MailBox::new();
        let db = notmuch::Database::create(&mailbox.path()).unwrap();
        assert!(db.version() > 0);
    }

}


mod atomic {
    use super::*;

    // TODO: how do I test this??

}


mod revision {
    use super::*;

    #[test]
    fn test_single_rev(){
        let mailbox = MailBox::new();
        let db = notmuch::Database::create(&mailbox.path()).unwrap();

        let rev0 = db.revision();
        let rev1 = db.revision();

        assert!(rev0 == rev1);
        assert!(rev0 <= rev1);
        assert!(rev0 >= rev1);
        assert!(!(rev0 < rev1));
        assert!(!(rev0 > rev1));
    }

    #[test]
    fn test_diff_db(){
        let mailbox0 = MailBox::new();
        let db0 = notmuch::Database::create(&mailbox0.path()).unwrap();
        let rev0 = db0.revision();


        let mailbox1 = MailBox::new();
        let db1 = notmuch::Database::create(&mailbox1.path()).unwrap();
        let rev1 = db1.revision();

        assert!(rev0 != rev1);
        assert!(rev0.uuid != rev1.uuid);
    }

    #[test]
    fn test_cmp(){
        let mailbox = MailBox::new();
        let db = notmuch::Database::create(&mailbox.path()).unwrap();

        let rev0 = db.revision();

        let (_, filename) = mailbox.deliver(None, None, None, None, vec![], true, None, false, false, false).unwrap();

        db.index_file(&filename, None).unwrap();

        let rev1 = db.revision();

        assert!(rev0 < rev1);
        assert!(rev0 <= rev1);
        assert!(!(rev0 > rev1));
        assert!(!(rev0 >= rev1));
        assert!(!(rev0 == rev1));
        assert!(rev0 != rev1);


    }

    // TODO: add tests for revisions comparisons

}
 

mod messages {
    use super::*;

    #[test]
    fn test_add_message() {
        let mailbox = MailBox::new();
        let db = notmuch::Database::create(&mailbox.path()).unwrap();

        let (msgid, filename) = mailbox.deliver(None, None, None, None, vec![], true, None, false, false, false).unwrap();
        let msg = db.index_file(&filename, None).unwrap();

        assert!(msg.filename() == filename);
        assert!(msg.id() == msgid);
        
    }

    #[test]
    fn test_remove_message() {
        let mailbox = MailBox::new();
        let db = notmuch::Database::create(&mailbox.path()).unwrap();

        let (msgid, filename) = mailbox.deliver(None, None, None, None, vec![], true, None, false, false, false).unwrap();
        let msg = db.index_file(&filename, None).unwrap();
        assert!(db.find_message(&msgid).unwrap().is_some());

        db.remove_message(&filename).unwrap();
        assert!(db.find_message(&msgid).unwrap().is_none());
    }
    
    #[test]
    fn test_find_message() {
        let mailbox = MailBox::new();
        let db = notmuch::Database::create(&mailbox.path()).unwrap();

        let (msgid, filename) = mailbox.deliver(None, None, None, None, vec![], true, None, false, false, false).unwrap();
        let msg0 = db.index_file(&filename, None).unwrap();
        
        let msg1 = db.find_message(&msgid).unwrap().unwrap();
        assert!(msg0.id() == msgid);
        assert!(msg0.id() == msg1.id());

        assert!(msg0.filename() == filename);
        assert!(msg0.filename() == msg1.filename());
    }

    #[test]
    fn test_find_message_notfound() {
        let mailbox = MailBox::new();
        let db = notmuch::Database::create(&mailbox.path()).unwrap();

        assert!(db.find_message(&"foo").unwrap().is_none());
    }
    
}

mod tags {
    use super::*;

    #[test]
    fn test_none() {
        let mailbox = MailBox::new();
        let db = notmuch::Database::create(&mailbox.path()).unwrap();

        let tags = db.all_tags().unwrap();

        assert!(tags.count() == 0);
    }

    #[test]
    fn test_some() {
        let mailbox = MailBox::new();
        let db = notmuch::Database::create(&mailbox.path()).unwrap();
        let (_, filename) = mailbox.deliver(None, None, None, None, vec![], true, None, false, false, false).unwrap();
        let msg = db.index_file(&filename, None).unwrap();

        msg.add_tag(&"hello").unwrap();
        let tags: Vec<String> = db.all_tags().unwrap().collect();

        assert!(tags.len() == 1);
        assert!(tags.iter().any(|x| x == "hello"));
    }

    #[test]
    fn test_iters() {
        let mailbox = MailBox::new();
        let db = notmuch::Database::create(&mailbox.path()).unwrap();
        
        let t1: Vec<String> = db.all_tags().unwrap().collect();
        let t2: Vec<String> = db.all_tags().unwrap().collect();
        assert!(t1 == t2);
    }

}

struct PopulatedDatabase {
    // Return a read-write Database.
    // The database will have 3 messages, 2 threads.

    pub mailbox: MailBox,
    pub database: notmuch::Database,
}

impl PopulatedDatabase {
    pub fn new() -> Self{
        let mailbox = MailBox::new();

        let (msgid, _) = mailbox.deliver(None, Some("foo".to_string()), None, None, vec![],  true, None, false, false, false).unwrap();
        mailbox.deliver(None, Some("bar".to_string()), None, None, vec![], true, None, false, false, false).unwrap();
        mailbox.deliver(None, Some("baz".to_string()), None, None, vec![("In-Reply-To".to_string(), format!("<{}>", msgid))], true, None, false, false, false).unwrap();

        let cmd = NotmuchCommand::new(&mailbox.path());
        cmd.run(vec!["new"]).unwrap();

        let database = notmuch::Database::open(&mailbox.path(), notmuch::DatabaseMode::ReadWrite).unwrap();
    
        Self {
            mailbox,
            database
        }
    }
}

mod query {
    use super::*;

    #[test]
    fn test_count_messages() {
        let db = PopulatedDatabase::new();

        let query = db.database.create_query("*").unwrap();
        assert!(query.count_messages().unwrap() == 3);
    }

    #[test]
    fn test_message_no_results() {
        let db = PopulatedDatabase::new();

        let query = db.database.create_query("not_a_matching_query").unwrap();
        let mut messages = query.search_messages().unwrap();
        let msg = messages.next();
        assert!(msg.is_none());
    }

    #[test]
    fn test_message_match() {
        let db = PopulatedDatabase::new();

        let query = db.database.create_query("*").unwrap();
        let mut messages = query.search_messages().unwrap();
        let msg = messages.next();
        assert!(msg.is_some());
    }

    #[test]
    fn test_count_threads() {
        let db = PopulatedDatabase::new();

        let query = db.database.create_query("*").unwrap();
        assert!(query.count_threads().unwrap() == 2);
    }

    #[test]
    fn test_threads_no_results() {
        let db = PopulatedDatabase::new();

        let query = db.database.create_query("not_a_matching_query").unwrap();
        let mut threads = query.search_threads().unwrap();
        let thrd = threads.next();
        assert!(thrd.is_none());
    }

    #[test]
    fn test_threads_match() {
        let db = PopulatedDatabase::new();

        let query = db.database.create_query("*").unwrap();
        let mut threads = query.search_threads().unwrap();
        let thrd = threads.next();
        assert!(thrd.is_some());
    }
}

