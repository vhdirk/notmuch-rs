#![feature(core, libc, std_misc)]
extern crate libc;

#[macro_use]
mod macros;

mod ffi;
mod utils;

pub mod error;
pub mod database;
