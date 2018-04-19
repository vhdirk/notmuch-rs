#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[macro_use]
mod macros;

extern crate libc;

mod utils;
mod ffi;

mod error;
mod database;
mod directory;
mod query;
mod messages;
mod message;
mod tags;
mod threads;
mod thread;
mod filenames;

pub use error::Error;
pub use database::Database;
pub use directory::Directory;
pub use query::Query;
pub use messages::Messages;
pub use message::Message;
pub use tags::Tags;
pub use threads::Threads;
pub use thread::Thread;
pub use filenames::Filenames;

pub use ffi::DatabaseMode;
