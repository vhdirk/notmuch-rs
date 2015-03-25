#![feature(convert, core, libc, std_misc, unsafe_destructor)]
extern crate libc;

#[macro_use]
mod macros;

mod ffi;
mod utils;

pub mod error;
pub mod database;
