use std::sync::Arc;
use std::path::PathBuf;
use fixtures::MailBox;

struct MessageFixture {
    // Return a single thread with 2 messages
    pub mailbox: MailBox,
    pub database: Arc<notmuch::Database>,
    pub maildir_msg: (String, PathBuf),
    pub message: notmuch::Message<'static, notmuch::Database>,
}

impl MessageFixture {
    pub fn new() -> Self{
        let mailbox = MailBox::new();

        let (msgid, filename) = mailbox.deliver(None, None, None, None, vec![],  true, None, false, false, false).unwrap();

        let database = Arc::new(notmuch::Database::create(&mailbox.path()).unwrap());
        let message = <notmuch::Database as notmuch::DatabaseExt>::index_file(database.clone(), &filename, None).unwrap();
    
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
        let copy = <notmuch::Database as notmuch::DatabaseExt>::find_message_by_filename(msg.database.clone(), &msg.message.filename()).unwrap().unwrap();
        assert_eq!(msg.message.id(), copy.id())
    }

    #[test]
    fn test_messageid_find() {
        let msg = MessageFixture::new();
        let copy = <notmuch::Database as notmuch::DatabaseExt>::find_message(msg.database.clone(), &msg.message.id()).unwrap().unwrap();
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
        assert_eq!(msg.message.header(&"from").unwrap(), Some("<src@example.com>"));
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

        let copy = <notmuch::Database as notmuch::DatabaseExt>::find_message(msg.database.clone(), &msg.message.id()).unwrap().unwrap();
        assert!(copy.tags().any(|x| x == "foo"));

        msg.message.thaw().unwrap();

        assert!(!msg.message.tags().any(|x| x == "foo"));

        let copy2 = <notmuch::Database as notmuch::DatabaseExt>::find_message(msg.database.clone(), &msg.message.id()).unwrap().unwrap();
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

            let copy = <notmuch::Database as notmuch::DatabaseExt>::find_message(msg.database.clone(), &msg.message.id()).unwrap().unwrap();
            assert!(copy.tags().any(|x| x == "foo"));
        }

        let copy2 = <notmuch::Database as notmuch::DatabaseExt>::find_message(msg.database.clone(), &msg.message.id()).unwrap().unwrap();
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
 

// struct MessagePropertiesFixture {
//     // Return a single thread with 2 messages
//     pub mailbox: MailBox,
//     pub database: Arc<notmuch::Database>,
//     pub maildir_msg: (String, PathBuf),
//     pub message: notmuch::Message<'static, notmuch::Database>,
// }

// impl MessagePropertiesFixture {
//     pub fn new() -> Self{
//         let mailbox = MailBox::new();

//         let (msgid, filename) = mailbox.deliver(None, None, None, None, vec![],  true, None, false, false, false).unwrap();

//         let database = Arc::new(notmuch::Database::create(&mailbox.path()).unwrap());
//         let message = <notmuch::Database as notmuch::DatabaseExt>::index_file(database.clone(), &filename, None).unwrap();
//         let properties = <notmuch::Message as notmuch::MessageExt>::properties(&message, false);

//         Self {
//             mailbox,
//             database,
//             maildir_msg: (msgid, filename),
//             message
//         }
//     }
// }


mod properties {
    use super::*;

    #[test]
    fn test_add_single() {
        let msg = MessageFixture::new();
        msg.message.add_property(&"foo", &"bar").unwrap();
        assert_eq!(msg.message.property(&"foo", true).unwrap(), "bar");

        msg.message.add_property(&"bar", &"baz").unwrap();
        assert_eq!(msg.message.property(&"bar", true).unwrap(), "baz");
    }

    #[test]
    fn test_add_dup() {
        let msg = MessageFixture::new();
        msg.message.add_property(&"foo", &"bar").unwrap();
        msg.message.add_property(&"foo", &"baz").unwrap();

        assert_eq!(msg.message.property(&"foo", true).unwrap(), "bar");

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
    fn test_remove() {
        let msg = MessageFixture::new();
        msg.message.add_property(&"foo", &"a").unwrap();
        msg.message.add_property(&"foo", &"b").unwrap();

        msg.message.remove_property(&"foo", &"a").unwrap();
        
        assert_eq!(msg.message.property(&"foo", true).unwrap(), "b");

    }

}
 

 
 
//      def test_del(self, props):
//          props.add('foo', 'a')
//          props.add('foo', 'b')
//          del props['foo']
//          with pytest.raises(KeyError):
//              props['foo']
 
//      def test_remove(self, props):
//          props.add('foo', 'a')
//          props.add('foo', 'b')
//          props.remove('foo', 'a')
//          assert props['foo'] == 'b'
 
//      def test_view_abcs(self, props):
//          assert isinstance(props.keys(), collections.abc.KeysView)
//          assert isinstance(props.values(), collections.abc.ValuesView)
//          assert isinstance(props.items(), collections.abc.ItemsView)
 
//      def test_pop(self, props):
//          props.add('foo', 'a')
//          props.add('foo', 'b')
//          val = props.pop('foo')
//          assert val == 'a'
 
//      def test_pop_default(self, props):
//          with pytest.raises(KeyError):
//              props.pop('foo')
//          assert props.pop('foo', 'default') == 'default'
 
//      def test_popitem(self, props):
//          props.add('foo', 'a')
//          assert props.popitem() == ('foo', 'a')
//          with pytest.raises(KeyError):
//              props.popitem()
 
//      def test_clear(self, props):
//          props.add('foo', 'a')
//          props.clear()
//          assert len(props) == 0
 
//      def test_getall(self, props):
//          props.add('foo', 'a')
//          assert set(props.getall('foo')) == {('foo', 'a')}
 
//      def test_getall_prefix(self, props):
//          props.add('foo', 'a')
//          props.add('foobar', 'b')
//          assert set(props.getall('foo')) == {('foo', 'a'), ('foobar', 'b')}
 
//      def test_getall_exact(self, props):
//          props.add('foo', 'a')
//          props.add('foobar', 'b')
//          assert set(props.getall('foo', exact=True)) == {('foo', 'a')}
