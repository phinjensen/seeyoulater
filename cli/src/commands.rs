use std::io::{self, Read, Write};

use serde_json;

use syl_lib::{
    colors::{color, Color},
    commands::{Add, Delete, Interface, Search, Tags},
    db::Database,
    util::singular_plural,
    web::{get_metadata, Metadata},
};

pub struct DatabaseInterface {
    db: Database,
}

impl DatabaseInterface {
    pub fn from(db: Database) -> Self {
        Self { db }
    }
}

impl Interface for DatabaseInterface {
    fn add(&mut self, args: Add) {
        let metadata = if let Some(title) = args.title {
            Metadata {
                title: Some(title),
                description: None,
            }
        } else {
            get_metadata(&args.url).unwrap_or(Metadata {
                title: None,
                description: None,
            })
        };
        match self.db.add_bookmark(&args.url, metadata, &args.tags) {
            Ok(bookmark) => println!("{}", bookmark),
            Err(e) => eprintln!("Error adding bookmark to database: {:?}", e),
        }
    }

    fn find(&self, args: Search) {
        match self
            .db
            .search_bookmarks(&args.query, &args.tags, args.all_tags)
        {
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

    fn tags(&self, args: Tags) {
        match self.db.get_tags(args.sort_by_count, args.reverse) {
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

    fn delete(&self, args: Delete) {
        match self
            .db
            .search_bookmarks(&args.query, &args.tags, args.all_tags)
        {
            Ok(bookmarks) => {
                for (i, bookmark) in bookmarks.iter().enumerate() {
                    if i > 0 {
                        print!("\n");
                    }
                    println!("{bookmark}");
                }
                let mut confirm = String::from("");
                while confirm != "y" && confirm != "n" {
                    confirm = String::from("");
                    print!(
                        "Are you sure you want to delete {} {} (y/N)? ",
                        singular_plural("these", bookmarks.len() as isize),
                        singular_plural("bookmarks", bookmarks.len() as isize)
                    );
                    io::stdout().flush().ok();
                    let stdin = io::stdin();
                    stdin.take(1).read_to_string(&mut confirm).ok();
                    if confirm == "\n" {
                        confirm = String::from("n")
                    }
                }
                if confirm == "y" {
                    match self
                        .db
                        .delete_bookmarks(bookmarks.iter().map(|b| b.id).collect())
                    {
                        Ok(count) => println!("Deleted {count} bookmarks"),
                        Err(e) => eprintln!("Error deleting bookmarks: {:?}", e),
                    }
                } else {
                    println!("No bookmarks deleted.");
                }
            }
            Err(e) => eprintln!("Error searching database: {:?}", e),
        }
    }
}

pub struct ServerInterface {
    url: String,
}

impl ServerInterface {
    pub fn new(url: String) -> Self {
        Self { url }
    }

    fn request(&self, verb: &str, path: &str, body: Option<&str>) {
        let mut request = ureq::request(verb, &(self.url.to_string() + path));
        let result;
        if let Some(body) = body {
            request = request.set("Content-Type", "application/json");
            result = request.send_string(&body);
        } else {
            result = request.call();
        }
        match result {
            Ok(result) => {
                if let Ok(result) = result.into_string() {
                    println!("{}", result);
                }
            }
            Err(e) => eprintln!(
                "Error sending deleting bookmarks on server:\n{}",
                match e {
                    ureq::Error::Status(code, response) => format!(
                        "({code}) {}",
                        response.into_string().unwrap_or("".to_string())
                    ),
                    _ => format!("{}", e),
                }
            ),
        }
    }
}

impl Interface for ServerInterface {
    fn add(&mut self, args: Add) {
        self.request("POST", "/add", Some(&serde_json::to_string(&args).unwrap()));
    }

    fn find(&self, args: Search) {
        self.request(
            "GET",
            &("/search?".to_string() + &serde_qs::to_string(&args).unwrap()),
            None,
        );
    }

    fn tags(&self, args: Tags) {
        self.request(
            "GET",
            &("/tags?".to_string() + &serde_qs::to_string(&args).unwrap()),
            None,
        );
    }

    fn delete(&self, args: Delete) {
        self.request(
            "DELETE",
            &("/search?".to_string() + &serde_qs::to_string(&args).unwrap()),
            None,
        );
    }
}
