use syl_lib::{
    colors::{color, Color},
    db::Database,
    util::singular_plural,
    web::{get_metadata, Metadata},
};

pub trait Interface {
    fn add(&mut self, url: &String, tags: &Vec<String>);
    fn find(&self, query: &Option<String>, tags: &Vec<String>, all_tags: bool);
    fn tags(&self, sort_by_count: bool, reverse: bool);
}

impl Interface for Database {
    fn add(&mut self, url: &String, tags: &Vec<String>) {
        match self.add_bookmark(
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

    fn find(&self, query: &Option<String>, tags: &Vec<String>, all_tags: bool) {
        match self.search_bookmarks(query, tags, all_tags) {
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

    fn tags(&self, sort_by_count: bool, reverse: bool) {
        match self.get_tags(sort_by_count, reverse) {
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
                            "{:longest$} ({} {})",
                            color(&tag, Color::Yellow),
                            count,
                            singular_plural("bookmarks", count as isize)
                        );
                    }
                }
            }
            Err(e) => eprintln!("Error finding tags: {:?}", e),
        }
    }
}
