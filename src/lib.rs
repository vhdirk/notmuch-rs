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
pub use messages::{Messages, MessagesOwner};
pub use message::{Message, MessageOwner};
pub use tags::{Tags, TagsOwner};
pub use threads::{Threads, ThreadsOwner};
pub use thread::{Thread, ThreadOwner};
pub use filenames::{Filenames, FilenamesOwner};

pub use ffi::{DatabaseMode, Sort};
