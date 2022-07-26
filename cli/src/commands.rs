use crate::{
    db::{self, add_bookmark, search_bookmarks},
    web::{get_metadata, Metadata},
};

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
