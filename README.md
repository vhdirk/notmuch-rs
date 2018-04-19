notmuch-rs
==========

This is not much more than a wrapper for the [notmuch](https://notmuchmail.org/) C api.

[![Build Status](https://travis-ci.org/vhdirk/notmuch-rs.svg?branch=master)](https://travis-ci.org/vhdirk/notmuch-rs)
[![Crate version](https://img.shields.io/crates/v/notmuch.svg)](https://crates.io/crates/notmuch)
[![Download statistics](https://img.shields.io/crates/d/notmuch.svg)](https://crates.io/crates/notmuch)
[![License](https://img.shields.io/crates/l/notmuch.svg)](https://crates.io/crates/notmuch)

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

    let db = notmuch::Database::open(&mail_path.to_str().unwrap().to_string(), notmuch::DatabaseMode::ReadOnly).unwrap();
    let query = db.create_query(&"".to_string()).unwrap();
    let mut threads = query.search_threads().unwrap();

    loop {
        match threads.next() {
            Some(thread) => {
                println!("thread {:?} {:?}", thread.subject(), thread.authors());
            },
            None => { break }
        }
    }
}

```

## Concurrency

Notmuch makes no claims regarding thread safety. The underlying Xapian database
does not use any globals/thread locals, so it can be used safely accross thread
bounds, although explicit locking may be needed.  

As of v0.1.0, notmuch-rs uses an internal refcounting mechanism to keep track
of the various pointers in the upward tree to the ```notmuch_database_t``` root.
That is to ensure that these pointers are dropped when they are no longer used,
while not explicitely stating lifetime requirements. The objects live on the
heap anyway.

The internal refcounting uses simple ```Rc```, so at this point notmuch-rs is
inherently single threaded. As the reference where the object is dependent on is
private - e.g. ```Rc<Query>``` is private within ```Message``` -, it might just
work to replace Rc with an ```Arc```. Since this may generate weird segmentation
faults, I have opted for a simple ```Rc``` for now. This keeps the overhead a
little lower and I have no inherent use for it anyway.


## Acknowledgements

notmuch-rs started out from the following projects:
 - https://github.com/Stebalien/notmuch-sys/blob/master/src/lib.rs
 - https://github.com/cmhamill/rust-notmuch

Any contributions are welcome!
