use std::io::{self, Read, Write};

use serde_json;

use syl_lib::{
    commands::{
        Add, Delete, Edit, Error as CommandError, Interface, RenameTag, Result, Search, Tags,
    },
    config::Server,
    db::Bookmark,
    util::singular_plural,
};

pub fn confirm_delete(bookmarks: &Vec<Bookmark>) -> bool {
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
        } else if confirm == "y" {
            eprintln!(
                "confirm is: {:?}, doesn't match? {}",
                confirm,
                confirm != "y"
            );
            break;
        }
    }
    confirm == "y"
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
            "/bookmark",
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

    fn rename_tag(&self, args: RenameTag) -> Result<usize> {
        serde_json::from_str(&self.request(
            "PATCH",
            &format!("/tags?{}", &serde_qs::to_string(&args).unwrap()),
            None,
        )?)
        .map_err(|_| CommandError::SerdeError)
    }

    fn edit(&mut self, args: Edit) -> Result<Bookmark> {
        serde_json::from_str(&self.request(
            "PUT",
            "/bookmark",
            Some(&serde_json::to_string(&args).unwrap()),
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
        eprintln!("in delete");
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
