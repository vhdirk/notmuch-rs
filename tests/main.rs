extern crate notmuch;

use std::rc::Rc;

fn main() {

    let mut mail_path = std::env::home_dir().unwrap();
    mail_path.push(".mail");

    match notmuch::Database::open(&mail_path.to_str().unwrap().to_string(), notmuch::DatabaseMode::ReadOnly){
        Ok(db) => {
            let query = db.create_query(&"".to_string()).unwrap();
            let mut threads = query.search_threads().unwrap();

            // loop {
            //     match threads.next() {
            //         Some(thread) => {
            //             println!("thread {:?} {:?}", thread.subject(), thread.authors());
            //         },
            //         None => { break }
            //     }
            // }

            // println!("refcount{:?}", Rc::strong_count(&db.0));

        },
        Err(err) =>{
            println!("Got error while trying to open db: {:?}", err);
        }
    }

}
