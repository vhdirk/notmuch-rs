use std::sync::Arc;
use std::path::PathBuf;
use fixtures::{MailBox, NotmuchCommand};

struct TagSetFixture {
    // An non-empty immutable tagset.
    // This will have the default new mail tags: inbox, unread.
    pub mailbox: MailBox,
    pub cmd: NotmuchCommand,
    pub database: Arc<notmuch::Database>,
    pub message: notmuch::Message<'static, notmuch::Database>
}

impl TagSetFixture {
    pub fn new(mutable: bool, flagged: bool) -> Self{
        let mailbox = MailBox::new();
        let (_msg, filename) = mailbox.deliver(None, None, None, None, vec![], !flagged, None, false, false, flagged).unwrap();
      
        let cmd = NotmuchCommand::new(&mailbox.path());
        cmd.run(vec!["new"]).unwrap();

        let database = Arc::new(notmuch::Database::open(&mailbox.path(), if !mutable {notmuch::DatabaseMode::ReadOnly} else { notmuch::DatabaseMode::ReadWrite }).unwrap());
        let message = <notmuch::Database as notmuch::DatabaseExt>::find_message_by_filename(database.clone(), &filename).unwrap().unwrap();    

        Self {
            mailbox,
            database,
            cmd, 
            message
        }
    }
}

mod immutable {

    use super::*;

    #[test]
    fn test_neg(){
        let tagset = TagSetFixture::new(false, false);

        let tags: Vec<String> = tagset.database.all_tags().unwrap().collect();
        tagset.cmd.run(vec!["tag", "+foo", "*"]).unwrap();

        let database = notmuch::Database::open(&tagset.mailbox.path(), notmuch::DatabaseMode::ReadOnly).unwrap();
        let ntags: Vec<String> = database.all_tags().unwrap().collect();

        assert_ne!(tags, ntags);
    }

    #[test]
    fn test_contains(){
        let tagset = TagSetFixture::new(false, false);
        let tags: Vec<String> = tagset.database.all_tags().unwrap().collect();

        assert!(tags.iter().any(|x| x == "unread"));
        assert!(!tags.iter().any(|x| x == "foo"));
    }


    #[test]
    fn test_len(){
        let tagset = TagSetFixture::new(false, false);
        assert_eq!(tagset.database.all_tags().unwrap().count(), 2);
    }

}

mod mutable {

    use super::*;

    #[test]
    fn test_add(){
        let tagset = TagSetFixture::new(true, false);
        assert!(!tagset.message.tags().any(|x| x == "foo"));

        tagset.message.add_tag("foo").unwrap();
        assert!(tagset.message.tags().any(|x| x == "foo"));
    }

    #[test]
    fn test_discard(){
        let tagset = TagSetFixture::new(true, false);
        assert!(tagset.message.tags().any(|x| x == "inbox"));

        tagset.message.remove_tag("inbox").unwrap();
        assert!(!tagset.message.tags().any(|x| x == "unbox"));
    }

    #[test]
    fn test_discard_not_present(){
        let tagset = TagSetFixture::new(true, false);
        assert!(!tagset.message.tags().any(|x| x == "foo"));

        tagset.message.remove_tag("foo").unwrap();
    }

    #[test]
    fn test_clear(){
        let tagset = TagSetFixture::new(true, false);
        assert!(tagset.message.tags().count() > 0);
        tagset.message.remove_all_tags().unwrap();

        assert!(tagset.message.tags().count() == 0);
    }

    #[test]
    fn test_from_maildir_flags(){
        let tagset = TagSetFixture::new(true, true);

        let msgid = tagset.message.id();
        tagset.message.remove_tag(&"flagged").unwrap();
        tagset.message.maildir_flags_to_tags().unwrap();

        assert!(tagset.message.tags().any(|x| x == "flagged"));
    }


    #[test]
    fn test_to_maildir_flags(){

        let tagset = TagSetFixture::new(true, true);

        let filename = tagset.message.filename();
        let filestr = filename.to_string_lossy();

        let file_parts: Vec<&str> = filestr.split(",").collect();
        let flags = file_parts.last().unwrap();
        println!("Flags {:?}", flags);

        assert!(flags.contains("F"));
        tagset.message.remove_tag(&"flagged").unwrap();
        tagset.message.tags_to_maildir_flags().unwrap();

        let filename = tagset.message.filename();
        let filestr = filename.to_string_lossy();

        let file_parts: Vec<&str> = filestr.split(",").collect();
        let flags = file_parts.last().unwrap();
        assert!(!flags.contains("F"));
    }

}