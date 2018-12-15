extern crate dirs;
extern crate notmuch;

use std::sync::Arc;

use notmuch::{Query, QueryExt, Threads};

fn main() {
    let mut mail_path = dirs::home_dir().unwrap();
    mail_path.push(".mail");

    match notmuch::Database::open(
        &mail_path.to_str().unwrap().to_string(),
        notmuch::DatabaseMode::ReadOnly,
    ) {
        Ok(db) => {
            #[cfg(feature = "v0_21")]
            {
                let rev = db.revision();
                println!("db revision: {:?}", rev);
            }
            let query = {
                let dbr = Arc::new(db);

                notmuch::Query::create(dbr.clone(), &"".to_string()).unwrap()
            };

            // let mut threads = query.search_threads().unwrap();

            // let mut threads = db.create_query(&"".to_string()).unwrap().search_threads().unwrap();

            let mut threads = Arc::new(<Query as QueryExt>::search_threads(query).unwrap());

            for thread in Arc::get_mut(&mut threads).unwrap()
            {
                println!("thread {:?} {:?}", thread.subject(), thread.authors());
            }
        }
        Err(err) => {
            println!("Got error while trying to open db: {:?}", err);
        }
    }
}
