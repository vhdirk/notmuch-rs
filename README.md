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
not thread safe.  
So why do all structs implement ```Send``` and ```Sync```? Well, it _is_ safe to
access pointers from different threads (as long as you know what you are doing :) ).
Up till now I haven't done a lot of multithreaded stuff with notmuch-rs. If you
feel this is too permissive, let me know.

## Lifetime

All structs are strictly linked together with their lifetime. The root of the
tree is ```Database```, which has a lifetime that must outlive any child
objects, for instance ```Query```. The ```Threads``` iterator that you can get
from a ```Query``` is always outlived by the parent query. The ```Threads```
does not own any individual ```Thread```. These are bound to the owner of
the ```Threads``` iterator itself. Each structure keeps a ```PhantomCow```
marker for its owner.

Typically, using a lifetimes structure like this in an application poses
significant difficulties in satisfying these lifetime requirements. While other
libraries force the application developers towards crates like ```owningref```
or ```rental``` to get around this, ```notmuch-rs``` makes use of the
excellent [Supercow](https://crates.io/crates/supercow), to alleviate this.

This way, you get to choose your own container type, and even keep the parent
object alive so you don't have to juggle lifetimes. To use this, most types
are accompagnied with an ```*Ext``` trait, that accepts ```Rc```, ```Arc``` or
comparable.

```rust
    use std::sync::Arc;
    use notmuch::{DatabaseExt};

    let query = {
        let dbr = Arc::new(db);

        <Database as DatabaseExt>::create_query(dbr.clone(), &"".to_string()).unwrap()
    };

```

## Acknowledgements

notmuch-rs started out from the following projects:
 - https://github.com/Stebalien/notmuch-sys/blob/master/src/lib.rs
 - https://github.com/cmhamill/rust-notmuch

Any contributions are welcome!
