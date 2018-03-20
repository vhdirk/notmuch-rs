
#[macro_use]
mod macros;

extern crate notmuch_sys as ffi_sys;
extern crate libc;

mod utils;
mod ffi;

pub mod error;
pub mod database;
pub mod directory;
