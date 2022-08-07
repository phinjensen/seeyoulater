use clap::Args;
use serde::Serialize;

#[derive(Args, Serialize)]
pub struct Add {
    /// The URL to bookmark
    #[clap(value_parser)]
    pub url: String,
    /// Tag(s) to add to this bookmark; use this option multiple times to add multiple tags
    #[clap(short, long, value_parser)]
    pub tags: Vec<String>,
}

#[derive(Args, Serialize)]
pub struct Search {
    /// A word or phrase to match in the URL, title, or description
    #[clap(value_parser)]
    pub query: Option<String>,
    /// Limit search to tag(s); use this option multiple times to specify multiple tags
    #[clap(short, long = "tag", value_parser)]
    pub tags: Vec<String>,
    /// Match only bookmarks that contain *all* tags provided with -t (default behavior matches *any* tag provided)
    #[clap(short, long, action)]
    pub all_tags: bool,
}

#[derive(Args, Serialize)]
pub struct Tags {
    #[clap(short = 'c', long, action)]
    pub sort_by_count: bool,
    #[clap(short, long, action)]
    pub reverse: bool,
}
