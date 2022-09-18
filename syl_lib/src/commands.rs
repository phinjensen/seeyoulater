use std::{io, result, usize};

use clap::Args;
use serde::{Deserialize, Serialize};

use crate::db::Bookmark;

#[derive(Args, Serialize, Deserialize)]
pub struct Add {
    /// The URL to bookmark
    #[clap(value_parser)]
    pub url: String,
    /// Tag(s) to add to this bookmark; use this option multiple times to add multiple tags
    #[clap(short, long, value_parser)]
    #[serde(default)]
    pub tags: Vec<String>,
    /// Title for this bookmark; automatically fetched if not set
    #[clap(short = 'T', long, value_parser)]
    pub title: Option<String>,
}

#[derive(Args, Serialize, Deserialize)]
pub struct Search {
    /// A word or phrase to match in the URL, title, or description
    #[clap(value_parser)]
    pub query: Option<String>,
    /// Limit search to tag(s); use this option multiple times to specify multiple tags
    #[clap(short, long = "tag", value_parser)]
    #[serde(default)]
    pub tags: Vec<String>,
    /// Match only bookmarks that contain *all* tags provided with -t (default behavior matches *any* tag provided)
    #[clap(short, long, action)]
    pub all_tags: bool,
}

#[derive(Args, Serialize, Deserialize)]
pub struct Tags {
    #[clap(short = 'c', long, action)]
    pub sort_by_count: bool,
    #[clap(short, long, action)]
    pub reverse: bool,
}

// TODO: Figure out if there's a better way to keep this in sync with the Search API
#[derive(Args, Serialize, Deserialize)]
pub struct Delete {
    /// A word or phrase to match in the URL, title, or description
    #[clap(value_parser)]
    pub query: Option<String>,
    /// Limit search to tag(s); use this option multiple times to specify multiple tags
    #[clap(short, long = "tag", value_parser)]
    pub tags: Vec<String>,
    /// Match only bookmarks that contain *all* tags provided with -t (default behavior matches *any* tag provided)
    #[clap(short, long, action)]
    pub all_tags: bool,
    /// Force deletion (i.e. don't show a confirmation prompt)
    #[clap(short, action)]
    pub force: bool,
}

#[derive(Debug)]
pub enum Error {
    RusqliteError(rusqlite::Error),
    UreqError(ureq::Error),
    SerdeError,
    IOError(io::Error),
}

pub type Result<T, E = Error> = result::Result<T, E>;

pub trait Interface {
    fn add(&mut self, args: Add) -> Result<Bookmark>;
    fn find(&self, args: Search) -> Result<Vec<Bookmark>>;
    fn tags(&self, args: Tags) -> Result<Vec<(String, usize)>>;
    fn delete(&self, args: Delete) -> Result<usize>;
}
