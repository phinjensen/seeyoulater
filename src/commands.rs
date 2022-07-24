use std::io::BufReader;
use std::{collections::HashMap, str::from_utf8};

use quick_xml::events::attributes::Attribute;
use quick_xml::events::Event;
use quick_xml::Reader;

use crate::db::{self, add_bookmark, Bookmark};

pub fn add(url: &String) -> Result<(), ureq::Error> {
    let mut title = None;
    let mut description: Option<String> = None;
    let mut current_tag = String::from("");
    let mut reader = Reader::from_reader(BufReader::new(ureq::get(url).call()?.into_reader()));
    reader.check_end_names(false);
    let mut buf = Vec::new();
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                current_tag = from_utf8(e.name()).unwrap().to_lowercase();
                if current_tag == "meta" {
                    let attributes: HashMap<String, String> = e
                        .html_attributes()
                        .filter_map(|attr| attr.ok())
                        .map(|Attribute { key, value }| {
                            (
                                from_utf8(key).unwrap().to_lowercase(),
                                from_utf8(&value).unwrap().to_string(),
                            )
                        })
                        .collect();
                    if let Some(name) = attributes.get("name") {
                        if name.as_str() == "description" || name.as_str() == "og:description" {
                            description = attributes.get("content").cloned();
                        }
                    }
                }
            }
            Ok(Event::Text(e)) => {
                if current_tag == "title" {
                    title = Some(e.unescape_and_decode(&reader).unwrap());
                }
            }
            Ok(Event::End(_)) => current_tag = String::from(""),
            Err(e) => println!("Error, {}", e),
            Ok(Event::Eof) => break,
            _ => (),
        };
        buf.clear();
    }
    match add_bookmark(url.to_string(), &title, &description) {
        Ok(Bookmark {
            title,
            url,
            description,
            ..
        }) => {
            if let Some(title) = title {
                println!("{}", title)
            };
            println!("{}", url);
            if let Some(description) = description {
                println!("{}", description)
            };
        }
        Err(e) => println!("Error adding bookmark to database: {:?}", e),
    }
    Ok(())
}

pub fn init() {
    match db::initialize() {
        Ok(_) => println!("Database initialized!"),
        Err(e) => println!("Error initializing database: {}", e),
    }
}
