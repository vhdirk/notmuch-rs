#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

#[macro_use]
mod macros;

use libc;


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

pub use crate::database::{Database, DatabaseExt};
pub use crate::directory::{Directory, DirectoryExt};
pub use crate::error::Error;
pub use crate::filenames::{Filenames, FilenamesOwner};
pub use crate::message::{Message, MessageExt, MessageOwner};
pub use crate::messages::{Messages, MessagesExt};
pub use crate::query::{Query, QueryExt};
pub use crate::tags::{Tags, TagsExt, TagsOwner};
pub use crate::thread::{Thread, ThreadExt};
pub use crate::threads::{Threads, ThreadsExt};

pub use crate::ffi::{DatabaseMode, Sort};
