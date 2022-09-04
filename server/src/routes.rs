use rouille::input::json_input;
use rouille::{try_or_400, Request, Response};
use serde::Serialize;
use syl_lib::commands::{Add, Delete, Search, Tags};
use syl_lib::db::Database;
use syl_lib::web::{get_metadata, Metadata};

#[derive(Serialize)]
struct Error {
    message: String,
}

pub fn add(db: &mut Database, request: &Request) -> Response {
    let args: Add = try_or_400!(json_input(request));
    match db.add_bookmark(
        &args.url,
        get_metadata(&args.url).unwrap_or(Metadata {
            title: None,
            description: None,
        }),
        &args.tags,
    ) {
        Ok(bookmark) => Response::json(&bookmark),
        Err(e) => Response::json(&Error {
            message: format!("Error writing bookmark to database: {e}"),
        }),
    }
}

pub fn search(db: &mut Database, request: &Request) -> Response {
    let args: Search = serde_qs::from_str(request.raw_query_string()).unwrap();
    match db.search_bookmarks(&args.query, &args.tags, args.all_tags) {
        Ok(bookmarks) => Response::json(&bookmarks),
        Err(e) => Response::json(&Error {
            message: format!("Error searching bookmarks: {e}"),
        }),
    }
}

pub fn tags(db: &mut Database, request: &Request) -> Response {
    let args: Tags = serde_qs::from_str(request.raw_query_string()).unwrap();
    match db.get_tags(args.sort_by_count, args.reverse) {
        Ok(tags) => Response::json(&tags),
        Err(e) => Response::json(&Error {
            message: format!("Error searching bookmarks: {e}"),
        }),
    }
}
