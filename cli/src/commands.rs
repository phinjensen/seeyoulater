use crate::{
    db::{self, add_bookmark, search_bookmarks},
    web::{get_metadata, Metadata},
};

// TODO: Automatic database initialization and updating
// Initialization is easy, but subsequent updates will be a little trickier.
// Probably just wanna do numbered migrations and have a meta table in the DB
// with name/value columns to keep track of the latest migration done.
pub fn init() {
    match db::initialize() {
        Ok(_) => println!("Database initialized!"),
        Err(e) => println!("Error initializing database: {}", e),
    }
}

pub fn add(url: &String, tags: &Vec<String>) {
    match add_bookmark(
        url,
        get_metadata(url).unwrap_or(Metadata {
            title: None,
            description: None,
        }),
        tags,
    ) {
        Ok(bookmark) => println!("{}", bookmark),
        Err(e) => println!("Error adding bookmark to database: {:?}", e),
    }
}

pub fn find(query: &String, tags: &Vec<String>) {
    match search_bookmarks(query, tags) {
        Ok(bookmarks) => {
            println!(
                "Found {} {}.",
                bookmarks.len(),
                if bookmarks.len() == 1 {
                    "bookmark"
                } else {
                    "bookmarks"
                }
            );
            for bookmark in bookmarks {
                println!("{}\n", bookmark);
            }
        }
        Err(e) => println!("Error searching database: {:?}", e),
    }
}
