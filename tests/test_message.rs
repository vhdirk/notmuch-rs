use std::sync::Arc;
use std::path::PathBuf;
use fixtures::MailBox;

struct MessageFixture {
    // Return a single thread with 2 messages
    pub mailbox: MailBox,
    pub database: notmuch::Database,
    pub maildir_msg: (String, PathBuf),
    pub message: notmuch::Message,
}

impl MessageFixture {
    pub fn new() -> Self{
        let mailbox = MailBox::new();

        let (msgid, filename) = mailbox.deliver(None, None, None, None, vec![],  true, None, false, false, false).unwrap();

        let database = notmuch::Database::create(&mailbox.path()).unwrap();
        let message = database.index_file(&filename, None).unwrap();

        Self {
            mailbox,
            database,
            maildir_msg: (msgid, filename),
            message
        }
    }
}

mod message {

    use super::*;

    #[test]
    fn test_messageid() {
        let msg = MessageFixture::new();
        let copy = msg.database.find_message_by_filename(&msg.message.filename()).unwrap().unwrap();
        assert_eq!(msg.message.id(), copy.id())
    }

    #[test]
    fn test_messageid_find() {
        let msg = MessageFixture::new();
        let copy = msg.database.find_message(&msg.message.id()).unwrap().unwrap();
        assert_eq!(msg.message.id(), copy.id())
    }

    #[test]
    fn test_path() {
        let msg = MessageFixture::new();
        assert_eq!(msg.message.filename(), msg.maildir_msg.1)
    }


    #[test]
    fn test_filenames() {
        let msg = MessageFixture::new();
        let mut filenames = msg.message.filenames();
        let filename = filenames.next().unwrap();

        assert_eq!(filename, msg.message.filename());

        assert!(filenames.next().is_none());
        let names: Vec<PathBuf> = msg.message.filenames().collect();

        assert_eq!(names, vec![msg.maildir_msg.1]);
    }

    #[test]
    fn test_header() {
        let msg = MessageFixture::new();
        assert_eq!(msg.message.header(&"from").unwrap().unwrap().to_string(), "<src@example.com>");
    }

    #[test]
    fn test_header_not_present() {
        let msg = MessageFixture::new();
        assert_eq!(msg.message.header(&"foo").unwrap(), None);
    }

    #[test]
    fn test_freeze() {
        let msg = MessageFixture::new();

        msg.message.freeze().unwrap();
        msg.message.add_tag(&"foo").unwrap();
        msg.message.add_tag(&"bar").unwrap();
        msg.message.remove_tag(&"foo").unwrap();
        msg.message.thaw().unwrap();

        assert!(msg.message.tags().all(|x| x != "foo"));
        assert!(msg.message.tags().any(|x| x == "bar"));
    }

    #[test]
    fn test_freeze_context() {
        let msg = MessageFixture::new();

        {
            let _frozen = notmuch::FrozenMessage::new(&msg.message).unwrap();
            msg.message.add_tag(&"foo").unwrap();
            msg.message.add_tag(&"bar").unwrap();
            msg.message.remove_tag(&"foo").unwrap();

        }
        assert!(msg.message.tags().all(|x| x != "foo"));
        assert!(msg.message.tags().any(|x| x == "bar"));
    }


    #[test]
    fn test_freeze_err() {
        // not sure if this test is ok?
        let msg = MessageFixture::new();

        msg.message.add_tag(&"foo").unwrap();

        msg.message.freeze().unwrap();
        assert!(msg.message.remove_all_tags().is_ok());

        let copy = msg.database.find_message(&msg.message.id()).unwrap().unwrap();
        assert!(copy.tags().any(|x| x == "foo"));

        msg.message.thaw().unwrap();

        assert!(!msg.message.tags().any(|x| x == "foo"));

        let copy2 = msg.database.find_message(&msg.message.id()).unwrap().unwrap();
        assert!(!copy2.tags().any(|x| x == "foo"));
    }

    #[test]
    fn test_freeze_context_err() {
        // not sure if this test is ok?
        let msg = MessageFixture::new();
        msg.message.add_tag(&"foo").unwrap();

        {
            let _frozen = notmuch::FrozenMessage::new(&msg.message).unwrap();
            assert!(msg.message.remove_all_tags().is_ok());
            assert!(!msg.message.tags().any(|x| x == "foo"));

            let copy = msg.database.find_message(&msg.message.id()).unwrap().unwrap();
            assert!(copy.tags().any(|x| x == "foo"));
        }

        let copy2 = msg.database.find_message(&msg.message.id()).unwrap().unwrap();
        assert!(!copy2.tags().any(|x| x == "foo"));
        assert!(!msg.message.tags().any(|x| x == "foo"));
    }

    #[test]
    fn test_replies() {
        let msg = MessageFixture::new();
        assert_eq!(msg.message.replies().count(), 0);
    }

}


