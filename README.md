# notmuch-rs

This is not much more than a wrapper for the [notmuch](https://notmuchmail.org/) C api.

[![Build Status](https://travis-ci.org/vhdirk/notmuch-rs.svg?branch=master)](https://travis-ci.org/vhdirk/notmuch-rs)
[![Crate version](https://img.shields.io/crates/v/notmuch.svg)](https://crates.io/crates/notmuch)
[![Download statistics](https://img.shields.io/crates/d/notmuch.svg)](https://crates.io/crates/notmuch)
[![License](https://img.shields.io/crates/l/notmuch.svg)](https://crates.io/crates/notmuch) [![Join the chat at https://gitter.im/notmuch-rs/Lobby](https://badges.gitter.im/notmuch-rs/Lobby.svg)](https://gitter.im/notmuch-rs/Lobby?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

## Building

**notmuch-rs** expects libnotmuch development files to be installed on your system.

## Using

Add this to your `Cargo.toml`:

```toml
[dependencies]
notmuch = "*"
```

and this to your crate root:

```rust
extern crate notmuch;
```

## Example

```rust
extern crate notmuch;


fn main() {

    let mut mail_path = std::env::home_dir().unwrap();
    mail_path.push(".mail");

    let db = notmuch::Database::open(&mail_path, notmuch::DatabaseMode::ReadOnly).unwrap();
    let query = db.create_query("").unwrap();
    let mut threads = query.search_threads().unwrap();

    for thread in threads {
        println!("thread {:?} {:?}", thread.subject(), thread.authors());
    }
}

```

## Concurrency

Notmuch makes no claims regarding thread safety. It does not seem to use any
thread locals, but I did not spot any locks. So, as far as I am concerned, it is
not thread safe. Hence, all pointers are internally tracked with `Rc`s.

## Acknowledgements

notmuch-rs started out from the following projects:
 - https://github.com/Stebalien/notmuch-sys/blob/master/src/lib.rs
 - https://github.com/cmhamill/rust-notmuch

Any contributions are welcome!
