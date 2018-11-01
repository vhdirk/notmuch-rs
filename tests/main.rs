extern crate notmuch;
extern crate dirs;

use notmuch::StreamingIterator;

fn main() {

    let mut mail_path = dirs::home_dir().unwrap();
    mail_path.push(".mail");

    match notmuch::Database::open(&mail_path.to_str().unwrap().to_string(), notmuch::DatabaseMode::ReadOnly){
        Ok(db) => {
            
            #[cfg(feature = "v0_21")]
            {
                let rev = db.revision();
                println!("db revision: {:?}", rev);
            }
            
            let query = db.create_query(&"".to_string()).unwrap();
            let mut threads = query.search_threads().unwrap();

            while let Some(thread) = threads.next() {
                println!("thread {:?} {:?}", thread.subject(), thread.authors());
            }


        },
        Err(err) =>{
            println!("Got error while trying to open db: {:?}", err);
        }
    }
}
