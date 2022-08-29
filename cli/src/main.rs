use clap::{Parser, Subcommand};

use syl::commands::{DatabaseInterface, ServerInterface};
use syl_lib::commands::{Add, Delete, Interface, Search, Tags};
use syl_lib::config::Config;
use syl_lib::db::Database;

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
    #[clap(visible_alias = "s")]
    /// Search bookmarks
    Search(Search),
    #[clap(visible_alias = "t")]
    /// View/edit tags
    Tags(Tags),
    #[clap(visible_alias = "d")]
    /// Delete bookmark(s) using the same interface as search
    Delete(Delete),
}

fn main() {
    let args = Args::parse();
    let config = Config::new();
    let mut interface: Box<dyn Interface>;
    if let Some(server) = &args.server {
        interface = Box::new(ServerInterface::new(server.to_string()));
    } else {
        interface = Box::new(DatabaseInterface::from(
            Database::open(&config.database()).unwrap(),
        ));
    }
    match args.command {
        Command::Add(args) => interface.add(args),
        Command::Search(args) => interface.find(args),
        Command::Tags(args) => interface.tags(args),
        Command::Delete(args) => interface.delete(args),
    };
}
