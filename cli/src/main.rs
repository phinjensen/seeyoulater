use clap::{Parser, Subcommand};

use syl::{
    commands::{add, find},
    db::Database,
};

#[derive(Parser)]
#[clap(author, version, about)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Add {
        #[clap(value_parser)]
        url: String,
        #[clap(short, long, value_parser)]
        tags: Vec<String>,
    },
    Search {
        #[clap(value_parser)]
        query: String,
        #[clap(short, long, value_parser)]
        tags: Vec<String>,
    },
    // TODO: Make a list command (for listing all bookmarks or all tags)
    // TODO: Consider what (if any) the "default" command should be, e.g.:
    //      syl -t blog https://phinjensen.com
    // Should this add a bookmark with the tab "blog" or search for bookmarks
    // at https://phinjensen.com with the tag "blog"?
}

fn main() {
    let args = Args::parse();
    let mut db = Database::open("./seeyoulater.db").unwrap();
    match &args.command {
        Command::Add { url, tags } => add(&mut db, url, tags),
        Command::Search { query, tags } => find(db, query, tags),
    };
}
