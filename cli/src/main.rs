use clap::{Parser, Subcommand};

use cli::commands::{add, find, init};

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
    Init,
    Search {
        #[clap(value_parser)]
        query: String,
        #[clap(short, long, value_parser)]
        tags: Vec<String>,
    },
}

fn main() {
    let args = Args::parse();
    match &args.command {
        Command::Init => init(),
        Command::Add { url, tags } => add(url, tags),
        Command::Search { query, tags } => find(query, tags),
    };
}
