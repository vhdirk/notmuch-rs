#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

#[macro_use]
mod macros;

extern crate libc;
extern crate supercow;

mod ffi;
mod utils;

mod database;
mod directory;
mod error;
mod filenames;
mod message;
mod messages;
mod query;
mod tags;
mod thread;
mod threads;
mod index;
mod config_list;

pub use database::{Database, DatabaseExt, AtomicOperation};
pub use directory::{Directory, DirectoryExt};
pub use error::Error;
pub use filenames::{Filenames, FilenamesOwner};
pub use message::{Message, MessageExt, MessageOwner, FrozenMessage};
pub use messages::{Messages, MessagesExt};
pub use query::{Query, QueryExt};
pub use tags::{Tags, TagsExt, TagsOwner};
pub use thread::{Thread, ThreadExt};
pub use threads::{Threads, ThreadsExt};
pub use index::IndexOpts;
pub use config_list::ConfigList;

pub use ffi::{Status, DatabaseMode, Sort, DecryptionPolicy};

pub use utils::{ScopedSupercow, ScopedPhantomcow};