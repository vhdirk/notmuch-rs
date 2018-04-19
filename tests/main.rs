extern crate notmuch;

use std::rc::Rc;

fn main() {

    let mut mail_path = std::env::home_dir().unwrap();
    mail_path.push(".mail");

    match notmuch::Database::open(&mail_path.to_str().unwrap().to_string(), notmuch::DatabaseMode::ReadOnly){
        Ok(db) => {
            let rev = db.revision();
            println!("db revision: {:?}", rev);

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

        },
        Err(err) =>{
            println!("Got error while trying to open db: {:?}", err);
        }
    }

}
