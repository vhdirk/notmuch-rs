use notmuch::ConfigKey;

use crate::fixtures::{MailBox, NotmuchCommand};

struct ConfigFixture {
    // Return a read-write Database.
    // The database will have 3 messages, 2 threads.
    pub mailbox: MailBox,
    pub database: notmuch::Database,
}

impl ConfigFixture {
    pub fn new() -> Self {
        let mailbox = MailBox::new();

        let cmd = NotmuchCommand::new(&mailbox.path());
        cmd.run(vec!["new"]).unwrap();

        let database = notmuch::Database::open_with_config(
            Some(&mailbox.path()),
            notmuch::DatabaseMode::ReadWrite,
            Some(mailbox.path().join("notmuch-config")),
            None,
        )
        .unwrap();

        Self { mailbox, database }
    }
}

mod config {
    use super::*;

    #[test]
    fn test_config() {
        let db = ConfigFixture::new();

        assert_eq!(
            db.database.config(ConfigKey::UserName).unwrap(),
            "Some Hacker"
        );

        assert_eq!(
            db.database.config(ConfigKey::MailRoot).unwrap(),
            db.mailbox.path().to_str().unwrap()
        );
    }

    #[test]
    fn test_config_set() {
        let db = ConfigFixture::new();

        const USER_NAME: &str = "Hideo Kojima";

        db.database
            .config_set(ConfigKey::UserName, USER_NAME)
            .unwrap();

        assert_eq!(db.database.config(ConfigKey::UserName).unwrap(), USER_NAME);
    }

    #[test]
    fn test_config_values() {
        let db = ConfigFixture::new();

        let tags: Vec<_> = db
            .database
            .config_values(ConfigKey::NewTags)
            .unwrap()
            .collect();

        assert_eq!(tags.len(), 2);
        assert!(tags.iter().any(|x| x == "unread"));
        assert!(tags.iter().any(|x| x == "inbox"));
    }

    #[test]
    fn test_config_values_string() {
        let db = ConfigFixture::new();

        let tags: Vec<_> = db
            .database
            .config_values_string("search.exclude_tags")
            .unwrap()
            .collect();

        assert_eq!(tags.len(), 2);
        assert!(tags.iter().any(|x| x == "deleted"));
        assert!(tags.iter().any(|x| x == "spam"));
    }

    #[test]
    fn test_config_pairs() {
        let db = ConfigFixture::new();

        let pairs: Vec<(_, _)> = db.database.config_pairs("user").unwrap().collect();

        println!("{pairs:?}");

        assert_eq!(pairs.len(), 3);
        assert!(pairs
            .iter()
            .any(|(k, v)| k == "user.name" && v.as_deref() == Some("Some Hacker")));
        assert!(pairs
            .iter()
            .any(|(k, v)| k == "user.primary_email" && v.as_deref() == Some("dst@example.com")));
        assert!(pairs
            .iter()
            .any(|(k, v)| k == "user.other_email" && *v == None));
    }

    #[test]
    fn test_config_bool() {
        let db = ConfigFixture::new();

        assert_eq!(
            db.database.config_bool(ConfigKey::MaildirFlags).unwrap(),
            true
        );
    }

    #[test]
    fn test_config_path() {
        let db = ConfigFixture::new();

        assert_eq!(
            db.database.config_path().unwrap(),
            db.mailbox.path().join("notmuch-config"),
        );
    }
}
