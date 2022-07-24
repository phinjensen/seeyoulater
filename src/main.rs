use clap::{Parser, Subcommand};

use cli::commands::{add, init};

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
    },
    Init,
}

fn main() -> Result<(), ureq::Error> {
    let args = Args::parse();
    match &args.command {
        Command::Init => init(),
        Command::Add { url } => add(url)?,
    };
    Ok(())
}
