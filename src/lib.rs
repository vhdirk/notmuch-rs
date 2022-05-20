#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

#[macro_use]
mod macros;

extern crate from_variants;
extern crate libc;

mod ffi;
mod utils;

mod config_list;
mod config_pairs;
mod config_values;
mod database;
mod directory;
mod error;
mod filenames;
mod index_opts;
mod message;
mod message_properties;
mod messages;
mod query;
mod tags;
mod thread;
mod threads;

pub use config_list::ConfigList;
pub use config_pairs::ConfigPairs;
pub use config_values::ConfigValues;
pub use database::{AtomicOperation, Database, Revision};
pub use directory::Directory;
pub use error::Error;
pub use filenames::Filenames;
pub use index_opts::IndexOpts;
pub use message::{FrozenMessage, Message};
pub use message_properties::MessageProperties;
pub use messages::Messages;
pub use query::Query;
pub use tags::Tags;
pub use thread::Thread;
pub use threads::Threads;

pub use ffi::{ConfigKey, DatabaseMode, DecryptionPolicy, Exclude, MessageFlag, Sort, Status};
