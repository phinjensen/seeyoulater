use serde_json;

use syl_lib::{
    colors::{color, Color},
    commands::{Add, Search, Tags},
    db::Database,
    util::singular_plural,
    web::{get_metadata, Metadata},
};

pub trait Interface {
    fn add(&mut self, args: Add);
    fn find(&self, args: Search);
    fn tags(&self, args: Tags);
}

impl Interface for Database {
    fn add(&mut self, args: Add) {
        match self.add_bookmark(
            &args.url,
            get_metadata(&args.url).unwrap_or(Metadata {
                title: None,
                description: None,
            }),
            &args.tags,
        ) {
            Ok(bookmark) => println!("{}", bookmark),
            Err(e) => eprintln!("Error adding bookmark to database: {:?}", e),
        }
    }

    fn find(&self, args: Search) {
        match self.search_bookmarks(&args.query, &args.tags, args.all_tags) {
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
        match self.get_tags(args.sort_by_count, args.reverse) {
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

pub struct Server {
    pub url: String,
}

impl Server {
    fn request(&self, verb: &str, path: &str, body: Option<&str>) {
        let request = ureq::request(verb, &(self.url.to_string() + path));
        let result;
        if let Some(body) = body {
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
            Err(e) => eprintln!("Error sending bookmark add to server:\n{}", e),
        }
    }
}

impl Interface for Server {
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
}
