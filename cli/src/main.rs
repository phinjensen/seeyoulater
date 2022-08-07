use clap::{Parser, Subcommand};

use syl::commands::Interface;
use syl_lib::{config::Config, db::Database};

#[derive(Parser)]
#[clap(author, version, about)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[clap(visible_alias = "a")]
    /// Add a bookmark
    Add {
        /// The URL to bookmark
        #[clap(value_parser)]
        url: String,
        /// Tag(s) to add to this bookmark; use this option multiple times to add multiple tags
        #[clap(short, long, value_parser)]
        tags: Vec<String>,
    },
    #[clap(visible_alias = "get")]
    /// Search bookmarks
    Search {
        /// A word or phrase to match in the URL, title, or description
        #[clap(value_parser)]
        query: Option<String>,
        /// Limit search to tag(s); use this option multiple times to specify multiple tags
        #[clap(short, long = "tag", value_parser)]
        tags: Vec<String>,
        /// Match only bookmarks that contain *all* tags provided with -t (default behavior matches *any* tag provided)
        #[clap(short, long, action)]
        all_tags: bool,
    },
    Tags {
        #[clap(short = 'c', long, action)]
        sort_by_count: bool,
        #[clap(short, long, action)]
        reverse: bool,
    },
    // TODO: Consider what (if any) the "default" command should be, e.g.:
    //      syl -t blog https://phinjensen.com
    // Should this add a bookmark with the tab "blog" or search for bookmarks
    // at https://phinjensen.com with the tag "blog"?
}

fn main() {
    let args = Args::parse();
    let config = Config::new();
    let mut db = Database::open(&config.database()).unwrap();
    match &args.command {
        Command::Add { url, tags } => db.add(url, tags),
        Command::Search {
            query,
            tags,
            all_tags,
        } => db.find(query, tags, *all_tags),
        Command::Tags {
            sort_by_count,
            reverse,
        } => db.tags(*sort_by_count, *reverse),
    };
}