//      def test_date(self, msg):
//          # XXX Someone seems to treat things as local time instead of
//          #     UTC or the other way around.
//          now = int(time.time())
//          assert abs(now - msg.date) < 3600*24

mod properties {
    use super::*;

    #[test]
    fn test_add_single() {
        let msg = MessageFixture::new();
        msg.message.add_property(&"foo", &"bar").unwrap();
        assert_eq!(msg.message.property(&"foo").unwrap(), "bar");

        msg.message.add_property(&"bar", &"baz").unwrap();
        assert_eq!(msg.message.property(&"bar").unwrap(), "baz");
    }

    #[test]
    fn test_add_dup() {
        let msg = MessageFixture::new();
        msg.message.add_property(&"foo", &"bar").unwrap();
        msg.message.add_property(&"foo", &"baz").unwrap();

        assert_eq!(msg.message.property(&"foo").unwrap(), "bar");

        let props = msg.message.properties(&"foo", true);
        let expect = vec![("foo", "bar"), ("foo", "baz")];
        for (&(ek, ev), (pk, pv)) in expect.iter().zip(props) {
            assert_eq!(ek, pk);
            assert_eq!(ev, pv);
        }
    }

    #[test]
    fn test_len() {
        let msg = MessageFixture::new();
        msg.message.add_property(&"foo", &"a").unwrap();
        msg.message.add_property(&"foo", &"b").unwrap();
        msg.message.add_property(&"bar", &"a").unwrap();

        let num_props = msg.message.properties(&"", false).count();
        assert_eq!(num_props, 3);

        let mut prop_keys: Vec<String> = msg.message.properties(&"", false).map(|x| x.0).collect();
        prop_keys.sort();
        prop_keys.dedup();
        assert_eq!(prop_keys.len(), 2);

        let mut prop_vals: Vec<String> = msg.message.properties(&"", false).map(|x| x.1).collect();
        prop_vals.sort();
        prop_vals.dedup();
        assert_eq!(prop_vals.len(), 2);
    }

    #[test]
    fn test_del() {
        let msg = MessageFixture::new();
        msg.message.add_property(&"foo", &"a").unwrap();
        msg.message.add_property(&"foo", &"b").unwrap();

        msg.message.remove_all_properties(Some(&"foo")).unwrap();
        assert!(msg.message.property(&"foo").is_err());
    }

    #[test]
    fn test_remove() {
        let msg = MessageFixture::new();
        msg.message.add_property(&"foo", &"a").unwrap();
        msg.message.add_property(&"foo", &"b").unwrap();

        msg.message.remove_property(&"foo", &"a").unwrap();
        assert_eq!(msg.message.property(&"foo").unwrap(), "b");
    }

    #[test]
    fn test_clear() {
        let msg = MessageFixture::new();
        msg.message.add_property(&"foo", &"a").unwrap();

        msg.message.remove_all_properties(None).unwrap();
        assert!(msg.message.property(&"foo").is_err());
    }

    #[test]
    fn test_getall() {
        let msg = MessageFixture::new();
        msg.message.add_property(&"foo", &"a").unwrap();

        let prop_keys: Vec<String> = msg.message.properties(&"foo", false).map(|x| x.0).collect();
        assert_eq!(prop_keys.len(), 1);
        assert_eq!(prop_keys, vec!["foo"]);

        let prop_vals: Vec<String> = msg.message.properties(&"foo", false).map(|x| x.1).collect();
        assert_eq!(prop_vals.len(), 1);
        assert_eq!(prop_vals, vec!["a"]);
    }

    #[test]
    fn test_getall_prefix() {
        let msg = MessageFixture::new();
        msg.message.add_property(&"foo", &"a").unwrap();
        msg.message.add_property(&"foobar", &"b").unwrap();

        let prop_keys: Vec<String> = msg.message.properties(&"foo", false).map(|x| x.0).collect();
        assert_eq!(prop_keys.len(), 2);
        assert_eq!(prop_keys, vec!["foo", "foobar"]);

        let prop_vals: Vec<String> = msg.message.properties(&"foo", false).map(|x| x.1).collect();
        assert_eq!(prop_vals.len(), 2);
        assert_eq!(prop_vals, vec!["a", "b"]);
    }

    #[test]
    fn test_getall_exact() {
        let msg = MessageFixture::new();
        msg.message.add_property(&"foo", &"a").unwrap();
        msg.message.add_property(&"foobar", &"b").unwrap();

        let prop_keys: Vec<String> = msg.message.properties(&"foo", true).map(|x| x.0).collect();
        assert_eq!(prop_keys.len(), 1);
        assert_eq!(prop_keys, vec!["foo"]);

        let prop_vals: Vec<String> = msg.message.properties(&"foo", true).map(|x| x.1).collect();
        assert_eq!(prop_vals.len(), 1);
        assert_eq!(prop_vals, vec!["a"]);
    }
}

