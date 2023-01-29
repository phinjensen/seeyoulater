use rouille::input::json_input;
use rouille::{try_or_400, Request, Response};
use serde::Serialize;
use syl_lib::commands::{Add, DatabaseInterface, Delete, Interface, Search, Tags};
use urlencoding::decode;

#[derive(Serialize)]
struct Error {
    message: String,
}

pub fn add(interface: &mut DatabaseInterface, request: &Request) -> Response {
    let args: Add = try_or_400!(json_input(request));
    match interface.add(args) {
        Ok(bookmark) => Response::json(&bookmark),
        Err(e) => Response::json(&Error {
            message: format!("Error writing bookmark to database: {e:?}"),
        }),
    }
}

pub fn search(interface: &mut DatabaseInterface, request: &Request) -> Response {
    let args: Search =
        serde_qs::from_str(&decode(request.raw_query_string()).expect("invalid UTF-8!")).unwrap();
    match interface.find(args) {
        Ok(bookmarks) => Response::json(&bookmarks),
        Err(e) => Response::json(&Error {
            message: format!("Error searching bookmarks: {e:?}"),
        }),
    }
}

pub fn delete(interface: &mut DatabaseInterface, request: &Request) -> Response {
    let args: Delete =
        serde_qs::from_str(&decode(request.raw_query_string()).expect("invalid UTF-8!")).unwrap();
    match interface.delete(args) {
        Ok(deleted) => Response::json(&deleted),
        Err(e) => Response::json(&Error {
            message: format!("Error deleting bookmarks: {e:?}"),
        }),
    }
}

pub fn tags(interface: &mut DatabaseInterface, request: &Request) -> Response {
    let args: Tags =
        serde_qs::from_str(&decode(request.raw_query_string()).expect("invalid UTF-8!"))
            .expect("Invalid arguments");
    match interface.tags(args) {
        Ok(tags) => Response::json(&tags),
        Err(e) => Response::json(&Error {
            message: format!("Error searching bookmarks: {e:?}"),
        }),
    }
}
