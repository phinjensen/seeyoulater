use std::io::{self, Read, Write};

use serde_json;

use syl_lib::{
    commands::{Add, Delete, Error as CommandError, Interface, Result, Search, Tags},
    config::Server,
    db::{Bookmark, Database, Error as DatabaseError},
    util::singular_plural,
    web::{get_metadata, Metadata},
};

fn wrap_db_err(err: DatabaseError) -> CommandError {
    syl_lib::commands::Error::RusqliteError(err)
}

fn confirm_delete(bookmarks: &Vec<Bookmark>) -> bool {
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
    confirm == "y"
}

pub struct DatabaseInterface {
    db: Database,
}

impl DatabaseInterface {
    pub fn from(db: Database) -> Self {
        Self { db }
    }
}

impl Interface for DatabaseInterface {
    fn add(&mut self, args: Add) -> Result<Bookmark> {
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
        self.db
            .add_bookmark(&args.url, metadata, &args.tags)
            .map_err(wrap_db_err)
    }

    fn find(&self, args: Search) -> Result<Vec<Bookmark>> {
        self.db
            .search_bookmarks(&args.query, &args.tags, args.all_tags)
            .map_err(wrap_db_err)
    }

    fn tags(&self, args: Tags) -> Result<Vec<(String, usize)>> {
        self.db
            .get_tags(args.sort_by_count, args.reverse)
            .map_err(wrap_db_err)
    }

    fn delete(&self, args: Delete) -> Result<usize> {
        let search = self
            .db
            .search_bookmarks(&args.query, &args.tags, args.all_tags);
        if let Ok(bookmarks) = search {
            if confirm_delete(&bookmarks) {
                self.db
                    .delete_bookmarks(bookmarks.iter().map(|b| b.id).collect())
                    .map_err(wrap_db_err)
            } else {
                Ok(0)
            }
        } else {
            search.map(|_| 0).map_err(wrap_db_err)
        }
    }
}

pub struct ServerInterface {
    url: String,
    username: String,
    password: String,
}

impl ServerInterface {
    pub fn new(server: Server) -> Self {
        Self {
            url: server.url,
            username: server.username,
            password: server.password,
        }
    }

    fn request(&self, verb: &str, path: &str, body: Option<&str>) -> Result<String> {
        let mut request = ureq::request(verb, &(self.url.to_string() + path))
            .set("X-Username", &self.username)
            .set("X-Password", &self.password);
        let result;
        if let Some(body) = body {
            request = request.set("Content-Type", "application/json");
            result = request.send_string(&body);
        } else {
            result = request.call();
        }

        let result = result.map_err(CommandError::UreqError)?;

        result.into_string().map_err(CommandError::IOError)
    }
}

impl Interface for ServerInterface {
    fn add(&mut self, args: Add) -> Result<Bookmark> {
        serde_json::from_str(&self.request(
            "POST",
            "/add",
            Some(&serde_json::to_string(&args).unwrap()),
        )?)
        .map_err(|_| CommandError::SerdeError)
    }

    fn find(&self, args: Search) -> Result<Vec<Bookmark>> {
        serde_json::from_str(&self.request(
            "GET",
            &("/search?".to_string() + &serde_qs::to_string(&args).unwrap()),
            None,
        )?)
        .map_err(|_| CommandError::SerdeError)
    }

    fn tags(&self, args: Tags) -> Result<Vec<(String, usize)>> {
        serde_json::from_str(&self.request(
            "GET",
            &("/tags?".to_string() + &serde_qs::to_string(&args).unwrap()),
            None,
        )?)
        .map_err(|_| CommandError::SerdeError)
    }

    fn delete(&self, args: Delete) -> Result<usize> {
        let bookmarks = serde_json::from_str(&self.request(
            "GET",
            &("/search?".to_string() + &serde_qs::to_string(&args).unwrap()),
            None,
        )?)
        .map_err(|_| CommandError::SerdeError)?;
        if confirm_delete(&bookmarks) {
            serde_json::from_str(&self.request(
                "DELETE",
                &("/search?".to_string() + &serde_qs::to_string(&args).unwrap()),
                None,
            )?)
            .map_err(|_| CommandError::SerdeError)
        } else {
            Ok(0)
        }
    }
}
