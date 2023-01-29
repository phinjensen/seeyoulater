use std::error::Error;
use std::io::BufReader;
use std::{collections::HashMap, str::from_utf8};

use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;

pub struct Metadata {
    pub title: Option<String>,
    pub description: Option<String>,
}

pub fn get_metadata(url: &String) -> Result<Metadata, Box<dyn Error>> {
    let mut result = Metadata {
        title: None,
        description: None,
    };
    let mut current_tag = String::from("");
    let mut reader = Reader::from_reader(BufReader::new(ureq::get(url).call()?.into_reader()));
    reader.check_end_names(false);
    reader.expand_empty_elements(true);
    let mut buf = Vec::new();
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref tag)) => {
                current_tag = from_utf8(tag.name()).unwrap_or("error").to_lowercase();
                if current_tag == "meta" {
                    result.description = parse_meta_description(tag);
                }
            }
            Ok(Event::Text(e)) => {
                if current_tag == "title" {
                    result.title = Some(
                        reader
                            .decode(&e.to_owned().unescaped().unwrap_or(e.into_inner()))
                            .unwrap_or("")
                            .to_string(),
                    );
                }
            }
            Ok(Event::End(_)) => current_tag = String::from(""),
            Ok(Event::Eof) => break,
            _ => (),
        };
        buf.clear();
    }
    Ok(result)
}

fn parse_meta_description(tag: &BytesStart) -> Option<String> {
    // Convert attributes into hashmap, ignoring anything that has errors from the parser or in the
    // utf8 of the key
    let attributes: HashMap<String, String> = tag
        .html_attributes()
        .filter_map(|attr| attr.ok())
        .filter_map(|attr| {
            if let Ok(key) = from_utf8(attr.key) {
                Some((
                    // No need to unescape the key of a meta tag, because we're only
                    // interested in a specific set of possible keys
                    key.to_lowercase(),
                    String::from_utf8_lossy(
                        &attr.to_owned().unescaped_value().unwrap_or(attr.value),
                    )
                    .to_string(),
                ))
            } else {
                None
            }
        })
        .collect();

    // If it's a name=description tag, we can then get the description tag. This could probably be a
    // lot more efficient (if we're only looking for the one content attribute in tag, we don't need
    // to memory for every single attribute) but it works for now.
    if let Some(name) = attributes.get("name") {
        if name.as_str() == "description" || name.as_str() == "og:description" {
            return attributes.get("content").cloned();
        }
    }
    None
}
