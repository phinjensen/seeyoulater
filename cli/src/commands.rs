use crate::{
    db::Database,
    util::singular_plural,
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
                singular_plural("bookmarks", bookmarks.len() as isize)
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

pub fn tags(db: Database, sort_by_count: bool, reverse: bool) {
    match db.get_tags(sort_by_count, reverse) {
        Ok(tags) => {
            println!(
                "Found {} {}.",
                tags.len(),
                singular_plural("tags", tags.len() as isize)
            );
            if tags.len() > 0 {
                let longest = tags.iter().map(|t| t.0.len()).max().unwrap();
                for (tag, count) in tags {
                    println!(
                        "\x1b[33m{:longest$}\x1b[m ({} {})",
                        tag,
                        count,
                        singular_plural("bookmarks", count as isize)
                    );
                }
            }
        }
        Err(e) => eprintln!("Error finding tags: {:?}", e),
    }
}
