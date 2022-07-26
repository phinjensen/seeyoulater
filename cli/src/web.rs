use std::io::BufReader;
use std::{collections::HashMap, str::from_utf8};

use quick_xml::events::attributes::Attribute;
use quick_xml::events::Event;
use quick_xml::Reader;

pub struct Metadata {
    pub title: Option<String>,
    pub description: Option<String>,
}

// TODO: Handle HTML entities (I'm seeing &gt; and &lt; and such)
pub fn get_metadata(url: &String) -> Result<Metadata, ureq::Error> {
    let mut result = Metadata {
        title: None,
        description: None,
    };
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
                            result.description = attributes.get("content").cloned();
                        }
                    }
                }
            }
            Ok(Event::Text(e)) => {
                if current_tag == "title" {
                    result.title = Some(e.unescape_and_decode(&reader).unwrap());
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
