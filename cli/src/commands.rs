use crate::{
    db::Database,
    web::{get_metadata, Metadata},
};

pub fn add(db: &mut Database, url: &String, tags: &Vec<String>) {
    match db.add_bookmark(
        url,
        get_metadata(url).unwrap_or(Metadata {
            title: None,
            description: None,
        }),
        tags,
    ) {
        Ok(bookmark) => println!("{}", bookmark),
        Err(e) => eprintln!("Error adding bookmark to database: {:?}", e),
    }
}

pub fn find(db: Database, query: &Option<String>, tags: &Vec<String>, all_tags: bool) {
    match db.search_bookmarks(query, tags, all_tags) {
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
            for (i, bookmark) in bookmarks.iter().enumerate() {
                if i > 0 {
                    print!("\n");
                }
                println!("{bookmark}");
            }
        }
        Err(e) => eprintln!("Error searching database: {:?}", e),
    }
}
