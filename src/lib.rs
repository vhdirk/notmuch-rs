#![feature(core, libc)]
extern crate libc;

#[macro_use]
mod macros;

mod ffi;
mod utils;

pub mod error;
