use std::collections::HashMap;
use std::io::BufReader;

use clap::{Parser, Subcommand};
use quick_xml::events::attributes::Attribute;
use quick_xml::events::Event;
use quick_xml::Reader;

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
}

enum TagState {
    Null,
    Reading,
    Text(String),
}

fn u8_to_utf8(slice: &[u8]) -> String {
    String::from_utf8(slice.to_vec()).unwrap_or("".to_string())
}

fn main() -> Result<(), ureq::Error> {
    let args = Args::parse();
    match &args.command {
        Command::Add { url } => {
            let mut title = TagState::Null;
            let mut description = TagState::Null;
            let mut reader =
                Reader::from_reader(BufReader::new(ureq::get(url).call()?.into_reader()));
            reader.check_end_names(false);
            let mut buf = Vec::new();
            loop {
                match reader.read_event(&mut buf) {
                    Ok(Event::Start(ref e)) => match u8_to_utf8(e.name()).as_str() {
                        "title" => title = TagState::Reading,
                        "meta" => {
                            let attributes: HashMap<String, String> = e
                                .html_attributes()
                                .filter_map(|attr| attr.ok())
                                .map(|Attribute { key, value }| {
                                    (u8_to_utf8(key), u8_to_utf8(&value))
                                })
                                .collect();
                            if let Some(name) = attributes.get("name") {
                                if name.as_str() == "description"
                                    || name.as_str() == "og:description"
                                {
                                    description = match attributes.get("content") {
                                        Some(desc) => TagState::Text(desc.to_string()),
                                        _ => TagState::Null,
                                    };
                                }
                            }
                        }
                        &_ => (),
                    },
                    Ok(Event::Text(e)) => {
                        if let TagState::Reading = title {
                            title = TagState::Text(e.unescape_and_decode(&reader).unwrap());
                        }
                        if let TagState::Reading = description {
                            description = TagState::Text(e.unescape_and_decode(&reader).unwrap());
                        }
                    }
                    Err(e) => println!("Error, {}", e),
                    Ok(Event::Eof) => break,
                    _ => (),
                };
                buf.clear();
            }
            if let TagState::Text(t) = title {
                println!("title: {}", t);
            }
            if let TagState::Text(d) = description {
                println!("description: {}", d);
            }
        }
    };
    Ok(())
}
