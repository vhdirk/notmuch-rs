#[macro_use]
mod macros;

extern crate libc;

mod utils;
mod ffi;

pub mod error;
pub mod database;
pub mod directory;
pub mod query;
pub mod messages;
pub mod tags;
pub mod threads;

pub use database::Database;
pub use query::Query;
pub use messages::Messages;
pub use tags::Tags;
pub use threads::Threads;

pub use ffi::DatabaseMode;
