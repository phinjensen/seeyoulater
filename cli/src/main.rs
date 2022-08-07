use clap::{Parser, Subcommand};

use syl::commands::{Interface, Server};
use syl_lib::commands::{Add, Search, Tags};
use syl_lib::{config::Config, db::Database};

#[derive(Parser)]
#[clap(author, version, about)]
struct Args {
    #[clap(subcommand)]
    command: Command,
    #[clap(short, long, value_parser)]
    server: Option<String>,
}

#[derive(Subcommand)]
enum Command {
    #[clap(visible_alias = "a")]
    /// Add a bookmark
    Add(Add),
    #[clap(visible_alias = "get")]
    /// Search bookmarks
    Search(Search),
    Tags(Tags),
    // TODO: Consider what (if any) the "default" command should be, e.g.:
    //      syl -t blog https://phinjensen.com
    // Should this add a bookmark with the tab "blog" or search for bookmarks
    // at https://phinjensen.com with the tag "blog"?
}

fn main() {
    let args = Args::parse();
    let config = Config::new();
    let mut interface: Box<dyn Interface>;
    if let Some(server) = &args.server {
        interface = Box::new(Server {
            url: server.to_string(),
        });
    } else {
        interface = Box::new(Database::open(&config.database()).unwrap());
    }
    match args.command {
        Command::Add(args) => interface.add(args),
        Command::Search(args) => interface.find(args),
        Command::Tags(args) => interface.tags(args),
    };
}
