use clap::{Parser, Subcommand};

use syl::commands::{confirm_delete, ServerInterface};
use syl_lib::colors::{color, Color};
use syl_lib::commands::{Add, DatabaseInterface, Delete, Interface, Search, Tags};
use syl_lib::config::{Config, ConfigPath};
use syl_lib::db::Database;
use syl_lib::util::singular_plural;
use syl_lib::web::WebClient;

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
    let config = Config::open(ConfigPath::ClientDefault);
    let mut interface: Box<dyn Interface>;
    if let Some(server) = config.server {
        interface = Box::new(ServerInterface::new(server));
    } else {
        interface = Box::new(DatabaseInterface::from(
            Database::open(&config.database()).unwrap(),
            WebClient::new(config.timeout),
        ));
    }
    match args.command {
        Command::Add(args) => match interface.add(args) {
            Ok(bookmark) => println!("{}", bookmark),
            Err(e) => eprintln!("Error adding bookmark to database: {:?}", e),
        },
        Command::Search(args) => match interface.find(args) {
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
        },
        Command::Tags(args) => match interface.tags(args) {
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
        },
        Command::Delete(args) => {
            let search_args = Search {
                query: args.query.clone(),
                tags: args.tags.clone(),
                all_tags: args.all_tags,
            };
            match interface.find(search_args) {
                Ok(bookmarks) => {
                    if confirm_delete(&bookmarks) {
                        match interface.delete(args) {
                            Ok(0) => println!("No bookmarks deleted."),
                            Ok(count) => println!("Deleted {count} bookmarks"),
                            Err(e) => eprintln!("Error deleting bookmarks: {:?}", e),
                        }
                    }
                }
                Err(e) => println!("Error searching bookmarks: {e:?}"),
            }
        }
    };
}
