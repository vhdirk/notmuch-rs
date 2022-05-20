extern crate dirs;
extern crate gethostname;
extern crate lettre;
extern crate lettre_email;
extern crate maildir;
extern crate notmuch;
extern crate tempfile;

mod fixtures;
#[cfg(feature = "v0_32")]
mod test_config;
mod test_database;
mod test_message;
mod test_query;
mod test_tags;
mod test_thread;
