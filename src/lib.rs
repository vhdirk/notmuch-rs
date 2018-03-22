#[macro_use]
mod macros;

extern crate libc;

mod utils;
mod ffi;

pub mod error;
pub mod database;
pub mod directory;
pub mod query;

pub use database::Database;
pub use query::Query;

pub use ffi::DatabaseMode;
