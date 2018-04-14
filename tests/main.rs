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
